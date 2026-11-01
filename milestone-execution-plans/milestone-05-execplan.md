# Milestone 5: enrich synthetic observations with scoped, time-valid identities


This ExecPlan follows repository-root `PLANS.md`. Keep `Progress`, `Surprises & Discoveries`, `Decision Log`, `Outcomes & Retrospective` and the bottom revision note current at every stopping point. The workspace is `/home/mbellary/work/projects/DeepMon`; all paths below are repository-relative. `EXECPLAN.md` tracks the complete prototype program. `milestone-execution-plans/README.md` indexes its detailed plans.

Current state: M5.1–M5.8 are complete and accepted for synthetic functional correctness. The Rust resolver, strict fixture adapters, seven-case extension, separate current ledgers and evidence are delivered. All 161 tests pass, including 32 new tests and 129 prior tests. Evidence is `evidence/prototype/milestone-5.md`. Milestone 6 is next; budgets remain unagreed, real capabilities unverified and production architecture unscoped.

## Purpose / Big Picture


Let a developer see which process, container, replica or resource set a synthetic measurement can legitimately describe, and why an association remains unknown or ambiguous. After implementation, the Rust demonstration will keep an old process observation attached to its original container after PID reuse, retain both possible publishers when ownership is ambiguous and attach one replica counter to a GPU set without copying that counter per device.

Enrichment adds context to an existing measurement. A join connects records using explicit identity and evidence. An incarnation identifies one lifetime of a reused ID. A validity interval states when a relationship applies. The emitter publishes a measurement; its subject is what the measurement describes. Available or allocated resources are distinct from resources observed participating in execution. A NIC's capabilities and configured preferences are distinct from an observed traffic path.

This is an intermediate Rust prototype using invented data. Retain all 250 metric requirements, all four layers and all 58 associations. The selected fixtures do not narrow product scope or source eligibility. No existing cluster is available; production architecture remains unscoped and DeepInfer remains deferred. No real discovery, runtime socket, Kubernetes API, procfs/device inventory, NVML, network probe, library interception, workload modification or cloud service is required. Use Rust for code, fixture helpers and tests; do not invoke Python. Functional correctness remains separate from unagreed runtime budgets and real-system qualification.

## Progress


Times use Asia/Kolkata, UTC+05:30. Entries distinguish historical planning from completed implementation; implementation acceptance was recorded on 2026-11-02.

- [x] (2026-11-01 23:39 IST, inspected) Read `PLANS.md`, master M5 scope, metadata definitions, accepted M4 interfaces/ledger and PX04/PX05/PX06/PX07/PX11 inputs/oracles.
- [x] (2026-11-01 23:39 IST, prerequisite verified) Accepted M4's 260-file checksum list passes; fixture validation reports 12 groups/36 variants/82 payloads/two intentional invalid exports. Traceability retains all 250/58 IDs with none missing and readiness false.
- [x] (2026-11-01, plan authoring) Created this detailed plan and synchronized master/index tracking. Authoring verification is recorded separately below.
- [x] (2026-11-01 23:47 IST, plan verified) Required sections/heading format, current links, eight pending tasks and manifest-backed five-group/ten-variant/17-assertion scope pass. Existing Rust source/traceability/fixture checks and accepted M4/M2/original hashes pass; no code, fixture or execution ledger changed.
- [x] (2026-11-02 00:15 IST, accepted) M5.1: verify/preserve accepted M4 predecessor inputs and define exact association/assertion coverage and compatibility boundaries.
- [x] (2026-11-02 00:15 IST, accepted) M5.2: define typed scoped identity/relationship/query records and author separate versioned synthetic boundary fixtures with independent expected answers.
- [x] (2026-11-02 00:15 IST, accepted) M5.3: implement deterministic evidence- and time-aware identity resolution, including ambiguity, missing evidence and history preservation.
- [x] (2026-11-02 00:15 IST, accepted) M5.4: implement publisher/remote-subject and process/container lifecycle fixture adapters without inventing ownership or telemetry.
- [x] (2026-11-02 00:15 IST, accepted) M5.5: implement set-valued replica/resource enrichment, NIC intent/use separation and visibility/privacy handling without duplicating measurements.
- [x] (2026-11-02 00:15 IST, accepted) M5.6: integrate independent run/verify commands and separate current association execution/coverage tracking while preserving M4 outputs/ledger.
- [x] (2026-11-02 00:15 IST, accepted) M5.7: run meaningful Rust boundary/integration/regression tests, static/integrity checks and capture actual evidence.
- [x] (2026-11-02 00:15 IST, accepted) M5.8: review scoped acceptance, synchronize detailed/master/index progress and hand off to M6 only after required checks pass.

## Surprises & Discoveries


Observation: some existing relationship records have no query time or validity bounds.

Evidence: PX04's listener lists p1/p2 candidates without an observation time; PX05's member link lists devices and a completeness flag without link validity times. Preserve these as supplied candidate/set facts with temporal validity unspecified. Do not manufacture a timestamp or inherit a replica's lifetime for a link. Additional synthetic boundary fixtures will provide explicit test times where timed resolution is exercised.

Observation: the current runtime reader intentionally does not accept all M5 input fields.

