# Milestone 3: build an isolated Rust harness and local fixture server


This ExecPlan follows repository-root `PLANS.md`. Maintain `Progress`, `Surprises & Discoveries`, `Decision Log`, `Outcomes & Retrospective` and the bottom revision note at every stopping point. The workspace is `/home/mbellary/work/projects/DeepMon`; all paths below are relative to that root. `EXECPLAN.md` tracks the whole prototype program, and `milestone-execution-plans/README.md` indexes its detailed plans.

Current state: the user authorized execution, and M3.1–M3.7 are complete for the synthetic Rust harness. Actual acceptance evidence is in `evidence/prototype/milestone-3.md`. Milestones 1–2 remain accepted and unchanged. All twelve PX semantic experiments remain not_run, runtime readiness remains false and M4–M8 implementation remains pending. Next is M4 native endpoint parsing and selected calculations.

## Purpose / Big Picture


Give a developer a working Rust entry point for the synthetic Aether experiments. After implementation, they can list and validate the accepted fixture bundle, serve one explicitly selected invented endpoint on the local machine, and run a repeatable demonstration that retrieves its exact bytes and reports their identity. This proves that the future experiment harness can consume the existing bundle without confusing test inputs, expected answers and actual results.

A harness is a small program that runs controlled tests. A fixture is invented input, an oracle is its independently authored expected answer, and a manifest is the file index with identities, sizes and content hashes. A hash identifies file bytes; matching hashes are evidence of byte equality, not of engine compatibility. Loopback is the machine's private network interface: this plan uses only numeric IPv4 address `127.0.0.1`. The endpoint server publishes invented bytes; it is not an inference server or a real collector.

All implementation, helpers and tests are Rust. Keep all 250 metric requirements, 58 metadata associations, four observability layers and twelve PX experiment specifications unchanged. No existing cluster is available. Production architecture remains unscoped. DeepInfer work remains deferred. This milestone does not implement native metric interpretation, calculations, metadata joins, event replay, storage, Arrow/Parquet/DataFusion execution or real hardware access. Those remain later prototype milestones. Finishing this harness cannot mark PX01–PX12 passed or turn proposed budgets into agreed budgets.

## Progress


Times use Asia/Kolkata, UTC+05:30. Checked entries describe actual work; harness acceptance is separate from semantic experiment/runtime acceptance.

- [x] (2026-11-01 22:41 IST, inspected) Read `PLANS.md`, master Milestone 3, accepted M2 plan/format/manifest and Rust validation interfaces; verified no prototype engine package exists.
- [x] (2026-11-01 22:41 IST, prerequisite verified) Rust fixture validation passed: twelve groups, 36 variants, 82 payloads, two intentional invalid payloads, zero executed experiments and readiness false. M2 artifact and original baseline checksums match; installed Rust/Cargo are 1.99.0.
- [x] (2026-11-01, plan authoring) Created this detailed plan and synchronized master/index tracking; plan verification is recorded in Outcomes and Revision Notes.
- [x] (2026-11-01 22:46 IST, plan verified) Required sections/heading format, current Markdown links and seven pending implementation tasks pass checks. Rust evidence/traceability/fixture commands, accepted M2/original baseline hashes and whitespace checks pass. No engine implementation/test result is claimed.
- [x] (2026-11-01 22:48 IST, verified) M3.1: recorded accepted input hashes, starting Git/worktree and matching environment in `evidence/prototype/milestone-3/inputs.json`; prerequisite checks pass.
- [x] (2026-11-01 22:58 IST, verified) M3.2: created independent Rust package/lock/configuration, fixture loader, errors/reports and dependency documentation; shared validation source unchanged.
- [x] (2026-11-01 22:58 IST, verified) M3.3: complete 12/36/82/2 listing/validation passes; confined input reads and endpoint rehashing reject changed/unsafe files; semantic oracles stay outside runtime inputs.
- [x] (2026-11-01 22:58 IST, verified) M3.4: bounded loopback server/client, finite deadlines, HTTP errors and cleanup tests pass through approved socket access.
- [x] (2026-11-01 22:58 IST, verified) M3.5: two CLI round trips returned 398 exact bytes and byte-identical reports; unsupported future commands fail explicitly and semantic execution stays not_run.
- [x] (2026-11-01 22:58 IST, verified) M3.6: 29 new tests plus 70 regressions pass, alongside formatting/strict Clippy/checks and unchanged-input verification; actual logs/report captured.
- [x] (2026-11-01 22:58 IST, accepted) M3.7: acceptance reviewed, master/detail/index synchronized and handoff recorded. M4 is next; no semantic experiment or real capability is marked verified.

