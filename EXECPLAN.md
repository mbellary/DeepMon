# Build Aether proof-of-concept prototypes with synthetic fixtures


This ExecPlan is a living document. Keep `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes & Retrospective` current at every stopping point. Maintain it according to repository-root `PLANS.md`, which preserves the user's uploaded planning instructions. The workspace is `/home/mbellary/work/projects/DeepMon`.

Milestones 1–3 are accepted for specification/traceability, fixture construction and the synthetic Rust harness respectively. Evidence is in `evidence/prototype/milestone-1.md`, `milestone-2.md` and `milestone-3.md`. The user authorized M3 execution; its detailed plan records all seven completed tasks. Milestones 4–8 remain pending. Next is M4 native endpoint parsing and selected calculations. All twelve PX semantic experiments remain not_run; no real engine/hardware acquisition or production work has occurred.

## Purpose / Big Picture


Build a Rust proof of concept that tests Aether's ideas with intermediate prototypes and clearly labeled synthetic fixtures. A synthetic fixture is invented test data with known answers. It can imitate an engine metric endpoint, GPU inventory, network event or storage failure without running the actual system. The result should help us decide what is worth carrying into the later production-ready bare-metal engine.

After this plan is implemented, a developer can run one local demonstration: serve synthetic engine metrics, collect them, attach justified simulated metadata, calculate selected metrics, save records as Parquet, query them with DataFusion, and replay selected delays and failures. The demonstration must preserve counts, show missing evidence honestly and recover from the interruptions included in its tests. It must not claim real engine compatibility, hardware support, microsecond accuracy or production reliability.

**This plan ends at the proof of concept and its evidence report.** Production architecture and implementation have not been designed or scoped. Production collectors, deployment, hardening, security qualification, real-cluster benchmarking and release work are outside this plan. The prototype provides experiments, fixtures, evidence and possible reusable components; none is automatically an approved production design.

The product requirements do not change. Keep all 250 metric-family requirements, all four observability layers and the 58 metadata associations. Keep the intended Rust implementation, no-application-code-change goal, fine-grained visibility goal, and open storage/query direction without a required central ClickHouse/time-series database. Testing a representative subset does not remove the other requirements. Unproven requirements stay visible for later investigation.

## Progress


Times use Asia/Kolkata, UTC+05:30. Entries marked verified record an inspection time, not an invented historical completion time.

- [x] (2026-11-01 19:58 IST, verified) Created the 250-family requirements catalog across four layers in `aether-metrics-catalog.md` and `.json`.
- [x] (2026-11-01 19:58 IST, verified) Created `engine-native-metrics-baseline.md` for the software reviewed first: vLLM, SGLang and NVIDIA Dynamo. These are not an exclusive support list.
- [x] (2026-11-01 19:58 IST, verified) Created `metadata-enrichment-matrix.md` with 58 associations and rules for identity, evidence, time validity and ambiguity.
- [x] (2026-11-01 19:58 IST, verified) Created the 250-row source-route matrix. Its E:59, R:39, H:100, I:26 and U:26 counts describe research classifications, not working support; mappings affected by the earlier engine-scope mistake remain provisional.
- [x] (2026-11-01, planning work) Preserved the uploaded requirements for plans in `PLANS.md` and created an execution plan.
- [x] (2026-11-01, scope correction) Clarified that reviewing three engines first did not reduce the 250 requirements or exclude other sources.
- [x] (2026-11-01 20:26 IST, plan revision) Limited this document to intermediate prototypes and synthetic fixtures. Removed production implementation, qualification and release milestones from its executable scope.
- [x] (2026-11-01 20:49 IST, collection contract) Defined `zero-application-change-collection-contract.md` and `collection-acceptance-budget-template.json`: ten initial interface profiles, permitted/restricted actions, missing-data and durability semantics, and budget agreement/evaluation rules. Numeric limits and deployed compatibility remain unset; no collection or prototype milestone has started.
- [x] (2026-11-01 21:00 IST, evidence preparation) Captured ten upstream documentation/source inputs at exact commits with Git blob and SHA-256 integrity checks. Added `evidence/manifest-v1.json`, `evidence/reconciliation-v1.json` and `validation-evidence-baseline.md`; all 26 historical U rows have pinned references, with 25 E routes and L1-001 still U.
- [x] (2026-11-01 21:00 IST, environment selection) Recorded the inspected local Ubuntu/kernel/Rust/Cargo reference in `validation/first-environment-v1.json` for synthetic tests. User confirmed no existing cluster. No engine/hardware support or portable machine image is implied.
- [ ] Real deployed-schema/capability validation: pending an available test environment or authentic captures. Versioned documentation inputs exist; no deployed schema, optional feature, hardware capability or source freshness has been verified. This is later qualification, not a requirement to connect to a real cluster in the synthetic milestones.
- [x] (2026-11-01 21:02 IST, evidence integrity) Verified ten upstream inputs, 24 checksummed artifacts and all 26 historical U IDs. A modified temporary copy was correctly rejected. Recorded results in `evidence/verification-v1.md`; these are integrity checks, not fixture, deployed-schema or hardware tests.
- [x] (2026-11-01 21:14 IST, Milestone 1 planning) Created the shared milestone-plan folder/index and `milestone-execution-plans/milestone-01-execplan.md` under `PLANS.md`. The detailed plan tracks M1.1–M1.7, twelve experiment specifications, complete 250/58 tracking, proposed synthetic budgets, traceability checks and acceptance. No execution deliverable or experiment is marked complete.
- [x] (2026-11-01 21:15 IST, milestone-plan validation) Checked master/detail structure, references, original IDs and pending-task consistency; baseline/frozen evidence hashes remain unchanged and the existing integrity checker passes. Milestone 1 execution outputs remain pending.
- [x] (2026-11-01 21:24 IST, Rust-only plan revision) Updated the master and detailed milestone plans to use a Rust validation-tool crate, native evidence/traceability commands, Rust record-generation helpers and tests. No port or milestone execution was performed; the historical verifier and frozen records remain intact.
- [x] (2026-11-01 21:39 IST, Milestone 1 accepted) Completed M1.1–M1.7 in Rust: scope, twelve experiment specifications/budget proposals, complete 250-metric/58-association tracking and locked evidence/generation/traceability tooling. Selected 23 metrics and 42 associations; all other requirements retained. Integrity/traceability, 30 Rust integration tests, Clippy, format and unchanged-input checks pass. Evidence: `evidence/prototype/milestone-1.md`. Detailed plan/index synchronized; next is Milestone 2, not yet started.
- [x] (2026-11-01 21:51 IST, Milestone 2 planning) Created `milestone-execution-plans/milestone-02-execplan.md` under PLANS.md with M2.1–M2.7, bundle/record/oracle formats, twelve-group fixture coverage, Rust validation and tests, predecessor preservation, separate fixture-status tracking and acceptance/handoff. No execution tasks are marked complete.
- [x] (2026-11-01 21:52 IST, Milestone 2 plan validation) Checked required plan sections/format, links, twelve fixture-group IDs and master/detail pending states. Existing Rust evidence/traceability checks pass; M1 artifacts and original baseline hashes remain unchanged. Fixture execution is not claimed.
- [x] (2026-11-01 22:13 IST, Milestone 2 accepted) Completed M2.1–M2.7: twelve groups, 36 variants, 82 indexed payload files, independent expected answers, format/status records, verified M1 predecessor and Rust fixture validation/authoring. All 70 integration tests (40 new plus 30 regressions), Clippy, formatting, source/traceability and preservation checks pass. Evidence: `evidence/prototype/milestone-2.md`. Runtime experiments remain unexecuted; next is Milestone 3.
- [x] (2026-11-01 22:41 IST, Milestone 3 planning) Created `milestone-execution-plans/milestone-03-execplan.md` with M3.1–M3.7, exact Rust CLI/interfaces, bounded loopback serving, deterministic byte-roundtrip acceptance, input/oracle separation, regression/evidence requirements and synchronized tracking. Existing fixture construction and accepted artifact checks pass. No harness implementation is claimed.
- [x] (2026-11-01 22:46 IST, Milestone 3 plan validation) Verified required plan sections/format, current links and seven pending implementation tasks. Rust evidence/traceability/fixture checks, accepted M2/original baseline checksums and whitespace checks pass. All 250/58 IDs remain, with zero executed experiments; no engine implementation is claimed.
- [x] (2026-11-01 22:58 IST, Milestone 3 accepted) Completed M3.1–M3.7: separate locked Rust harness, complete listing/validation, bounded loopback server/client and deterministic 398-byte roundtrip. Passed 29 new tests plus 70 regressions, strict Clippy/format/checks and accepted-input preservation. Actual socket checks used approved execution after sandbox denial. Evidence: `evidence/prototype/milestone-3.md`; all PX semantic experiments remain not_run. Next is M4.
- [ ] Milestone 4: prototype native metric parsing and selected calculations against the fixture endpoints; preserve source meanings and unsupported cases.
- [ ] Milestone 5: prototype metadata enrichment using simulated identities and lifecycle changes; prevent duplicate counts and incorrect remote-worker attribution.
- [ ] Milestone 6: prototype bounded buffering, a local recovery journal, Arrow/Parquet output and DataFusion queries with simulated storage faults.
- [ ] Milestone 7: prototype timing/correlation logic using synthetic event replay and an in-process demonstration alert.
- [ ] Milestone 8: complete a repeatable local demonstration, compare results with expected answers and write the production-design handoff report.
- [x] (2026-11-01 20:42 IST, research recheck) Rechecked all 26 former U rows and 10 nearby mappings against original sources. Restored 25 U rows and four R rows to documented endpoint routes; retained L1-001 as U for counting semantics and L1-005 as a dependent calculation. Current counts: E:88, R:35, H:100, I:26, U:1. Evidence and per-row outcomes are in `source-mapping-recheck.md`. Requirements are unchanged; no source was excluded by engine-review scope.