Evidence: `metric_input.rs` rejects PX06 link fields process_record/container/cgroup and denied policy fields source/status/container, plus PX07 capability/configuration/socket fields. M5 must add strict typed handling rather than bypass validation or silently discard these records. `QualityState` also lacks the resolved state used by PX06's existing oracle.

Observation: existing M4 functional evidence and new association evidence must remain distinguishable.

Evidence: the five M5-related groups contain ten variants and 17 original assertions. Seven assertions are newly assigned to M5; ten already have M4 functional matches. Rechecking ten assertions must not add ten to the original 49-assertion total or rewrite M4's historical deferrals.

Observation: capability metadata is not automatically associated with the only NIC in a file.

Evidence: PX07's capability record has no explicit NIC/port foreign key or operation path. Preserve the record separately from NIC identity and byte count. Co-occurrence is not enough to attach model, driver, configured RoCE or loaded-library claims to a verified device/path.

Observation: the first fabric test exposed integer byte-count coercion through floating-point decoding.

Evidence: the initial test log records one failure. The strict adapter now retains `serde_json::Number`, preserving the supplied 1024 exactly; all 32 enrichment tests pass without changing its independent expected answer.

Observation: query scope must belong to the measurement being enriched.

Evidence: `associate_observation` rejects `observation_query_mismatch` unless the query source matches an explicitly keyed emitter or subject. Boundary tests also verify history, input-order invariance and unchanged native facts/cardinality.

## Decision Log


Decision: the initial plan-authoring turn delivered planning only and kept all eight implementation tasks pending at that time.
Rationale: the user requested a detailed execution plan and updates to both plans; authoring is not implementation evidence.
Date/Author: 2026-11-01 / Codex, applying user direction.

Decision: preserve accepted M4 files and its functional ledger, then add a separate M5 association ledger and stage-aware verification entry point.
Rationale: current M4 commands and 49-assertion accounting must remain reproducible. M5 can truthfully advance current association evidence without editing historical not_run/deferred fields or double-counting repeated checks.
Date/Author: 2026-11-01 / Codex.

Decision: keep frozen v1 fixtures unchanged; create an additive `fixtures/enrichment-v1/` bundle for explicit lifecycle/time-boundary tests.
Rationale: the original bundle's file index rejects unindexed additions, and several original links lack time evidence. Explicitly invented additional queries/relationships test the resolver without silently supplying facts absent from original records. New fixture values are local test choices, not production requirements or inferred cluster facts.
Date/Author: 2026-11-01 / Codex.

Decision: missing time, host/boot/namespace or resource identity remains missing; only explicit evidence can produce a resolved association.
Rationale: a symbolic entity string, model alias, local PID, GPU ordinal or TCP control peer is insufficient to establish ownership or active execution. Reporting supplied candidates/sets is useful even when their physical or temporal validity is unverified.
Date/Author: 2026-11-01 / Codex.

Decision: make enrichment an immutable reference operation over one measurement, with set-valued relationships and explicit evidence/validity state.
Rationale: resource cardinality must not multiply measurements or alter native meaning. Lifecycle updates must not relabel old records through a current-only lookup.
Date/Author: 2026-11-01 / Codex.

Decision: accept M5 functional implementation and preserve prior-only evidence separately.
Rationale: 161 passing tests, independent original/extension comparisons, deterministic outputs, conservative 58-definition facet tracking and byte-preserved M4 evidence satisfy this milestone. A prior unsupported-stage regression now uses stage 6 because stage 5 is implemented. No requirement, original oracle or budget was changed.
Date/Author: 2026-11-02 / Codex.

## Outcomes & Retrospective


Milestone 5 is accepted for its defined synthetic functional scope. Typed scoped keys, immutable historical relationships, half-open validity, explicit clocks, publisher ambiguity, remote subjects, process incarnations, set-valued resources and NIC intent/use separation now execute in Rust. Independent comparisons pass for 17 original assertions (seven newly exercised and ten rechecked) and 69 extension checks across seven cases. Measurement identity, value, native facts and cardinality survive enrichment. Unknown and partial evidence remains explicit.

All 161 tests pass: 32 new enrichment tests and 129 predecessor tests. Six saved demonstration pairs are byte-identical; the current stage-4 summary is byte-identical to accepted M4 output. Strict Clippy, formatting, baseline/source/traceability/fixture and predecessor checks pass. All 58 original definitions remain in the separate facet ledger, with 23 exercised facets and 35 unexercised entries; this is not complete association support. The combined 49 original assertions are 31 matched, two expected rejections, one proof, one deferred queue assertion and 14 later-stage assertions not run. Prior-only checks are linked historical evidence, not claimed as rerun association checks.

The original M4 ledger, requirements, source mappings, original fixtures and budgets remain unchanged. The 16-file extension and separate M5 ledgers are additive. Actual artifacts, commands and limitations are in `evidence/prototype/milestone-5.md`; adjacent logs and checksums preserve reproduction inputs. Milestone 6 buffering, journal recovery, Arrow/Parquet/DataFusion and simulated storage faults is next. No live acquisition, physical identity, timing accuracy, overhead budget or production durability is qualified.

