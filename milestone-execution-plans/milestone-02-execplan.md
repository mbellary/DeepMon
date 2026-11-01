# Milestone 2: create reproducible synthetic fixtures and explicit expected answers


This ExecPlan is maintained under repository-root `PLANS.md`. Keep `Progress`, `Surprises & Discoveries`, `Decision Log`, `Outcomes & Retrospective` and the bottom revision note current at each stopping point. The workspace is `/home/mbellary/work/projects/DeepMon`. The master is `EXECPLAN.md`; this detailed progress record is `milestone-execution-plans/milestone-02-execplan.md`. The shared index is `milestone-execution-plans/README.md`.

Current task: the user authorized Milestone 2 execution. M2.1–M2.7 are now complete for synthetic fixture construction and validation, with actual evidence at `evidence/prototype/milestone-2.md`. Milestone 1 remains accepted and its predecessor bytes are preserved. All twelve telemetry experiments remain unexecuted; Milestones 3–8 remain pending. Next is the separate Rust prototype harness and loopback fixture server.

## Purpose / Big Picture


Give the next Rust harness a small, repeatable set of invented endpoint exports, identities, timestamps and faults with answers that a reviewer can check by hand. After Milestone 2 executes, a developer can inspect a fixture manifest, find every input and expected outcome for PX01–PX12, and run a Rust command that accepts the complete valid bundle while rejecting tampered, inconsistent or falsely attributed fixtures.

A fixture is invented input for a later experiment. A manifest is the index of its files, identities, hashes, source references and expected outcomes. An oracle is the independently written answer against which a later implementation is compared. Milestone 2 verifies fixture construction, consistency and evidence links. It does not implement or execute the inference observability engine, enrich real telemetry, replay real faults, prove physical timing accuracy or run the twelve experiment behaviors to completion.

All code, generation helpers and tests remain Rust. Keep all 250 metric requirements, all 58 associations and the current source routes; the selected 23 metrics and 42 associations are a representative experiment set, not a product-support boundary. There is no existing cluster. No real engine/device access, network listener, container control socket, probe, workload change, cloud service or external notification is required. Production architecture remains unscoped; DeepInfer remains deferred.

## Progress


Times use Asia/Calcutta, UTC+05:30. Checked items are actual observations or completed authoring work, not claims of future implementation.

- [x] (2026-11-01 21:44 IST, inspected) Read `PLANS.md`, the master Milestone 2 scope, Milestone 1's registry/status/scope, validation-tool command interfaces and accepted artifact hashes.
- [x] (2026-11-01 21:44 IST, prerequisite checked) Ran the existing Rust traceability command: 250 metrics, 58 associations, twelve specifications, 23/42 selected, no missing/duplicate IDs, zero executed experiments and runtime readiness false.
- [x] (2026-11-01, plan authoring) Created this detailed Milestone 2 plan and linked it from the master and folder index. Authoring validation is recorded separately from implementation.
- [x] (2026-11-01 21:52 IST, plan validation) Checked PLANS.md sections/format, existing links, all twelve PX IDs and synchronized pending states. Evidence/traceability prerequisites pass; accepted M1 artifact and original-input hashes match. No fixture payload, ledger or code implementation was created.
- [x] (2026-11-01 22:13 IST, verified) M2.1: verify the accepted inputs, preserve the Milestone 1 tool/input snapshot and define the fixture manifest/record interfaces.
- [x] (2026-11-01 22:13 IST, verified) M2.2: author six-producer endpoint variants, snapshots and explicit expected native observations for PX01–PX04 and PX12.
- [x] (2026-11-01 22:13 IST, verified) M2.3: author identity/inventory/lifecycle inputs and expected associations for PX04–PX07, including unknown and conflicting cases.
- [x] (2026-11-01 22:13 IST, verified) M2.4: author virtual events, uncertainty records, controlled fault descriptions and hand-checkable oracles for PX03 and PX08–PX12.
- [x] (2026-11-01 22:13 IST, verified) M2.5: extend the Rust validation tool with the `fixtures` command and isolated positive/negative tests; preserve evidence/traceability behavior.
- [x] (2026-11-01 22:13 IST, verified) M2.6: create the separate fixture-status ledger, run fixture acceptance and regressions, and record actual logs, hashes and limits in `evidence/prototype/milestone-2.md`.
- [x] (2026-11-01 22:13 IST, verified) M2.7: synchronize this plan, the master and folder index; mark Milestone 2 complete only when all outputs and checks pass, then hand off to Milestone 3.

## Surprises & Discoveries


Observation: exact device coverage requires explicit compatible event clocks, not only valid interval endpoints.

Evidence: final review tightened temporal-record clock requirements and added missing-clock/cross-clock stream rejection tests. The final suite passes 40 fixture tests plus all 30 Milestone 1 regressions; no default local clock is invented for a missing domain.