## Surprises & Discoveries


Observation: a real loopback demonstration needs socket permission, and timeout/connection-close details affect bounded local behavior.

Evidence: sandbox socket denial was recorded and final socket tests used approved execution. M3 now passes 29 new tests plus 70 regressions, including partial peers, active/idle deadlines, rejected bodies and cleanup. Two CLI reports contain the same 398 bytes/hash. This proves local harness behavior only; no semantic/runtime acceptance is inferred.

Observation: the existing validation library can check the complete fixture bundle without exposing its private manifest structures.

Evidence: `check_fixtures`, `FixtureSummary` and `sha256` are public exports. M3 can reuse this checker through a local path dependency and load its own small manifest view without changing accepted M2 source bytes. A transferred malformed payload is transport evidence only, not semantic experiment execution.

Observation: Milestone 2 can pass construction validation while engine behavior remains untested.

Evidence: twelve groups/36 variants/82 payload files and their links/oracles pass Rust checks and 70 integration tests. The separate fixture ledger records construction evidence, while accepted M1 tracking retains all 250/58 IDs, experiment execution not_run and runtime readiness false. No source route, owner agreement or real qualification was changed.

Observation: extending the shared validation tool changes files named in the accepted Milestone 1 artifact hash list.

Evidence: Milestone 2's detailed plan preserves a verified predecessor snapshot before code changes and records new current-tool hashes separately. The original M1 hashes, planning files and frozen v1 evidence must not be regenerated. Fixture-authoring status is tracked separately so unexecuted experiments remain not_run.

Observation: PX02's counting unit is successful requests per second, not tokens per second.

Evidence: the accepted registry uses a Triton success count of 100 to 150 over ten virtual seconds. The Milestone 4 illustrative sentence is corrected below to match that source population; product requirements and source mappings remain unchanged.

Observation: Milestone 1 acceptance does not require or establish runtime experiment acceptance.

Evidence: Rust planning-tool checks and 30 integration tests pass, while the twelve registry experiments remain not_run and every runtime budget remains proposed/not agreed with readiness false. All 250/58 original identities remain visible; 23 metrics and 42 associations are selected for later fixtures.

Observation: using Rust throughout Milestone 1 adds an early validation-tool build, without starting the engine harness milestone.

Evidence: the revised M1.1 creates a small crate and dependency lock under `tools/aether-validation/`; its evidence verifier replaces the historical Python runtime path. Milestone 3 still creates the engine prototype under `prototypes/aether-poc/`. Frozen evidence and recorded earlier results remain unchanged.

Observation: a milestone plan being complete is different from the milestone outputs passing acceptance.

Evidence: at Milestone 1 plan creation those outputs were future work; they were later implemented and accepted with thirty Rust integration tests. Milestone 2's current plan authoring follows the same distinction: its fixture artifacts and validator remain pending.

Observation: no existing cluster is available, so a real deployment cannot be inferred as the first validation environment.

Evidence: explicit user reply. The environment manifest records the actual local Ubuntu 26.04.1, kernel 7.0.0-38-generic and Rust/Cargo 1.99.0 reference for future synthetic tests. Exact source commits and verified blob hashes freeze research inputs, not deployed schemas. Fixture and dependency locks remain pending.

Observation: zero application change does not mean every semantic source is already enabled, or that host access is automatic.

Evidence: the collection contract admits existing endpoints and selected read interfaces, reports disabled/denied/unsupported sources explicitly, and excludes workload flag changes and instrumentation activation from its initial profiles. Acceptance separates source update delay, collection delay, query visibility, timing accuracy and loss.

Observation: the earlier plan mixed prototype experiments with a production release program. That made the objective unclear.

Evidence: it included hardware qualification, system-service packaging and release hardening before production architecture and implementation had been designed. The user has now explicitly limited this plan to intermediate prototypes and synthetic fixtures.

Observation: we reviewed vLLM, SGLang and Dynamo first; this was a research starting point, not an exclusive product-support boundary.

Evidence: all 250 requirement IDs remain in the catalog. Some entries cite TGI, Triton or TensorRT-LLM sources. Those sources must be considered where relevant rather than requiring every measurement to exist in the three engines reviewed first.

Observation: a named source in documentation is not a working collector. A calculation can depend on unavailable events or an unresolved identity.

Evidence: after the source-mapping recheck, the matrix has 35 calculated routes and one unresolved route. It previously had 39 and 26 respectively. These counts are research classifications, not a support percentage. The remaining U row, L1-001, has a raw TGI counter but no verified accepted-inference-request-only counting boundary; the arrival-rate calculation L1-005 inherits that gap.

Observation: metrics published by a frontend can describe remote workers. A replica can span several GPUs. Adding hardware labels carelessly can duplicate counts or attach the wrong host.

Evidence: the existing baseline and enrichment matrix distinguish the publisher, the measured subject and the resource set. The prototype must exercise these distinctions with fixtures.

Observation: a precisely timestamped synthetic event proves only that the prototype handles that timestamp. It does not demonstrate real measurement accuracy.

Evidence: no Aether hardware, engine-compatibility, event-loss or overhead measurement has been performed. Synthetic tests must not be described as such measurements.

Observation: inspected Dynamo documentation pages reported different versions: v1.6.1 for the metric catalog and v1.4.2 for its linked label page.

Evidence: this is recorded in `engine-native-metrics-baseline.md`. Keep the mismatch visible in fixture provenance rather than treating the pages as one verified deployed schema.

## Decision Log


Decision: this plan delivers a proof of concept, not a production-ready engine. Production architecture and implementation require a later, separately scoped plan.
Rationale: explicit user correction. Intermediate experiments should inform that design instead of pretending it is already settled.
Date/Author: 2026-11-01 / user direction, recorded by Codex.

Decision: use synthetic inputs and local demonstrations only in the executable milestones. Do not make real GPU/NIC access, kernel attachment, serving deployments, cloud storage or production credentials prerequisites.
Rationale: the user limited this document to intermediate prototypes and synthetic fixtures. This permits reproducible tests without claims about untested environments.
Date/Author: 2026-11-01 / Codex, applying user scope.

Decision: keep the original requirements and metadata associations unchanged. Research mappings may be corrected with source evidence and a recorded review history. Record which requirements are exercised, unresolved or not yet prototyped in a separate prototype status file.
Rationale: a limited experiment scope is not a reduction of product requirements.
Date/Author: 2026-11-01 / Codex.