## Surprises & Discoveries


Observation: the sandbox blocks loopback sockets, while the approved execution path allows a real local exchange.

Evidence: initial `Operation not permitted` trials are retained in `initial-sandbox-tests.txt`; final tests pass with approved socket access. The actual two CLI reports match 398 bytes and their indexed SHA-256. No network test was skipped or substituted.

Observation: rejected requests can contain unread buffered bodies, and per-operation timeouts alone do not enforce a whole-operation deadline.

Evidence: response closure now sends a FIN and discards bounded pending input for ordinary rejected bodies; absolute read/write loops enforce peer/run deadlines. Tests cover request errors, partial-peer timeout, active run expiry and shutdown while a header is incomplete.

Observation: some policy/metadata experiments legitimately have no selected metric requirement.

Evidence: an initial listing assertion was overly strict. The corrected tests preserve the accepted registry links instead of inventing a nonempty-metric requirement for every group. All 29 new tests and 70 regressions pass.

Observation: the validation crate already exports the construction checker, but its fixture data structures are private modules.

Evidence: `tools/aether-validation/src/lib.rs` exports `check_fixtures`, `FixtureSummary`, `sha256` and `Violation`; it does not export `FixtureManifest`. The new package can reuse the public checker as a local path dependency and define only its required manifest view. No shared-tool edit is needed merely to load the bundle.

Observation: successfully transferring an intentionally malformed payload is different from interpreting it correctly.

Evidence: PX01 includes `PX01-malformed-type-v1` and `PX01-inconsistent-histogram-v1`, whose declared rejection reasons are already construction-checked. M3 can serve their exact bytes with HTTP 200 while leaving metric parsing and its runtime rejection tests to M4. Transport success must never become a PX01 experiment pass.

Observation: accepted fixture construction and runtime acceptance have separate records.

Evidence: `docs/prototype-fixture-status.json` records construction passed while every experiment remains `not_run`; budget records remain proposed/not agreed. M3 will add harness evidence separately rather than weaken existing validators or overwrite those records.

## Decision Log


Decision: create the detailed M3 plan now and keep all seven implementation tasks pending.
Rationale: the user explicitly requested a milestone execution plan and synchronized progress. Authoring does not deliver a running harness.
Date/Author: 2026-11-01 / Codex, applying user direction.

Decision: reuse the public validation library through a local Cargo path dependency; preserve its accepted source and all accepted fixture/research records.
Rationale: this keeps the full construction checks authoritative without duplicating them, exposing new shared APIs or invalidating M2 source hashes. The harness adds its own small, strict manifest view only after construction succeeds.
Date/Author: 2026-11-01 / Codex.

Decision: use synchronous Rust standard-library TCP for a finite local HTTP subset; use the existing exact serde/serde_json versions and no new networking runtime.
Rationale: M3 needs a bounded, single-response test server, not a production HTTP service. The limited protocol is specified below and tested on both client and server sides. This choice does not prescribe production networking or concurrency.
Date/Author: 2026-11-01 / Codex.

Decision: defer Arrow/Parquet/DataFusion dependency selection and compatibility execution to M6, where those libraries are exercised together. Record that deferral in `docs/prototype-dependencies.md`.
Rationale: the former master outline placed that selection in M3, but M3 has no storage/query behavior against which to validate it. Moving selection to its executable experiment avoids an unused lockfile claim; the required Rust/open-storage/query direction and M6 scope are unchanged. This is an implementation-sequencing decision, not a reduction in requirements.
Date/Author: 2026-11-01 / Codex.

Decision: report a separate harness-only verdict and do not advance accepted M1/M2 experiment/status/budget files.
Rationale: a byte round trip demonstrates harness plumbing, not metric semantics, source capabilities, measured timing or agreed runtime acceptance. All twelve PX experiments remain not_run after M3.
Date/Author: 2026-11-01 / Codex.

