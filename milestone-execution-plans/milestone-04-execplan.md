# Milestone 4: parse native metrics and calculate justified synthetic results


This ExecPlan follows repository-root `PLANS.md`. Keep `Progress`, `Surprises & Discoveries`, `Decision Log`, `Outcomes & Retrospective` and the bottom revision note current at every stopping point. The workspace is `/home/mbellary/work/projects/DeepMon`; paths below are repository-relative. `EXECPLAN.md` tracks the full prototype program, and `milestone-execution-plans/README.md` indexes detailed milestone plans.

Current state: the user authorized execution; M4.1–M4.8 are complete for the defined synthetic functional scope. Actual evidence is `evidence/prototype/milestone-4.md`. The new ledger truthfully records current function execution and per-assertion comparisons while preserving historical planning bytes. Runtime acceptance remains blocked/unagreed, real verification not_run and readiness false. M5–M8 implementation remains pending; M5 enrichment/lifecycle handling is next.

## Purpose / Big Picture


Let a developer run the Rust harness against invented native metric exports and see what the values mean, which calculations are justified and which answers the inputs cannot establish. After implementation, commands will preserve original metric names/types/labels and source scope, calculate five successful requests per second from the supplied snapshots, retain ten explicitly linked logical requests instead of double-counting twenty and explain why missing evidence prevents exact goodput, variance or freshness claims.

A native observation retains the metric as published. A counter accumulates a count and may reset; a gauge reports a current value; a histogram publishes cumulative bucket counts, a count and a sum. A histogram does not retain individual request events. A fixture is invented input, and an oracle is its independently authored expected answer. A population means the particular events being counted, such as successful requests at one publisher boundary. A scope identifies what a measurement describes, such as a server or replica. Different populations/scopes cannot be combined merely because their names or model labels look similar.

This remains an intermediate Rust proof of concept. Retain all 250 metric requirements, 58 metadata associations, four observability layers and twelve PX specifications. vLLM/SGLang/Dynamo are initial reviewed sources, not an exclusive list; include the relevant TGI/Triton/TensorRT-LLM fixtures. DeepInfer remains deferred. No cluster exists; no live engine, hardware, container socket, probe, external URL, cloud storage or production architecture is required or approved. Rust is used for implementation, generators and tests; do not invoke Python.

## Progress


Times use Asia/Kolkata, UTC+05:30. Historical planning checks and actual functional acceptance are recorded separately.

- [x] (2026-11-01 23:04 IST, inspected) Read `PLANS.md`, master M4 scope, accepted Rust interfaces, registry, fixture inputs/oracles and M3 evidence.
- [x] (2026-11-01 23:04 IST, prerequisite verified) M3's 43-artifact checksum list matches; current Rust fixture validation reports 12 groups/36 variants/82 payloads/two intentional invalid payloads. Traceability retains all 250/58 IDs, no missing IDs, zero executed PX experiments and runtime readiness false.
- [x] (2026-11-01, plan authoring) Created this detailed plan and synchronized master/index tracking. Authoring verification is recorded separately below.
- [x] (2026-11-01 23:10 IST, plan verified) Required sections/heading format, current links, eight pending tasks and seven-group/23-variant references pass. Existing Rust evidence/traceability/fixture checks and M3/M2/original baseline hashes pass; no M4 code or execution is claimed.
- [x] (2026-11-01 23:34 IST, verified) M4.1: preserved M3 predecessor and dependency inputs; original 43-file checksums and predecessor fixture validation pass; recorded inputs/index and exact coverage.
- [x] (2026-11-01 23:34 IST, verified) M4.2: added typed records, observations, source context/quality/results and separate bounded input/oracle loaders; runtime receives no oracle worlds.
- [x] (2026-11-01 23:34 IST, verified) M4.3: independent bounded parser and actual loopback acquisition pass seven native cases and two precise rejections; optional absence stays unknown.
- [x] (2026-11-01 23:34 IST, verified) M4.4: rates/reset/population guards, logical-count selection, occupancy and aggregate insufficiency pass independent comparisons and edge tests.
- [x] (2026-11-01 23:34 IST, verified) M4.5: denied/disabled/unknown states, safe projections, simulated forbidden-action rejection, blocked readiness and unknown freshness pass; no real action executes.
- [x] (2026-11-01 23:34 IST, verified) M4.6: run/verify/milestone commands and separate ledger account for all 49 assertions: 24 matches, two rejections, one proof, three in-scope deferrals and 19 later-stage assertions not run.
- [x] (2026-11-01 23:34 IST, verified) M4.7: 30 new tests plus all 99 regressions pass; formatting/check/strict Clippy/integrity and five deterministic CLI pairs pass. Actual logs/hashes/report captured.
- [x] (2026-11-01 23:34 IST, accepted) M4.8: reviewed functional acceptance and synchronized master/detail/index. M5 is next; runtime acceptance and real capabilities remain unverified.
- [x] (2026-11-01 23:36 IST, final verification) Plan structure/current links, eight completed task states and whitespace pass. All 187 indexed predecessor files match their recorded hashes; runtime JSON/logs/ledger contain no synthetic sensitive marker. Current-source artifact hashes are recorded separately from historical acceptance.

## Surprises & Discoveries


Observation: checking a parsed floating number alone can hide rounding of a large fractional counter token.

Evidence: runtime count/counter validation now checks original decimal integer tokens within 2^53−1 before accepting exact counts; the precision test rejects both oversized integers and a large fractional token. No fixture values or expected answers changed.

