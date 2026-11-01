use aether_poc::{
    config::{MANIFEST, parse_args},
    endpoint::{self, ServerConfig},
    fixture::{self, EndpointInput},
    report::FixtureListing,
};
use serde_json::Value;
use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn bundle() -> fixture::ValidatedBundle {
    fixture::load_bundle(&root(), Path::new(MANIFEST)).unwrap()
}
fn input(id: &str) -> EndpointInput {
    fixture::select_endpoint(&bundle(), id).unwrap()
}
fn tree() -> tempfile::TempDir {
    let t = tempfile::tempdir().unwrap();
    let source = root();
    let m: Value =
        serde_json::from_slice(&fs::read(source.join("evidence/manifest-v1.json")).unwrap())
            .unwrap();
    let mut files: Vec<String> = m["artifacts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["path"].as_str().unwrap().into())
        .collect();
    files.extend([
        "evidence/manifest-v1.json".into(),
        "evidence/manifest-v1.sha256".into(),
    ]);
    for file in files {
        let target = t.path().join(&file);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::copy(source.join(file), target).unwrap();
    }
    aether_validation::generate_records(t.path()).unwrap();
    aether_validation::author_fixtures(t.path()).unwrap();
    t
}
fn raw_request(server: &endpoint::FixtureServer, bytes: &[u8]) -> Vec<u8> {
    let mut s = TcpStream::connect_timeout(&server.bound_addr(), Duration::from_secs(1)).unwrap();
    s.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
    s.set_write_timeout(Some(Duration::from_secs(1))).unwrap();
    s.write_all(bytes).unwrap();
    let mut body = Vec::new();
    s.read_to_end(&mut body).unwrap();
    body
}
fn assert_released(addr: std::net::SocketAddr) {
    assert!(
        TcpListener::bind(addr).is_ok(),
        "listener still owned: {addr}"
    );
}
fn cli(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_aether-poc"))
        .current_dir(root())
        .args(args)
        .output()
        .unwrap()
}
#[test]
fn lists_complete_sorted_bundle_and_preserves_provenance() {
    let b = bundle();
    assert_eq!(b.counts, [12, 36, 82, 2]);
    assert!(
        b.manifest
            .fixtures
            .windows(2)
            .all(|w| w[0].fixture_id < w[1].fixture_id)
    );
    for g in &b.manifest.fixtures {
        assert!(g.variants.windows(2).all(|w| w[0].id < w[1].id));
        assert!(g.requirement_ids.iter().all(|id| id.starts_with('L')));
    }
    let value = serde_json::to_value(FixtureListing::new(&b, true)).unwrap();
    assert_eq!(value["semantic_execution_status"], "not_run");
    assert_eq!(value["runtime_acceptance_readiness"], false);
    assert_eq!(
        b.manifest
            .fixtures
            .iter()
            .flat_map(|g| &g.variants)
            .filter(|v| v.artifacts.iter().any(|a| a.role == "endpoint"))
            .count(),
        10
    );
}
#[test]
fn endpoint_selection_rejects_missing_and_json_only() {
    let b = bundle();
    assert_eq!(
        fixture::select_endpoint(&b, "missing").unwrap_err().code,
        "variant_unknown"
    );
    assert_eq!(
        fixture::select_endpoint(&b, "PX01-optional-absent-v1")
            .unwrap_err()
            .code,
        "endpoint_unavailable"
    );
}
#[test]
fn changed_endpoint_after_loading_rejected() {
    let t = tree();
    let b = fixture::load_bundle(t.path(), Path::new(MANIFEST)).unwrap();
    fs::write(
        t.path()
            .join("fixtures/prototype/endpoints/PX01-vllm-v1.prom"),
        "corrupt",
    )
    .unwrap();
    assert_eq!(
        fixture::select_endpoint(&b, "PX01-vllm-v1")
            .unwrap_err()
            .code,
        "artifact_changed"
    );
}
#[test]
fn bad_hash_or_missing_payload_fails_before_exposing_bundle() {
    for missing in [false, true] {
        let t = tree();
        let p = t
            .path()
            .join("fixtures/prototype/endpoints/PX01-vllm-v1.prom");
        if missing {
            fs::remove_file(p).unwrap();
        } else {
            fs::write(p, "corrupt").unwrap();
        }
        assert!(fixture::load_bundle(t.path(), Path::new(MANIFEST)).is_err());
    }
}
#[test]
fn oversized_or_truncated_manifest_fails() {
    for bytes in [vec![b' '; 1024 * 1024 + 1], b"{".to_vec()] {
        let t = tree();
        fs::write(t.path().join(MANIFEST), bytes).unwrap();
        assert!(fixture::load_bundle(t.path(), Path::new(MANIFEST)).is_err());
    }
}
#[test]
fn omitted_group_fails() {
    let t = tree();
    let p = t.path().join(MANIFEST);
    let mut v: Value = serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
    v["fixtures"].as_array_mut().unwrap().pop();
    fs::write(p, serde_json::to_vec(&v).unwrap()).unwrap();
    assert!(fixture::load_bundle(t.path(), Path::new(MANIFEST)).is_err());
}
#[test]
fn traversal_and_arbitrary_manifests_refused() {
    for p in ["../manifest.json", "/tmp/manifest.json", "other.json"] {
        assert_eq!(
            fixture::load_bundle(&root(), Path::new(p))
                .unwrap_err()
                .code,
            "unsupported_manifest"
        );
    }
}
#[cfg(unix)]
#[test]
fn symlink_endpoint_refused_even_with_identical_target_bytes() {
    let t = tree();
    let b = fixture::load_bundle(t.path(), Path::new(MANIFEST)).unwrap();
    let p = t
        .path()
        .join("fixtures/prototype/endpoints/PX01-vllm-v1.prom");
    let saved = t.path().join("saved.prom");
    fs::rename(&p, &saved).unwrap();
    std::os::unix::fs::symlink(&saved, p).unwrap();
    assert_eq!(
        fixture::select_endpoint(&b, "PX01-vllm-v1")
            .unwrap_err()
            .code,
        "unsafe_path"
    );
}
#[test]
fn invalid_cli_arguments_rejected_before_loading_or_network() {
    for args in [
        vec![],
        vec!["unknown"],
        vec!["list-fixtures", "--url", "http://example.com"],
        vec!["list-fixtures", "--root"],
        vec!["list-fixtures", "--root", ".", "--root", "."],
        vec!["serve-fixture", "--variant", "x", "--port", "-1"],
        vec!["serve-fixture", "--variant", "x", "--port", "65536"],
        vec!["serve-fixture", "--variant", "x", "--max-requests", "17"],
        vec!["serve-fixture", "--variant", "x", "--max-run-ms", "0"],
        vec!["serve-fixture", "--variant", "x", "--max-run-ms", "30001"],
    ] {
        let args: Vec<_> = args.into_iter().map(str::to_string).collect();
        assert_eq!(parse_args(&args).unwrap_err().code, "usage");
    }
}
#[test]
fn all_nonlocal_binds_rejected_before_binding() {
    for addr in [
        "0.0.0.0",
        "localhost",
        "::1",
        "10.1.1.1",
        "8.8.8.8",
        "127.0.0.2",
    ] {
        let cfg = ServerConfig {
            bind: addr.into(),
            ..Default::default()
        };
        assert_eq!(cfg.validate().unwrap_err().code, "invalid_bind");
        assert!(
            parse_args(&["serve-fixture", "--variant", "x", "--bind", addr].map(str::to_string))
                .is_err()
        );
    }
}
#[test]
fn future_commands_fail_with_no_success_output() {
    for args in [
        vec!["query"],
        vec!["collect"],
        vec!["replay"],
        vec![
            "run-scenario",
            "--scenario",
            "token-rate",
            "--variant",
            "PX02-success-rate-v1",
        ],
    ] {
        let result = cli(&args);
        assert_eq!(result.status.code(), Some(1));
        assert!(result.stdout.is_empty());
        assert!(String::from_utf8_lossy(&result.stderr).contains("not_prototyped"));
    }
    assert_eq!(cli(&["unknown"]).status.code(), Some(2));
}
#[test]
fn cli_help_and_fixture_commands_work() {
    assert!(cli(&["--help"]).status.success());
    for name in ["list-fixtures", "validate-fixtures"] {
        let o = cli(&[name]);
        assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
        let v: Value = serde_json::from_slice(&o.stdout).unwrap();
        assert_eq!(v["fixture_variants"], 36);
        assert_eq!(v["runtime_acceptance_readiness"], false);
    }
}
#[test]
fn real_roundtrip_is_byte_exact_and_deterministic() {
    let first =
        serde_json::to_vec(&endpoint::run_endpoint_roundtrip(input("PX01-vllm-v1")).unwrap())
            .unwrap();
    let second =
        serde_json::to_vec(&endpoint::run_endpoint_roundtrip(input("PX01-vllm-v1")).unwrap())
            .unwrap();
    assert_eq!(first, second);
    let v: Value = serde_json::from_slice(&first).unwrap();
    assert_eq!(v["expected_bytes"], 398);
    assert_eq!(v["actual_bytes"], 398);
    assert_eq!(v["expected_sha256"], v["actual_sha256"]);
    assert_eq!(v["semantic_execution_status"], "not_run");
}
#[test]
fn intentionally_invalid_payloads_transfer_without_semantic_claim() {
    for id in ["PX01-malformed-type-v1", "PX01-inconsistent-histogram-v1"] {
        let report = endpoint::run_endpoint_roundtrip(input(id)).unwrap();
        assert_eq!(report.harness_status, "passed");
        assert_eq!(report.semantic_execution_status, "not_run");
        assert!(!report.production_support_claimed);
    }
}
#[test]
fn serves_exact_bytes_and_releases_listener() {
    let i = input("PX01-sglang-v1");
    let expected = i.bytes().to_vec();
    let s = endpoint::spawn_fixture_server(i, ServerConfig::default()).unwrap();
    let addr = s.bound_addr();
    assert_eq!(addr.ip().to_string(), "127.0.0.1");
    assert_eq!(endpoint::fetch_fixture(&s).unwrap(), expected);
    let outcome = s.wait().unwrap();
    assert_eq!(outcome.accepted_connections, 1);
    assert_eq!(outcome.rejected_requests, 0);
    assert_released(addr);
}
#[test]
fn bad_requests_are_counted_and_server_continues_within_bound() {
    let requests: [(&[u8], u16); 5] = [
        (b"GET /missing HTTP/1.1\r\nHost: fixture\r\n\r\n", 404),
        (b"POST /metrics HTTP/1.1\r\nHost: fixture\r\n\r\n", 405),
        (b"bad\r\n\r\n", 400),
        (
            b"GET /metrics HTTP/1.1\r\nHost: fixture\r\nContent-Length: 1\r\n\r\nx",
            400,
        ),
        (
            b"GET /metrics HTTP/1.1\r\nHost: fixture\r\nTransfer-Encoding: chunked\r\n\r\n",
            400,
        ),
    ];
    let s = endpoint::spawn_fixture_server(
        input("PX01-vllm-v1"),
        ServerConfig {
            max_requests: 6,
            ..Default::default()
        },
    )
    .unwrap();
    for (req, code) in requests {
        let response = raw_request(&s, req);
        assert!(response.starts_with(format!("HTTP/1.1 {code} ").as_bytes()));
    }
    assert_eq!(endpoint::fetch_fixture(&s).unwrap().len(), 398);
    let o = s.wait().unwrap();
    assert_eq!(o.accepted_connections, 6);
    assert_eq!(o.rejected_requests, 5);
}
#[test]
fn oversized_headers_receive_431() {
    let s = endpoint::spawn_fixture_server(input("PX01-vllm-v1"), ServerConfig::default()).unwrap();
    // Send just past the bounded header length; no unread tail is needed.
    let mut req = b"GET /metrics HTTP/1.1\r\nHost: fixture\r\nX: ".to_vec();
    req.resize(8193, b'a');
    assert!(raw_request(&s, &req).starts_with(b"HTTP/1.1 431 "));
    assert_eq!(s.wait().unwrap().rejected_requests, 1);
}
#[test]
fn incomplete_peer_times_out_then_server_serves_next_connection() {
    let s = endpoint::spawn_fixture_server(
        input("PX01-vllm-v1"),
        ServerConfig {
            max_requests: 2,
            ..Default::default()
        },
    )
    .unwrap();
    let start = Instant::now();
    let response = raw_request(&s, b"GET /metrics");
    assert!(response.is_empty());
    assert!(start.elapsed() < Duration::from_secs(3));
    assert_eq!(endpoint::fetch_fixture(&s).unwrap().len(), 398);
    let outcome = s.wait().unwrap();
    assert_eq!(outcome.rejected_requests, 1);
}
#[test]
fn idle_run_deadline_fails_and_releases_listener() {
    let s = endpoint::spawn_fixture_server(
        input("PX01-vllm-v1"),
        ServerConfig {
            max_run: Duration::from_millis(40),
            ..Default::default()
        },
    )
    .unwrap();
    let addr = s.bound_addr();
    assert_eq!(s.wait().unwrap_err().code, "deadline_exceeded");
    assert_released(addr);
}
#[test]
fn occupied_port_fails_without_leaking_a_worker() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let config = ServerConfig {
        port: listener.local_addr().unwrap().port(),
        ..Default::default()
    };
    assert!(endpoint::spawn_fixture_server(input("PX01-vllm-v1"), config).is_err());
}
#[test]
fn explicit_shutdown_and_drop_release_listener() {
    for explicit in [true, false] {
        let s =
            endpoint::spawn_fixture_server(input("PX01-vllm-v1"), ServerConfig::default()).unwrap();
        let addr = s.bound_addr();
        if explicit {
            assert_eq!(s.shutdown_and_join().unwrap().termination, "cancelled");
        } else {
            drop(s);
        }
        assert_released(addr);
    }
}
#[test]
fn failed_fetch_and_shutdown_release_listener() {
    let s = endpoint::spawn_fixture_server(input("PX01-vllm-v1"), ServerConfig::default()).unwrap();
    let addr = s.bound_addr();
    raw_request(&s, b"GET /wrong HTTP/1.1\r\nHost: fixture\r\n\r\n");
    assert!(endpoint::fetch_fixture(&s).is_err());
    s.shutdown_and_join().unwrap();
    assert_released(addr);
}
struct OwnedChild(std::process::Child);
impl Drop for OwnedChild {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
#[test]
fn standalone_binary_announces_real_url_and_terminates_after_request() {
    let mut child = OwnedChild(
        Command::new(env!("CARGO_BIN_EXE_aether-poc"))
            .current_dir(root())
            .args(["serve-fixture", "--variant", "PX01-vllm-v1"])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap(),
    );
    let stdout = child.0.stdout.take().unwrap();
    // Server lifetime is bounded by the CLI, even if its output is broken.
    let mut reader = BufReader::new(stdout);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let v: Value = serde_json::from_str(&line).unwrap();
    let addr = v["url"]
        .as_str()
        .unwrap()
        .strip_prefix("http://")
        .unwrap()
        .strip_suffix("/metrics")
        .unwrap();
    let mut s = TcpStream::connect(addr).unwrap();
    s.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
    s.write_all(b"GET /metrics HTTP/1.1\r\nHost: fixture\r\nConnection: close\r\n\r\n")
        .unwrap();
    let mut bytes = Vec::new();
    s.read_to_end(&mut bytes).unwrap();
    assert!(bytes.starts_with(b"HTTP/1.1 200 OK"));
    line.clear();
    reader.read_line(&mut line).unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(&line).unwrap()["event"],
        "server_stopped"
    );
    let start = Instant::now();
    loop {
        if let Some(status) = child.0.try_wait().unwrap() {
            assert!(status.success());
            break;
        }
        assert!(start.elapsed() < Duration::from_secs(2));
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[test]
fn active_partial_request_cannot_extend_run_deadline() {
    let s = endpoint::spawn_fixture_server(
        input("PX01-vllm-v1"),
        ServerConfig {
            max_run: Duration::from_millis(150),
            ..Default::default()
        },
    )
    .unwrap();
    let addr = s.bound_addr();
    let mut peer = TcpStream::connect(addr).unwrap();
    peer.write_all(b"G").unwrap();
    let start = Instant::now();
    assert_eq!(s.wait().unwrap_err().code, "deadline_exceeded");
    assert!(start.elapsed() < Duration::from_secs(2));
    drop(peer);
    assert_released(addr);
}
#[test]
fn explicit_shutdown_interrupts_partial_header() {
    let s = endpoint::spawn_fixture_server(input("PX01-vllm-v1"), ServerConfig::default()).unwrap();
    let addr = s.bound_addr();
    let mut peer = TcpStream::connect(addr).unwrap();
    peer.write_all(b"G").unwrap();
    std::thread::sleep(Duration::from_millis(20));
    let start = Instant::now();
    let _ = s.shutdown_and_join().unwrap();
    assert!(start.elapsed() < Duration::from_millis(800));
    drop(peer);
    assert_released(addr);
}