Observation: the completed bundle has twelve groups, 36 variants and 82 payload files, including two intentionally malformed exposition samples.

Evidence: the Rust fixtures command validates every indexed artifact and expected rejection. Forty new integration tests and all thirty prior tests pass; strict Clippy/formatting and unchanged baseline/planning checks pass. Unknown timing/transport/ownership and queue-loss cases retain qualified outcomes.

Observation: ledger advancement is a deliberate change, not a reason to regenerate authored files.

Evidence: authoring produced 85 files and an identical rerun produced zero. After actual construction evidence was added to the separate ledger, repeated read-only fixture checks passed; authoring protects advanced or manually changed files instead of resetting them. Experiment execution stays not_run.

Observation: fixture creation can finish while experiment execution remains unperformed.

Evidence: Milestone 1's registry intentionally requires `execution_status=not_run`, empty actual results and false runtime readiness. Milestone 2 must add fixture-authoring evidence in a separate ledger, rather than overwrite those states with a misleading synthetic success.

Observation: the Milestone 1 artifact list includes the validation tool's current source paths.

Evidence: `evidence/prototype/milestone-1/artifacts.sha256` records Cargo files and Rust source/tests as well as planning files. Extending the tool changes those live paths. Preserve a verifiable copy of the accepted bytes first; do not rewrite old expected hashes or pretend they describe the later tool revision.

Observation: the registry's rate case is a request-success rate, while an older master paragraph called its answer tokens per second.

Evidence: PX02 links Triton's request-success counter to L1-002/L1-006 and expects (150−100)/10 = five requests per second. This plan and the master use that population and unit. No requirement or source definition is changed to make the example fit.

## Decision Log


Decision: this turn produces a detailed plan only; M2.1–M2.7 remain pending.
Rationale: the user's concrete instruction is to create and track the milestone plan, then update both plans. A written execution plan is not evidence that its fixture bundle or code exists.
Date/Author: 2026-11-01 / Codex, applying user direction.

Decision: extend the existing `tools/aether-validation/` crate for fixture construction checks, without creating the engine package or a fixture HTTP server here.
Rationale: Rust tooling already exists and its locked dependencies cover local JSON/hash/path checks. Milestone 3 remains responsible for the engine harness and serving invented endpoints; Milestones 4–7 exercise the actual experiment behaviors.
Date/Author: 2026-11-01 / Codex.

Decision: leave accepted Milestone 1 planning files, original requirements, source routes and frozen v1 inputs unchanged. Add a separate `docs/prototype-fixture-status.json` for authored/validated fixture progress.
Rationale: preserve the 250/58 baseline and all unresolved meanings, retain reproducible Milestone 1 results, and avoid weakening its validator solely to report later activity as complete.
Date/Author: 2026-11-01 / Codex.

Decision: preserve the accepted Milestone 1 working-tree inputs/tool under a new predecessor snapshot before changing shared code; keep its old hash list intact.
Rationale: old acceptance evidence must remain verifiable even when current code evolves. This is archival input for Milestone 2, not a rewrite of the original v1 bundle or a claim that code was committed.
Date/Author: 2026-11-01 / Codex.

Decision: distinguish intentionally invalid payloads inside a valid fixture bundle from corrupt or untrusted fixture packaging.
Rationale: missing required files, broken hashes and unsafe paths always fail bundle validation. A declared malformed metric sample is accepted as a useful negative fixture only when a specific bounded check rejects it for the independently specified reason; a generic skip/invalid flag cannot bypass validation.
Date/Author: 2026-11-01 / Codex.

Decision: retain proposed budgets and pending agreements. Treat fixture values and offline validation limits as test-data choices, not user-agreed production thresholds.
Rationale: fixture authoring needs finite data and known answers. It does not establish measured overhead, freshness, timing resolution or a runtime acceptance pass.
Date/Author: 2026-11-01 / Codex.

Decision: execute the authorized fixture milestone, accept construction checks and keep engine/runtime acceptance separate.
Rationale: all seven tasks have actual artifacts and evidence. The twelve-group bundle, Rust validation command and 70 integration tests pass without Python or any live source connection. Source/registry/status/budget inputs remain unchanged, and budget agreement/real qualification remain pending.
Date/Author: 2026-11-01 22:13 IST / Codex, applying user direction.

Decision: use a narrowly documented finite exposition checker and explicit construction bounds; preserve unknown associations and stage-boundary facts as authored test data.
Rationale: a general production parser and storage implementation belong to later milestones. Negative samples must exercise exact rejection reasons, but packaging/path/provenance checks cannot be skipped. No fixture result should claim actual engine behavior.
Date/Author: 2026-11-01 22:13 IST / Codex.

## Outcomes & Retrospective


Milestone 2 is complete for its defined construction/validation scope. The fixture manifest indexes twelve groups, 36 variants and 82 payload files; format notes, independent oracles, separate fixture-status ledger and Rust fixtures/author-fixtures commands exist. Actual construction evidence is attached to every ledger entry while experiment execution remains not_run and runtime readiness false.