Observation: complete predecessor reproduction needs planning dependencies beyond M3's own artifact list.

Evidence: after adding unchanged scope/budget planning inputs and the M2 construction log, full fixture validation passes against the archive. The original 43 M3 artifact hashes also pass there; source/index hashes are recorded separately from current M4 outputs.

Observation: two early new-test failures were incorrect test expectations rather than engine results.

Evidence: the native alias is fixture-model, and an exact-field assertion replaced a substring check that matched part of exact_goodput_and_variance. Final 30 new tests and all 99 regressions pass. Initial logs remain clearly distinct from final evidence.

Observation: the positive rate fixture's `.prom` file contains only the first value, while its JSON input contains both time-stamped counter records.

Evidence: `endpoints/PX02-success-rate-v1.prom` exports 100; the paired JSON records 100 at zero and 150 at 10,000,000 virtual microseconds. M4 must label the rate as calculation over supplied synthetic records. Serving that one endpoint snapshot does not demonstrate polling, a second capture or source freshness.

Observation: expected proof material is deliberately withheld from runtime input.

Evidence: both PX03 inputs contain aggregate count/sum/buckets and separate success count. Only `expected/PX03-aggregate-proof-v1.json` supplies the two hypothetical event worlds with different variance/goodput. Those worlds may prove information loss in independent tests but must not become runtime request observations.

Observation: M4-related groups include assertions whose implementation belongs to later milestones.

Evidence: PX04 ambiguous publisher resolution and PX05 membership/composition belong to M5; PX12 queue admission belongs to M6. Keep every assertion visible with its assigned stage. Comparing the M4 subset must not mark a whole multi-stage group accepted.

Observation: all per-PX budgets remain proposed/not agreed and existing validators preserve the accepted planning records.

Evidence: required runtime agreements/limits remain unset and readiness false. M4 needs a new functional-result ledger and honest execution evidence, not edits to historical not_run fields or a runtime-acceptance pass inferred from unit tests.

## Decision Log


Decision: author this plan now; leave all eight implementation tasks pending.
Rationale: the user's concrete request is a detailed execution plan and synchronized progress. Plan completion is separate from implementation acceptance.
Date/Author: 2026-11-01 / Codex, applying user direction.

Decision: run bounded synthetic functional correctness checks separately from evaluated PX runtime acceptance, which remains blocked by unagreed budgets.
Rationale: tests can record that parser/calculation functions ran and matched known answers without asserting measured overhead, freshness, acquisition loss or an agreed budget pass. Preserve the original planning ledgers and add explicit current functional evidence rather than hiding executed code behind a blanket not_run claim.
Date/Author: 2026-11-01 / Codex.

Decision: implement a documented finite exposition parser in the prototype without importing the validation tool's private parser as the runtime implementation.
Rationale: the construction checker and runtime engine serve different purposes. The runtime must emit observations and failures from actual input, not reuse an oracle consistency verdict as a measurement. Keep the existing validation source unchanged and use independent literal expected answers/tests.
Date/Author: 2026-11-01 / Codex.

Decision: preserve M1/M2 records and frozen fixtures/sources; preserve a verifiable M3 predecessor before changing the engine package.
Rationale: M3's accepted artifact list includes the engine source/docs/lock. New code cannot still match old hashes at live paths. Archive exact accepted bytes, verify the original checksum list against the archive and record M4 hashes separately. Never regenerate historical expectations to force a pass.
Date/Author: 2026-11-01 / Codex.

Decision: explicitly separate M4 assertions from M5/M6/M7 behavior inside shared PX groups; do not reduce the product requirements or silently exclude sources.
Rationale: source parsing, calculation, relationship resolution, queue/storage and event replay are different experiments. A partial functional result must list its remaining assertions and cannot imply complete capability or engine support.
Date/Author: 2026-11-01 / Codex.

Decision: accept the authorized M4 implementation for scoped functional correctness and retain later-stage/runtime/real qualification as pending.
Rationale: actual loopback parser results, precise negative errors, independent calculations/unknowns, full assertion accounting, 129 passing tests and preserved inputs satisfy the plan. The new ledger reports executed functions without rewriting accepted historical not_run fields or granting a whole-PX budget pass. No dependency or research requirement changed.
Date/Author: 2026-11-01 23:34 IST / Codex, applying user direction.

Decision: verify original count tokens before floating conversion and keep report-safe projection distinct from transient native records.
Rationale: precision and sanitation must remain explicit; passing small positive fixtures cannot justify silently rounded counters or leaking sensitive fields. These finite local safeguards do not establish production numeric/exporter/security compatibility.
Date/Author: 2026-11-01 23:34 IST / Codex.

## Outcomes & Retrospective


M4 is complete within its defined functional prototype scope. Runtime native parsing, source-scoped calculations, missing/policy states, independent comparison, seven scenario adapters and a separate per-assertion ledger are implemented in Rust. All 30 new metric tests, 29 prior harness/client tests and 70 validation regressions pass (129 total), alongside strict Clippy, formatting, locked check and preserved-input verification. Actual evidence is `evidence/prototype/milestone-4.md` and its adjacent logs/reports/hash list.

Authoring verification at 23:10 IST passed required sections/heading spacing, current Markdown links, eight pending task entries and manifest-backed seven-group/23-variant references. Existing Rust evidence/traceability/fixture commands and accepted M3/M2/original baseline checksums pass, as does `git diff --check`. The original 250/58 identities and all scope/semantic gaps remain unchanged.