Decision: execute the authorized harness milestone and accept functional transport/lifecycle results separately from semantic runtime experiments.
Rationale: all seven tasks now have actual artifacts and passing evidence. The sandbox socket denial required approved execution; it did not justify skipping networking. Preserve unagreed budgets, unrun PX states and accepted M1/M2 bytes. No shared validation-source revision was needed.
Date/Author: 2026-11-01 22:58 IST / Codex, applying user direction.

Decision: use absolute read/write deadlines and bounded rejection-body discard before closing the peer.
Rationale: individual system-call timeouts can refresh during partial I/O, and unread request bodies can reset a connection before its rejection is observed. The finite local subset now handles ordinary cases while preserving bounded shutdown; it is not production HTTP hardening.
Date/Author: 2026-11-01 22:58 IST / Codex.

## Outcomes & Retrospective


M3 is complete for its defined synthetic harness scope. The independent Rust package/lock, listing/validation commands, bounded local HTTP server/client, deterministic roundtrip and dependency/handoff documentation exist. Four new unit tests and 25 new integration tests pass, alongside all 70 prior integration tests, strict Clippy, formatting, locked check and preserved-input verification. Actual evidence is `evidence/prototype/milestone-3.md` and its adjacent logs/hash list.

Historical plan-authoring verification at 22:46 IST passed required sections/heading spacing, current links, seven then-pending tasks and existing integrity checks. At that point no engine test suite existed. Subsequent implementation acceptance adds 29 harness tests while retaining all 250/58 original IDs, zero executed PX semantic experiments and readiness false.

The two real CLI reports are byte-identical, with 398 received bytes matching the indexed hash. Approved loopback access was required because sandbox socket creation is denied. Accepted M1/M2 artifact bytes, source mappings, all 250/58 requirements/associations and proposed budget states remain unchanged. All PX semantic experiments and real capabilities remain unverified. Next is M4 native endpoint parsing and selected calculations, including explicit unknown results; production architecture remains unscoped.

## Context and Orientation


`aether-metrics-catalog.json` retains all 250 metric families. `metadata-enrichment-matrix.md` defines 58 associations, meaning relationships such as process-to-container and replica-to-device. `metric-to-source-coverage-matrix.json` retains the research routes E:88, R:35, H:100, I:26, U:1. Those letters mean endpoint, calculation, additional host/device/fabric collector, deeper instrumentation and unresolved mapping. They are research classifications, not support results. L1-001's accepted-request population remains unresolved; L1-005 inherits that gap. Reviewing vLLM/SGLang/Dynamo first did not exclude TGI/Triton/TensorRT-LLM sources. No source mapping changes are required here.

`docs/prototype-experiments.json` specifies PX01–PX12. `docs/prototype-status.json` retains all requirements/associations, selecting 23/42 for later experiments and leaving all actual execution not_run. `docs/prototype-budgets/PX01.json` through `PX12.json` contain proposed limits, unset agreements and readiness false. `zero-application-change-collection-contract.md` permits selected read interfaces while excluding automatic source activation, workload modification and privilege escalation. M3 uses none of those real interfaces: only accepted local files and its own loopback server.

`fixtures/prototype/manifest.json` identifies bundle `aether-synthetic-fixtures-v1`, schema 1, synthetic true, twelve groups, 36 variants and 82 payloads. `docs/prototype-fixture-format.md` specifies exact envelopes and bounds. A group links its experiment, requirements, associations, source inputs and budget. Each variant has one input, one expected file and optionally one endpoint artifact. Artifact paths are relative to `fixtures/prototype`; roles are input/expected/endpoint and each has exact byte length, SHA-256 and media type. Only ten variants have endpoint text. A variant without an endpoint is still a valid fixture, but cannot be served as an endpoint. `PX01-vllm-v1` resolves to `endpoints/PX01-vllm-v1.prom`; its paired JSON envelope is input metadata, not the HTTP response body.

