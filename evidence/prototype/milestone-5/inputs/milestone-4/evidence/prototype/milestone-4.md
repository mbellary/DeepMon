# Milestone 4 acceptance: synthetic native parsing and justified calculations

Milestone 4 is complete for its defined functional prototype scope. Rust now parses the selected invented native endpoints, calculates supported values, preserves insufficient/denied/disabled states and compares actual outputs with independently authored expectations. Runtime acceptance remains blocked by unagreed budgets; real engine, hardware and production capabilities remain unverified.

Accepted 2026-11-01 23:32 IST (UTC+05:30). Detailed task tracking is `milestone-execution-plans/milestone-04-execplan.md`; aggregate tracking is `EXECPLAN.md`. Starting Git base and pre-existing planning changes are recorded in `milestone-4/inputs.json`. This is not a claim that working-tree changes are committed.

## Delivered behavior

Seven scenario adapters cover 23 variants from PX01/PX02/PX03/PX04/PX05/PX11/PX12. All twelve original groups, 36 variants and 49 oracle assertions remain accounted for. Actual results are 24 matched assertions, two precise expected parser rejections, one separately verified counterexample proof, three genuine later-stage deferrals within the considered groups and 19 assertions not run in later-stage groups. There are zero required mismatches in final verification. A deferred assertion is not a passing experiment or unsupported requirement.

Native parsing acquires actual bytes from the owned 127.0.0.1 fixture server for seven producer variants and two malformed exports. It preserves family/sample names, TYPE, HELP, labels and source scope/context; unknown families remain unmapped. The vLLM `_total` family is a histogram, Triton's shared memory pool stays server-wide and source labels do not prove physical ownership. Optional absence bypasses network acquisition and remains unknown despite its input placeholder zero. TYPE_INVALID and HISTOGRAM_INCONSISTENT negative assertions require those actual runtime errors.

The positive rate result is 5 successful requests/second from supplied JSON records 100→150 over ten virtual seconds. Its endpoint file has only the first value; the report explicitly says synthetic_record_input. Reset/incarnation changes and raw TGI accepted-arrival semantics remain unknown. Explicit front/back logical-ID overlap selects ten requests while retaining both raw counts. Per-device memory ratios are .5 and .5, and the supplied replica count 100 is retained once. Missing denominators, incompatible populations/clocks, aggregate-only exact variance/goodput and missing freshness evidence produce unknown, not fabricated zeros.

Visibility/policy scenarios retain denied, disabled, unknown and blocked states, retain safe fixture labels, redact sensitive fields and reject action descriptions without executing them. Queue admission, publisher/composition resolution and event replay are deferred to their specified milestones. Exact token rates/cache-hit ratios lack necessary fixture inputs and remain unprototyped; no requirement or source was removed.

The finite independent runtime parser and new typed records/results use the existing Rust dependencies. No Python, new dependency downloads, live inference source, kernel/device SDK, external URL, workload change, cloud service or production database is used. Arrow/Parquet/DataFusion selection remains M6. `docs/prototype-metric-semantics.md` documents grammar, arithmetic, privacy and boundaries.

## Verification and current execution tracking

`run-scenario` produces actual outputs without semantic expected values. `verify-scenario` computes actual output/error first, then loads expected assertions and compares target/value/unit/state. Oracle-only proof material is verified separately and never supplied as runtime events. A mismatch exits nonzero; expected negative verification passes only for the precise error. `verify-milestone --milestone 4` reports every original assertion and its assigned stage.

`docs/prototype-metric-execution-status.json` records truthful function execution, comparisons, proof/deferred categories and evidence/input hashes. It indexes all 49 assertions and links the exact recorded report. Original registry/status/fixture/oracle fields stay historical bytes. No whole-PX runtime pass is granted: readiness remains false, real_verification not_run and px_runtime_acceptance_status blocked_unagreed_budget. The Rust record_metric_evidence example verifies identity/coverage and protects conflicting ledger output; an identical rerun passes. It indexes evidence rather than providing a signed execution attestation.

## Actual checks