All twelve groups/36 variants/49 assertions remain visible. Current functional outcomes are 24 matches, two expected rejections, one proof, three genuine later-stage deferrals in the seven considered groups and 19 assertions not run in later-stage groups. Five CLI pairs are byte-identical. M3's accepted bytes verify in their preserved predecessor root and its fixture validation passes; original M1/M2/source/fixture/budget records remain unchanged. Loopback acquisition/tests used approved execution after sandbox denial. No whole-PX runtime pass, real capabilities or production design is established. Next is M5 simulated metadata enrichment and lifecycle handling.

## Context and Orientation


The repository's research baseline is `aether-metrics-catalog.json` and `.md`, `metadata-enrichment-matrix.md`, `metric-to-source-coverage-matrix.json` and `.md`, `engine-native-metrics-baseline.md` and `source-mapping-recheck.md`. They retain all 250 metric families and 58 associations. Research routes E:88/R:35/H:100/I:26/U:1 mean endpoint/calculation/additional collector/deeper instrumentation/unresolved mapping; they are not a support percentage. L1-001's raw TGI counter still lacks a verified accepted-inference-request-only population; L1-005 inherits that gap. L2-071's skipped/speculated-token semantics remain qualified. M4 does not resolve those gaps by relabeling data.

`evidence/manifest-v1.json` pins ten source inputs E01–E10 and local baseline inputs B01–B11. E01/E02 cover vLLM, E03/E04 SGLang, E05 Dynamo, E06/E07 TGI, E08 Triton, E09 native TensorRT-LLM and E10 its Triton backend. Exact source commits pin research, not deployed releases. `docs/prototype-experiments.json` links twelve cases to requirements, associations, profiles, budgets and source inputs. `docs/prototype-status.json` retains all original objects and selected 23/42 requirement/association references. All original execution states remain accepted historical planning bytes.

`fixtures/prototype/manifest.json` is the supported synthetic v1 bundle: twelve groups, 36 variants, 82 payloads and two intentional invalid endpoint samples. Each variant indexes input/expected and optional endpoint artifacts, with paths relative to `fixtures/prototype`, exact sizes/hashes and declared rejection codes. `docs/prototype-fixture-format.md` specifies envelopes. Inputs contain schema version, synthetic flag, variant identity, clocks and tagged records. Virtual timestamps are invented integer microseconds; a clock domain identifies directly comparable times, with uncertainty/offset metadata. They do not establish physical precision. An incarnation identifies one lifetime of a reused source identity.

The accepted package `prototypes/aether-poc/` has a local `aether-validation` dependency, exact serde 1.0.228/serde_json 1.0.149, test-only tempfile 3.27.0 and its own 34-entry lockfile. `src/config.rs` parses commands, `src/fixture.rs` exposes `load_bundle`/`select_endpoint` and confined bounded reads, `src/endpoint.rs` owns a finite loopback server/client, and `src/report.rs` defines listing and byte-roundtrip reports. `EndpointInput` has private indexed bytes/identities and no expected assertions. Existing commands validate/list fixtures, serve one endpoint and run `endpoint-roundtrip`. Current tests are four client unit tests plus 25 harness integration tests. `tools/aether-validation/` separately has 70 passing integration tests and public `check_evidence`, `check_traceability`, `check_fixtures` and `sha256`. Its private `fixture_prom` parser is construction tooling, not a runtime observation API.

`evidence/prototype/milestone-3.md` records 99 total tests and actual 398-byte/identical-report loopback results; its adjacent `artifacts.sha256` covers 43 accepted files. `docs/prototype-harness.md` and `docs/prototype-dependencies.md` describe the accepted finite HTTP contract and deferred M6 Arrow/Parquet/DataFusion selection. The local reference is Ubuntu 26.04.1, kernel 7.0.0-38-generic, glibc 2.43, Rust/Cargo 1.99.0, x86_64-unknown-linux-gnu. Verify drift; do not infer a supported Linux/GPU/NIC/CUDA/NCCL range. Sandbox socket binds previously required approved execution; never call skipped loopback tests passing.

`zero-application-change-collection-contract.md` prohibits automatic source enabling, workload mutation and privilege escalation in the initial boundary. `docs/prototype-budgets/PX*.json` remain proposed/not agreed with readiness false. Functional test limits below are explicit local test choices, not approved runtime budgets. There is no real collector or workload to qualify here.

## Plan of Work


### M4.1: preserve accepted inputs and define exact coverage


Inspect the checkout and applicable instructions, record starting Git/worktree/tool identities and verify the original M3 artifact list before changes. Create `evidence/prototype/milestone-4/inputs/milestone-3/` as a predecessor root. Copy all paths named in M3's checksum list at their original relative paths, plus the original list, accepted M3 report and any prerequisite fixture/source/registry/status/budget/environment files required to reproduce the checker and prior tests. Resolve source snapshot dependencies through the v1 manifest; copy no target directories, credentials or unrelated work. Verify M3's unchanged list from the predecessor root before editing the live package. Record source-to-snapshot paths and hashes in `evidence/prototype/milestone-4/inputs.json`. Live M2 artifact hashes must continue to pass because the shared validation tooling and accepted fixtures are unchanged; later M3 hash checks use the predecessor root.

