# Aether synthetic Rust harness

This is an intermediate prototype. It validates/lists the accepted invented fixture bundle and transfers an explicitly selected endpoint over an owned 127.0.0.1 HTTP listener. It does not collect from inference engines, interpret metrics, access devices or establish production compatibility.

From `/home/mbellary/work/projects/DeepMon`:

    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- validate-fixtures --root .
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- list-fixtures --root .
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- run-scenario --root . --scenario endpoint-roundtrip --variant PX01-vllm-v1
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- serve-fixture --root . --variant PX01-vllm-v1 --bind 127.0.0.1 --port 0 --max-requests 1 --max-run-ms 5000

Only `fixtures/prototype/manifest.json` is supported; `--manifest` may specify that exact relative path. `--root` defaults to the working directory. Use `--help` for options. Validation/listing/report commands emit one JSON object. The standalone server emits a flushed startup line with its actual local URL, then a shutdown line. A browser may fetch `/metrics` at that URL; one accepted connection exhausts the default limit. The automatic scenario owns both ends and is the repeatable demonstration. Identical scenario inputs produce byte-identical JSON reports.

Endpoint-bearing negative variants are served unchanged. A successful byte transfer does not mean the metrics parser has rejected their content; semantic execution remains `not_run`. JSON-only variants cannot be served. `counter-rate`, `query`, `collect` and `replay` return nonzero `not_prototyped` errors. Exit 0 means the requested harness action completed; exit 1 is runtime/validation failure and exit 2 is bad command usage. Rejected HTTP requests are counted in the standalone server outcome; completing its connection count does not mean every request was accepted as valid.

The server is finite: at most 16 connections, at most 30 seconds, 8 KiB headers and 1 MiB fixture bodies. Its per-peer I/O deadline is one second, also bounded by the overall deadline. Partial headers and writes cannot refresh these deadlines. It supports only a small HTTP/1.1 GET subset, Content-Length and connection close. It never resolves remote names or follows redirects. These are local functional test limits, not agreed PX runtime budgets or a production HTTP implementation.

    cargo fmt --manifest-path prototypes/aether-poc/Cargo.toml -- --check
    cargo check --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml
    cargo test --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml
    cargo clippy --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml --all-targets -- -D warnings

Socket tests require permission to bind loopback sockets. If a sandbox denies them, run through the environment's approved execution path; skipped sockets are not a passing demonstration. Build artifacts stay under the ignored `target/` directory. Details: [harness contract](../../docs/prototype-harness.md), [dependencies](../../docs/prototype-dependencies.md), [execution plan](../../milestone-execution-plans/milestone-03-execplan.md).