## Context and Orientation


`aether-metrics-catalog.json` and `.md` retain 250 metric families. `metadata-enrichment-matrix.md` defines M01–M58 and rules for emitter/subject/resource separation, validity, evidence and ambiguity. `metric-to-source-coverage-matrix.json` retains research routes E88/R35/H100/I26/U1, meaning endpoint/calculation/additional collector/deeper instrumentation/unresolved mapping, not implemented support. L1-001/L1-005 accepted-arrival semantics and L2-071 skipped/speculated-token interpretation remain unresolved/qualified. Nothing in M5 changes these source routes or requirements. vLLM/SGLang/Dynamo are first reviewed engines, not an exclusive list; relevant TGI/Triton/TensorRT-LLM sources remain eligible.

`evidence/manifest-v1.json` pins exact source snapshots E01–E10 and local baseline inputs. `docs/prototype-experiments.json` defines twelve PX cases; `docs/prototype-status.json` retains every original requirement/association, selecting 23 metrics and 42 associations for the overall prototype program. An experiment's association list declares inherited questions, not proof that every association is present in its records. All 58 associations must remain visible even when no M5 facet is exercised.

`fixtures/prototype/manifest.json` indexes the unchanged synthetic v1 bundle: twelve groups, 36 variants, 82 payload files and two intentionally malformed endpoint exports. Input and expected files are separate, hashed roles. The new extension must live outside that directory because its index permits only its original payloads/control files. `docs/prototype-fixture-format.md` defines current envelopes; `docs/prototype-metric-semantics.md` defines accepted M4 behavior. `docs/prototype-metric-execution-status.json` accounts for all 49 original assertions: 24 matches, two precise rejections, one proof, three considered-group deferrals and 19 later-stage assertions not run.

The five original groups here are PX04 publisher/remote worker, PX05 GPU composition, PX06 process/cgroup/container incarnation, PX07 NIC capability versus use and PX11 visibility/privacy/policy. Their ten variants contain 17 original assertions. Newly exercised M5 assertions are PX04 publisher; PX05 composition; PX06 historical_owner/container; and PX07 capabilities/active_transport/operation_to_nic. The other ten are existing M4 calculation/unknown/policy assertions to recheck without counting them as new original assertions. PX12 queue remains M6, PX08/PX09 timing remains M7 and PX10 storage remains M6.

`prototypes/aether-poc/` is the independent Rust package. Its `fixture.rs` exposes validated bundle selection and separate bounded runtime-input/oracle loaders. `metric_input.rs` has known record kinds and strict field allowlists, `observation.rs` has native/source/derived-result types and safe output projection, `scenario.rs` implements seven M4 scenarios and independent comparison, and `endpoint.rs` owns the finite loopback HTTP test path. `config.rs`/`main.rs` dispatch CLI actions; `examples/record_metric_evidence.rs` authors the protected M4 ledger. `identity.rs` now implements the typed graph and resolver; `enrichment_fixture.rs`, `enrichment_scenario.rs` and `enrichment_data.rs` provide strict extension loading, independent verification and literal fixture authoring. Reuse existing functionality without weakening validation or copying expected answers into runtime inputs.

M4 evidence is `evidence/prototype/milestone-4.md` and its adjacent 260-file artifact list, including its verified M3 predecessor and dependency inputs. M4 delivered 30 new metric tests plus 99 previous tests: 129 passing in total. Dependencies remain local aether-validation, serde 1.0.228, serde_json 1.0.149 and test-only tempfile 3.27.0, with unchanged Cargo.lock. The separate validation crate still verifies evidence/traceability/fixtures and has 70 tests within that total.

The inspected synthetic reference is Ubuntu 26.04.1, kernel 7.0.0-38-generic, glibc 2.43, Rust/Cargo 1.99.0, x86_64-unknown-linux-gnu. Verify actual environment drift. This does not define a Linux/GPU/NIC/CUDA/NCCL support range. M4 endpoint regressions need loopback permission through the normal approved execution path; M5's new association scenarios need only local files and virtual-time calculations, with no sleeping or real discovery.

`zero-application-change-collection-contract.md` defines allowed initial read interfaces and prohibits automatic metric enabling, workload mutation and privilege escalation. M5 simulates records rather than exercising those real interfaces. All proposed PX budget agreements remain unset/readiness false. Functional test correctness cannot grant runtime acceptance, source visibility, hardware support or a production guarantee.

## Plan of Work


### M5.1: preserve accepted inputs and define association coverage


Inspect the checkout/instructions and verify current M4/M2/original checksum lists before changing accepted source/docs. Record starting Git revision, worktree changes, toolchain/OS identities and drift in `evidence/prototype/milestone-5/inputs.json`. Do not equate a revision with uncommitted files. Preserve exact accepted M4 artifacts at original relative paths under `evidence/prototype/milestone-5/inputs/milestone-4/`, including its checksum list and report. Resolve all prerequisite sources, original fixtures, scope/registry/status/budgets, construction evidence and validation/engine source/lockfiles required for predecessor reproduction. Deduplicate paths and exclude targets, secrets and unrelated work. Verify the original M4 list against the predecessor before live edits and run fixture validation against that root. Do not regenerate historical hashes when live code intentionally changes.