Create `docs/prototype-metric-semantics.md` documenting the exact scenario-to-variant/source/unit/population bindings below and the distinction between function checks and PX acceptance. It must retain all twelve groups and every existing oracle assertion, including deferred ones. M4 touches seven groups with 23 variants: PX01 ten, PX02 three, PX03 two, PX04 two, PX05 two, PX11 two and PX12 two. This is an accounting boundary, not a claim that all 23 variants or their assertions pass here. PX06/PX07/PX08/PX09/PX10 stay not_run for M4. The number of comparisons must be derived from actual assertion records, not invented in this plan.

PX01 `native-parse` covers all ten PX01 variants. Seven positive endpoint variants are `PX01-vllm-v1`, `PX01-sglang-v1`, `PX01-dynamo-v1`, `PX01-tgi-v1`, `PX01-triton-v1`, `PX01-trt-native-v1` and `PX01-trt-backend-v1`. Two negative endpoints are `PX01-malformed-type-v1` and `PX01-inconsistent-histogram-v1`, with their exact declared parser rejection codes. `PX01-optional-absent-v1` has no endpoint; its explicit family-absent flag must yield unknown/unavailable duration despite a placeholder zero. Do not invent an endpoint for it.

PX02 `counter-rate` covers its positive success snapshots, reset and raw-TGI variants. PX03 `aggregate-evidence` covers both aggregate-only inputs with exact goodput/variance unknown; the proof-only assertion in `PX03-aggregate-proof-v1` is separately verified as oracle counterexample evidence, not an engine output. PX04 `count-boundary` evaluates the explicitly linked frontend/backend count and preserves unknown remote host; ambiguous publisher resolution remains deferred to M5. PX05 `memory-occupancy` evaluates per-device ratios, preserves the already supplied replica count once and reports missing-total occupancy unknown; membership/composition resolution remains deferred to M5. PX11 `visibility-policy` evaluates both supplied policy inputs as data, including sanitation, denied/disabled/unknown states, forbidden-action rejection and blocked readiness. PX12 `freshness-evidence` evaluates the two gauge snapshots and missing source/persistence/visibility times as insufficient evidence; its queue assertion is deferred to M6. Unit tests for pure calculations do not implement deferred joins, persistence or queue admission.

Acceptance is a verified predecessor plus exact documented assertion coverage. Do not modify accepted registry/status/fixture/budget files just to advance M4 tracking.

### M4.2: define typed runtime inputs and observation/results


Add `src/observation.rs`, `src/derive.rs` and `src/scenario.rs`. Extend `fixture.rs` with separate typed loaders for runtime input versus oracle access. Reuse its confined bounded read logic, recomputing size/hash immediately before use. Runtime input access returns clocks and typed records, never expected assertions or proof worlds. Oracle access is restricted to the comparison layer, after actual results have been produced. Reject unsupported required record fields and invalid IDs/types/clocks instead of returning a successful empty result. Retain unrelated input records as explicitly not_evaluated where a scenario shares a multi-stage fixture.

`NativeObservation` must retain family name, exact sample name, declared type, HELP text, native labels, numeric value, source producer/scope, explicit unit provenance, fixture/record/source-input IDs, publisher context, optional subject and virtual capture clock/time. Preserve the distinction between no subject evidence and a measured zero. Counter units come from explicit source context, not name guesses. An unknown family with a valid declared type remains an unmapped native observation with unknown unit/subject; it is not silently discarded or assigned a canonical requirement. Publisher/subject fields can remain unknown; this milestone must not infer a process/GPU/replica from model labels.

Use separate structures for raw parsed series and report-safe projection. Original labels are preserved within the bounded transient parser result; published results apply the explicit fixture redaction policy and record redacted label keys. No unredacted raw payload or sensitive marker may appear in diagnostics, reports, comparison diffs or persisted logs. Known safe native labels remain exact. Document this boundary so preservation does not become accidental data disclosure.

A derived result contains a target, value or null, unit, quality state, reason code, input record/source references, counting scope/population and virtual interval/clock evidence. Use known/reported_only/unknown/unavailable/ambiguous/partial/denied/not_enabled as distinct data states. Deferred work is not_prototyped with its next milestone, not a numeric zero or a passing result. Invalid syntax is a parser error, not an unknown measurement. Result determinism requires sorted maps/records, with no ports/wall time/random paths in the stable payload.

### M4.3: parse actual endpoint bytes and preserve native semantics


Implement `parse_metric_fixture` in `src/endpoint.rs` or a private `src/metric_text.rs` called by it. Do not repurpose `aether_validation::check_fixtures` or copy expected native values into the parser. Retain the M3 byte-roundtrip command unchanged. For endpoint-bearing `native-parse` runs, acquire bytes through the existing owned loopback server/client, verify indexed byte identity and parse those received bytes. For explicit absence, report no endpoint acquisition and unknown family state. The parser must also be directly testable against invented in-memory text.

Use the documented finite fixture grammar: HELP/TYPE, counter/gauge samples and histogram buckets/count/sum, quoted labels without embedded commas/quotes/braces/escapes, no timestamps/exemplars/summaries. Bound payloads to 1 MiB, samples to 64, label values to 256 bytes and total report records to the bounded input population. Reject unsupported syntax with a stable code instead of silently partially parsing. Preserve spelling, labels and sample values; TYPE determines interpretation. `vllm:iteration_tokens_total` is a histogram despite `_total`. Strip bucket/count/sum suffixes only when explicitly grouping members of a declared histogram; preserve original sample names as well as family membership.