All 70 Rust integration tests pass (40 new plus 30 original regressions), alongside strict Clippy, formatting, source evidence/traceability and preserved-input checks. The accepted Milestone 1 bytes are archived and verify against their original hash list; original M1 planning records and frozen v1 sources remain unchanged. Current M2 source/data/code/lock hashes and logs are recorded in `evidence/prototype/milestone-2.md` and its adjacent directory.

No engine/harness, source acquisition, physical timing measurement, actual fault replay or real capability validation has occurred. Proposed budgets remain unagreed. The next implementation milestone is Milestone 3: the isolated Rust harness and loopback fixture server. Milestones 3–8 remain pending.

## Context and Orientation


`docs/prototype-experiments.json` is the authoritative registry. It contains twelve PX IDs with requirement/association IDs, profile IDs, source references, future execution milestones, prose input specifications, positive/negative expected results, limitations and budget-record paths. `docs/prototype-status.json` retains 250 complete original metric objects, source/dependency mappings and 58 association definitions. Twenty-three metrics and 42 associations are selected; the other 227 and 16 remain not_prototyped. Do not select extra IDs or omit existing ones just to simplify a fixture.

The four layers are request/generation, serving/cache, distributed execution/fabric and host/GPU infrastructure. E/R/H/I/U are proposed source routes, not execution states. Current totals are E:88, R:35, H:100, I:26, U:1. The historical 26 U rows were reconciled as 25 documented endpoint routes and one semantic gap; the 26 I integration rows are a different set. L1-001's raw TGI request counter does not establish accepted requests; L1-005 inherits that gap. L2-071's raw skipped/speculated-token interpretation remains conditional even though its endpoint route exists. Fixtures preserve such limits.

`evidence/manifest-v1.json` pins ten source inputs E01–E10 and frozen local inputs B01–B11. E01/E02 cover vLLM, E03/E04 SGLang, E05 Dynamo, E06/E07 TGI, E08 Triton, E09 native TensorRT-LLM and E10 the Triton TensorRT-LLM backend. Resolve their paths and hashes through the manifest. Exact repository commits pin research input, not compatible running releases. Invented hardware/event records use the frozen B01/B02/B03/B05 baseline contracts and must not be labeled captured or validated hardware schemas.

C01–C06 identify existing engine endpoint profiles; C07–C09 are selected host/NVIDIA/NIC/RDMA interfaces; C10 is metadata. In this milestone they are simulated shapes, never live connections. An emitter publishes data; its subject is what the data describes. Allocated resources and observed resources can differ. Native engine indexes, PIDs, IP addresses, ordinals and model names alone do not prove physical ownership, replica membership or active fabric use.

A counter accumulates observations and can reset. A gauge publishes a latest value. A histogram gives bucket counts/count/sum, not the original event stream. Prometheus HELP/TYPE describe metric family meaning/type. A fixture with a histogram whose name ends in `_total` must remain a histogram. A response cache is not a KV-prefix cache; Triton's shared pinned pool is not per-model allocation; an engine iteration gauge cannot recover all missed iterations.

Virtual time is an integer supplied in the data, with a unit, clock domain and uncertainty. It is not real elapsed time and must not cause sleeping. Use half-open intervals [start,end) for overlap calculations so adjoining events do not introduce double counting. A clock domain identifies timestamps that can be compared directly; different domains need an explicit offset/bounds record. A rank is a group participant; its number is scoped to a group incarnation, not a universal GPU ID.

The existing Rust crate at `tools/aether-validation/` has `evidence`, `traceability` and non-destructive `generate` commands. Its library exposes `check_evidence`, `check_traceability`, common path/hash/JSON helpers and structured `Violation` errors. Thirty integration tests passed at Milestone 1 acceptance. `Cargo.toml` and `Cargo.lock` pin serde/serde_json, sha2, Git-identity-only sha1 and test-only tempfile. Python must not be invoked, including through old documentation examples. The environment reference is local Ubuntu 26.04.1, Linux 7.0.0-38-generic, Rust/Cargo 1.99.0, target x86_64-unknown-linux-gnu and glibc 2.43. Report any current drift; do not silently install/switch tools or select real hardware.

## Plan of Work


### M2.1: verify and preserve the starting point, then specify the bundle


Inspect the workspace and run the current Rust evidence and traceability commands. Check the Milestone 1 delivered hashes before editing shared code. Record Git base if available, actual toolchain identity and observed differences without assuming changes are committed. Preserve unrelated work. If expected input bytes have drifted, explain the difference and establish a deliberate revision instead of regenerating expected hashes to force a pass.