Create `docs/prototype-enrichment-contract.md` with the exact fixture adapters and evidence/time rules below. Create `docs/prototype-enrichment-coverage.json` retaining exactly M01–M58, each with its original definition reference, selected scenario/facet, evidence state, temporal/physical limits and untested remainder. Preserve complete original definitions or hash/reference their frozen copies; never turn one facet test into association-wide supported=true. M02/M30 publisher candidates, M11/M12/M31 remote subjects, M23/M25/M26 process/cgroup/container, M09/M37/M40 replica resource sets, M43/M47/M49/M50/M52 interface/capability/intent/path and M58 visibility are relevant facets. M21/M22/M24/M32/M33/M35/M38/M44/M55/M56 require host/boot/namespace/device/use/version evidence that may remain absent or symbolic. Keep their missing facts explicit rather than claiming those associations resolved from a name.

Acceptance is a verifiable predecessor and a coverage inventory retaining all 58 associations and the original 17-assertion M5 scope. No new hardware or production support list is selected.

### M5.2: define typed relationships and additive boundary fixtures


Add `src/identity.rs` and, if useful for separation, `src/enrichment_fixture.rs` and `src/enrichment_scenario.rs`. Define scoped entity keys, explicit incarnation, evidence references, quality state, optional source clock/validity and query types below. Treat existing strings such as host-boot/PID17 or fixture-netns/ifindex7 as opaque symbolic entity locators; do not parse their spelling into verified host/boot/PID/namespace facts. A fixture correlation scope based on bundle/variant identity isolates test records but is not a discovered host namespace. Additional cases provide explicit host/boot/namespace fields where that distinction is tested.

Extend `metric_input.rs` narrowly for the known PX06/PX07 link/gauge/policy fields and decode them into strict typed records. Preserve rejection of unknown required fields/tags. Add QualityState::Resolved for the existing historical_owner oracle without changing M4 serialization for existing states. Keep original M4 adapters/commands/output schemas stable. No general permissive flatten-and-ignore path is acceptable.

Author `fixtures/enrichment-v1/manifest.json` and README plus local inputs/expected files, using Rust-only authoring helpers and literal independent expectations. Use schema 1, synthetic true, bundle ID aether-enrichment-fixtures-v1, original bundle/association-baseline/registry hashes, per-artifact paths/roles/media/bytes/SHA-256, case IDs and claim limits. Bound each file to 1 MiB, each case to 32 unique entities/32 unique relationships/64 observations or queries, with each initial or update record array limited to 64 and a deduplicating merge limited to 128 raw records, label values to 256 bytes and the extension to seven named cases. These are explicit local test limits, not agreed PX runtime budgets. Validate packaging, schema, references, clocks, sizes/hashes and original-input provenance before execution. Expected values are never generated from resolver output. An identical authoring rerun changes nothing and conflicting existing files are protected.

The seven cases are E5-01 through E5-07, with explicit purpose and expected outcomes. E5-01 supplies virtual queries at 5, 10, 15 and 20 microseconds over the original old/new intervals [0,10) and [10,20): old resolves to container-A at 5, new to container-B at 10/15, old at 10 and either at 20 are outside their validity, and an interval crossing [5,15) cannot resolve to one owner. E5-02 tests zero/one/two publisher candidates, overlapping evidence and reversed input order; no first-candidate selection. E5-03 supplies same local numeric IDs across different explicit boot/PID/net namespaces and a missing-incarnation query; no cross-scope/lifetime merge. E5-04 supplies one count 100 with two heterogeneous allocated GPU members and a same-model other replica; output remains one measurement and no participating-GPU claim is inferred. E5-05 supplies explicit resource-membership windows changing at 10 and proves an old enriched record remains unchanged after new membership is ingested. E5-06 separates NIC capability, configured preference, control socket and absent payload/library evidence across explicit NIC scopes; active transport/operation mapping stay unknown. E5-07 combines denied metadata, known process facts, safe/sensitive labels and identical/conflicting repeated evidence; known facts survive, unknown owner stays unknown, safe serialization and idempotent cardinality are tested, and conflicting identity reuse is rejected or reported ambiguous as prescribed below.

These are intentionally authored test values, not facts inferred from a missing deployment. Original v1 fixtures and their file index remain unchanged. New extension assertions have their own E5 IDs and counts; they do not increase the original 49 or metric/association catalogs.

### M5.3: implement deterministic time-aware resolution


Implement scoped lookup and association in `identity.rs`. A matching local ID is insufficient: require the supplied entity scope and incarnation, compatible evidence kind, and appropriate validity. A unique justified candidate can be resolved; none is unknown/unavailable with a reason; several non-equivalent candidates remain ambiguous with sorted candidate IDs and evidence. Do not choose by insertion order, first PID, model alias, hostname pattern, IP, GPU ordinal or similar spelling.