Require consistent declarations and unique series identity (sample name plus label set). Reject duplicate labels, invalid/nonfinite numbers, conflicting/missing types and malformed sample lines. The fixture subset uses nonnegative values; document that negative arbitrary gauges are outside this prototype grammar, not invalid in all real exporters. Histogram validation must group matching non-`le` labels, require increasing bucket bounds, nondecreasing integer cumulative counts, `+Inf=count` and valid count/sum. Preserve the cumulative form; do not treat bucket totals as independent requests. Bad TYPE returns `TYPE_INVALID`; inconsistent histogram returns `HISTOGRAM_INCONSISTENT`, matching the accepted negative samples.

Bind producer/scope/unit to explicit input snapshot/source context, not metric suffix or model alias. Retain separate vLLM, SGLang, Dynamo frontend/router/backend, TGI, Triton and two TensorRT-LLM producer boundaries. Triton's pinned-memory pool remains `server_all_models`; the TRT backend's GPU memory label remains reported source data, not proof of a physical device. A KV-block gauge is not cache-hit rate or a raw iteration event. Missing optional data is absent/unknown, never a zero-valued observation manufactured from a placeholder.

Acceptance is independent parsed output matching seven positive native oracles and exact errors for two negatives, with explicit absence for the tenth. Construction validation still passes for intentionally invalid payloads; the engine parser must actually reject their content. Packaging/hash/path errors cannot be waived because a variant is negative.

### M4.4: implement only justified calculations


In `derive.rs`, implement counter rate over a supplied ordered pair of matching source population/family/labels/incarnation/unit and comparable clock evidence. For PX02 the source binding is the fixture's documented Triton request-success population, and times are supplied virtual microseconds. `(150-100)/(10,000,000/1,000,000)` is 5 requests/second. Preserve both raw counters. A reset/decrease or incarnation change with unknown reset instant yields unknown rate and a reset/incarnation reason; never a negative rate or an invented continuation. Missing samples, unequal labels/scopes, nonpositive duration or uncertain incompatible clocks must not yield an exact rate. A raw TGI `tgi_request_count` remains raw: accepted-arrival semantics and L1-001/L1-005 are unresolved.

The accepted `.prom` rate artifact contains only 100. Do not pretend it contains the second sample, mutate it or use the oracle's value 5 as runtime input. The rate scenario consumes the explicit two-record JSON stream, labels its acquisition kind `synthetic_record_input` and makes no polling/freshness claim. A token rate would require actual token counters and a defined population; none is present here. Leave token-rate mapping not_prototyped. A pure arithmetic unit test does not establish a missing source route.

For PX04, preserve front/back raw counts of ten separately. Use the explicitly supplied logical-ID lists and documented selected counting boundary to return ten, never sum to twenty. Validate lists are unique and count-consistent. Without explicit overlap/linkage or with incompatible populations, report unknown combined logical count. Do not infer overlap from model names. Worker-host absence stays unknown; candidate identity resolution is M5.

For PX05, use explicit `used/total` in matching MiB units for each supplied device: 10/20 and 20/40 are both ratio 0.5. Validate finite nonnegative quantities, positive total and used<=total. Missing/zero/inconsistent totals yield unknown or an explicit invalid-input error, never capacity inferred from used/free. Retain the supplied replica-wide count 100 once; no per-device copies. This is arithmetic over supplied records, not a membership join or certified GPU metric. Do not call this cache-hit ratio. Future cache ratios require matching token/block/request populations and denominators; no such native hit-rate fixture is present.

For PX03, preserve count/sum/buckets and success count but return unknown exact variance and joint goodput with `insufficient_input`. Goodput here would count requests meeting both success and a declared latency objective per window; separate marginal counts cannot prove their intersection. No supplied objective, raw events or second moment may be invented. Independent tests inspect oracle-only worlds: [.25,.75] and [.5,.5] seconds share count/sum/buckets but population variances .0625 and 0; the separate latency/success worlds give .1 and 0 requests/second goodput. Keep those demonstrations in proof verification, never in actual observations. A threshold not exactly represented by buckets yields unavailable exact qualification with a reason; bounded/estimated quantile algorithms are not required by M4.

Use exact integer counters/timestamps within representable bounds and checked arithmetic. Do not silently coerce integers above precise f64 range into exact counters. All fixture answers 5 and .5 are exactly representable; comparisons use explicit target/unit/state semantics. Any later approximate arithmetic needs a documented formula and tolerance before acceptance, not an undocumented epsilon added after a mismatch.

### M4.5: preserve missing-data and policy states


Implement `visibility-policy` as pure evaluation of supplied PX11 data. Preserve permission_denied source status, not_enabled optional family and unknown identity independently. Retain safe labels such as fixture=safe; remove the synthetic sensitive marker from every exported value/error/log. Verify sanitation by searching serialized output and diagnostics, not by assuming the already safe expected map proves redaction. Apply a fixed documented projection of policy fields and allowlisted safe labels; report which fields were redacted without reproducing their values.

Treat attempted enable_metric/escalate_privilege/restart_workload actions as rejected data decisions. Do not invoke a shell, enable metrics, change flags, restart processes or request real privilege escalation to evaluate these descriptions. Missing required limits/agreements yield readiness blocked/false. This tests a simulated policy evaluator only; real permissions and collector enforcement remain unverified and M5 retains association work.

For PX12 retain both latest gauge snapshots with times and units, but equal values do not prove no intervening updates. Source update/persistence/query-visible timestamps that are absent remain unknown; do not substitute the poll interval, wall clock or capture timestamp. Record the queue assertion as deferred M6; authored admitted/rejected IDs are not actual queue admission outcomes. Do not execute storage or estimate durability from their presence.

