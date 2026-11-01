# Aether synthetic Rust harness

This is an intermediate prototype. It validates/lists the accepted invented fixture bundle and transfers an explicitly selected endpoint over an owned 127.0.0.1 HTTP listener. It does not collect from inference engines, access devices or establish production compatibility. M4 adds finite synthetic metric interpretation as documented below.

From `/home/mbellary/work/projects/DeepMon`:

    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- validate-fixtures --root .
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- list-fixtures --root .
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- run-scenario --root . --scenario endpoint-roundtrip --variant PX01-vllm-v1
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- serve-fixture --root . --variant PX01-vllm-v1 --bind 127.0.0.1 --port 0 --max-requests 1 --max-run-ms 5000

Only `fixtures/prototype/manifest.json` is supported; `--manifest` may specify that exact relative path. `--root` defaults to the working directory. Use `--help` for options. Validation/listing/report commands emit one JSON object. The standalone server emits a flushed startup line with its actual local URL, then a shutdown line. A browser may fetch `/metrics` at that URL; one accepted connection exhausts the default limit. The automatic scenario owns both ends and is the repeatable demonstration. Identical scenario inputs produce byte-identical JSON reports.

Endpoint-bearing negative variants are served unchanged. A successful byte transfer does not mean the metrics parser has rejected their content; semantic execution remains `not_run`. JSON-only variants cannot be served. `query`, `collect`, `replay` and `token-rate` return nonzero `not_prototyped` errors. `counter-rate` is implemented in M4 over supplied records. Exit 0 means the requested harness action completed; exit 1 is runtime/validation failure and exit 2 is bad command usage. Rejected HTTP requests are counted in the standalone server outcome; completing its connection count does not mean every request was accepted as valid.

The server is finite: at most 16 connections, at most 30 seconds, 8 KiB headers and 1 MiB fixture bodies. Its per-peer I/O deadline is one second, also bounded by the overall deadline. Partial headers and writes cannot refresh these deadlines. It supports only a small HTTP/1.1 GET subset, Content-Length and connection close. It never resolves remote names or follows redirects. These are local functional test limits, not agreed PX runtime budgets or a production HTTP implementation.

    cargo fmt --manifest-path prototypes/aether-poc/Cargo.toml -- --check
    cargo check --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml
    cargo test --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml
    cargo clippy --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml --all-targets -- -D warnings

Socket tests require permission to bind loopback sockets. If a sandbox denies them, run through the environment's approved execution path; skipped sockets are not a passing demonstration. Build artifacts stay under the ignored `target/` directory. Details: [harness contract](../../docs/prototype-harness.md), [dependencies](../../docs/prototype-dependencies.md), [execution plan](../../milestone-execution-plans/milestone-03-execplan.md).

## Metric functions added in Milestone 4

The harness now implements finite native parsing and selected calculations. Run and independent verification are separate:

    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- run-scenario --root . --scenario native-parse --variant PX01-vllm-v1
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- run-scenario --root . --scenario counter-rate --variant PX02-success-rate-v1
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- verify-scenario --root . --scenario native-parse --variant PX01-malformed-type-v1
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- verify-milestone --root . --milestone 4

Other named metric scenarios are aggregate-evidence, count-boundary, memory-occupancy, visibility-policy and freshness-evidence. Select variants in the corresponding PX03/PX04/PX05/PX11/PX12 groups. Expected negative parsing passes verification only for the precise actual error; direct run still exits nonzero. Unknown inputs yield explicit unknown results, not zero. The summary accounts for all twelve groups, including later-stage assertions, and leaves runtime acceptance blocked/unagreed. Query, collect, replay and token-rate remain not_prototyped.

The rate command calculates from supplied timed JSON records; the paired endpoint contains only one counter sample. It is not a demonstrated polling rate or token rate. Detailed semantics and limits: [metric semantics](../../docs/prototype-metric-semantics.md). Accepted M3 source/lock/report bytes are preserved under the verified M4 predecessor root; current hashes/evidence are separate.

## Metadata functions added in Milestone 5

    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- run-scenario --root . --scenario publisher-identity --variant PX04-ambiguous-publisher-v1
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- verify-scenario --root . --scenario process-lifecycle --variant PX06-pid-reuse-v1
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- run-scenario --root . --scenario resource-membership --variant PX05-replica-set-v1
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- run-scenario --root . --scenario fabric-context --variant PX07-inventory-only-v1
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- verify-scenario --root . --scenario enrichment-boundaries --case E5-01
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- verify-milestone --root . --milestone 5

The five original metadata scenarios are publisher-identity, process-lifecycle, resource-membership, fabric-context and visibility-enrichment (PX04/PX06/PX05/PX07/PX11 respectively). Boundary cases E5-01 through E5-07 use the separate hashed enrichment bundle. Runtime/expected inputs remain separate. Every measurement stays one measurement; ambiguous/partial/unknown facts do not become guessed ownership or active fabric use. Query times and all values are explicitly invented.

The original M4 summary and historical metric ledger stay unchanged. Stage-5 tracking imports prior-only evidence without rerunning or double-counting it; runtime acceptance remains blocked. Details: [enrichment contract](../../docs/prototype-enrichment-contract.md), [Milestone 5 plan](../../milestone-execution-plans/milestone-05-execplan.md).

Fixture/evidence authoring uses Rust examples `author_enrichment_fixtures` and `record_enrichment_evidence`. The first writes the fixed extension bundle; identical reruns write zero files and conflicting files are protected. The second indexes `evidence/prototype/milestone-5/verify-milestone-1.json` and writes separate coverage/current-ledger files without resetting M4 history. These helpers do not acquire real metadata or establish agreed runtime budgets.