Use half-open intervals [start,end): a relation ending at 10 does not apply at 10. For an interval observation require one relationship to cover the entire interval; crossing lifetimes stays unknown/ambiguous instead of being labeled with a current owner. Validate ordered nonempty intervals and clock identity. Unknown or incompatible clock evidence prevents exact temporal resolution; do not use wall time. For original identity/membership records with integer validity bounds, document the finite adapter binding to the envelope's single declared microsecond clock; zero/multiple unmatched clock domains cannot silently select a default. This binding is local fixture format interpretation, not generic host-clock discovery. A relation with no validity bounds remains temporally unspecified, even if a related entity has its own interval.

Distinguish reported candidates/declared membership from resolved time-valid ownership. Original PX04's untimed listener can retain an ambiguous reported set; it cannot certify which process owned a real endpoint at collection time. Original PX05's untimed membership can be reported as a declared device set with a completeness flag; it cannot establish participation or time validity. Keep explicit unknown validity in every such output. The additive cases supply times to exercise stronger resolution honestly.

Preserve immutable measurement identity/native fields/value and attach relationship references separately. The association engine returns one enriched record per input observation. Resource relationships are sets, never a Cartesian expansion of values. Identical repeated evidence is idempotent; the same stable evidence ID with different content is a structured conflict, not last-write-wins. Distinct evidence records supporting incompatible valid owners produce ambiguity. Dangling typed entity references are invalid; explicitly marked symbolic/unresolved resource references may be retained as such without inventing physical nodes.

Store historical evidence by scoped incarnation and validity. Adding a newer relationship cannot mutate old enriched records or replace a historical query's evidence. Preserve emitter/subject/allocated/observed-participant references separately; unknown does not mean an empty verified set. Maintain deterministic sorting and finite bounds.

### M5.4: implement publisher, remote subject and process/container adapters


Add named scenarios publisher-identity for both PX04 variants and process-lifecycle for both PX06 variants. Use original runtime inputs, not oracle target values, to build entity/candidate/relationship records. PX04-overlap-and-remote must retain frontend as emitter and remote-worker as subject, while worker_host remains unknown. Reuse M4's explicit logical-count calculation for its original count assertion, preserving both raw counts; enrichment cannot merge their timing populations. PX04-ambiguous-publisher must output sorted p1/p2 with ambiguous state, retaining unspecified listener time and no claimed actual host. The existing oracle's candidate IDs are read only after runtime computation for comparison.

PX06-pid-reuse must join process_record old with old-membership using the explicit incarnation and [0,10) bounds, yielding historical_owner=container-A with resolved synthetic association state and evidence old/old-membership. Record the new incarnation/membership separately and prove additional explicit queries return container-B only within [10,20). The original input supplies metadata, not an actual sampled GPU/engine measurement; report a metadata association query, not fabricated telemetry. The denied variant preserves the known process incarnation while container stays unknown and source denial remains explicit. Do not read a real runtime socket or attempt automatic permission escalation.

Missing query time/incarnation, conflicting candidates, cgroup movement or an observation spanning a change must retain qualified outcomes. A cgroup/container string is a symbolic membership assertion in this fixture, not a verified production runtime-task mapping. Do not treat parentage or cgroup-path substrings as engine/replica identity.

### M5.5: implement resource, NIC and visibility adapters


Add resource-membership for both PX05 variants and fabric-context for both PX07 variants. In the complete PX05 case retain count 100 exactly once, reference the supplied set gpu-A/gpu-B and retain the two symbolic models as a set. The supplied model/member alignment is an explicit fixture declaration, not driver discovery. Keep the same-model replica-B distinct. Allocation/declared membership does not imply observed GPU use, UUID/PCI/MIG identity or a resolved physical host. In the partial case composition is the supplied [gpu-A] set with partial state; missing total still prevents occupancy. Reuse M4 occupancy calculations without creating one count per member. Any identity/temporal property not supplied stays unknown.

PX07-inventory-only retains symbolic NIC identity, byte gauge 1024 at its supplied scope, capability set [ethernet,rdma], configured preference roce and synthetic driver text as separately sourced facts. There is no explicit device/port key on the capability record; do not attach its model/driver facts to the NIC merely because both occur in one file. Active transport and loaded CUDA/NCCL remain unknown. PX07-control-socket-only retains protocol tcp and the symbolic control peer but operation_to_nic stays unknown; no payload path or remote host is inferred. Additional explicit scoped tests prove the resolver keeps NIC namespaces/lifetimes separate without certifying real topology or fabric support.

Add visibility-enrichment for both PX11 variants. Reuse accepted M4 policy/label sanitation behavior and carry denied/disabled/unknown evidence into association outputs. Known supplied facts remain available even when runtime metadata is denied. Reject forbidden action descriptions as data and keep readiness false; do not execute any action. Do not join on redacted label placeholders or leak the synthetic sensitive marker through evidence keys, source fields, diagnostics, comparison diffs or ledgers. Preserve original source facts in bounded transient structures and apply the existing safe output boundary before serialization. Historical fixture/source archives retain exact original invented bytes and remain separate from sanitized output evidence.

Acceptance is the original 17 assertions plus explicit extension expectations and single-observation cardinality invariants, not a claim that every inherited M association is fully implemented.

### M5.6: integrate independent verification and current tracking