Acceptance is explicit unknown/denied/disabled/blocked states and sanitized output without fabricated zeros, activation or hidden deferred assertions.

### M4.6: integrate named scenarios and independent comparison/tracking


Extend `config.rs` with named scenarios `native-parse`, `counter-rate`, `aggregate-evidence`, `count-boundary`, `memory-occupancy`, `visibility-policy` and `freshness-evidence`, preserving `endpoint-roundtrip`. Each requires a compatible manifest-indexed variant; incompatible pairs fail before execution. Do not dispatch on an oracle's expected target/value, silently skip unknown record tags or accept arbitrary external endpoints. The seven bindings are explicitly documented fixture adapters; production routing is not finalized. Existing query/collect/replay commands remain nonzero not_prototyped.

Add `verify-scenario --scenario NAME --variant ID`, which computes actual results through the same runtime API first and then separately loads the independent oracle for comparison. A direct scenario parser error must return nonzero with its stable code and no success object; the verification command can pass a negative test only if the actual error matches the declared oracle rejection. Unknown is a data outcome that can correctly match an unknown oracle; it does not mean the source is healthy. Oracle-only proof checks and deferred assertions must have distinct categories. A comparison failure returns nonzero with a safe structured mismatch; missing expected assertions cannot be called matched.

Add `verify-milestone --milestone 4` to produce a deterministic JSON summary across exactly the selected seven groups/23 variants, retaining deferred assertions and all twelve-group stage accounting. It does not write research/fixture files or grant PX runtime acceptance. `run-scenario` emits actual results with `functional_execution_status=executed`, without oracle values or comparison claims. Verification reports use `functional_check_status=matched|mismatched|expected_rejection|deferred|proof_verified`, source/input/oracle hashes, stage and claim limits. A transport error or unhandled required assertion fails verification; a known declared later-stage assertion is explicitly deferred rather than ignored.

Create `docs/prototype-metric-execution-status.json` as the new functional ledger, authored only during execution. Include schema, synthetic flag, current artifact/input identities, per-group/variant/per-assertion IDs (variant ID plus stable assertion index/target/kind), assigned milestone, actual function execution status, actual/result/evidence hashes, comparison outcome or deferred reason and runtime-budget status. Enumerate all twelve groups and original assertions; groups not exercised here remain not_run and point to their future stage. Retain separate `px_runtime_acceptance_status=blocked_unagreed_budget`, `real_verification=not_run` and readiness false. Original registry/status/fixture ledger and oracle execution fields are historical planning bytes and stay unchanged; the new ledger explicitly supersedes them for current function-execution evidence only.

At repeated runs, compare generated reports first and update a ledger only through deliberate evidence-authoring after successful checks. Protect conflicting existing evidence. No blanket whole-group synthetic_verified state is allowed merely because a subset matched. M4 completion means its assigned functional assertions pass, with proof/deferred/runtime-gate accounting complete; later-stage work remains pending.

### M4.7: test and record actual results


Add `tests/metrics.rs` and isolated unit tests as needed. Test seven native producer variants, histogram `_total` handling, server scope, original labels/sample identities, unknown families, optional absence, malformed TYPE/histogram, duplicate labels/series, nonfinite/oversized text and unsupported grammar. Test positive rate, resets/incarnations, zero/backward duration, mismatched clock/source/labels/units, large-counter precision, raw-TGI unresolved population, explicit overlap versus missing linkage, occupancy valid/missing/inconsistent denominator, aggregate insufficiency and proof/runtime separation. Test policy states, forbidden-action data rejection, sanitation across stdout/stderr/error/mismatch paths and unknown freshness. Test compatible/invalid CLI pairs, expected negative verification, deliberately altered actual result/oracle mismatch, deferred-assertion retention and deterministic reports. Mutate only temporary test trees; never rehash accepted workspace inputs to force an algorithm result.

Preserve the 29 M3 and 70 validation regressions. If an old test assumed counter-rate was not_prototyped, update that assertion to an actual still-unimplemented scenario while adding real counter-rate coverage; record the intentional regression-test revision and retain the predecessor. Tests must not become production source proofs. Capture actual counts; this plan does not invent a future test total.

Run locked/offline check/tests, format and strict Clippy for both affected Rust code and existing validation tooling. Execute actual owned-loopback native scenarios through the approved environment path if required; retain permission failures separately and never skip sockets as success. Record actual commands/results, parser finite grammar, functional comparison/deferred/proof counts and unresolved real/runtime questions in `evidence/prototype/milestone-4.md`, with logs under the adjacent directory. Add new `artifacts.sha256` for current delivered code/lock/docs/ledger/reports, excluding itself/build directories. Verify historical M3 hashes against its predecessor and unchanged M2/original baselines against live inputs.

### M4.8: synchronize acceptance and hand off


Mark M4 complete only after its assigned functional assertions, negative cases, unknowns, redaction, regressions and preservation checks pass. Update all eight tasks and living sections here, master aggregate progress and the shared index. Report actual execution and comparison counts, deferred assertions and blocked runtime acceptance clearly. If a required assertion cannot be implemented, keep its task pending with evidence; do not rename it deferred solely to claim completion.