`tools/aether-validation/` is the existing Rust validation package, not the engine. Its public function `check_fixtures(&Path) -> ValidationResult<FixtureSummary>` verifies source evidence, traceability, manifest, files, negative payloads, arithmetic consistency and fixture ledger. The result contains group, variant, file and invalid-payload counts. `Violation` has file, optional ID and reason; failures must stay visible. Reusing this checker performs construction validation; it does not execute an observability algorithm. Its current 70 integration tests cover 40 fixture and 30 original checks. `evidence/prototype/milestone-2.md` and the adjacent `artifacts.sha256` preserve accepted results. The original M1 code predecessor is already preserved under M2's `inputs/milestone-1/`; never overwrite old expected hashes.

The pinned local reference is Ubuntu 26.04.1, kernel 7.0.0-38-generic, glibc 2.43, Rust/Cargo 1.99.0, target x86_64-unknown-linux-gnu. Verify current identities and record drift. These are one test environment, not a Linux compatibility promise. No Python runtime, root, GPU/NIC, CUDA/NCCL, serving engine, container runtime, cloud account or object store is needed. No `prototypes/aether-poc/` package exists at plan authoring.

## Plan of Work


### M3.1: establish a verifiable starting point


Inspect the checkout and applicable repository instructions, preserve unrelated edits, then run the prerequisite commands below. Record actual toolchain/OS identities, starting Git revision if available and working-tree differences in `evidence/prototype/milestone-3/inputs.json`. Do not equate a Git revision with uncommitted files. Record SHA-256 for the M2 artifact list, fixture manifest/ledger, experiment registry/status, twelve proposed budgets, environment manifest and the validation crate source/lockfiles. Verify the accepted M2 checksum list before work and again at acceptance. M3 should add files and modify only current plans/index, leaving all accepted M1/M2 artifact bytes intact.

If a prerequisite fails, identify the drift rather than regenerating accepted hashes. No shared validation source change is planned. If one becomes necessary, first preserve a complete verifiable M2 predecessor snapshot at original relative paths, document why in both plans, and distinguish new code evidence from accepted M2 bytes. Never silently modify shared code to make a new command pass.

Acceptance for this task is a recorded, passing prerequisite inspection and a clear list of immutable inputs. Starting the harness does not require budget agreement because this is a bounded local functional check, not an evaluated PX runtime experiment. Its proposed test limits below must remain labeled implementation test limits, never user-agreed production budgets.

### M3.2: bootstrap the separate Rust package and interfaces


Create `prototypes/aether-poc/Cargo.toml`, its own `Cargo.lock`, `.gitignore` containing `/target/`, `README.md`, `src/lib.rs`, `src/main.rs`, `src/config.rs`, `src/fixture.rs`, `src/endpoint.rs` and `src/report.rs`. Use package/binary name `aether-poc`, edition 2024, rust-version 1.99 and publish false. Depend on `aether-validation` by relative path `../../tools/aether-validation`; pin serde 1.0.228 with derive, serde_json 1.0.149 and test-only tempfile 3.27.0, matching existing tooling. Use the library's public `sha256`. Generate and inspect this package's lockfile offline from the existing cache. If a genuinely needed dependency is unavailable, use the environment's normal approval process rather than bypassing network restrictions. Do not create a root Cargo workspace or copy build directories into evidence.

Implement the interfaces below and CLI parsing with Rust standard-library arguments. Reject unknown commands/flags, duplicate flags, invalid numbers and missing required values before opening sockets. `--root` is the repository root and defaults to the current working directory; canonicalize it. Accept only the supported relative manifest `fixtures/prototype/manifest.json` for schema v1. This avoids pretending arbitrary manifests can pass the repository-specific construction contract. List/validate/run print one structured JSON result on stdout; diagnostics go to stderr. Exit 0 means the requested harness action succeeded, 1 means validation/I/O/action failed and 2 means bad CLI usage. Do not print a success object after a failure.

`docs/prototype-dependencies.md` must list exact chosen/locked dependencies, Rust reference, why each is present, the local validation-tool dependency and the deferred M6 Arrow/Parquet/DataFusion compatibility check. Experimental `observation`, `identity`, `derive`, `journal`, `store`, `query` and `replay` modules from the master are future organization options. Do not add successful no-op implementations just to populate that outline. Future command requests must return an explicit `not_prototyped` error with a nonzero exit.