Extend CLI dispatch with the five scenarios above and enrichment-boundaries for E5-01–E5-07. Reuse run-scenario/verify-scenario with explicit compatible variant selection; the boundary command uses --case E5-01 rather than pretending an extension case is an original variant. Reject incompatible flags/scenario pairs, unsupported case IDs and arbitrary paths/URLs before execution. For example, run-scenario --scenario enrichment-boundaries --case E5-01 consumes the fixed hashed extension manifest. Keep existing M4 scenarios and verify-milestone --milestone 4 behavior/results unchanged. Query/collect/replay/token-rate remain not_prototyped.

Add verify-milestone --milestone 5. Runtime association APIs receive input records/queries only; verification obtains actual results first and then separately loads original/extension expectations. Unknown, partial, reported_only, ambiguous, denied and resolved are meaningful distinct outcomes. Exact original target/value/unit/state matches must be tested; supplemental relation evidence/validity/cardinality is checked independently. A required mismatch or missing assertion exits nonzero; no skipped assertion can count as matched.

Current M5 verification considers five groups/ten original variants/17 original assertions, seven newly exercised and ten prior M4 checks reverified. Retain a unique identity for every original assertion and all twelve groups/49 original assertions. Link unchanged M4 evidence for prior-only assertions, rather than calling them executed by M5 if they were not rerun. Keep the original M4 ledger immutable and add `docs/prototype-enrichment-execution-status.json`. Each entry records assigned stage, current function execution/comparison, provenance/result/evidence hashes, temporal/physical limits and any remaining work. Extension assertions live in a separate section with E5 IDs; original totals remain 49.

If all planned checks pass, the combined original-assertion view can contain 31 functional matches, two M4 expected rejections, one M4 proof, the PX12 queue deferral and 14 PX08/PX09/PX10 assertions not run. This is a conditional accounting target derived from inputs, not a current execution result or whole-PX pass. M5's 17 comparisons and extension counts must also be reported separately so repeated checks cannot inflate coverage. Preserve selected/untested facets for every M01–M58 in the new coverage file.

Use a separate Rust evidence-authoring helper such as `examples/record_enrichment_evidence.rs`; do not reset the M4 metric ledger or change its hard-coded historical evidence path. Protect differing existing output; identical reruns should be safe. Include source/original-bundle/extension/expected hashes, actual record cardinality and readiness false. Keep px_runtime_acceptance_status=blocked_unagreed_budget, real_verification=not_run and production_support_claimed=false. Functional resolution of invented metadata does not prove the data can be collected non-invasively on a real host.

### M5.7: test actual association and lifecycle behavior


Add `tests/enrichment.rs` and isolated Rust unit tests as needed. Cover all ten original variants and seven extension cases, original assertions, exact boundary times, interval crossing, unknown clocks, missing incarnations, boot/namespace collisions, ambiguous/conflicting candidates, dangling references, duplicate idempotence, input-order independence, unchanged old outputs after updates, membership changes and incomplete composition. Assert emitter/subject/allocation/participation remain separate and compare measurement IDs/counts/values/native names/labels/units before and after enrichment. A count 100 linked to two GPUs must remain one measurement totaling 100.

Test capability/configuration/use/version separation, unattached capability facts, TCP control-path ambiguity, denial with known process context, sanitation across all serialization/error/evidence paths and missing data without healthy zeros. Test extension manifest size/hash/path/symlink/duplicate/schema failures and independent expected-answer mismatches; negative packaging must never bypass validation. Test invalid CLI pairing, deterministic output, complete 58-association inventory and non-inflated 49-original-assertion accounting. Mutate only temporary trees and never edit original oracles to make a resolver pass.

All 129 predecessor tests must pass alongside meaningful new tests, formatting/check/strict Clippy and unchanged evidence/traceability/fixture checks. Retain current M4 summary/ledger behavior as a regression; record the actual 32 new plus 129 prior passing tests. Existing network tests still need approved loopback access if sandbox sockets are denied; do not skip or substitute them. New lifecycle tests use virtual time and never sleep to simulate a process lifetime.

Write `evidence/prototype/milestone-5.md` and adjacent logs/reports/inputs/hash list. Run representative publisher, PID lifecycle, GPU membership, fabric context and boundary commands twice and compare stable output. Verify M4 historical artifacts in the preserved predecessor root and unchanged M2/original baselines against live files. Record actual discovery/fix history and unresolved association facets. No live collector/device, runtime overhead or hardware compatibility is accepted.

### M5.8: review acceptance, synchronize and hand off


Mark M5 complete only after original assigned assertions, extension expectations, cardinality/history/privacy invariants, predecessor regressions and preservation checks pass. Update all eight tasks and living sections here, aggregate status in `EXECPLAN.md` and the folder index. A remaining required task stays pending; do not relabel an unimplemented case deferred simply to finish.

Keep original requirements/mappings/fixtures/budgets and historical execution ledgers intact. Report what was functionally resolved, reported only, unknown, ambiguous or partial; do not equate a resolved synthetic relationship with real device/runtime support. Suggest M6 bounded buffering, local recovery journal, Arrow/Parquet output and DataFusion queries with simulated storage faults next. M6 dependency selection and production storage architecture remain unperformed by M5.

## Concrete Steps