Decision: vLLM, SGLang and Dynamo remain the first engine fixture families. Other catalog sources remain eligible for fixtures and research when relevant. DeepInfer analysis remains deferred at the user's request.
Rationale: retain the earlier scope correction and avoid an exclusive engine list.
Date/Author: 2026-11-01 / user direction, recorded by Codex.

Decision: put experimental code under `prototypes/aether-poc/` and its inputs under `fixtures/prototype/`. Proposed modules and interfaces in this document belong to that prototype only.
Rationale: provide an understandable local test harness without committing to production package boundaries, concurrency models or APIs.
Date/Author: 2026-11-01 / Codex.

Decision: test the Rust/Arrow/Parquet/DataFusion direction locally and simulate the object-store boundary. Do not provision an S3/MinIO service or a central database.
Rationale: test data representation and read/write behavior while preserving the user's open-storage direction. A local file simulator does not certify object-store behavior.
Date/Author: 2026-11-01 / Codex.

Decision: give every synthetic scenario an explicit expected result. Any invented timings, limits or delays are fixture values, never proposed production guarantees.
Rationale: reproducibility requires concrete inputs, but production thresholds have not been established.
Date/Author: 2026-11-01 / Codex.

Decision: accept original TGI, Triton and TensorRT-LLM sources without requiring equivalents in the first three reviewed engines. An endpoint route must retain feature, population and attribution limits. Keep L1-001 unresolved for counting semantics, not because of engine scope.
Rationale: the user requested correction of the scope mistake before prototype work. The primary-source recheck supports 29 route changes while preserving all 250 requirements. Documentation evidence is not real-environment validation.
Date/Author: 2026-11-01 / Codex, applying user direction.

Decision: use the initial collection contract and budget process for prototype experiment selection. Initial profiles cover existing engine endpoints, selected host accounting, NVML getters, NIC/RDMA reads and configured metadata; future source extensions remain eligible.
Rationale: the user asked for this boundary before milestone execution. No workload mutation, live collection, inferred hardware compatibility or invented production budget is authorized. Finite synthetic run limits and expected answers will be selected per experiment; real-system qualification remains separate.
Date/Author: 2026-11-01 / Codex, applying user direction.

Decision: pin the first validation reference to the inspected local workspace and freeze source evidence by exact repository commits and content hashes. Keep all real deployed-schema/capability checks unperformed.
Rationale: the user confirmed no existing cluster and the plan remains synthetic-only. Exact input versions make research reproducible without inventing deployment versions or equating documentation with observed support. Retain the historical 26-row ledger, one remaining U counting gap and all requirements.
Date/Author: 2026-11-01 21:00 IST / Codex, applying user direction.

Decision: store all milestone execution plans in `milestone-execution-plans/` using `milestone-NN-execplan.md`; use the per-milestone file for detailed task/evidence tracking and this master for aggregate status.
Rationale: explicit user request. Update both living plans at stopping points and change the folder index when milestone status changes. Plan completion must not stand in for implementation or acceptance.
Date/Author: 2026-11-01 21:14 IST / Codex, applying user direction.

Decision: use Rust for milestone validation, record generation and tests as well as the engine. Replace active Python commands with a separate Rust validation crate and native Rust tests; retain historical evidence without rewriting it.
Rationale: explicit user language preference. A small initial Cargo build/lock step is appropriate and avoids a second runtime language. The validation crate is tooling, not a production engine design or completion of Milestone 3.
Date/Author: 2026-11-01 21:24 IST / Codex, applying user direction.

Decision: accept Milestone 1's Rust specification/traceability deliverables and retain all later milestones as pending.
Rationale: scope/status/registry/twelve budget records now exist; the Rust verifier/generator/validator and 30 integration tests pass without Python. Frozen evidence and research requirements are unchanged. The user requested completion followed by a next-milestone suggestion, not automatic execution of Milestone 2.
Date/Author: 2026-11-01 21:39 IST / Codex.

Decision: use `milestone-execution-plans/milestone-02-execplan.md` for detailed fixture execution progress, with a separate fixture-authoring ledger and preserved Milestone 1 predecessor snapshot.
Rationale: the user requested a detailed plan and updates to both plans. Existing source/tracking evidence must remain reproducible; fixture-validation success must not be confused with engine experiment execution or real capability verification. Rust-only implementation continues.
Date/Author: 2026-11-01 21:51 IST / Codex, applying user direction.

Decision: accept Milestone 2 construction/validation deliverables and retain Milestones 3–8 as pending.
Rationale: the user authorized execution followed by a next-milestone suggestion. The Rust fixture commands, independent data/oracles and 70 passing tests provide the planned evidence. Verified predecessor bytes preserve M1 reproducibility; neither authored expectations nor construction checks count as engine execution.
Date/Author: 2026-11-01 22:13 IST / Codex, applying user direction.

Decision: use `milestone-execution-plans/milestone-03-execplan.md` for M3.1–M3.7 tracking; this turn delivers planning only. Reuse the validation library, implement bounded standard-library loopback HTTP and a deterministic byte round trip, and retain all PX experiments as not_run.
Rationale: the user requested a detailed milestone plan and synchronized tracking. Observable harness behavior does not establish metric interpretation, real collection or budget acceptance. Preserve accepted M1/M2 records and hashes.
Date/Author: 2026-11-01 / Codex, applying user direction.

Decision: move Arrow/Parquet/DataFusion version selection and compatibility testing from M3 to M6.
Rationale: those libraries have no executable storage/query behavior in M3. Selecting and testing them together in M6 provides meaningful evidence without unused dependencies; the Rust/open-storage/query requirement and M6 scope are unchanged.
Date/Author: 2026-11-01 / Codex.

Decision: accept the authorized M3 harness implementation and retain M4–M8 as pending.
Rationale: complete fixture listing/validation, actual bounded HTTP requests, deterministic reports, 29 new tests plus 70 regressions and preserved accepted-input hashes satisfy its scope. All twelve semantic experiments remain unexecuted, budgets unagreed and real capabilities unverified. The user requested completion and a next-milestone suggestion, not automatic M4 execution.
Date/Author: 2026-11-01 22:58 IST / Codex, applying user direction.

## Outcomes & Retrospective


Milestones 1 and 2 are complete within the prototype preparation scope. Milestone 1 retains all 250 metrics and 58 associations, with 23/42 selected for twelve experiments. Milestone 2 delivers the actual invented fixture bundle: twelve groups, 36 variants and 82 indexed payload files, plus format notes, independent expected answers and a separate construction-status ledger.

Rust evidence/traceability/fixture commands pass, along with 70 integration tests, strict Clippy, formatting and preservation checks. M1's accepted source/input bytes are preserved under `evidence/prototype/milestone-2/inputs/milestone-1/`; current M2 code/data hashes and logs are in `evidence/prototype/milestone-2.md`. Original requirements, source routes, M1 planning records and frozen v1 sources remain unchanged. No Python was required and no new dependency was added.

M3 is now complete: its independent Rust package/lock, bounded loopback server/client, complete fixture listing/validation and deterministic roundtrip exist. Four new unit and 25 integration tests plus all 70 prior validation integration tests pass, with strict Clippy/format/checks and preserved accepted bytes. Two CLI reports match 398 received bytes/hash exactly. Evidence is `evidence/prototype/milestone-3.md`. Real socket checks required approved execution after sandbox denial; no network test was replaced or skipped.

All twelve PX semantic experiments remain unexecuted; no real source schema, device capability, physical timing, inference overhead or production durability has been verified. Proposed budgets remain unagreed. M3's seven detailed tasks are complete; M4 native endpoint parsing and selected calculations is next. Milestones 4–8 remain pending; production architecture remains unscoped.

## Context and Orientation