Acceptance is a compiling independent package with help/invalid-command behavior, a lockfile, clear dependency notes and no real-system access or false feature claims.

### M3.3: load and list validated fixture inputs


In `fixture::load_bundle`, invoke `aether_validation::check_fixtures(root)` before exposing any fixture. Bound manifest reads to 1 MiB before allocation, parse its required schema fields with strict typed serde structures, and cross-check the summary with the loaded index. Preserve group/variant IDs, source/requirement/association references, artifact roles, bytes/hashes and expected rejection metadata. Return all 12 groups and 36 variants sorted by ID. Listing must distinguish endpoint-present variants from JSON-only variants and keep the two intentional invalid endpoint declarations visible. Emit the counts 12/36/82/2, synthetic true, engine experiments not_run and runtime readiness false.

Use a confined file reader: reject absolute/traversal paths and symlinks, verify every component stays under the canonical fixture root, bound reads to the declared length and at most 1 MiB, and recompute size/hash when opening an artifact for use. Hold validated endpoint bytes in memory for serving. Document that a repository concurrently writable by an adversary is outside this local harness test boundary; validation followed by a later read is not a production secure acquisition guarantee. Tests must still reject ordinary changed-file, path-escape and symlink cases.

Keep expected-answer artifacts out of the runtime payload API. The construction checker may inspect oracles to validate the bundle, but scenario functions receive input/endpoint bytes and declared hashes only; they cannot copy an oracle into an actual observation. The M3 round trip checks bytes against their declared input identity and does not consume semantic assertions.

Acceptance is read-only listing and validation of the complete accepted bundle, clear errors for a missing group/file or bad hash and no writes to accepted fixture/status files. `validate-fixtures` reports construction and harness-load success, not execution of PX experiments.

### M3.4: serve one bounded local endpoint


Implement `endpoint::spawn_fixture_server` using `std::net::TcpListener` bound only to `127.0.0.1`, with port 0 by default so the operating system selects an available port. Only this numeric address is supported; reject wildcard, hostname, public, private-network and IPv6 addresses before binding. No user-supplied fetch URL is supported in M3. The client receives the socket address created by this server; it cannot scan or contact other endpoints. The CLI selects one validated variant with one endpoint artifact. Reject missing/ambiguous endpoint selection before binding.

Support a finite HTTP/1.1 subset: `GET /metrics` returns 200, `Content-Type: text/plain; version=0.0.4`, an exact Content-Length, `Connection: close` and the original bytes, including intentionally malformed metric text. Unknown paths return 404; other methods return 405; malformed request lines/headers return 400 and oversized headers return 431. Limit request headers to 8 KiB, apply one-second socket read/write timeouts and do not accept request bodies, upgrades, persistent connections, chunking or proxying. Count every accepted connection toward a configured finite maximum; a partial request must not hold the server indefinitely. A bad peer is reported and closed; the server continues only within its remaining connection/run bounds. No request can select a filesystem path or execute a fault description.

For `serve-fixture`, defaults are max requests 1 and maximum run duration 5 seconds; enforce ceilings 16 requests and 30 seconds, both positive. On expiry stop and report `deadline_exceeded` when the requested connection count was not reached. Use nonblocking accept with a bounded shutdown check, and always join the owned server thread. `GET /metrics` success alone cannot override a timeout or a failed worker. Emit a flushed JSON startup line with actual local URL, variant ID and synthetic true; send diagnostics to stderr and a final JSON shutdown line when done. A standalone process may also be stopped by the user; tests must prove orderly bounded shutdown without relying on manual signals.

The local HTTP client must bound header/body reads, require 200, exact supported Content-Length and connection-close behavior, and reject truncated/oversized responses, unsupported framing and invalid status lines. It connects only to the owned listener. It never follows redirects or resolves names. These are deterministic test limits, not a reusable production HTTP implementation.

Acceptance is a real local request returning the exact indexed bytes, a listener reachable only through 127.0.0.1, bounded failure behavior, explicit errors on occupied ports and no server left running after a completed or failed scenario.

### M3.5: demonstrate a deterministic endpoint round trip