Run from `/home/mbellary/work/projects/DeepMon`. Current prerequisite commands exist now:

    git status --short
    rustc --version --verbose
    cargo --version --verbose
    sha256sum --check --quiet evidence/prototype/milestone-4/artifacts.sha256
    sha256sum --check --quiet evidence/prototype/milestone-2/artifacts.sha256
    sha256sum --check --quiet evidence/prototype/milestone-1/baseline-before.sha256
    cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- evidence --root .
    cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- traceability --root .
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- validate-fixtures --root .

Expect 250/58 original IDs, twelve specifications, fixture counts 12/36/82/2 and runtime readiness false. Historical traceability not_run fields do not erase actual M4 function execution recorded in its separate ledger. Check each exit code. After snapshot creation, use working directory `/home/mbellary/work/projects/DeepMon/evidence/prototype/milestone-5/inputs/milestone-4` and run:

    sha256sum --check --quiet evidence/prototype/milestone-4/artifacts.sha256

Return to repository root and verify frozen-fixture reproduction there:

    cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- fixtures --root evidence/prototype/milestone-5/inputs/milestone-4

These commands are implemented and their actual outputs are saved in the M5 evidence directory:

    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- run-scenario --root . --scenario publisher-identity --variant PX04-ambiguous-publisher-v1
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- verify-scenario --root . --scenario process-lifecycle --variant PX06-pid-reuse-v1
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- run-scenario --root . --scenario resource-membership --variant PX05-replica-set-v1
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- run-scenario --root . --scenario fabric-context --variant PX07-inventory-only-v1
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- verify-scenario --root . --scenario enrichment-boundaries --case E5-01
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- verify-milestone --root . --milestone 5

Expected behaviors are ambiguous [p1,p2] with unspecified listener time; historical container-A matched from actual resolution; one count 100 with a device set; reported capability with unknown active path; explicit time-boundary outcomes; and complete stage-aware original/extension accounting. Save each into fresh named evidence files twice and compare with cmp. Stable JSON excludes wall time, ephemeral ports and random paths.

After implementation run:

    cargo fmt --manifest-path prototypes/aether-poc/Cargo.toml -- --check
    cargo check --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml
    cargo test --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml
    cargo clippy --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml --all-targets -- -D warnings
    cargo test --locked --offline --manifest-path tools/aether-validation/Cargo.toml
    cargo fmt --manifest-path tools/aether-validation/Cargo.toml -- --check
    cargo clippy --locked --offline --manifest-path tools/aether-validation/Cargo.toml --all-targets -- -D warnings
    git diff --check

Use the environment's normal approval for socket regressions or genuinely required unavailable dependencies; do not bypass restrictions. No new dependency is planned. Repeat existing M4 whole-milestone verification through approved loopback access to prove its output semantics remain unchanged; preserve its old ledger/report hashes rather than overwrite them.

## Validation and Acceptance


Plan-authoring acceptance requires all PLANS.md sections/format, eight pending implementation tasks, valid current links, synchronized master/index and passing existing fixture/traceability/M4 hash checks. No M5 resolver or new test execution is claimed by writing this plan.

Implementation acceptance requires actual evidence-based resolution and independent comparisons for five original groups/ten variants/17 assertions and all seven separately versioned boundary cases. Preserve all 58 association definitions and non-inflated original 49-assertion tracking. Ambiguous ownership retains candidates, missing ownership stays unknown, partial membership remains partial and reported NIC capability stays distinct from observed traffic. Temporal queries honor half-open validity and compatible clocks; newer evidence cannot relabel old records. Enrichment preserves observation count, ID, value, scope and native facts, with set references instead of per-resource value copies.

Every stronger association needs supplied evidence, identity, time and scope; unspecified fields remain unspecified. Supplemental tests must not masquerade as captured device schemas or fill gaps in original inputs. Actual new/regression counts, deterministic reports, redaction, CLI errors, predecessor hashes and static checks must pass and be recorded. All 129 predecessor tests remain required. Original catalogs/mappings/source snapshots/fixtures/budgets and M4 ledger stay unchanged.

Functional success does not grant whole-PX runtime acceptance, real schema/capability support, physical microsecond accuracy, zero overhead or production readiness. Unagreed required budgets remain blocked, real verification not_run and readiness false. M5 can still be accepted for the scoped functional behavior when that distinction is explicit and no required functional case remains incomplete.

## Idempotence and Recovery


Read-only run/verify commands do not modify original sources or fixtures. Rust authoring helpers create the extension and ledger only at intended destinations, accept identical reruns and refuse conflicts. Keep metadata enrichment immutable: repeated application returns the same measurement IDs/values and set membership, not appended duplicate rows. Use Rust-owned temporary directories for mutation tests; never delete broad temporary or repository paths.

Complete and verify the M4 predecessor before editing accepted code/docs. On interruption verify existing archive bytes before continuing; copy only missing verified files. Preserve unrelated work and never rewrite old checksum expectations. If source inputs drift, stop the affected acceptance claim and explain why. A genuine fixture defect needs an additive versioned correction with evidence and a plan revision; original v1 remains historical.