The repository is `/home/mbellary/work/projects/DeepMon`. `README.md` currently contains the heading `DeepMon`. Product research uses the name Aether. The current deliverables are `aether-metrics-catalog.md` and `.json`, `engine-native-metrics-baseline.md`, `metadata-enrichment-matrix.md`, and `metric-to-source-coverage-matrix.md` and `.json`. `source-mapping-recheck.md` records the completed scope-error audit and remaining source-validation limits. `zero-application-change-collection-contract.md` defines the initial source/action boundary, and `collection-acceptance-budget-template.json` records the unset run-specific limits and decision process. `validation-evidence-baseline.md` explains the pinned source bundle, historical-U ledger and first synthetic environment. `scripts/check-evidence-inputs.py` is the historical evidence verifier. The revised executable path replaces it with `aether-validate evidence` from `tools/aether-validation/`, implemented in Rust in the completed M1.1; no Python subprocess is permitted. `milestone-execution-plans/README.md` indexes per-milestone plans; `milestone-execution-plans/milestone-01-execplan.md` contains accepted Milestone 1 tracking; `milestone-execution-plans/milestone-02-execplan.md` records accepted fixture tasks, interfaces, commands and handoff; `fixtures/prototype/manifest.json`, `docs/prototype-fixture-format.md` and `docs/prototype-fixture-status.json` are the delivered construction artifacts. `PLANS.md` contains the plan-writing rules. `EXECPLAN.md` is this plan. Before milestone execution no Rust package existed. Milestone 1 now adds the isolated validation package and Cargo.lock under `tools/aether-validation/`; the separate synthetic harness package at `prototypes/aether-poc/` is now delivered by Milestone 3 with its own Cargo.lock. Inspect the checkout before future execution; do not overwrite new user work if this changes.

The four metric layers describe request/generation behavior, serving-engine/cache behavior, distributed execution/fabric behavior, and host/GPU infrastructure. Requirement IDs such as `L1-001` are stable references into the 250-family catalog. Association IDs `M01` through `M58` describe context relationships such as process-to-container or replica-to-GPU. All must remain represented in the prototype tracking file, even when not tested.

The route letters in the current coverage matrix mean E for an existing endpoint, R for a calculation over inputs, H for an additional host/device/fabric collector, I for deeper instrumentation and U for an unresolved exact mapping. They describe research, not implemented support. In this plan H and I sources are simulated. A fixture imitating their records does not implement the real collector or settle whether its data can be obtained without application changes.

An endpoint is an address/path publishing data. A counter accumulates events and may reset; a gauge is a current value; a histogram contains counts in value ranges plus totals. Histograms do not contain the original request sequence. The emitter is the publisher; the subject is what the data describes. A replica is a serving group that may span devices. A rank is a participant within a particular parallel group, not a universal GPU number. A NIC is a network adapter. A fabric is the communication network or GPU interconnect. RDMA is device-assisted communication that can bypass ordinary kernel networking. eBPF observes supported operating-system events; device/library profiling supplies other kinds of events. None is implemented by merely replaying synthetic records.

Arrow represents structured batches in memory; Parquet stores columns in files; DataFusion evaluates analytical SQL over supported data. SQL is a query language. A journal is a local file recording accepted batches for replay. A manifest lists which files a query may read. A simulated object store is a local test interface that imitates upload success/failure and committed-file visibility. These are test mechanisms, not an approved production durability or storage design.

A synthetic fixture is explicitly marked invented data. A documented-source fixture follows a cited source's names and meanings, but remains synthetic unless actually captured. A real captured export, if used later, must retain its own version and collection evidence and must not be relabeled as synthetic or assumed compatible with other builds. This plan requires no live capture.

## Plan of Work


### Milestone 1: define the prototype experiments and keep every requirement visible


Use [the detailed Milestone 1 execution plan](milestone-execution-plans/milestone-01-execplan.md) for task progress, decisions and evidence. Its M1.1–M1.7 tasks are complete and accepted; actual results and hashes are in `evidence/prototype/milestone-1.md`. Alongside the principal scope/status outputs below, it specifies `docs/prototype-experiments.json`, twelve experiment budget planning records under `docs/prototype-budgets/`, the Rust traceability validator and `evidence/prototype/milestone-1.md`. The twelve cases include parsing, calculations/uncertainty, attribution, lifecycle, NIC intent/use, distributed timing, stream overlap, storage/replay and collection policy; all future execution remains synthetic. Proposed safety limits are experimental proposals, not user-agreed runtime budgets or production guarantees.

Update the detailed plan at every stopping point, mirror the actual milestone status and next unfinished action here, and update the folder index when status changes. Mark Milestone 1 complete only after its specified outputs and traceability acceptance pass. Later engine-fixture execution is not part of Milestone 1's completion claim. Building and testing the separate Rust validation-tool crate is part of Milestone 1.

Read `zero-application-change-collection-contract.md` and `validation-evidence-baseline.md` first. In M1.1 create `tools/aether-validation/` as a separate Rust validation package, port every evidence check without invoking Python, select and lock its dependencies, and verify the installed Rust 1.99.0 reference identities. Then run `cargo run --locked --manifest-path tools/aether-validation/Cargo.toml -- evidence --root .`; it must verify the ten source snapshots and 26 historical mappings. Use `validation/first-environment-v1.json` as the recorded local reference; verify actual toolchain identity before later execution and record environment drift as a new revision. Earlier preparation did not create fixtures, a Rust package or `Cargo.lock`. The revised Milestone 1 now creates a validation-tool package/lockfile; the engine package and its dependencies remain Milestone 3 work. Use its eligible profiles without claiming they are implemented. Copy `collection-acceptance-budget-template.json` into each selected experiment's planning record, label it synthetic, choose finite safety limits and define applicable fixture acceptance criteria before an evaluated run. Record who agreed the criteria and why; unset required limits or missing reference truth cannot yield a pass. Production limits may remain unset because these are synthetic experiments.

Create `docs/prototype-scope.md` and `docs/prototype-status.json`. State that the program is a local proof of concept and does not discover or change real serving workloads. Select representative cases that test the major questions: parsing native metrics, distinguishing framework/backend boundaries, calculating rates, preserving identity, avoiding count duplication, handling missing evidence, recovering a local interrupted write, querying only committed files, and comparing simulated distributed events.

The status file must include all 250 original metric IDs and all 58 association IDs. For each entry record the original source route, whether it is selected for a fixture, prototype state, evidence references and what remains unproven. An entry not selected for the first experiments remains `not_prototyped`, not removed or marked unsupported. Keep the existing research documents unchanged. Put corrections discovered during prototyping in `docs/prototype-source-review.md` with the original ID and reason; apply changes to the research baseline only in a separately requested update.

Use the completed source recheck as the current baseline. Investigate remaining semantic gaps as research, not by assuming only three engines count. Keep a relevant original TGI/Triton/TensorRT-LLM source when it answers that requirement. If its exact meaning remains unclear, mark it pending. Do not manufacture an equivalent or new support count.

Acceptance is a reviewer-readable experiment list and a status file with exactly the original 250 metric IDs and 58 association IDs. Each selected case has an input, an expected answer and an explanation of the production question it informs. No production architecture, hardware support range or performance target is approved by this milestone.

### Milestone 2: create reproducible synthetic fixtures


Use [the detailed Milestone 2 execution plan](milestone-execution-plans/milestone-02-execplan.md) for M2.1–M2.7 progress and evidence. Its M2.1–M2.7 tasks are implemented and accepted; actual results are in `evidence/prototype/milestone-2.md`. Besides the bundle below, it specifies `docs/prototype-fixture-format.md`, a separate `docs/prototype-fixture-status.json`, a Rust `aether-validate fixtures` command/tests and `evidence/prototype/milestone-2.md`. Preserve a verifiable predecessor copy of accepted M1 tool/input files before changing shared code; do not rewrite M1 hashes, accepted planning records or the frozen v1 bundle.

The separate ledger records authored/validated fixture state while original experiment execution stays not_run and runtime budgets remain proposed/unagreed. Parent groups must match all twelve PX registry entries, with independently written positive and negative oracles; variants are not new product requirements. Update detailed/master/index status at every stopping point, and mark Milestone 2 complete only when actual artifact and Rust construction checks pass. Engine/harness behavior remains later work.