Implement `run-scenario --scenario endpoint-roundtrip --variant PX01-vllm-v1`. Load and validate the bundle, select endpoint input, start the owned ephemeral-port server, fetch `/metrics` once, close/join the server and compare actual response length/SHA-256 to indexed input length/SHA-256. Only then emit one deterministic `ScenarioReport`. Its stable fields include schema 1, synthetic true, scenario/variant/bundle IDs, input manifest hash, expected/actual byte counts and hashes, `transport_status=passed`, `harness_status=passed`, `semantic_execution_status=not_run`, `runtime_acceptance_readiness=false` and `production_support_claimed=false`. Do not include ephemeral ports, wall time, random output paths or elapsed measurements in this stable report. Two identical invocations must produce identical report bytes.

The command supports any selected endpoint-bearing variant, including declared malformed payloads; `semantic_execution_status` remains not_run for all of them. A JSON-only variant returns `endpoint_unavailable`. A scenario such as `counter-rate`, query, collection or event replay returns `not_prototyped` rather than reading expected values or fabricating results. Print the JSON report to stdout only; callers can capture it into a fresh evidence file. There is no persistent output store in M3 and no need for an `--output` directory.

Define future observation and metadata relationship shapes in `docs/prototype-harness.md`, explaining native name/type/labels, publisher versus measured subject, virtual interval/clock, unknown/ambiguous state, relationship evidence and validity windows. This is a documented handoff, not actual parsed observations or a production API. `report.rs` must not claim fields are observed merely because those shapes exist.

Acceptance is two byte-identical reports from real local round trips, a matched indexed byte identity, explicit not-run semantic state and process termination with no leaked listener on success or failure. This exercises transport and lifecycle only; M4 will implement the first metric behavior.

### M3.6: prove behavior and capture actual evidence


Add integration tests under `prototypes/aether-poc/tests/` using temporary copies or small test bundles; never corrupt the accepted workspace. Cover complete listing, invalid CLI/config/manifest/hash/path/symlink cases, missing endpoint, deterministic reports, exact response bytes, intentionally invalid payload transfer without semantic success, HTTP errors and request bounds, partial/truncated client behavior, occupied port, run deadline, shutdown after error and unsupported future commands. Bind port 0 in tests, own every child/thread and use hard test deadlines. Make assertions about observable behavior, not source-code spelling or mere struct existence. No arbitrary external URL test should actually contact that URL: prove rejection before connect/bind. Do not assert invented test counts; record actual passed/failed counts after running.

Run formatting, strict Clippy, engine tests and all existing validation-tool tests, plus evidence/traceability/fixtures checks and original/current artifact hashes. Write actual logs under `evidence/prototype/milestone-3/`; record commands, identities, test count and limitations in `evidence/prototype/milestone-3.md`. Add a new M3 `artifacts.sha256` for delivered source, Cargo files, documentation and representative deterministic reports; do not hash a checksum file into itself. Hash logs/input index separately in the report if useful. Retain failed trials as clearly labeled history, never as final passing evidence.

If the environment denies loopback sockets, report the permission failure and use normal environment approval. Do not replace the HTTP acceptance test with an in-memory call and claim the listener worked. Missing runtime acceptance agreements remain missing; a harness-functional pass is its own verdict.

### M3.7: review, synchronize and hand off


Mark M3 complete only after observable acceptance, regressions and unchanged accepted-input checks pass. Update this plan's seven tasks and living sections with actual outcomes, `EXECPLAN.md` with aggregate completion/evidence and the index with current status. Do not rewrite M1/M2 accepted ledgers or mark any PX experiment executed. Explicitly retain all 250/58 original tracking and pending production feasibility questions.

Record the next implementation milestone as M4: parse native endpoint payloads and calculate selected, justified values against independent oracles. M4 must validate histogram types, reset/counting boundaries, five successful requests per second for PX02 and unknown results where supplied data is insufficient. Do not begin M4 merely to declare M3 complete.

## Concrete Steps


Run all commands from `/home/mbellary/work/projects/DeepMon`. These initial commands exist now; their outputs are prerequisite evidence, not M3 implementation results:

    git status --short
    rustc --version --verbose
    cargo --version --verbose
    sha256sum --check --quiet evidence/prototype/milestone-2/artifacts.sha256
    sha256sum --check --quiet evidence/prototype/milestone-1/baseline-before.sha256
    cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- evidence --root .
    cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- traceability --root .
    cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- fixtures --root .