Use existing lockfiles for retries. Close/join owned server threads on regression failures. Do not run metric evidence authoring to reset the frozen M4 ledger. A changed M4 summary is a regression to diagnose, not a reason to replace expected history. Additional source fields or cases must be documented with strict schemas and clear test-only provenance before being accepted.

## Artifacts and Notes


Delivered M5 artifacts are `identity.rs` and supporting typed loaders/scenario/verification code, `tests/enrichment.rs`, Rust authoring/evidence helpers, `fixtures/enrichment-v1/`, `docs/prototype-enrichment-contract.md`, `docs/prototype-enrichment-coverage.json`, `docs/prototype-enrichment-execution-status.json`, updated current harness/dependency notes and `evidence/prototype/milestone-5.md` with adjacent predecessor/input/report/log/hash files. All now exist; the evidence report and artifact checksum list record acceptance.

The saved publisher output has the following meaning (an abbreviated projection of actual fields):

    scenario: publisher-identity
    target: publisher
    candidates: [p1, p2]
    state: ambiguous
    temporal_status: unspecified_input
    functional_execution_status: executed
    px_runtime_acceptance_status: blocked_unagreed_budget
    runtime_acceptance_readiness: false
    production_support_claimed: false

## Interfaces and Dependencies


In `identity.rs` define `EntityKey`, `EntityRef`, `Relationship`, `Validity`, `AssociationQuery`, `AssociationResult`, `EnrichmentInputMeasurement`, `EnrichedObservation` and `IdentityGraph`. An EntityKey contains explicit correlation scope, entity kind, opaque locator/incarnation and optional evidenced host/boot/namespace facts. EntityRef distinguishes fully keyed entities from explicitly symbolic/unresolved references. Validity is an optional interval plus clock evidence, never a default current timestamp. AssociationQuery selects a point, full interval or unspecified time and preserves requested relation semantics.

Expose `IdentityGraph::from_records(records: &[MetadataRecord]) -> Result<IdentityGraph>`, `resolve_association(graph: &IdentityGraph, query: &AssociationQuery) -> AssociationResult` and `associate_observation(observation: &EnrichmentInputMeasurement, graph: &IdentityGraph, query: &AssociationQuery) -> Result<EnrichedObservation>`. Results contain quality/candidates/evidence, temporal status and physical-identity limitations. One EnrichedObservation references the original measurement and relationship sets without cloning its value per member. Add explicit immutable update/merge behavior for identical evidence and structured conflicts; no global mutable current-owner map.

In `enrichment_fixture.rs` or a similarly named isolated module, expose `load_enrichment_bundle(root: &Path) -> Result<EnrichmentFixtureBundle>` and typed strict original metadata adapters. Validate the fixed extension manifest and references through bounded confined readers. Reuse separate original input/oracle loaders, extend known original record fields narrowly and never expose expected values to the resolver. New fixture authors use literal independent answers.

In `enrichment_scenario.rs`, expose `run_enrichment_scenario(bundle: &ValidatedBundle, scenario: EnrichmentScenario, variant_id: &str) -> Result<EnrichmentReport>` and `verify_enrichment_scenario(bundle: &ValidatedBundle, scenario: EnrichmentScenario, variant_id: &str) -> Result<EnrichmentVerification>`. For extension cases expose `run_boundary_case(bundle: &EnrichmentFixtureBundle, case_id: &str) -> Result<BoundaryReport>` and `verify_boundary_case(bundle: &EnrichmentFixtureBundle, case_id: &str) -> Result<BoundaryVerification>`. `EnrichmentScenario` names publisher-identity, process-lifecycle, resource-membership, fabric-context and visibility-enrichment. Stage-5 verification can reuse M4 computations for shared assertions while producing association evidence independently. CLI milestone 4 dispatch stays unchanged; milestone 5 uses these new adapters and a combined unique-original-assertion view.

Reuse serde/serde_json, standard-library containers/time arithmetic, existing SHA-256/confined readers and test-only tempfile. Add no real runtime/device/library clients or production workspace/concurrency model. M6 will select and test Arrow/Parquet/DataFusion; this milestone does not finalize storage, catalog management, production identity services or release compatibility.

## Revision Notes


2026-11-01 23:39 IST: created the detailed M5 plan under PLANS.md following the user's request. Defined eight pending tasks, five original groups/ten variants/17 assertions, seven additive synthetic boundary cases, strict identity/time/evidence/cardinality rules, preserved M4 behavior/ledger, complete 58-association/current execution tracking and acceptance/evidence requirements. Current M4 artifact/fixture/traceability checks pass. No M5 implementation is delivered by authoring; master/index tracking updated.

2026-11-01 23:47 IST: verified plan structure/current links, eight pending task entries and original fixture coverage/references; existing Rust evidence/traceability/fixture checks and accepted artifact hashes pass. Mirrored results in the master. Only planning is complete; all M5 implementation remains pending.

2026-11-02 00:15 IST: completed M5.1–M5.8 in Rust, recorded actual functional results and 161 passing tests, preserved and verified M4 predecessor evidence, added strict seven-case extension/current facet ledgers and synchronized master/index. Updated current state, discoveries, decisions, outcomes, interfaces and validation evidence. M6 is next; no production or live-system support is claimed.