Create `evidence/prototype/milestone-2/inputs/milestone-1/` as a predecessor snapshot root. Copy the files named in `evidence/prototype/milestone-1/artifacts.sha256` at their original relative paths, its checksum list, the v1 manifest/checksum and all v1 artifacts required by the evidence verifier. Also retain the working metadata table used by the original parser tests and its expected baseline hash. Deduplicate overlapping paths and copy no target directories, secrets or unrelated files. Validate the original M1 artifact hash list against the copy before changing the live tool. The original hash list/report and frozen v1 inputs stay intact. Preserve original-path-to-snapshot mapping in a new M2 input index. Later M1 source-hash verification uses this predecessor root; M2 output hashes describe the changed live tool. Do not edit the old report to make changed bytes appear identical.

Define the authoritative bundle at `fixtures/prototype/manifest.json`, with directories `endpoints`, `metadata`, `events`, `faults` and `expected`. Add `fixtures/prototype/README.md` explaining provenance and how to inspect each parent fixture. No file is a live export. Top-level fields are `schema_version=1`, `bundle_id=aether-synthetic-fixtures-v1`, `synthetic=true`, v1 source manifest SHA-256, Milestone 1 registry/status SHA-256, environment ID and `fixtures`.

Use exactly twelve parent fixture IDs, `synthetic-PX01-v1` through `synthetic-PX12-v1`, matching each registry `planned_fixture_id`. Each parent has its PX ID, exact requirement/association/profile/source links from the registry, evidence kind, invented-field description, claim limits, linked budget path/hash and a nonempty `variants` array. Define positive and negative/incomplete-evidence variants for every parent. Variants have unique stable IDs, a short purpose, bounded virtual-clock description, input artifacts, expected-result artifact, expected input-validity/rejection details and `experiment_execution_status=not_run`. Derive exact variant/file counts from the completed bundle rather than claiming them in advance.

Every artifact reference contains a path relative to the fixture root, role, encoding/media type, byte count and SHA-256. Reject absolute paths, parent traversal and resolved paths escaping that root. Accept only local fixture artifacts; URLs remain provenance text and never retrieval targets. Shared files are allowed with one consistent identity/hash and explicit reuse. Missing files and conflicting role/hash declarations fail. Every managed payload file must be indexed; allow only the documented README and manifest as unindexed control files. Do not store expected runtime results in an `actual_results` field.

Specify JSON record envelopes with `schema_version=1`, `synthetic=true`, fixture/variant IDs, explicit units and source/clock context, plus typed `records`. Metadata records need entity/incarnation IDs and validity intervals. Events need stable event IDs, group/stream identity where relevant, integer timestamps, clock domain and nonnegative supplied uncertainty. Fault records describe a simulated action boundary and expected adapter result, not an OS command. Expected files describe typed assertions, expected quality states and reasoning. A proposed example is:

    {"schema_version":1,"synthetic":true,"variant_id":"PX02-success-rate-v1",
     "assertions":[{"kind":"source_rate","unit":"requests/second",
                    "expected_value":5,"reason":"(150-100)/10"}],
     "experiment_execution_status":"not_run"}

Finalize the exact field names/types in `docs/prototype-fixture-format.md` before implementing consumers. This is a fixture interchange contract, not a production telemetry schema. Acceptance of M2.1 is an intact, preserved predecessor and an explicit format that prevents provenance or expected/actual confusion.

### M2.2: author endpoint exports and snapshots


Write tiny synthetic `.prom` files under `fixtures/prototype/endpoints/`, with observation wrappers identifying capture time, source incarnation and intended field meaning. Values, labels and model aliases are invented and declared. Read the matching pinned source input when selecting each family/type; record specific source ID/path/definition context. A documented family is not a license to invent universal labels. Mark any invented labels separately from native ones. Keep each producer variant within its proposed per-variant input/series envelope.

PX01 needs distinct valid samples for vLLM, SGLang, Dynamo, TGI, Triton and native TensorRT-LLM, plus the separately scoped Triton TensorRT-LLM backend form where required by the source boundary. Include counters, gauges and histograms, the vLLM iteration histogram ending in `_total`, TGI batch size, TensorRT direct KV blocks, Triton shared pinned pool and an iteration-gauge case with absent optional fields. Keep model aliases, publisher/worker labels and framework/backend families source-scoped. Add explicitly malformed type/exposition variants. Their expected rejection must identify the malformed declaration, not an unrelated missing file.

For histogram samples, include cumulative buckets, count and sum with coherent types and units; for missing optional families, omit the family and describe why absence is not a zero. Keep request-level, batch-level, shared-server and device observations distinct. Normalize no field solely from its name. The fixture checker needs only the documented finite subset used here; a complete Prometheus parser and production endpoint adapter remain later work.

PX02 uses success counter snapshots 100 and 150 at virtual times 0 and 10,000,000 microseconds, yielding five source-defined successful requests/second. Add a reset/incarnation variant that cannot emit a negative or unjustified exact rate. Preserve a separate raw TGI request counter and explicitly unavailable accepted-request normalization; do not manufacture an L1-001/L1-005 solution. If later token-rate examples are added, give them their own token population/units and approved links rather than relabel these requests as tokens.