Suggest M5 simulated metadata enrichment/lifecycle handling next. It will resolve selected publisher/subject/resource relationships with explicit lifetime evidence and test PID reuse, remote subjects and ambiguous ownership without duplicating observations. Do not begin M5 during M4 completion or claim its assertions have already passed because some M4 calculations use supplied identity labels.

## Concrete Steps


Work from `/home/mbellary/work/projects/DeepMon`. These current prerequisite commands exist now:

    git status --short
    rustc --version --verbose
    cargo --version --verbose
    sha256sum --check --quiet evidence/prototype/milestone-3/artifacts.sha256
    sha256sum --check --quiet evidence/prototype/milestone-2/artifacts.sha256
    sha256sum --check --quiet evidence/prototype/milestone-1/baseline-before.sha256
    cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- evidence --root .
    cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- traceability --root .
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- validate-fixtures --root .

Expected current results retain 250/58 IDs, twelve specifications, 12/36/82/2 fixture counts, semantic execution not_run and readiness false. Quiet hashes exit 0 on a full match. Check each exit code. After preserving M3, run its original checksum list from the predecessor root; do not expect intentionally changed live M4 source to match historical source hashes.

After the snapshot exists, use working directory `/home/mbellary/work/projects/DeepMon/evidence/prototype/milestone-4/inputs/milestone-3` and run:

    sha256sum --check --quiet evidence/prototype/milestone-3/artifacts.sha256

Return to the repository root for all remaining commands. This validates historical source bytes under their original relative paths inside the archive.

These commands were specified at plan authoring and are now implemented; each was run twice with byte-identical output:

    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- run-scenario --root . --scenario native-parse --variant PX01-vllm-v1
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- verify-scenario --root . --scenario native-parse --variant PX01-malformed-type-v1
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- run-scenario --root . --scenario counter-rate --variant PX02-success-rate-v1
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- verify-scenario --root . --scenario aggregate-evidence --variant PX03-insufficient-joint-data-v1
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- verify-milestone --root . --milestone 4

The first preserves the declared histogram family/type/sample names; the negative verification reports expected_rejection with TYPE_INVALID from an actual parser failure. The rate command returns known value 5 in requests/second from synthetic records. Aggregate verification matches unknown/insufficient-input, with no hidden request events. The summary lists actual matches, expected rejections, proof-only checks, explicit later-stage deferrals and blocked runtime acceptance. Run each twice into fresh evidence files and compare deterministic JSON; do not include elapsed/port/path noise in the stable results.

After implementation run:

    cargo fmt --manifest-path prototypes/aether-poc/Cargo.toml -- --check
    cargo check --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml
    cargo test --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml
    cargo clippy --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml --all-targets -- -D warnings
    cargo test --locked --offline --manifest-path tools/aether-validation/Cargo.toml
    cargo fmt --manifest-path tools/aether-validation/Cargo.toml -- --check
    cargo clippy --locked --offline --manifest-path tools/aether-validation/Cargo.toml --all-targets -- -D warnings
    git diff --check

Use the approved socket execution path when sandbox binds fail. Report actual new/regression counts and all failures/fixes; no additional dependency download is planned. If an implementation needs a new dependency, record the reason, lock exact versions and follow normal network approval instead of bypassing restrictions.

## Validation and Acceptance


Plan-authoring acceptance requires required PLANS.md sections/format, valid current links, eight pending implementation tasks, consistent master/index status and passing existing fixture/traceability/artifact checks. No new engine test is claimed by writing this plan.

Implementation acceptance is observable native output from real owned-loopback bytes, exact negative parser errors, five source-success requests/second, ten explicitly linked logical requests, two .5 occupancy ratios, preserved single replica count and unknown outputs for all specified insufficient cases. Required policy results remain denied/disabled/blocked with no sensitive marker anywhere in emitted evidence. Unknown families/native series retain source scope and cannot become invented canonical support. Run/verify are separate; actual computation precedes independent expected comparison.

Every existing assertion must be assigned, functionally compared, proof-verified or explicitly deferred to its genuine later stage. Seven groups/23 variants are considered, not indiscriminately passed. All twelve groups remain accounted for. Existing 99 tests plus necessary new tests/static checks must pass, with actual totals recorded. Historical M3 checksums must verify in the predecessor root and accepted M1/M2/source/requirements/fixtures/budgets must remain unchanged. Any necessary source-document correction is recorded separately with IDs/evidence and does not silently rewrite the research baseline.

M4 functional success is not an agreed PX runtime acceptance pass. Required agreements/limits remain unset, readiness false and real source/capability verification not_run. Actual function execution must nevertheless be reported truthfully in the new ledger. No production speed, physical microsecond accuracy, zero overhead, reliable acquisition, durability or universal parser/engine compatibility claim follows.

## Idempotence and Recovery


All scenario/verification commands are read-only toward source fixtures/research and write stable JSON to stdout. Use Rust-owned temporary test directories and fresh explicitly named evidence destinations; refuse conflicting evidence rather than overwriting manually advanced results. Repeated validation must not rerun author-fixtures to reset the accepted ledger. Preserve unrelated work and do not delete broad temporary or repository paths.

Before touching M3 sources, complete and verify the predecessor snapshot. If interrupted, resume only after checking already copied bytes against historical hashes; do not trust a partial archive. Do not edit M3/M2 checksum expectations. Restore a file only from a verified predecessor after assessing user changes. Build/test retries use the existing lockfile. Owned listeners must stop/join on parser/comparison errors as well as success; preserve the M3 cleanup tests.

