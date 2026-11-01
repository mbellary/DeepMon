use crate::{
    PrototypeError, Result,
    fixture::{EndpointInput, MAX_BYTES},
    report::ScenarioReport,
};
use aether_validation::sha256;
use serde::Serialize;
use std::{
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};
const HEADER_LIMIT: usize = 8192;
const IO_LIMIT: Duration = Duration::from_secs(1);
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub bind: String,
    pub port: u16,
    pub max_requests: usize,
    pub max_run: Duration,
}
impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind: "127.0.0.1".into(),
            port: 0,
            max_requests: 1,
            max_run: Duration::from_secs(5),
        }
    }
}
impl ServerConfig {
    pub fn validate(&self) -> Result<()> {
        if self.bind != "127.0.0.1" {
            return Err(PrototypeError::new(
                "invalid_bind",
                "only numeric 127.0.0.1 is supported",
            ));
        }
        if !(1..=16).contains(&self.max_requests)
            || self.max_run.is_zero()
            || self.max_run > Duration::from_secs(30)
        {
            return Err(PrototypeError::new(
                "invalid_limit",
                "requests 1..16; run duration >0 and <=30s",
            ));
        }
        Ok(())
    }
}
#[derive(Debug, Serialize)]
pub struct ServerOutcome {
    pub accepted_connections: usize,
    pub rejected_requests: usize,
    pub peer_errors: Vec<String>,
    pub termination: &'static str,
}
pub struct FixtureServer {
    addr: SocketAddr,
    stop: Arc<AtomicBool>,
    worker: Option<JoinHandle<Result<ServerOutcome>>>,
}
impl FixtureServer {
    pub fn bound_addr(&self) -> SocketAddr {
        self.addr
    }
    pub fn wait(mut self) -> Result<ServerOutcome> {
        self.join()
    }
    pub fn shutdown_and_join(mut self) -> Result<ServerOutcome> {
        self.stop.store(true, Ordering::Release);
        self.join()
    }
    fn join(&mut self) -> Result<ServerOutcome> {
        self.worker
            .take()
            .ok_or_else(|| PrototypeError::new("server_join", "already joined"))?
            .join()
            .map_err(|_| PrototypeError::new("server_panic", "worker panicked"))?
    }
}
impl Drop for FixtureServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
fn io_timeout(stream: &TcpStream, deadline: Instant) -> Result<()> {
    let left = deadline.saturating_duration_since(Instant::now());
    if left.is_zero() {
        return Err(PrototypeError::new(
            "deadline_exceeded",
            "HTTP I/O deadline",
        ));
    }
    stream.set_read_timeout(Some(left.min(Duration::from_millis(100))))?;
    stream.set_write_timeout(Some(left.min(IO_LIMIT)))?;
    Ok(())
}
fn read_header(stream: &mut TcpStream, deadline: Instant, stop: &AtomicBool) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    loop {
        if stop.load(Ordering::Acquire) {
            return Err(PrototypeError::new("cancelled", "server shutdown"));
        }
        io_timeout(stream, deadline)?;
        let mut one = [0];
        match stream.read(&mut one) {
            Ok(0) => return Err(PrototypeError::new("http_invalid", "truncated header")),
            Ok(_) => bytes.push(one[0]),
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::TimedOut
                        | std::io::ErrorKind::WouldBlock
                        | std::io::ErrorKind::Interrupted
                ) =>
            {
                continue;
            }
            Err(e) => return Err(e.into()),
        }
        if bytes.len() > HEADER_LIMIT {
            return Err(PrototypeError::new("header_limit", "headers exceed 8KiB"));
        }
        if bytes.ends_with(b"\r\n\r\n") {
            return Ok(bytes);
        }
    }
}
fn header_lines(bytes: &[u8]) -> Result<(&str, Vec<(&str, &str)>)> {
    let text = std::str::from_utf8(bytes).map_err(|e| PrototypeError::new("http_invalid", e))?;
    let mut lines = text
        .strip_suffix("\r\n\r\n")
        .ok_or_else(|| PrototypeError::new("http_invalid", "missing terminator"))?
        .split("\r\n");
    let first = lines.next().unwrap_or("");
    if first.bytes().any(|b| b < 32 || b == 127) {
        return Err(PrototypeError::new("http_invalid", "invalid start line"));
    }
    let mut headers = Vec::new();
    for line in lines {
        let (name, value) = line
            .split_once(':')
            .ok_or_else(|| PrototypeError::new("http_invalid", "malformed header"))?;
        if name.is_empty()
            || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
            || value.bytes().any(|b| b < 32 || b == 127)
        {
            return Err(PrototypeError::new("http_invalid", "invalid header bytes"));
        }
        if headers
            .iter()
            .any(|(n, _): &(&str, &str)| n.eq_ignore_ascii_case(name))
        {
            return Err(PrototypeError::new("http_invalid", "duplicate header"));
        }
        headers.push((name, value.trim()));
    }
    Ok((first, headers))
}
fn request_status(bytes: &[u8]) -> Result<u16> {
    let (first, headers) = header_lines(bytes)?;
    let fields: Vec<_> = first.split(' ').collect();
    if fields.len() != 3
        || fields.iter().any(|s| s.is_empty())
        || fields[2] != "HTTP/1.1"
        || !fields[1].starts_with('/')
    {
        return Err(PrototypeError::new("http_invalid", "invalid request line"));
    }
    if !headers
        .iter()
        .any(|(n, v)| n.eq_ignore_ascii_case("host") && !v.is_empty())
    {
        return Err(PrototypeError::new("http_invalid", "Host required"));
    }
    for (n, v) in headers {
        if n.eq_ignore_ascii_case("transfer-encoding")
            || n.eq_ignore_ascii_case("upgrade")
            || (n.eq_ignore_ascii_case("content-length") && v != "0")
            || (n.eq_ignore_ascii_case("connection") && !v.eq_ignore_ascii_case("close"))
        {
            return Err(PrototypeError::new(
                "http_invalid",
                "unsupported framing or connection",
            ));
        }
    }
    Ok(if fields[0] != "GET" {
        405
    } else if fields[1] != "/metrics" {
        404
    } else {
        200
    })
}
fn serve_peer(mut stream: TcpStream, input: &[u8], end: Instant, stop: &AtomicBool) -> Result<u16> {
    let deadline = end.min(Instant::now() + IO_LIMIT);
    let status = match read_header(&mut stream, deadline, stop).and_then(|b| request_status(&b)) {
        Ok(code) => code,
        Err(e) if e.code == "header_limit" => 431,
        Err(e) if e.code == "http_invalid" => 400,
        Err(e) => return Err(e),
    };
    let body = if status == 200 {
        input
    } else {
        b"fixture request rejected\n"
    };
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "Request Header Fields Too Large",
    };
    let header = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: text/plain; version=0.0.4\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    write_until(&mut stream, header.as_bytes(), deadline, stop)?;
    write_until(&mut stream, body, deadline, stop)?;
    // A rejected body can already be buffered by the kernel. Flush our FIN
    // before discarding a bounded amount of pending input to avoid losing the
    // rejection response to an immediate TCP reset on ordinary small requests.
    stream.shutdown(std::net::Shutdown::Write)?;
    if status != 200 {
        stream.set_nonblocking(true)?;
        let mut discard = [0; HEADER_LIMIT];
        let _ = stream.read(&mut discard);
    }
    Ok(status)
}
fn write_until(
    stream: &mut TcpStream,
    mut bytes: &[u8],
    deadline: Instant,
    stop: &AtomicBool,
) -> Result<()> {
    while !bytes.is_empty() {
        if stop.load(Ordering::Acquire) {
            return Err(PrototypeError::new("cancelled", "server shutdown"));
        }
        io_timeout(stream, deadline)?;
        match stream.write(bytes) {
            Ok(0) => {
                return Err(PrototypeError::new(
                    "http_invalid",
                    "closed response writer",
                ));
            }
            Ok(n) => bytes = &bytes[n..],
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::TimedOut
                        | std::io::ErrorKind::WouldBlock
                        | std::io::ErrorKind::Interrupted
                ) => {}
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
pub fn spawn_fixture_server(input: EndpointInput, config: ServerConfig) -> Result<FixtureServer> {
    config.validate()?;
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, config.port))?;
    listener.set_nonblocking(true)?;
    let addr = listener.local_addr()?;
    let stop = Arc::new(AtomicBool::new(false));
    let worker_stop = stop.clone();
    let worker = thread::spawn(move || {
        let end = Instant::now() + config.max_run;
        let mut result = ServerOutcome {
            accepted_connections: 0,
            rejected_requests: 0,
            peer_errors: Vec::new(),
            termination: "completed",
        };
        while result.accepted_connections < config.max_requests {
            if worker_stop.load(Ordering::Acquire) {
                result.termination = "cancelled";
                return Ok(result);
            }
            if Instant::now() >= end {
                return Err(PrototypeError::new(
                    "deadline_exceeded",
                    format!(
                        "received {}/{} connections",
                        result.accepted_connections, config.max_requests
                    ),
                ));
            }
            match listener.accept() {
                Ok((stream, _)) => {
                    result.accepted_connections += 1;
                    match serve_peer(stream, &input.bytes, end, &worker_stop) {
                        Ok(200) => (),
                        Ok(status) => {
                            result.rejected_requests += 1;
                            result.peer_errors.push(format!("HTTP {status}"));
                        }
                        Err(e) => {
                            result.rejected_requests += 1;
                            result.peer_errors.push(e.to_string());
                        }
                    }
                    if Instant::now() >= end {
                        return Err(PrototypeError::new(
                            "deadline_exceeded",
                            "run expired during peer I/O",
                        ));
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(2))
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => (),
                Err(e) => return Err(e.into()),
            }
        }
        Ok(result)
    });
    Ok(FixtureServer {
        addr,
        stop,
        worker: Some(worker),
    })
}
fn read_response(stream: &mut TcpStream) -> Result<Vec<u8>> {
    let deadline = Instant::now() + IO_LIMIT;
    let raw = read_header(stream, deadline, &AtomicBool::new(false))?;
    let (first, headers) = header_lines(&raw)?;
    if first != "HTTP/1.1 200 OK" {
        return Err(PrototypeError::new("http_status", first));
    }
    let get = |name: &str| {
        headers
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, v)| *v)
    };
    if get("connection").is_none_or(|v| !v.eq_ignore_ascii_case("close"))
        || get("transfer-encoding").is_some()
        || get("upgrade").is_some()
    {
        return Err(PrototypeError::new(
            "http_framing",
            "requires Content-Length and close; no upgrades/chunking",
        ));
    }
    let size = get("content-length")
        .filter(|s| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()))
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|n| *n <= MAX_BYTES as usize)
        .ok_or_else(|| PrototypeError::new("http_framing", "invalid/oversized Content-Length"))?;
    let mut body = Vec::with_capacity(size);
    let mut block = [0; 4096];
    loop {
        io_timeout(stream, deadline)?;
        match stream.read(&mut block) {
            Ok(0) => break,
            Ok(n) => {
                if body.len() + n > size {
                    return Err(PrototypeError::new(
                        "http_framing",
                        "body exceeds declared length",
                    ));
                }
                body.extend_from_slice(&block[..n]);
            }
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::TimedOut
                        | std::io::ErrorKind::WouldBlock
                        | std::io::ErrorKind::Interrupted
                ) =>
            {
                continue;
            }
            Err(e) => return Err(e.into()),
        }
    }
    if body.len() != size {
        return Err(PrototypeError::new(
            "http_framing",
            "truncated response body",
        ));
    }
    Ok(body)
}
pub fn fetch_fixture(server: &FixtureServer) -> Result<Vec<u8>> {
    let mut stream = TcpStream::connect_timeout(&server.addr, IO_LIMIT)?;
    stream.set_write_timeout(Some(IO_LIMIT))?;
    write!(
        stream,
        "GET /metrics HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        server.addr
    )?;
    read_response(&mut stream)
}
pub fn run_endpoint_roundtrip(input: EndpointInput) -> Result<ScenarioReport> {
    let server = spawn_fixture_server(input.clone(), ServerConfig::default())?;
    let bytes = match fetch_fixture(&server) {
        Ok(b) => b,
        Err(e) => {
            let _ = server.shutdown_and_join();
            return Err(e);
        }
    };
    let outcome = server.wait()?;
    if outcome.rejected_requests != 0 || outcome.termination != "completed" {
        return Err(PrototypeError::new(
            "server_failed",
            "roundtrip peer failed",
        ));
    }
    let hash = sha256(&bytes);
    if bytes != input.bytes || hash != input.expected_hash {
        return Err(PrototypeError::new(
            "transport_mismatch",
            "received bytes differ from indexed input",
        ));
    }
    Ok(ScenarioReport {
        schema_version: 1,
        synthetic: true,
        scenario: "endpoint-roundtrip",
        variant_id: input.variant_id,
        bundle_id: input.bundle_id,
        manifest_sha256: input.manifest_sha256,
        expected_bytes: input.bytes.len(),
        actual_bytes: bytes.len(),
        expected_sha256: input.expected_hash,
        actual_sha256: hash,
        transport_status: "passed",
        harness_status: "passed",
        semantic_execution_status: "not_run",
        runtime_acceptance_readiness: false,
        production_support_claimed: false,
    })
}