Create `fixtures/prototype/manifest.json` and fixture directories for `endpoints`, `metadata`, `events`, `faults` and `expected`. The manifest records fixture ID, `synthetic=true`, source-document/build reference where available, invented fields, virtual timestamps, related L/M IDs, expected result and limits of the evidence. Virtual timestamps are supplied by the fixture so tests do not depend on wall-clock sleep.

For endpoints, provide vLLM-like, SGLang-like and Dynamo-like text exports with counters, gauges and histograms. They are format/meaning experiments, not genuine compatibility certificates. Include missing optional families, reset counters, malformed text, model aliases, source labels and backend/framework counts for the same logical output. Include a histogram whose name ends in `_total` to prevent blind suffix stripping. Do not claim a fixture matches an exact release when its documentation/build reference is unpinned or inconsistent.

For metadata, invent explicit host/process/boot, container, replica, rank, GPU, NIC and worker identities. Include PID reuse, a remote worker, multiple replicas with the same model name, a replica with two different GPU models and an ambiguous endpoint owner. GPU/NIC names can be symbolic fixture values; they are not chosen hardware. Include a NIC-capability record without operation-use evidence to test that the program does not infer the active transport.

For events, provide matched and incomplete rank records, two clock domains with supplied uncertainty, output events containing several tokens, and a stream-level gap while another stream remains active. For faults, specify when a local write, simulated upload, manifest publish or replay fails. Fixture data contains no customer requests, credentials or production environment dumps.

Acceptance is that expected answers can be checked by hand from small inputs. For example, a counter increasing from 100 to 150 over ten virtual seconds gives five per second. These numbers are test inputs only. Document why marginal success/timing aggregates cannot supply exact joint goodput and why missing rank/clock evidence prevents a precise skew result.

### Milestone 3: build the isolated Rust test harness


Use [the detailed Milestone 3 execution plan](milestone-execution-plans/milestone-03-execplan.md) for M3.1–M3.7 progress, exact commands, interfaces, test limits and acceptance evidence. All seven tasks are complete for the synthetic harness; actual evidence is in `evidence/prototype/milestone-3.md`. Accepted M1/M2 artifacts and all PX experiment/status/budget records remain unchanged. Continue updating both plans and the index at implementation stopping points.

Create one experimental Rust package at `prototypes/aether-poc/Cargo.toml`, with source under `prototypes/aether-poc/src` and integration tests under `prototypes/aether-poc/tests`. Do not create or prescribe a production workspace. Provide `validate-fixtures`, `list-fixtures`, `serve-fixture` and `run-scenario --scenario endpoint-roundtrip` to validate/list inputs, serve one endpoint on 127.0.0.1 and compare actual response bytes with indexed input identity. Reuse the validation library through a local path dependency and bounded standard-library networking. Record chosen dependency versions in the prototype's `Cargo.lock` and `docs/prototype-dependencies.md`. Arrow/Parquet/DataFusion selection and compatibility execution move to M6, where storage/query behavior is exercised; production dependency versions remain undecided.

M3 implements experimental `config`, `fixture`, `endpoint` and `report` modules. Future `observation`, `identity`, `derive`, `journal`, `store`, `query` and `replay` modules are local organization options, not production engine boundaries or successful placeholders. Document future observation shapes preserving native names/types/labels, emitter, optional subject, virtual interval and quality state, and metadata relationships with validity times/evidence and unknown/ambiguous states in `docs/prototype-harness.md`. Actual semantic implementations remain M4–M7 work.

Implement configuration/fixture validation before demonstrations. M3 accepts named fixtures and a numeric 127.0.0.1 bind address; its client connects only to the listener owned by the harness, with no arbitrary URL option. It must not scan host ports, discover live workloads, connect to cloud accounts, read container sockets or attach kernel/device probes. Missing dependencies fail clearly; unimplemented commands return nonzero `not_prototyped` errors.

Acceptance is complete fixture listing/validation, rejection of bad manifests/hashes/configurations, bounded real loopback requests returning indexed bytes and two identical deterministic round-trip reports. Test lifecycle/failure handling and retain semantic execution not_run/readiness false. No real serving engine, GPU, NIC, root permission or storage service is required.

### Milestone 4: prototype endpoint parsing and selected calculations


Implement fixture endpoint parsing in `src/endpoint.rs` and calculations in `src/derive.rs`. Preserve original series and native labels before adding experimental normalized views. Consult metric type information before interpreting suffixes. Keep vLLM/SGLang backend observations distinct from Dynamo frontend/router/component observations. Unknown families are retained or reported explicitly, not silently turned into supported canonical measurements.

Calculate output-token rates from one defined counting boundary. Handle counter reset and source incarnation. Use an explicit synthetic completion policy when a fixture contains multiple finish reasons. Calculate cache ratios only with matching token/block/request denominators. Where an SLO, meaning a latency/service objective, is used in a test, declare its threshold and population in that fixture. Do not reconstruct original request events, exact variance or joint goodput from insufficient histogram data.

PX02's positive source-success rate case must return five successful requests per second for the Milestone 2 snapshots. PX04's duplicated framework/backend view of ten logical requests must retain ten within the selected counting boundary, not twenty. A separate token-rate case would require explicit token counters, population, units and expected answers; do not relabel these request observations as tokens. A counter reset must not produce a negative rate. A threshold not represented exactly by histogram buckets must produce a bound/estimate with that label or an unavailable result, never an invented exact answer.

Acceptance consists of these assertions and a report showing which native fixture observations were read, which derived results were justified, and which requested values remain unknown. This demonstrates parser/calculation behavior only, not actual vLLM/SGLang/Dynamo deployment support.

### Milestone 5: prototype enrichment and lifecycle handling


Implement joins over fixture metadata in `src/identity.rs`. A join connects a measurement with a context record using matching IDs and valid times. Keep emitter, subject, assigned resource set and observed participating resources separate. For a simulated replica-wide observation, store one value and link it to a set of simulated GPUs; do not copy it into independent values that would be summed twice.

Replay process exit/PID reuse and resource membership changes. A current mapping must not relabel an old observation. A frontend gauge about a remote worker must retain the frontend as emitter and the worker as subject. When the worker-to-host relationship is missing, report it as unknown. A model name, rank number or device ordinal alone must not identify a global replica/device. Keep configured NIC capability separate from a selected operation path.

Use the M01–M58 definitions to explain each tested association, but implement only the selected experimental joins. Untested associations remain in the status file. Do not implement actual runtime, Kubernetes, NVML, NIC or fabric discovery in this milestone.

Acceptance is a set of deterministic lifecycle/join tests and a report that shows both correct associations and honest unresolved ones. Two-GPU membership must not double a replica counter. Reused identifiers must not merge unrelated lifetimes. This validates the association logic against invented inputs, not the feasibility of obtaining those inputs on a real host.

### Milestone 6: prototype local buffering, recovery and analytical files


Select and lock compatible Arrow/Parquet/DataFusion versions in the prototype package, document them in `docs/prototype-dependencies.md` and test their actual write/read/query behavior together. This dependency compatibility check was deferred from M3 to this executable experiment; it does not select production versions. Implement a bounded in-memory queue and experimental local journal. Every queued item has a stable batch ID, and the queue has a fixture-specified capacity/full behavior. Record accepted, rejected and replayed counts. Write Arrow batches to Parquet and expose committed files through a small local manifest. Read them with embedded DataFusion. The user should be able to query the fixture observations without a central database.

Use `src/store.rs` as a simulated object-store boundary backed by a dedicated temporary directory. Inject failures before/after data-file write and before/after manifest visibility. Write files into a temporary area before publishing their committed membership. Replay by stable batch ID and checksum so retrying a committed batch does not create a second logical copy. Queries must exclude temporary/uncommitted files. Prototype a small-file rewrite as an optional experiment only if its old/new membership can be checked without counting both versions.

The journal's tested claim is narrowly local: after the program acknowledges a fixture batch at its documented journal boundary, the specified restart experiment can recover it. State whether the prototype flushes to the operating system or requests a filesystem sync. Neither result proves survival of power loss, disk loss, multiple writers, real object-store errors or distributed transactions. Those are later design/validation questions. A local simulator is not an S3/MinIO implementation.