If input hashes drift, stop the affected acceptance claim and record why. If new parser output mismatches an oracle, diagnose the formula/population/unit first; do not change expected data merely to make the test green. A genuine fixture flaw requires a versioned additive correction with explicit provenance and a plan revision, preserving v1. Unavailable permissions or budget agreements remain explicit blockers for their respective claims, never inferred approvals.

## Artifacts and Notes


Delivered artifacts are the updated prototype source/tests, unchanged dependency lock, `docs/prototype-metric-semantics.md`, `docs/prototype-metric-execution-status.json`, current harness/dependency notes, `evidence/prototype/milestone-4.md` and adjacent predecessor/index/log/hash/report files. Historical reports remain unchanged. The Rust evidence-authoring example indexes all 49 assertions and refuses conflicting ledger output; an identical rerun passes.

The actual `evidence/prototype/milestone-4/rate-1.json` report contains these fields (the target/value/unit/state are within its results array):

    scenario: counter-rate
    acquisition_kind: synthetic_record_input
    functional_execution_status: executed
    target: source_success_rate
    value: 5
    unit: requests/second
    state: known
    px_runtime_acceptance_status: blocked_unagreed_budget
    runtime_acceptance_readiness: false
    production_support_claimed: false

## Interfaces and Dependencies


Keep M3 `load_bundle`, `select_endpoint`, owned `spawn_fixture_server`/`fetch_fixture` and byte-roundtrip behavior. Extend `fixture.rs` with `load_scenario_input(bundle: &ValidatedBundle, variant_id: &str) -> Result<ScenarioInput>` and separate `load_expected(bundle: &ValidatedBundle, variant_id: &str) -> Result<ExpectedAssertions>`. Runtime loaders expose no oracle proof/assertions. Typed `ScenarioInput` carries validated envelope, clocks, relevant record variants and stable artifact identities. `ExpectedAssertions` belongs only to verification code.

In `observation.rs`, define serde-serializable `NativeObservation`, `NativeSample`, `MetricType`, `SourceContext`, `QualityState` and `DerivedResult` with the fields/limits above. Preserve exact raw/native identity separately from safe report projection. Source context comes from verified input metadata and explicit scenario binding, never inferred from an expected numeric answer.

In `endpoint.rs`, expose `parse_metric_fixture(text: &str, source: &SourceContext) -> Result<Vec<NativeObservation>>`; use stable structured `PrototypeError` codes, including TYPE_INVALID and HISTOGRAM_INCONSISTENT. A parser implementation in `metric_text.rs` is permitted but must be called by this API. HTTP acquisition and parsing remain separate operations with independent failure reports.

In `derive.rs`, expose `counter_rate(previous: &CounterSnapshot, current: &CounterSnapshot) -> DerivedResult`, `memory_occupancy(record: &MemoryRecord) -> DerivedResult` and `logical_count(front: &LogicalCount, back: &LogicalCount, boundary: &CountingBoundary) -> DerivedResult`. Checked typed inputs retain population/units/clocks/identity and can return unknown with a precise reason. Aggregate/freshness inference functions must return unknown when necessary evidence is missing, not receive oracle worlds as parameters.

In `scenario.rs`, expose `run_metric_scenario(bundle: &ValidatedBundle, scenario: MetricScenario, variant_id: &str) -> Result<MetricScenarioReport>` and `verify_metric_scenario(bundle: &ValidatedBundle, scenario: MetricScenario, variant_id: &str) -> Result<ScenarioVerification>`. `MetricScenario` names the seven explicit adapters; reports retain source/input identities and actual results. Verification obtains actual output/error first, then loads expectations, with proof/deferred accounting distinct from result comparisons. Extend `report.rs` and CLI actions with verification summary types without weakening old byte-roundtrip guarantees.

Use the existing exact Rust dependencies and standard library first. No shared validation-parser API change, network/framework/runtime addition, Arrow/Parquet/DataFusion, real device/host SDK, probe or cloud service is required. This is an experimental API and finite parser, not a production package or compatibility decision.

## Revision Notes


2026-11-01 23:04 IST: created the detailed M4 plan following the user's request. Defined eight pending tasks, exact seven-group/23-variant assertion accounting, native parser and calculation behavior, missing/policy states, independent oracle verification, truthful functional execution versus blocked runtime acceptance and M3 predecessor preservation. Verified current fixture/traceability and M3 checksum prerequisites. No M4 code or execution is delivered by authoring; master/index tracking updated.

2026-11-01 23:10 IST: verified plan structure/links/pending tasks and manifest-backed variant coverage. Current Rust prerequisite checks, accepted M3/M2/original hashes and whitespace checks pass. Mirrored verification in the master; all M4 implementation remains pending.

2026-11-01 23:34 IST: executed and accepted M4.1–M4.8 for synthetic functional correctness. Delivered native parsing/calculations/policy/unknown handling, independent verification and a complete 49-assertion ledger; passed 129 tests, strict Clippy/format/checks, five deterministic CLI pairs and preserved-input checks. M3 predecessor bytes and fixture validation verify; original requirements/sources/fixtures/budgets remain unchanged. Current functional execution is recorded truthfully, runtime acceptance remains blocked and M5 is next. Updated master/index accordingly.

2026-11-01 23:36 IST: verified final plan/index links and task state, all 187 predecessor input hashes and absence of the synthetic sensitive marker in emitted runtime JSON/logs/ledger. Current M4 artifact hashes are separate from unchanged historical acceptance lists. No additional runtime/real compatibility claim is introduced.
