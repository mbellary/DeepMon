# Milestone 3 acceptance: Rust fixture harness and local server

Milestone 3 is complete for its intermediate synthetic harness scope. The independent `prototypes/aether-poc/` Rust package validates/lists the accepted bundle, serves selected endpoint bytes through a bounded 127.0.0.1 listener and runs a deterministic byte-roundtrip scenario. No metric interpretation, real collection, source compatibility or production architecture is established.

Accepted on 2026-11-01, Asia/Kolkata UTC+05:30. Detailed progress is in `milestone-execution-plans/milestone-03-execplan.md`; aggregate tracking is in `EXECPLAN.md`. `milestone-3/inputs.json` records inspected environment, starting Git base and pre-existing planning changes, plus immutable input hashes. The base revision is not a claim that the implementation is committed.

## Delivered behavior

`validate-fixtures` and `list-fixtures` report all twelve groups, 36 variants, 82 indexed payloads and two intentional invalid endpoint samples. Listing retains source/requirement/association references, artifact roles and rejection declarations. Exact supported manifest identity and confined bounded file reads are checked; missing/corrupt/oversized/truncated files, unsupported paths and symlinks fail.

`serve-fixture` owns a loopback listener, serves one selected endpoint unchanged and exits after its finite connection count or deadline. Its startup line exposes the actual ephemeral local URL. Limits are 8 KiB headers, 1 MiB bodies, one-second absolute peer deadlines, up to sixteen connections and up to thirty seconds; defaults are one connection/five seconds. Request errors are reported and bounded. Only the owned local client is used for scenarios; arbitrary remote targets are not supported. Cleanup joins the worker on success, cancellation and errors.

Two separate CLI invocations of `run-scenario --scenario endpoint-roundtrip --variant PX01-vllm-v1` produced byte-identical JSON, verified by `cmp`. Actual response length was 398 bytes. Expected and actual SHA-256 were both `cd00a6391c26c66d4d450f288d31b9e21f5815e5d7631e8138b2ff196c9e389e`. Reports preserve `semantic_execution_status=not_run`, readiness false and production support false. No semantic oracle is passed into the round-trip implementation.

The package has its own lockfile with 34 package entries, exact serde/serde_json pins, the unchanged local validation-library dependency and test-only tempfile. No new package downloads or Python runtime were needed. `docs/prototype-dependencies.md` records exact direct dependencies and defers Arrow/Parquet/DataFusion compatibility to M6's executable storage/query experiment. `docs/prototype-harness.md` documents the finite HTTP contract, input/oracle boundary, limitations and future observation/relationship shapes.

## Validation

Run from `/home/mbellary/work/projects/DeepMon`:

    cargo fmt --manifest-path prototypes/aether-poc/Cargo.toml -- --check
    cargo check --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml
    cargo test --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml
    cargo clippy --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml --all-targets -- -D warnings
    cargo test --locked --offline --manifest-path tools/aether-validation/Cargo.toml
    cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- evidence --root .
    cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- traceability --root .
    cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- fixtures --root .
    cmp evidence/prototype/milestone-3/roundtrip-1.json evidence/prototype/milestone-3/roundtrip-2.json
    sha256sum --check --quiet evidence/prototype/milestone-3/accepted-inputs.sha256
    sha256sum --check --quiet evidence/prototype/milestone-3/artifacts.sha256

Actual results: 4 new client unit tests and 25 new harness integration tests pass; all 70 existing validation integration tests pass (40 fixture plus 30 original). Total 99 tests pass, with zero final failures. Both packages pass strict Clippy and formatting; the harness passes locked offline cargo check. Evidence/traceability/fixture checks pass. Original requirements, mappings, frozen sources, M1/M2 tracking/budgets and accepted M2 validation source/lockfiles match their previous hashes.

`milestone-3/harness-tests.txt` and `validation-regression-tests.txt` record actual counts. `clippy.txt`, `format.txt`, `cargo-check.txt`, `validation-clippy.txt` and `validation-format.txt` record static checks. `prerequisite-evidence.txt`, `prerequisite-traceability.txt`, `prerequisite-fixtures.txt` and `final-fixtures.txt` record integrity/traceability/construction outcomes. `validate-fixtures.json`, `list-fixtures.json` and `roundtrip-1.json`/`roundtrip-2.json` are actual harness outputs. `accepted-inputs-check.txt`, `m2-preservation.txt` and `original-preservation.txt` attest preserved inputs. Tool identities are recorded in separate rustc/cargo/kernel/glibc logs. The new `artifacts.sha256` covers delivered package/source/docs and M3 evidence except itself; build directories and current living plans are excluded.

## Discoveries and limits

The sandbox initially denied loopback bind with `Operation not permitted`. The initial failed socket test log remains `initial-sandbox-tests.txt`, clearly separate from final passing evidence. The real demonstration and socket tests ran through the approved execution path with loopback access. No socket test was replaced with an in-memory substitute or skipped as passing. Repeat runs in a restricted environment require its normal socket permission.

An initial listing test incorrectly required every experiment to select a metric; some policy/metadata experiments legitimately select associations only. The test now checks retained identities without imposing that false assumption. A rejected request with an already buffered body exposed TCP-close behavior; the server sends its closing response and discards bounded pending input. Absolute read/write deadlines prevent partial I/O from refreshing the deadline. Tests cover wrong paths/methods/framing, oversized headers, partial peers, idle/active deadlines, occupied ports, explicit/drop cleanup, standalone process lifecycle and malformed metric transfer without semantic success.

The finite HTTP subset and ordinary filesystem confinement are local harness mechanisms, not production networking/security guarantees. Scheduling/OS timeout rounding is not real-time precision. All 250 requirements and 58 associations remain represented; all twelve PX semantic experiments remain not_run. Deployed schemas, host/device/fabric capabilities, physical timing, overhead, data loss and durability remain unverified. Proposed budgets remain unagreed and runtime readiness stays false.

Next implementation milestone: M4, native endpoint parsing and selected justified calculations against independent fixture oracles, including reset handling, correct histogram types/populations and explicit unknown results.