PX03 supplies only aggregate latency and success information to the later consumer. PX04 provides frontend/backend observations and a separately invented overlap/identity record for ten logical requests; the deduplicated counting-boundary answer is ten, not twenty. PX12 has two equal latest-value iteration snapshots with unknown intervening updates. Do not add hidden per-request events to an aggregate-only runtime input merely to make its expected answer computable.

Expected native observations go under `expected/`, preserving original names, labels, types, units, source scopes and unavailable states. Acceptance is every planned endpoint behavior represented by actual bounded data and an independent oracle, without claiming a parser/collector experiment already passed.

### M2.3: author metadata and lifecycle evidence


Write invented JSON records for PX04–PX07 under `metadata/`, including explicit host/boot/process/start/container/replica/group/worker/device/interface identities and validity windows. Use symbolic GPU/NIC model names; they are not selected commercial hardware. Include two replicas with the same model name, a heterogeneous two-device replica, a remote worker, an ambiguous publisher, PID reuse and cgroup movement. Model aliases and container ownership do not automatically identify a serving replica.

PX04 keeps the frontend publisher separate from a remote worker subject. Without target-side worker-location evidence, its host is unknown. A deliberate multiple-owner case retains candidates/ambiguity; it cannot choose the first PID. PX05 supplies replica count 100 once, two GPU members with used/free/total memory 10/10/20 MiB and 20/20/40 MiB, and per-device occupancy 0.5. Missing totals or incomplete membership produce unavailable/partial outcomes rather than invented capacity or composition.

PX06 reuses a host PID with a new start identity and changes cgroup membership over time. Old observations retain old relations; access denial leaves container association unknown. PX07 supplies NIC capability, configured preference and interface counters with no payload-operation transport evidence. Expected active transport and operation-to-NIC relation remain unknown. Include separate network namespace and interface lifetime where identity reuse is relevant.

Every selected metric retains all its registry metadata dependencies, even when evidence is deliberately missing to test uncertainty. Link focus associations and inherited dependencies; do not quietly drop a hard-to-author association or mark all associations resolved. Missing data is distinct from conflict, stale evidence, permission denial and not-applicable. Acceptance is small time-bounded records with hand-checkable positive and incomplete/contradictory outcomes, not discovery of the real host.

### M2.4: author virtual events, faults and independent oracles


Write event and fault descriptions only; do not execute failures against the machine. Virtual timestamps use integer microseconds and a declared origin/domain. Use sequence/order fields to resolve same-timestamp actions explicitly; an unresolvable order remains uncertain. Clock offset evidence must distinguish an exact invented common clock from bounded or unknown relationships.

PX03 includes two proof pairs in the expected/oracle material, not as hidden runtime inputs. For variance, use the explicitly hypothetical bucket set {1 second, +Inf}, without asserting a native engine bucket configuration. Observations [0.25,0.75] seconds and [0.5,0.5] seconds have the same count, sum and buckets with upper bound 1 second, but population variances 0.0625 and 0 seconds squared. For joint goodput, give identical aggregate latencies 0.2 and 2 seconds, success count one and a ten-second completion window with a fixture-only one-second SLO. Success paired with the fast request yields 0.1 requests/second; success paired with the slow request yields zero. Both are consistent with the same separate aggregates. Expected consumer result is insufficient information, not either hidden answer. These examples do not propose a production SLO.

PX08 has a complete two-rank group with arrivals 100/130 and completions 150/190 virtual microseconds in one supplied exact clock. Expected arrival skew is 30, completion skew 40 and rank durations 50/60 microseconds. Include a missing rank, group-generation mismatch and unknown cross-clock offset; those variants cannot claim exact whole-group skew. Any bounded result states the supplied uncertainty and complete/partial group evidence.

PX09 has stream A intervals [0,10) and [20,30), stream B [10,20) and window [0,30), all in virtual microseconds. Expected A gap is ten while fully observed device idle is zero. Removing stream B's coverage evidence makes whole-device idle unknown, not necessarily ten. Preserve event/stream/clock identities and interval-union reasoning without implementing the engine's timing algorithm here.

PX10 identifies two logical records and explicit stages: admitted, simulated-persisted, committed and query-visible. Define variants before/after persistence and commit visibility, duplicate retry and pre-persistence volatile loss. The oracle lists surviving, lost, replayed and visible IDs, not just totals. A protected-boundary variant expects two unique committed records and no uncommitted visibility; a volatile-loss variant declares which record is lost. Fault actions are finite data such as `write_failure` or `commit_visibility_interruption`, with no shell, URL or filesystem operation to execute. Storage mechanisms remain Milestone 6 choices; this fixture specifies required state facts and limits, not a production journal design.