/// Parse received bytes independently of construction/oracle checking.
pub fn parse_metric_fixture(
    text: &str,
    source: &crate::observation::SourceContext,
) -> Result<Vec<crate::observation::NativeObservation>> {
    crate::metric_text::parse(text, source)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn response(bytes: &'static [u8]) -> Result<Vec<u8>> {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let worker = thread::spawn(move || {
            let (mut peer, _) = listener.accept().unwrap();
            peer.set_write_timeout(Some(IO_LIMIT)).unwrap();
            let _ = peer.write_all(bytes);
        });
        let mut stream = TcpStream::connect_timeout(&addr, IO_LIMIT).unwrap();
        let result = read_response(&mut stream);
        drop(stream);
        worker.join().unwrap();
        result
    }
    #[test]
    fn client_accepts_exact_framing() {
        assert_eq!(
            response(b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\nConnection: close\r\n\r\nabc")
                .unwrap(),
            b"abc"
        );
    }
    #[test]
    fn client_rejects_truncated_or_excess_body() {
        for b in [
            b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\nConnection: close\r\n\r\na".as_slice(),
            b"HTTP/1.1 200 OK\r\nContent-Length: 1\r\nConnection: close\r\n\r\nabc",
        ] {
            assert_eq!(response(b).unwrap_err().code, "http_framing");
        }
    }
    #[test]
    fn client_rejects_redirect_status_and_malformed_headers() {
        for b in [b"HTTP/1.1 302 Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".as_slice(),b"nonsense\r\n\r\n", b"HTTP/1.1 200 OK\r\nbad\r\n\r\n",b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\ncontent-length: 0\r\nConnection: close\r\n\r\n"] { assert!(response(b).is_err()); }
    }
    #[test]
    fn client_rejects_unsupported_or_oversized_framing() {
        for b in [b"HTTP/1.1 200 OK\r\nContent-Length: 1048577\r\nConnection: close\r\n\r\n".as_slice(),b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nContent-Length: 0\r\nConnection: close\r\n\r\n", b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: keep-alive\r\n\r\n",b"HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n"] { assert_eq!(response(b).unwrap_err().code,"http_framing"); }
    }
}