Acceptance is an endpoint-fixture to Parquet to SQL demonstration, plus each injected interruption producing the expected final committed records without duplicate batch IDs. Queue saturation must have the declared counts and bounded behavior. Keep output/evidence isolated from research files. Record library and storage-design findings as recommendations, not an approved production pipeline.

### Milestone 7: prototype event replay and investigation logic


Implement `src/replay.rs` over synthetic events and a virtual clock. Replaying a fixture means presenting its events in the declared order/times without collecting real kernel, GPU or network data. Test out-of-order arrival, missing records, duplicate events and supplied clock uncertainty. Keep source event time separate from arrival time.

Demonstrate one positive correlation: a simulated throughput decrease overlaps a simulated host scheduling delay. Report the relationship and supporting event IDs, not a proven root cause. Demonstrate matched rank duration/arrival comparisons when complete identities and clock evidence exist. With insufficient members or uncertain clocks, produce a qualified or unavailable result. A single-stream gap cannot become whole-device idle when another stream is active. A long collective cannot become a fabric-fault diagnosis without the fixture's separate supporting evidence.

Add one in-process demonstration alert with fixture-defined rules and a bounded recent-event buffer. This is a test report, not an operator notification service. It must not send messages externally. Verify that evidence preceding a synthetic incident is retained within the configured fixture buffer, and that the intended duplicate/late-event rules are honored.

Acceptance is a repeatable replay report agreeing with fixture expectations for positive, negative and incomplete-evidence cases. Precisely handled synthetic microsecond timestamps are not evidence of real microsecond capture accuracy. No eBPF, CUPTI, NCCL plugin, kernel attachment, library injection or workload change is performed.

### Milestone 8: deliver the proof of concept and production-design handoff


Create `scripts/run-prototype-demo.sh` and `docs/prototype-results.md`. The script creates an isolated temporary output root, validates fixtures, starts only its loopback fixture endpoint, runs the capture/enrichment/storage/replay demonstrations, checks query results and stops its own helper processes. It retains failure output for inspection. Run it twice to prove safe repeatability without changing the research documents.

The results document must say what was implemented, which fixtures passed, which experiments failed, which requirements were not prototyped and which source questions remain unresolved. Include exact commands, dependency versions, actual outputs and evidence paths. Preserve all 250 requirement and 58 association entries. A successful representative demonstration does not mark all entries complete.

Produce `docs/production-design-handoff.md`. It lists evidence, candidate reusable pieces, alternatives rejected and the questions a later production-architecture plan must answer. These include actual source access, collector feasibility, zero-application-change boundaries, timing/overhead, compatibility, concurrency/isolation, distributed storage/metadata, security, real workload testing and operations. This is a question/evidence handoff, not a production design, implementation schedule or release promise.

Acceptance is a novice following the local instructions to run the demonstration and inspect correct results, with clear limits on every claim. Stop this plan after the prototype evidence and handoff are complete. Production architecture/scoping is a separate future task.

## Concrete Steps


For accepted Milestone 1 behavior, consult `milestone-execution-plans/milestone-01-execplan.md`. For the next fixture work, follow `milestone-execution-plans/milestone-02-execplan.md` in order; its fixtures command is now implemented and accepted, distinct from future engine experiment commands. Its commands distinguish current evidence-integrity checks from the future traceability checker; do not claim the future commands have run before their files exist. Keep the per-milestone and master Progress sections synchronized.

Before creating experiment artifacts, implement the Rust evidence command and tool lockfile in M1.1, then run from `/home/mbellary/work/projects/DeepMon`:

    cargo run --locked --manifest-path tools/aether-validation/Cargo.toml -- evidence --root .

Expect verified source snapshots, all 26 historical IDs and unchanged E:88/R:35/H:100/I:26/U:1 documentation totals. This checks evidence integrity only; deployed schema/capability validation is not run.

Run all commands from `/home/mbellary/work/projects/DeepMon`. Engine-prototype commands remain future interfaces. Milestone 1's Rust validation commands are implemented and have passed; do not conflate them with executing telemetry fixtures.

Start with a non-destructive inspection:

    cd /home/mbellary/work/projects/DeepMon
    git status --short
    rg --files -g AGENTS.md -g Cargo.toml -g Cargo.lock -g '*.rs'

If implementation appears after this plan revision, inspect it and update the plan instead of recreating or overwriting it. Read `PLANS.md` and this plan before executing an authorized milestone.

After Milestone 3 creates the package, run its portable checks:

    cargo fmt --manifest-path prototypes/aether-poc/Cargo.toml -- --check
    cargo check --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml
    cargo test --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml
    cargo clippy --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml --all-targets -- -D warnings

Expect all configured checks to succeed. Record the actual test count; none is invented here. These commands must not require special hardware, privileges or access to production services. Dependency downloads may need the environment's normal network authorization; do not bypass it.

Implement these prototype commands:

    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- validate-fixtures --root . --manifest fixtures/prototype/manifest.json
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- list-fixtures --root . --manifest fixtures/prototype/manifest.json
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- run-scenario --root . --manifest fixtures/prototype/manifest.json --scenario endpoint-roundtrip --variant PX01-vllm-v1

These M3 commands check fixture construction, list all groups/variants and run only the named byte-roundtrip scenario. They write reports to stdout and do not create a persistent output store. The detailed M3 plan also specifies `serve-fixture` and finite server limits. Semantic scenarios such as counter-rate remain pending until M4 and must return nonzero `not_prototyped` errors in M3. Later output-producing commands must refuse to overwrite unrelated existing files; tests use fresh temporary roots. A source name such as vLLM identifies invented fixture format, not a real deployment.

By Milestone 8 the repeatable entry point is:

    AETHER_POC_ROOT=$(mktemp -d /tmp/aether-poc.XXXXXX)
    bash scripts/run-prototype-demo.sh --output "$AETHER_POC_ROOT"
    cargo run --manifest-path prototypes/aether-poc/Cargo.toml -- query --output "$AETHER_POC_ROOT" --sql 'SELECT COUNT(*) AS committed_rows FROM observations'

The script owns its server process and stops it on exit. It must bind only to loopback, use a fresh test port rather than assume a serving-engine default and refuse arbitrary live targets. The query reads only the prototype output root. Set the expected row count after the fixtures are finalized and record it in the manifest; do not invent it in this initial plan.

The eventual success report should contain values such as:

    input_kind=synthetic
    source_success_rate_requests_per_second=5
    replica_count_duplicated=false
    remote_worker_host_inferred_from_frontend=false
    missing_evidence=reported_unknown
    interrupted_local_commit_replay=passed
    uncommitted_files_queried=false
    production_support_claimed=false

These are target assertions, not output from an existing implementation. Capture actual output when execution occurs. Test values and fixture limits do not become production targets.

Implement the Rust traceability command in `tools/aether-validation/` during Milestone 1 and run:

    cargo run --locked --manifest-path tools/aether-validation/Cargo.toml -- traceability --root .
    cargo test --locked --manifest-path tools/aether-validation/Cargo.toml

It must validate the status file against the original catalog and M-ID inventory, verify every selected fixture has expected results and ensure no requirements disappear. Its intended invariant is:

    metric_requirements=250
    metadata_associations=58
    missing_original_ids=0
    duplicate_ids=0

These Rust validation commands and 30 integration tests are now implemented and passed; see the Milestone 1 evidence report. This is specification acceptance, not runtime telemetry verification.

## Validation and Acceptance


Milestone 3 acceptance is now supported by `evidence/prototype/milestone-3.md`: a separate locked Rust package, complete read-only listing/validation, bounded real loopback requests, deterministic byte identity reports, 29 new passing failure/lifecycle/client tests, 70 regressions and preserved inputs. All seven detailed tasks are complete. No semantic experiment, real source or budget agreement is accepted by harness transport checks.

Milestone 2 plan-authoring acceptance is a complete PLANS.md-compliant detailed plan, valid prerequisite links, synchronized master/index progress, explicit future fixture/validator outputs and unchanged M1/v1 artifacts. That authoring check alone did not complete M2.1–M2.7. Their execution acceptance is now supported by the twelve-group bundle, 70 passing Rust integration tests, unchanged-input checks and `evidence/prototype/milestone-2.md`. Actual fixture acceptance additionally requires the defined bundle/oracles/ledger, preserved predecessor, Rust fixture checks and negative tests, regressions and an evidence report. No construction check can mark a telemetry experiment or deployed capability verified.