Expected current fixture summary is 12 groups, 36 variants, 82 files, two intentionally invalid payloads, zero executed experiments, readiness false and fixture construction pass. A quiet checksum command exits 0 when every named file matches. Record each exit code; do not hide an earlier failure behind a later successful command.

After creating the M3 package, generate the new lockfile once, inspect it, then use locked commands:

    cargo generate-lockfile --offline --manifest-path prototypes/aether-poc/Cargo.toml
    cargo fmt --manifest-path prototypes/aether-poc/Cargo.toml -- --check
    cargo check --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml
    cargo test --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml
    cargo clippy --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml --all-targets -- -D warnings
    cargo test --locked --offline --manifest-path tools/aether-validation/Cargo.toml

These harness commands were specified targets at plan authoring and are now implemented and verified:

    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- validate-fixtures --root . --manifest fixtures/prototype/manifest.json
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- list-fixtures --root . --manifest fixtures/prototype/manifest.json
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- run-scenario --root . --manifest fixtures/prototype/manifest.json --scenario endpoint-roundtrip --variant PX01-vllm-v1
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- serve-fixture --root . --manifest fixtures/prototype/manifest.json --variant PX01-vllm-v1 --bind 127.0.0.1 --port 0 --max-requests 1 --max-run-ms 5000

For the standalone command, the first JSON line prints the actual local URL. A local browser request to that `/metrics` URL should return invented metric text, then the server exits after its one connection; browser extra requests need not be served. The automated `endpoint-roundtrip` command owns both ends and is the authoritative repeatable test. CLI help must explain these differences. Save two round-trip JSON outputs to fresh evidence files and compare them with `cmp`; successful comparison exits 0. All build/test/output paths belong to the new package or fresh M3 evidence directory.

## Validation and Acceptance


Plan-authoring acceptance requires PLANS.md sections/format, explicit pending implementation states, executable command/interface specifications, synchronized master/index and unchanged accepted artifacts. Run prerequisite fixture/hash checks; no new engine test execution is claimed by writing this plan.

Implementation acceptance requires an independently buildable locked Rust package, complete read-only listing/validation of 12/36/82, a real bounded loopback HTTP exchange returning indexed bytes, two identical deterministic reports and no listener/thread left alive. Bad manifests/hashes/paths/configurations must fail clearly. Intentional malformed metric content must transfer unchanged while semantic execution remains not_run. Unsupported calculations/queries/collectors must return nonzero not_prototyped errors. All new tests/static checks and prior 70 integration tests must pass; record actual counts and platform limitations.

The M3 report must explicitly distinguish fixture construction, harness transport, semantic PX execution, real-source compatibility and runtime budget agreement. Only the first two can have evidence of success here. Every PX execution remains not_run; real schemas/capabilities remain unverified, proposed budgets remain unagreed and runtime readiness remains false. Unchanged M2/source/requirement/association hashes are required evidence. A harness success cannot establish zero overhead, microsecond physical resolution, data-loss guarantees or production readiness.

## Idempotence and Recovery


Validation/listing are read-only. Repeated named scenarios create only owned ephemeral listeners and report to stdout; they must not modify accepted inputs. Select fresh evidence destinations and refuse overwriting conflicting prior evidence. Preserve existing user edits. Use temporary test directories with Rust-owned cleanup; never remove broad `/tmp` patterns or repository directories.

On startup failure close any acquired listener and join owned threads. On client failure request server shutdown and join before reporting failure. On a standalone deadline return a clear failure and release the port. Retry occupied-port failures with port 0. Retry interrupted builds with the same lockfile rather than changing dependency versions. A repeat demo must work without manually killing a leftover process.

If accepted bytes differ, stop the acceptance claim, explain drift and restore only from verified predecessor bytes when appropriate; do not overwrite user work automatically. Do not rerun fixture authoring to reset an advanced ledger. Keep old acceptance logs/hashes historical and add corrected M3 evidence under new names. Record unresolved sandbox/permission failures rather than presenting skipped socket checks as success.

## Artifacts and Notes