PX11 uses simulated denied/disabled/missing states, attempted forbidden actions and explicitly synthetic sensitive-marker strings. Expected outcomes reject feature activation, privilege escalation and workload mutation; retained expected outputs contain no sensitive marker. No real credential, prompt/response or private payload is used. Keep runtime acceptance blocked when required limits/agreements are missing. Marking a malformed-input variant correctly does not mean that the engine's future policy implementation passed.

PX12 pairs equal gauge snapshots with source-update uncertainty and a separate two-record reject-new queue example: offered three, admitted two, explicitly rejected one. Do not count source updates that were never observed or claim all three were delivered. Specify identities for all three records and separate source freshness, capture delay, persistence and query visibility, leaving unavailable measurements unknown.

Each expected file contains asserted values/states, units, population/window, required evidence, reason and expected unavailable/error behavior. Use exact integers/rational values where possible; any floating-point tolerance must be declared per assertion with a reason. Write oracles independently of future engine operations. A fixture checker may verify simple arithmetic/data consistency, but must not derive expected results by invoking the implementation that will later be judged against them. Acceptance is complete, manually reviewable data and reasoning for all twelve groups and their negative variants.

### M2.5: extend the Rust validation tool


Add `tools/aether-validation/src/fixtures.rs`, export `check_fixtures`, and add the `aether-validate fixtures --root PATH` command. Reuse the current path/hash/JSON error helpers and the existing locked dependencies. Keep existing command names, output invariants, evidence/traceability checks and thirty tests working. Do not invoke Python, replace the production scope, create `prototypes/aether-poc/`, or add HTTP/device/cloud dependencies. The command reads local data only and exits zero only when the complete fixture bundle meets its declared construction contract.

Call `check_evidence` and `check_traceability` with the same root first. Verify the registry/status/budget hashes, twelve exact parent/experiment IDs, exact copied links, versioned source references, invented-field/provenance declarations, all artifact sizes/hashes, local root confinement, unique variant/event/entity identities within their declared scope, complete file indexing, and expected-result schemas. Validate finite payload/record/cardinality limits and deterministic order. Negative payload declarations need a nonempty recognized check/error code and an actual expected rejection; do not skip parsing because a sample is marked intentionally invalid.

Separate construction validation from future engine-result comparison. A valid fixture can expect an unknown result, a denied action or a parser failure. An invalid fixture bundle has missing/corrupt/untrusted packaging, contradictory expected values or wrong references. The tool must fail such a bundle even if it contains a negative scenario. Limit endpoint checking to the finite supported sample grammar and field declarations; report that scope rather than claiming production ingestion compatibility. Arithmetic sanity checks are independent checks of the supplied oracle, not results from an observability engine.

Use Rust integration tests in `tools/aether-validation/tests/fixtures.rs` with isolated temporary bundles. Include valid bundle/all twelve groups, intentional malformed payload accepted only with matching rejection reason, unknown source ID, missing parent/variant/oracle, duplicate identity in one incarnation, invalid clock/units/negative uncertainty, inconsistent bucket/count data, changed expected number, dropped metadata dependency, wrong budget hash, extra unmanaged file, truncated/oversized input, bad hash, path traversal, symlink escape, falsely real/captured provenance and false runtime success/readiness. Each corruption must fail for its intended reason; do not count missing-file failures as tests of deeper semantics. Keep all existing tests as regressions and report actual new/total counts.

Expose `pub fn check_fixtures(root: &std::path::Path) -> Result<FixtureSummary, Vec<Violation>>`; `FixtureSummary` reports groups, variants, files and deliberate negative samples without marking experiments executed. If a generator helps author fixture files, implement it in Rust, restrict writes to the declared output tree and refuse differing existing content by default. No generator is required merely to satisfy this plan. Acceptance is a working fixture-check command with meaningful positive/negative tests, not only compiled structures.

### M2.6: record fixture progress and acceptance


Create `docs/prototype-fixture-status.json` with its own schema version, the pinned registry/status hashes, fixture bundle hash and one entry for each of PX01–PX12. Each entry lists parent/variant IDs and file references, `authoring_status=authored`, `fixture_validation_status` with actual evidence when checked, and `experiment_execution_status=not_run`. Keep separate real/deployed verification at not_run and runtime readiness false. Before acceptance, partial work uses `planned`, `in_progress` or `validation_failed` honestly; the absence of a file cannot become authored. The validator verifies ledger links/hash consistency, without making a circular manifest/hash reference.

Keep the manifest independent of the ledger: the ledger may hash the manifest, but the manifest must not hash a ledger that hashes it back. Likewise oracle files must not recursively hash their own containing manifest. Archive source/registry/status/budget input hashes as immutable predecessor inputs; do not edit Milestone 1's delivered files to record this phase. Original 250/58 tracking remains visible and unchanged, including all unselected requirements.