The original milestone-plan authoring acceptance required a detailed Milestone 1 plan and shared folder index that comply with `PLANS.md`, consistent master/detail progress, working links to existing prerequisites, explicit future deliverables and no changes to the baseline/evidence inputs. That authoring check alone did not complete Milestone 1. Actual execution acceptance is now supported by `evidence/prototype/milestone-1.md`: all outputs exist, integrity/traceability pass, thirty Rust integration tests pass and baseline hashes remain unchanged.

Previously completed research preparation includes verified source snapshot/content hashes, a complete 26-row reconciliation ledger, unchanged route totals and an actual local environment fingerprint with real validation explicitly pending. The collection contract, budget template, source-matrix links and plan must continue to agree on initial profiles, allowed actions and unset budgets. The 250-family requirements catalog and 58-association matrix remain byte-for-byte unchanged. All milestone execution remains limited to prototypes and synthetic fixtures. Rust validation tooling is implemented; no inference-engine implementation or deployed compatibility is claimed. Required living sections and progress tracking remain present.

Every evaluated prototype run must include its source/fixture manifest, finite safety limits, agreed applicable criteria, expected answers and result evidence. Run the negative policy cases too: a disabled source stays disabled, denied access does not escalate, and missing required budgets prevent acceptance. Synthetic timestamps and delivery tests must not be presented as source event completeness or real timing accuracy.

Prototype acceptance requires behavior, not just compilation. Implement tests such as `native_labels_are_preserved`, `counter_reset_does_not_emit_negative_rate`, `histogram_does_not_invent_raw_events`, `framework_and_backend_counts_are_not_added`, `replica_observation_is_not_duplicated_per_gpu`, `pid_reuse_does_not_relabel_history`, `remote_worker_subject_is_not_frontend_host`, `nic_capability_is_not_assumed_active_transport`, `missing_evidence_is_not_zero`, `replay_does_not_duplicate_committed_batch` and `query_excludes_uncommitted_files`. Test names may be refined with recorded reasons; the behaviors must remain.

Each selected experiment has a positive case and a relevant negative or incomplete-evidence case. A fixture that cannot supply an answer must yield unknown, partial or unavailable with a reason. A synthetic failure must not be reported as a successful no-op. Replay clocks and explicit injected failures should keep tests deterministic without long sleeps.

Local recovery tests exercise only their stated write/acknowledgment boundary. End-to-end tests compare exact expected fixture records and values before/after replay. Bounded-queue tests check the configured maximum and declared accepted/dropped counts. Results must distinguish logical records from retried write attempts. All output goes into disposable prototype directories.

Final acceptance is a repeatable local demonstration plus evidence and the handoff report. It does not require or establish real engine compatibility, GPU/NIC/fabric support, kernel/driver backward compatibility, real distributed timing, production durability/security, or overhead/service guarantees. Those product requirements remain unchanged and unvalidated for later work.

## Idempotence and Recovery


Keep prototype code, fixtures and outputs separate from research artifacts. Do not rewrite the 250-family catalog, narrow engine support or change the source-route counts merely to make an experiment pass. Track experiment status separately. Check current user changes before each edit; no reset, destructive cleanup or unrelated commit is implied.

Use a fresh temporary output directory per run. If a run fails, retain its logs and generated journal/files for inspection. A rerun uses a new root unless it is explicitly testing recovery on that prior root. Cleanup removes only artifacts/processes created by that run, never shared files or arbitrary bucket prefixes. No real object store or remote query service is required.

An interrupted scenario can be rerun using stable fixture batch IDs to check replay behavior. If a fixture or expected result changes, create a new fixture revision and record why. Preserve prior evidence so the meaning of a previous pass is clear. Mark unsupported experiments as not prototyped instead of fabricating inputs that claim real collection feasibility.

At each stopping point update Progress, discoveries, decisions, outcomes and revision notes. Continue only within the user-authorized milestone. If later production work is requested, create a separate scoped design/implementation plan; do not silently expand this one.

## Artifacts and Notes


The research baseline remains:

    workload_metric_requirements=250
    observability_layers=4
    metadata_associations=58
    reviewed_first=vLLM,SGLang,NVIDIA_Dynamo
    exclusive_product_engine_list=not_established
    original_route_counts=E:59,R:39,H:100,I:26,U:26
    current_route_counts=E:88,R:35,H:100,I:26,U:1
    route_counts_status=scope_error_rechecked_documentation_only
    initial_collection_contract=zero-application-change-collection-contract.md
    acceptance_budget_template=collection-acceptance-budget-template.json
    numeric_production_budgets=unset
    versioned_evidence_bundle=aether-source-evidence-v1
    historical_u_rows_reconciled=26
    first_validation_environment=aether-validation-synthetic-local-v1
    real_deployment_available=false
    deployed_schemas_and_capabilities=unverified
    milestone_plan_folder=milestone-execution-plans
    milestone_1_plan=milestone-execution-plans/milestone-01-execplan.md
    milestone_1_planning=complete
    milestone_1_execution_acceptance=passed
    validation_tool_implemented=true
    telemetry_experiments_executed=0
    milestone_2_plan=milestone-execution-plans/milestone-02-execplan.md
    milestone_2_planning=complete
    milestone_2_execution_acceptance=passed
    fixture_groups_authored=12
    fixture_variants_authored=36
    fixture_payload_files=82
    prototype_implemented=false

The prototype does not need to simulate every requirement to be useful. It must account for every requirement without calling untested work complete. Keep the 18 previously proposed collection-health companion families separate from the original 250; testing some companion behavior does not silently change the catalog count.

For each milestone write `evidence/prototype/milestone-<number>.md` with date/time, code revision if available, fixture revision, dependency versions, commands, expected/actual results and remaining limits. Identify inputs as synthetic in both records and reports. Small local execution timings can be recorded as experiment observations, but must not be advertised as inference overhead or real-hardware timing accuracy.

`docs/prototype-status.json` should use distinct fields for requirement identity and experiment status. Suggested states are `not_prototyped`, `fixture_planned`, `prototype_in_progress`, `synthetic_verified`, `experiment_failed` and `research_pending`. A status file entry also records whether the actual collection source remains unverified. None of these states means production-ready. Preserve research provenance separately from test outcomes.

## Interfaces and Dependencies


Milestone 1 has a separate Rust validation package at `tools/aether-validation/`, with binary `aether-validate`, `evidence` and `traceability` subcommands, Rust unit/integration tests and its own Cargo.lock. Use serde/serde_json for JSON, sha2 for SHA-256, sha1 only for Git blob identity, and tempfile for isolated tests; select exact compatible versions and commit the tool lock during implementation. Use std argument/path/file handling and no Python runtime, subprocess or embedded interpreter. This early tooling package does not finalize the later engine workspace or its dependencies. The detailed milestone plan specifies `check_evidence` and `check_traceability` interfaces and the full port's acceptance checks.

The following interfaces are experimental and local to `prototypes/aether-poc`. They make the exercises concrete without establishing production module boundaries or APIs. Record changes and reasons when an experiment shows a better design.

In `src/fixture.rs`, expose:

    pub fn load_bundle(root: &std::path::Path, manifest: &std::path::Path)
        -> Result<ValidatedBundle, PrototypeError>;
    pub fn select_endpoint(bundle: &ValidatedBundle, variant_id: &str)
        -> Result<EndpointInput, PrototypeError>;

These are implemented M3 interfaces, replacing the original single-path loader sketch so validation can resolve the repository evidence/traceability inputs. `ValidatedBundle` retains manifest identities, related requirement/association IDs, file roles and provenance. `EndpointInput` exposes only selected validated endpoint bytes and identities, with no semantic oracle. `PrototypeError` gives a stable code/context and optional fixture ID. Confined bounded reads recheck hashes. The detailed M3 plan specifies implemented configuration/server/report interfaces. The parsing, association, replay and query interfaces below remain future experimental targets; do not infer they exist because the harness is complete.