Run from `/home/mbellary/work/projects/DeepMon`:

    cargo fmt --manifest-path prototypes/aether-poc/Cargo.toml -- --check
    cargo check --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml
    cargo test --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml
    cargo clippy --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml --all-targets -- -D warnings
    cargo test --locked --offline --manifest-path tools/aether-validation/Cargo.toml
    cargo fmt --manifest-path tools/aether-validation/Cargo.toml -- --check
    cargo clippy --locked --offline --manifest-path tools/aether-validation/Cargo.toml --all-targets -- -D warnings
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- verify-milestone --root . --milestone 4
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml --example record_metric_evidence

Final results: 30 new metric integration tests, all 29 previous harness/client tests and all 70 validation regressions pass: 129 tests total, zero final failures. Formatting, locked offline check and strict Clippy pass. Existing evidence/traceability/fixture commands pass. The original 250 metric/58 association identities, E88/R35/H100/I26/U1 research routes, original sources, M1/M2 planning/fixture/budget records and validation-tool bytes remain unchanged.

One prior unsupported-scenario regression now checks token-rate instead of counter-rate, because counter-rate is implemented in M4; new tests verify its actual positive/reset/unresolved behavior. The original regression source remains in the verified M3 predecessor. No prior transport/lifecycle behavior was removed.

Five CLI demonstrations were each run twice with fresh output files: native positive parse, expected negative parse verification, counter-rate, aggregate insufficiency verification and whole-M4 verification. Each pair compares byte-identically with cmp. The final summary also matches the earlier report linked by the ledger. Final parser acquisitions and tests used the approved loopback execution path after sandbox restrictions denied socket creation; no socket test was skipped or replaced with an in-memory substitute.

`milestone-4/final-harness-tests.txt` records four client unit, 25 harness integration and 30 metric integration tests. `validation-tests.txt` records 40 fixture plus 30 original tests. `clippy.txt`, `format.txt`, `cargo-check.txt`, `validation-clippy.txt` and `validation-format.txt` record static checks. `evidence-check.txt`, `traceability-check.txt` and `fixtures-check.txt` record unchanged baseline verification. `native-1.json`/`native-2.json`, `negative-1.json`/`negative-2.json`, `rate-1.json`/`rate-2.json`, `aggregate-1.json`/`aggregate-2.json` and `verify-milestone-final-1.json`/`verify-milestone-final-2.json` are actual deterministic outputs. `verify-milestone-1.json` is the identical report linked by the functional ledger; ledger-authoring/idempotence logs record both successful runs. Initial trials are retained separately and are not final acceptance evidence.

## Preservation and findings

Before changing live M3 source/docs, exact predecessor bytes were copied to `milestone-4/inputs/milestone-3/` at their original paths. Its original 43-file M3 checksum list passes there. Required frozen sources, fixtures and planning dependencies are also preserved. A dependency audit added the unchanged scope/budget planning files and M2 construction log required to rerun full fixture validation; predecessor fixture validation now passes. `inputs.json` indexes the archived paths/hashes. M2's accepted 100-artifact list still passes against live files; the original research baseline hashes pass. Current M4 hashes are separate and do not rewrite historical reports/lists.

Initial functional summary under the sandbox had nine endpoint acquisition failures; socket-enabled verification passes. Initial new tests exposed two incorrect test assumptions: the existing model alias is fixture-model, and a substring check mistook the result target exact_goodput_and_variance for a hidden goodput_a field. Corrected tests now examine actual fixture values and exact field names. Review also tightened original counter/count token checking so large fractional numbers cannot round into apparently exact integer counts; the final precision test and all regressions pass.

Final serialized functional reports/ledger and diagnostic trials contain no synthetic sensitive marker. Original invented input/source archives retain exact historical test bytes and are outside the sanitized customer-output boundary. The parser supports only the documented finite grammar; normal exporter formats beyond it remain unqualified. Test clocks are virtual and the local environment is one reference, not backward compatibility evidence. No measured inference overhead, real event-loss/durability, physical timing precision or production support follows from these checks.

Next implementation milestone: M5, simulated metadata enrichment and lifecycle handling, including ambiguous publishers, remote subjects, time-valid memberships and PID reuse without duplicating observations.