Run evidence, traceability and fixture checks; Rust tests; strict Clippy; formatting; and baseline-preservation checks. Record actual command output, exit codes, test count, environment drift, source/fixture/input/code/lock hashes, exact group/variant/file counts and missing capabilities. Milestone 1 code changes must be described as expected revision, with predecessor snapshot hashes verifiable. Do not assert the old live-code checksum list still passes after those intentional edits.

Write `evidence/prototype/milestone-2.md`, logs under `evidence/prototype/milestone-2/` and a new checksum index for current delivered artifacts. New tests and construction checks may pass while the twelve actual experiments remain unexecuted and budgets unagreed. Acceptance of M2.6 is reproducible fixture construction/validation evidence with no misleading runtime/capability success.

### M2.7: synchronize plans and hand off


Complete only the actually finished M2 tasks in this plan, then update discoveries, decisions, outcomes and revision notes. In `EXECPLAN.md` mark Milestone 2 accepted only with its report and concise result; leave Milestones 3–8 pending. Update the folder index. If stopped early, retain unchecked items and identify the next concrete action in both plans.

Handoff to Milestone 3 includes the fixture manifest, format notes, status ledger, independent expected outputs, negative variants, input hashes, allowed parser subset, unagreed budget fields and unchanged source limitations. Milestone 3 creates the Rust harness and loopback fixture server. Do not start it automatically as part of this milestone's status updates.

## Concrete Steps


Run from `/home/mbellary/work/projects/DeepMon`. Existing prerequisite commands available now are:

    git status --short
    rustc -Vv
    cargo -V
    cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- evidence --root .
    cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- traceability --root .
    sha256sum -c evidence/prototype/milestone-1/artifacts.sha256

At plan creation, the traceability command actually reported 250 metrics, 58 associations, twelve specifications, 23/42 selected, zero missing/duplicate IDs, zero executed experiments and runtime readiness false. The original thirty-test result is in the accepted Milestone 1 report; rerun tests when implementation starts. Offline mode needs the already cached locked dependencies; a fresh machine needs ordinary authorized dependency access first, not silent package installation.

After preserving the predecessor snapshot and implementing M2.1–M2.5, run the future command and required regression checks:

    cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- fixtures --root .
    cargo test --locked --offline --manifest-path tools/aether-validation/Cargo.toml
    cargo clippy --locked --offline --manifest-path tools/aether-validation/Cargo.toml --all-targets -- -D warnings
    cargo fmt --manifest-path tools/aether-validation/Cargo.toml -- --check
    git diff --check

The following command was a future interface at plan-writing time; it is now implemented and accepted. Its actual output includes:

    fixture_groups=12
    missing_experiment_groups=0
    unindexed_payload_files=0
    synthetic_experiments_executed=0
    deployed_schemas_and_capabilities=not_verified
    runtime_acceptance_readiness=false
    milestone_2_fixture_validation=pass

Also print actual variant, file and declared-negative counts; no count is invented here. A corrupted temporary bundle must exit nonzero and identify the relevant path/variant and reason. An intentional invalid metric payload passes only its negative-sample construction check when it produces the expected error; a tampered payload hash must still fail overall bundle validation. Record actual commands/results rather than copying this target transcript as evidence.

For old Milestone 1 artifact verification after shared code changes, resolve its original relative paths inside `evidence/prototype/milestone-2/inputs/milestone-1/`. Verify current M2 outputs against the new M2 checksum list. Never regenerate the M1 hash list from the new live source.

## Validation and Acceptance


The earlier plan-authoring acceptance was a complete self-contained plan under `PLANS.md`, synchronized master/index status, valid existing prerequisites and references, explicit future outputs/commands and no implementation, fixture or baseline changes. That authoring check alone did not mark M2.1–M2.7 executed; their current checkmarks refer to actual construction/validation evidence. Rust-only tooling remains required.

Execution acceptance requires the actual manifest, format notes, twelve parent groups, positive/negative variants for each, all referenced payloads/oracles, a complete separate fixture-status ledger, preserved predecessor, working Rust fixture validator and evidence report. Registry links must exactly preserve the selected 23 metrics and 42 associations; original tracking retains 250/58 and current research routes. New fixture-specific groups/variants do not add original metric IDs or remove unselected requirements.

A reviewer must be able to choose PX02 and verify five requests/second by hand, choose PX03 and see why identical aggregates do not determine variance/goodput, choose PX05 and verify no double-counted replica and 0.5 occupancies, and choose PX08/PX09 to check complete versus incomplete timing evidence. PX10/PX12 must account for exact IDs and declared losses/rejections, and PX11 must preserve denied/disabled/unknown states and unagreed budget readiness. Each negative construction test must reject its precise fault without modifying live inputs.