In `src/endpoint.rs`, expose:

    pub fn parse_metric_fixture(text: &str, source: &FixtureSource)
        -> Result<Vec<NativeObservation>, PrototypeError>;

`FixtureSource` identifies the synthetic publisher and declared schema. `NativeObservation` preserves name, metric type, labels, unit and fixture timing without inventing unsupported identity. A finite fixture can use a vector; this is not a production buffering decision. A local HTTP helper can serve that same text for an end-to-end adapter exercise.

In `src/identity.rs`, expose:

    pub fn associate_observation(
        observation: &NativeObservation,
        metadata: &FixtureMetadata,
    ) -> AssociationResult;

`FixtureMetadata` contains invented time-bounded relationships. `AssociationResult` preserves one measurement plus known, partial or ambiguous relationships; it must not multiply measurements by resource membership. Native labels stay available even when a proposed canonical relationship is unknown.

In `src/replay.rs`, expose:

    pub fn run_scenario(
        fixture: &ScenarioFixture,
        output_root: &std::path::Path,
    ) -> Result<ScenarioReport, PrototypeError>;

`ScenarioFixture` supplies virtual time, synthetic observations and injected faults. `ScenarioReport` contains expected/actual values, evidence links and explicit limits. It cannot claim real-world capture properties.

In `src/query.rs`, expose:

    pub async fn query_committed_fixture_data(
        output_root: &std::path::Path,
        sql: &str,
    ) -> Result<QueryReport, PrototypeError>;

Use DataFusion with Arrow/Parquet data and only the local committed-file view. `QueryReport` contains results and which fixture snapshot/files were read. This does not prescribe a production query service, distributed scheduler or access-control system.

Use Rust dependencies needed for these experiments. M3 locks fixture/configuration handling and the local validation-library dependency, with standard-library local HTTP. M4 adds parsing dependencies only if needed. M6 selects/locks compatible Arrow/Parquet/DataFusion versions and tests them together against actual prototype storage/query behavior. Optional asynchronous orchestration is a later prototype choice, not a finalized production concurrency model. Real NVML/DCGM, eBPF, CUPTI, NCCL-plugin, runtime-socket and switch dependencies are not required or exercised; simulate their proposed record shapes and keep actual collection feasibility pending.

## Revision Notes


2026-11-01: initial plan combined research tracking, prototype milestones and later production implementation/qualification. No implementation occurred.

2026-11-01: corrected the mistaken exclusive engine-scope wording. All 250 requirements remain; the affected mappings still need review.

2026-11-01 20:26 IST: rewrote the objective and all executable milestones to cover intermediate prototypes and synthetic fixtures only, following the user's correction. Removed production collector implementation, real-environment qualification, packaging, hardening and release from this plan. Experimental interfaces do not finalize production architecture. The requirements catalog, metadata matrix, source matrices and uploaded `PLANS.md` remain unchanged. No prototype implementation accompanies this revision.

2026-11-01 20:42 IST: completed the requested source-mapping recheck before prototype execution. Updated coverage documents and added `source-mapping-recheck.md`: 25 U→E and four R→E changes, with L1-001 retained as U for a real counting-boundary gap. Reviewed ten nearby mappings in addition to all 26 former U rows. Current totals are E:88, R:35, H:100, I:26, U:1. Requirements and metadata associations remain unchanged; all prototype milestones remain pending.

2026-11-01 20:49 IST: defined the initial zero-application-change collection contract and budget planning template before milestone execution. Added source profiles, action boundaries, field/identity/freshness states, loss/durability distinctions and a process for agreeing and evaluating limits. Updated prototype experiment and validation instructions to use them. No production thresholds, deployed support, live collection or prototype completion is implied; requirements remain unchanged.

2026-11-01 21:00 IST: established versioned upstream evidence and a 26-row historical reconciliation ledger, plus the actual local synthetic reference environment. User confirmed no existing cluster; real target versions and deployed schemas/capabilities remain unverified. Added evidence-integrity checking and connected the preparation artifacts to milestone selection and validation. All eight milestones remain pending; no fixtures, engine implementation or deployment were created.

2026-11-01 21:14 IST: created the reusable `milestone-execution-plans/` folder and detailed Milestone 1 ExecPlan following `PLANS.md`. Added detailed task tracking, twelve experiment specifications, output schemas, synthetic budget proposal handling, meaningful traceability acceptance and master/detail synchronization. Planning is complete; Milestone 1 execution and all later milestones remain pending. No research requirement, route, frozen input, fixture or engine implementation changed.

2026-11-01 21:15 IST: completed plan-authoring verification and mirrored the results in the detailed Milestone 1 plan. Required sections/links and stable-ID references pass; research and frozen-input hashes are unchanged; no milestone implementation is claimed.

2026-11-01 21:24 IST: applied the user's Rust-only preference to milestone tooling as well as the engine. M1.1 now creates an isolated Rust validation crate/lockfile and ports evidence verification before tracing/planning checks. All executable Python commands were removed from the master/detail plans; historical evidence is retained and no Rust implementation or tests are claimed complete.

2026-11-01 21:39 IST: completed and accepted Milestone 1 execution in Rust. Retained all 250 metrics/58 associations and source routes, selected 23 metrics/42 associations for twelve future cases, delivered locked validation tooling and fifteen planning outputs, and recorded 30 passing integration tests plus Clippy/format/integrity checks. Evidence is `evidence/prototype/milestone-1.md`. Detailed plan/index updated. Milestone 2 fixture authoring is the next implementation milestone; no later milestone or runtime capability is marked complete.

2026-11-01 21:51 IST: created the detailed Milestone 2 fixture ExecPlan under PLANS.md, added its folder-index entry and synchronized master tracking. Defined twelve-group coverage, fixture/variant formats, independent expected answers, Rust construction validation, predecessor preservation and separate fixture-authoring state. Corrected the old rate-example unit to requests/second in line with PX02, without changing requirements or mappings. Only planning/prerequisite inspection is complete; Milestone 2 execution and Milestones 3–8 remain pending.

2026-11-01 21:52 IST: completed Milestone 2 plan-authoring validation and synchronized the detailed/master progress records. Existing evidence and M1 delivered artifacts remain intact. All M2 execution tasks remain pending; no fixture, runtime result or validator implementation is delivered by this update.

2026-11-01 22:13 IST: executed and accepted Milestone 2. Delivered twelve groups/36 variants/82 payloads, independent oracles, format/status records and Rust construction checks; preserved M1 predecessor bytes and all original research/planning inputs. Passed 70 tests, Clippy, formatting, evidence/traceability and integrity checks. Detailed plan/index and evidence synchronized. Milestone 3 isolated Rust harness/loopback server is next and has not started; all runtime/real capability/budget claims remain unverified.

2026-11-01 22:45 IST: created the detailed M3 execution plan and synchronized master/index tracking. Defined seven pending tasks, Rust package/CLI/interfaces, bounded loopback server, deterministic byte-roundtrip acceptance, preservation and evidence requirements. Deferred unused Arrow/Parquet/DataFusion selection/compatibility to M6's executable storage/query experiment. Updated current commands and corrected the remaining illustrative five-per-second field to successful requests, consistent with the accepted PX02 counter. All requirements/source mappings and accepted artifacts remain unchanged; only planning/prerequisite checks are complete.

2026-11-01 22:46 IST: completed M3 plan-authoring verification and mirrored results in its detailed plan. Plan structure/links/pending tasks and existing Rust prerequisite/integrity checks pass. M3.1–M3.7 implementation and all later milestones remain pending.

2026-11-01 22:58 IST: completed and accepted M3 implementation. Added independent Rust harness/lock, complete fixture listing/validation, bounded owned loopback HTTP and deterministic reports, plus dependency/handoff docs and actual evidence. Passed 29 new tests and 70 regressions, strict Clippy/format/checks and unchanged-input verification. Approved socket access was used after sandbox denial. Master/detail/index synchronized; M4 is next, while all PX semantic execution/real capability/budget states remain unverified.