Delivered artifacts are the isolated `prototypes/aether-poc/` package/lock/tests/README, `docs/prototype-dependencies.md`, `docs/prototype-harness.md`, `evidence/prototype/milestone-3.md` and adjacent input identities/logs/checksum list. The three current plan/index files track progress. Accepted predecessor artifacts remain unchanged.

The actual `evidence/prototype/milestone-3/roundtrip-1.json` and `roundtrip-2.json` contain these fields among their identity/count/hash values:

    synthetic: true
    scenario: endpoint-roundtrip
    variant_id: PX01-vllm-v1
    transport_status: passed
    harness_status: passed
    semantic_execution_status: not_run
    runtime_acceptance_readiness: false
    production_support_claimed: false

## Interfaces and Dependencies


In `src/config.rs` expose `parse_args(args: &[String]) -> Result<Command, PrototypeError>`. `Command` represents validate, list, serve and named scenario actions with validated repository root/manifest and bounded server options. `PrototypeError` in `src/lib.rs` carries a stable code, human-readable context and optional fixture ID; implement Display/Error. CLI usage errors and runtime errors keep distinct exit codes.

In `src/fixture.rs` expose `load_bundle(root: &Path, manifest: &Path) -> Result<ValidatedBundle, PrototypeError>` and `select_endpoint(bundle: &ValidatedBundle, variant_id: &str) -> Result<EndpointInput, PrototypeError>`. `ValidatedBundle` retains identities, references, counts and a canonical root. `EndpointInput` contains variant/bundle/manifest identities, declared byte length/hash and validated endpoint bytes; it contains no oracle assertions. Use bounded reads and SHA-256 rechecks as specified above. Do not expose arbitrary file reads to scenario code.

In `src/endpoint.rs` expose `spawn_fixture_server(input: EndpointInput, config: ServerConfig) -> Result<FixtureServer, PrototypeError>`, `fetch_fixture(server: &FixtureServer) -> Result<Vec<u8>, PrototypeError>` and `run_endpoint_roundtrip(input: EndpointInput) -> Result<ScenarioReport, PrototypeError>`. `ServerConfig` enforces address, port and finite request/deadline limits. `FixtureServer` exposes its bound socket address and an explicit shutdown-and-join operation; cleanup must also occur on error. The scenario consumes actual received bytes to build its report and compares their identity, without metric parsing or oracle lookup.

In `src/report.rs` define serde-serializable sorted fixture listings and `ScenarioReport` with the stable fields above. Use deterministic vector/map ordering. Keep server-startup/shutdown events separate from the deterministic scenario report. The binary dispatches errors without partial success, and tests exercise the binary as well as the library.

Use Rust standard-library filesystem/TCP/thread/time facilities, exact serde/serde_json pins and the existing local validation library. Test-only tempfile owns disposable copies. No shell/Python generator, HTTP framework, asynchronous runtime, host/device SDK, eBPF loader, runtime socket client or cloud/database service is needed. Arrow/Parquet/DataFusion will be selected and compatibility-tested in M6, not implied by an M3 dependency document.

## Revision Notes


2026-11-01 22:41 IST: created the detailed M3 execution plan following the user's request. Defined seven pending Rust implementation tasks, local server/round-trip behavior, input/oracle separation, bounded failures, acceptance evidence and master/index tracking. Deferred unused Arrow/Parquet/DataFusion selection from M3 to its M6 executable experiment while retaining requirements. Verified current fixture construction and accepted checksums; no harness implementation or runtime acceptance is claimed.

2026-11-01 22:46 IST: verified plan structure/current links, seven pending tasks and synchronized master/index states. Existing Rust integrity/traceability/fixture checks and accepted artifact hashes pass. Recorded authoring evidence separately from future M3 implementation acceptance.

2026-11-01 22:58 IST: executed M3.1–M3.7 in Rust. Delivered the independent package/lock, complete fixture listing/validation, bounded real loopback server/client, deterministic byte-roundtrip and handoff docs. Passed 29 new tests and all 70 regressions, strict Clippy/format/checks and unchanged-input verification. Socket execution used approved access after a recorded sandbox denial. Two CLI reports match exactly; all PX semantic/real capability/budget states remain pending. Master/index synchronized; M4 is next.