All old and new Rust tests must pass, along with evidence/traceability/fixtures commands, strict Clippy, formatting and unchanged-requirement/frozen-input checks. Report actual test counts. A passing fixture-validation verdict does not mean the engine parsed, enriched, recovered or measured these events correctly. Those are future experiment verdicts. No real source, hardware capability, performance budget, timing accuracy or production architecture is approved or verified here.

## Idempotence and Recovery


Use deterministic IDs, stable ordering and fixed virtual timestamps. Rerunning a generator/checker must not change input bytes, source hashes or meanings. Inspect existing output paths before writing; preserve manual notes and refuse conflicting overwrite by default. A revised fixture gets a new version/variant and documented reason rather than rewriting a recorded successful artifact invisibly.

Keep expected answers independent of future engine result code. A failed check is fixed by correcting the declared data or a genuine validator defect with a reason; do not loosen validation to fit a wrong result. Add no live network/device actions as a shortcut. Unsafe paths or missing files fail before parsing; invalid expected provenance cannot be hidden by a negative-sample flag.

Faults run only as data and temporary-tree validator tests. Never fill the real disk, kill external processes, reset devices or enable telemetry. Use new disposable directories for rejection tests; preserve failed evidence if needed for diagnosis. If stopped midway, revalidate v1 inputs and predecessor integrity on resume, then validate the partial fixture set. Missing groups remain incomplete, not implicitly skipped.

## Artifacts and Notes


The completed outputs are this plan, its folder-index/master updates and all M2 artifacts: `fixtures/prototype/manifest.json`, its README and endpoint/metadata/event/fault/expected files; `docs/prototype-fixture-format.md`; `docs/prototype-fixture-status.json`; Rust fixture-validation code/tests; preserved predecessor inputs; and `evidence/prototype/milestone-2.md` with logs/checksums. Those fixture payloads and Rust commands have now been implemented during the authorized execution phase, rather than claimed from plan writing.

Input facts are twelve PX groups, 250 metrics/58 associations retained, 23/42 selected, v1 source evidence, local synthetic environment and unagreed proposed budgets. PX01–PX12 are experiments; their variants are not additional requirements. E/B/C/L/M IDs retain their existing meanings. The first native timing/metadata examples remain invented and explicitly limited.

## Interfaces and Dependencies


Reuse `tools/aether-validation/Cargo.toml` and Cargo.lock. New module `src/fixtures.rs` exports `check_fixtures` and typed `FixtureSummary`/manifest parsing helpers; `src/lib.rs` exposes the API and `src/main.rs` dispatches `fixtures`. `tests/fixtures.rs` exercises construction and rejection cases independently of later engine code. Existing `evidence`, `traceability` and `generate` behavior stays compatible; a new fixture generator, if useful, must have a distinct command and must not rewrite Milestone 1 outputs.

Define manifest/record/oracle types in Rust with serde and validate required fields, finite counts, explicit units and references. Use integer virtual time and either integer/rational expected quantities or explicitly declared finite tolerances; reject NaN/infinity/negative uncertainty. Keep input decoding, manifest validation, expected-oracle validation and future runtime-result comparison as distinct concerns. No production telemetry envelope or asynchronous execution architecture is implied.

The separate fixture-status ledger references the manifest hash and original registry/status hashes. Its validation outcome describes this tool run only; experiment execution and real verification remain separate not_run fields. Construction validation can be recorded while runtime budgets remain proposed/unagreed. Preserve the collection contract rather than treating a fixture budget as permission to start real collection.

At every stopping point, update detailed Progress/evidence first, then master milestone status and next action, then index status if changed. At accepted completion, hand off file formats and known limitations to Milestone 3 without implying that the Rust engine or synthetic experiment execution has already happened.

## Revision Notes


2026-11-01: created the detailed Milestone 2 plan under `PLANS.md`. Specified fixture formats, all twelve experiment groups, independent expected answers, metadata/timing/fault cases, Rust-only construction validation, predecessor preservation, separate fixture-progress tracking and acceptance/handoff. Corrected the master's old rate-example unit to match PX02's request-success population. Only planning and prerequisite inspection are complete; M2 execution remains pending. Requirements, accepted M1 artifacts, source routes and frozen v1 evidence are unchanged.

2026-11-01 21:52 IST: completed Milestone 2 plan-authoring validation and synchronized the detailed/master progress records. Existing evidence and M1 delivered artifacts remain intact. All M2 execution tasks remain pending; no fixture, runtime result or validator implementation is delivered by this update.

2026-11-01 22:13 IST: executed and accepted M2.1–M2.7. Preserved a verified M1 predecessor, authored twelve fixture groups/36 variants/82 payload files and independent expected answers, added Rust validation/authoring and a separate evidence-backed ledger, and passed 70 integration tests plus Clippy/format/integrity checks. Evidence and current hashes are in `evidence/prototype/milestone-2.md`. Master/index synchronized. Milestone 3 harness work is next; no engine experiment, runtime budget or deployed capability is marked verified.
