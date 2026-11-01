# Milestone 1: define prototype experiments and track every requirement


This ExecPlan is a living document maintained under repository-root `PLANS.md`. Update `Progress`, `Surprises & Discoveries`, `Decision Log`, `Outcomes & Retrospective` and the bottom revision note at every stopping point. The repository is `/home/mbellary/work/projects/DeepMon`. This is `milestone-execution-plans/milestone-01-execplan.md`; `EXECPLAN.md` remains the master plan. This plan contains the context and instructions needed to execute Milestone 1 without chat history.

Current task boundary: the user authorized execution of Milestone 1 after reviewing the Rust-only revision. M1.1–M1.7 are now complete for specification and traceability acceptance; all outputs, Rust checks and evidence exist. The twelve telemetry experiments remain unexecuted, and Milestones 2–8 remain pending. Next is Milestone 2 fixture authoring, not production deployment.

## Purpose / Big Picture


Give a reviewer one clear answer to what the proof of concept will test, why each experiment matters and what remains unproven. Keep all 250 metric requirements and all 58 metadata associations visible, even when only a representative subset is selected for experiments. A reviewer will be able to run a local checker, see complete requirement coverage in the tracking records, and trace every selected experiment to its source evidence and expected answer.

Milestone 1 produces the experiment scope, structured tracking records, experiment and budget specifications, a traceability checker and an evidence report. Traceability means a recorded link between a requirement, an experiment, its inputs and its expected result. The checker validates those links; it is not an inference collector. Later milestones create fixture files, build the Rust harness and execute the experiments. A fixture is invented input with a known expected answer.

This is a prototype preparation milestone. Production architecture, real collectors, hardware certification, source activation, deployment and production performance promises remain outside it. There is no existing cluster. No customer serving code, image, launch setting, pipeline or workload is changed. DeepInfer remains deferred. Rust is the implementation language for the engine, milestone validators, record-generation helpers and tests. No Python interpreter or Python subprocess is required by the revised execution path. Existing historical Python tooling and recorded environment facts are retained as provenance only; the first execution task replaces the evidence checker in Rust.

## Progress


Times use the client timezone Asia/Calcutta, UTC+05:30. Checkmarks mean the stated work actually exists or was checked; pending items are not historical accomplishments.

- [x] (2026-11-01 21:08 IST, preparation) Read `PLANS.md`, the master Milestone 1 description, the collection contract, status/budget input structures and pinned evidence/environment records.
- [x] (2026-11-01 21:08 IST, prerequisite verification) Ran the existing evidence checker: ten pinned upstream inputs, 24 artifact checksums and all 26 historical U rows passed integrity checks. This is documentation integrity, not engine or capability verification.
- [x] (2026-11-01, planning) Created the shared `milestone-execution-plans/` folder, its index and this detailed milestone plan; synchronized the master plan. Acceptance of the written plans is recorded separately from execution below.
- [x] (2026-11-01 21:15 IST, plan validation) Checked required sections, heading spacing, checklist placement, existing links, all referenced metric IDs and synchronized pending task states. Confirmed baseline/frozen input hashes unchanged, prerequisite integrity still passing and no future execution outputs created.
- [x] (2026-11-01 21:24 IST, language revision) Revised the milestone and master to require Rust for evidence checks, record generation, traceability and tests; validation-tool bootstrap is the next pending execution task. Earlier Python results remain historical only.
- [x] (2026-11-01 21:39 IST, verified) M1.1: create the isolated Rust validation-tool crate and dependency lock, implement the Rust evidence verifier, then verify baseline integrity and record drift without changing frozen evidence.
- [x] (2026-11-01 21:39 IST, verified) M1.2: write `docs/prototype-scope.md` and `docs/prototype-experiments.json` with the twelve representative experiment specifications, source limits, expected answers and later execution milestone.
- [x] (2026-11-01 21:39 IST, verified) M1.3: write `docs/prototype-status.json` with exactly 250 metric records and 58 metadata records; preserve original definitions, current routes, dependencies and unverified states.
- [x] (2026-11-01 21:39 IST, verified) M1.4: create one synthetic budget planning record per experiment under `docs/prototype-budgets/`; distinguish proposed limits, agreement status and actual results.
- [x] (2026-11-01 21:39 IST, verified) M1.5: implement the `aether-validate traceability` Rust command and meaningful rejection tests for omissions, false verification and broken evidence links.
- [x] (2026-11-01 21:39 IST, verified) M1.6: run positive and negative traceability checks, record actual outputs in `evidence/prototype/milestone-1.md`, and verify that baseline files remain unchanged.
- [x] (2026-11-01 21:39 IST, verified) M1.7: update this plan, the master and folder index with the acceptance outcome, outstanding decisions and handoff to Milestone 2. Mark Milestone 1 complete only when all required outputs and checks pass.

## Surprises & Discoveries


Observation: the first Rust-only implementation can be built from the local dependency cache without changing the pinned toolchain or downloading dependencies.

Evidence: Cargo locked 33 packages with exact direct versions; offline evidence/traceability commands and 30 integration tests passed. Generator output retains 23 selected/227 unselected metrics and 42 selected/16 unselected associations; all 250/58 identities remain present.

Observation: planning acceptance and runtime readiness need different verdicts.

Evidence: the traceability verdict passes while every proposed budget has unset agreement/actual-limit fields, readiness false and experiment execution not_run. The generator also preserves manual edits by refusing differing existing files, verified in temporary-tree tests.

Observation: using Rust for all validation is practical, but moves a small tool build and dependency lock into Milestone 1.

Evidence: the old prerequisite checker is a Python script, while the requested execution path now requires Rust only. The new package under `tools/aether-validation/` is validation tooling, separate from the engine prototype first created in Milestone 3. Existing evidence hashes and recorded historical results do not change when their verifier is ported.

Observation: the number 26 has two different meanings in earlier research.

Evidence: the historical unresolved set had 26 U rows. Its documentation review yielded 25 endpoint routes and one remaining U row, L1-001. The current whole-catalog totals are E:88, R:35, H:100, I:26, U:1. The 26 I integration rows are a separate set. Tracking must preserve these distinctions.

Observation: a documented source is different from a deployed source or tested capability.

Evidence: `evidence/reconciliation-v1.json` keeps deployed-schema and capability checks at `not_run`. L1-001's raw TGI counter exists, but its accepted-request population is unresolved; L1-005 inherits that gap. L2-071 also retains a source-meaning caveat. A fixture cannot resolve those real-source questions by inventing convenient values.

Observation: Milestone 1 can demonstrate a useful behavior without running an engine.

Evidence: a checker can establish that all original IDs are retained, selected experiments have complete references and expected answers, and malformed or falsely verified planning records are rejected. That is the milestone's working result, rather than a collector or fixture-server demonstration.

## Decision Log


Decision: use one shared `milestone-execution-plans/` folder with numbered files; keep this file as the detailed Milestone 1 progress record and the root plan as the program summary.
Rationale: the user explicitly requested a reusable location and synchronized per-milestone/master tracking.
Date/Author: 2026-11-01 / Codex, applying user direction.

Decision: this turn creates plans only. Milestone execution outputs remain pending in both plans.
Rationale: the user's concrete requested actions are to create the detailed plan and update both plans. A plan must not record its future deliverables as completed work.
Date/Author: 2026-11-01 / Codex.

Decision: use the frozen v1 evidence bundle as the input baseline and retain every requirement and association. Select representative experiments across all four layers and include original TGI/Triton/TensorRT-LLM routes alongside the first three reviewed engines.
Rationale: avoid repeating the prior exclusive-engine scope mistake. Selection controls experiments, not product eligibility or requirement removal.
Date/Author: 2026-11-01 / Codex.

Decision: define twelve experiment specifications here, with execution assigned to later milestones; keep generic pipeline/control experiments separate from metric counts.
Rationale: parsing, attribution, calculations, recovery and explicit uncertainty need representative cases, while storage/policy checks must not create new original metric IDs.
Date/Author: 2026-11-01 / Codex.

Decision: record concrete proposed synthetic safety limits separately from agreement and run status. Do not infer that the user agreed numeric limits or that a planning checker accepted a runtime budget.
Rationale: the collection contract requires explicit acceptance criteria before evaluated runs. Milestone 1 can complete a planning record while acceptance readiness remains false and production budgets remain unset.
Date/Author: 2026-11-01 / Codex.

Decision: implement evidence verification, record generation, traceability validation and tests in Rust, without invoking Python.
Rationale: explicit user language preference. A small separate validation crate avoids an additional runtime language while preserving the same integrity and traceability checks. Its Cargo dependencies and lockfile are Milestone 1 tooling artifacts, not production engine architecture.
Date/Author: 2026-11-01 21:24 IST / Codex, applying user direction.

Decision: execute only the authorized Milestone 1 work and record successful Rust tool/specification acceptance separately from future experiments.
Rationale: user said to proceed and suggest the next milestone. All seven tasks passed; 30 integration tests cover valid and corrupted records, and no inference/hardware run is needed for this milestone's defined behavior. Do not start Milestone 2 automatically.
Date/Author: 2026-11-01 21:39 IST / Codex, applying user direction.

Decision: propose a 100-ms poll interval, two retries and one-second capped backoff for small future synthetic cases, alongside the plan's proposed resource bounds; keep agreement and readiness false.
Rationale: the template's finite planning fields need explicit bounded proposals. These values are not production promises, user-agreed thresholds or evidence of feasible overhead.
Date/Author: 2026-11-01 21:39 IST / Codex.

## Outcomes & Retrospective


Milestone 1 is complete. `tools/aether-validation/` contains the locked Rust evidence verifier, generator and traceability validator. All fifteen planning outputs exist: scope, registry, complete status and twelve budget records. All 250 metrics and 58 associations retain their baseline facts; 23 metrics and 42 associations are selected for twelve planned cases, while 227 and 16 remain not_prototyped. Original routes and frozen inputs are unchanged.

The Rust evidence check verifies ten inputs and 24 artifacts; traceability passes with zero missing/duplicate IDs, no executed experiments, no deployed verification and runtime readiness false. All 30 integration tests, strict Clippy and formatting checks pass. Generator reruns are byte-stable and manual edits are protected. Actual logs, toolchain identities, input-preservation checks and delivered-file hashes are in `evidence/prototype/milestone-1.md` and its adjacent log directory.

These results demonstrate local integrity/specification tooling only. Budget proposals are not agreed, real schema/capability checks are unperformed and no fixture payload, engine harness or collector was created. Next is Milestone 2: author the synthetic fixtures and expected outputs from this registry. All later milestones remain pending.

## Context and Orientation


The master milestone defines two principal outputs, `docs/prototype-scope.md` and `docs/prototype-status.json`, plus a requirement completeness checker. This plan makes their interfaces and supporting experiment/budget records explicit. Create missing output directories only as needed; inspect existing files before editing because new user work may appear after this plan was written.

The original requirements are in `aether-metrics-catalog.json`, with 250 objects under `metrics`. Each object has `id`, `layer`, `category`, `metric`, `definition`, `unit_type`, `scope`, `source_identifiers_or_inputs`, `sources`, `collection_mode`, `aggregation` and `diagnostic_and_limits`. Preserve that complete object as baseline data, rather than shortening or redefining it. The original collection mode is separate from the later source-route classification.

`metric-to-source-coverage-matrix.json` has the same 250 IDs under `metrics`; its `route`, `source`, `dependencies`, `metadata_dependencies`, `gap` and `source_review` describe proposed acquisition and remaining evidence. E means documented endpoint, R calculation, H host/device/fabric collector, I deeper integration and U unresolved mapping. These are research classifications, not implementation states. The four layers cover request/generation, serving/cache, distributed execution/fabric and host/GPU infrastructure.

`metadata-enrichment-matrix.md` defines M01 through M58 in table rows. Capture each row's dimension, route/source, join key/cardinality, required evidence and unresolved behavior. A join is a relationship between identities; cardinality describes whether it links one or several entities. The emitting process, measured subject, allocated resources and observed active resources may differ. A replica-wide count must not be copied onto each GPU and then added. Missing ownership remains unknown, rather than a guessed host or GPU.

`evidence/manifest-v1.json` identifies ten pinned upstream inputs E01–E10 and frozen local copies B01–B11. Read frozen requirements from B01, metadata from B02 and routes from B03, resolving their snapshot paths through the manifest. Do not hard-code a new copy of the 250 IDs. E01/E02 cover vLLM, E03/E04 SGLang, E05 Dynamo, E06/E07 TGI, E08 Triton, E09 native TensorRT-LLM and E10 Triton's TensorRT-LLM backend. They are exact source/documentation snapshots, not running releases. `evidence/reconciliation-v1.json` records the historical U set and remaining limits. The Rust evidence command must verify immutable content; never regenerate v1 checksums merely to make it pass. Earlier baseline documents may show Python commands; those are superseded by this plan's Rust commands and must not be invoked as prerequisites.

`zero-application-change-collection-contract.md` defines C01–C10: existing engine endpoints, selected host reads, NVML queries, NIC/RDMA reads and configured metadata. NVML is NVIDIA's device-query interface; RDMA is a device-assisted communication interface. In the prototype these are invented records, not real reads. No feature activation, privilege escalation, workload modification, container control socket, device query, kernel attachment, process injection or real engine connection is permitted by this milestone.

`validation/first-environment-v1.json` records local Ubuntu 26.04.1, kernel 7.0.0-38-generic, Rust/Cargo 1.99.0 and Python 3.14.4 as the reference environment. This is an observed fingerprint, not an installed image or package lock. Python's recorded presence is historical inventory, not a prerequisite. Verify the installed Rust compiler, target and Cargo identities against the manifest before building the validation tool. A different current host must be reported as drift; do not silently install or switch toolchains. Create a separate Cargo.lock for the validation tool in M1.1; the engine harness still receives its own lockfile in Milestone 3.

A counter accumulates events and may reset; a gauge is a current observation; a histogram stores value-range counts and totals. Histogram snapshots do not contain original request events. HELP/TYPE are declarations accompanying a metric family. SQL is the query language used later by DataFusion; Parquet is the planned file format. Storage commit means a file is included in the declared readable snapshot. Admitting an observation into memory is different from acknowledging durable persistence.

## Plan of Work


### M1.1: inspect and verify the inputs


First create the Rust-only validation package at `tools/aether-validation/Cargo.toml` with `src/lib.rs`, `src/main.rs` and integration tests. Name its binary `aether-validate`; do not create the engine package here. Inspect installed compiler identities with `rustc -Vv` and `cargo -V`. Select compatible exact dependency versions, record the Rust 1.99.0 reference requirement, generate the tool's Cargo.lock and record the choice in the evidence report. Use serde/serde_json for JSON, sha2 for SHA-256, sha1 solely for Git blob identity checks and tempfile for isolated tests; avoid a CLI framework by parsing the small argument set with std. SHA-1 here matches upstream Git identifiers; it does not replace SHA-256 integrity checks. Inspect available dependency caches and use ordinary dependency-access permissions if downloads are needed. Lock dependency versions before evaluated checks and use `--locked` thereafter.

Implement `aether-validate evidence --root PATH` directly in Rust. Verify the manifest companion SHA-256, every artifact's length/hash, pinned commit URLs and recomputed Git blob identities, source-index agreement, original ID equality/route totals, all 26 historical U outcomes and native-name presence, and the synthetic/unverified environment states. Reject unsafe/duplicate paths, missing files, mismatches and false verification claims with useful errors. Preserve the historical checker's behavior without importing or invoking it; do not skip checks to avoid the port. Test a valid bundle and independently corrupted temporary copies. Confirm the unmodified input hashes remain intact.

Once that command exists, run `cargo run --locked --manifest-path tools/aether-validation/Cargo.toml -- evidence --root .` from the workspace. Expect ten pinned upstream inputs, 24 checksummed artifacts, all 26 historical U IDs and E:88/R:35/H:100/I:26/U:1. Save the actual output in the eventual milestone evidence report. List any pre-existing output files and preserve unrelated edits. Record SHA-256 values for the working original catalog Markdown/JSON, metadata matrix, route matrices, collection contract, budget template, source baseline, uploaded `PLANS.md` and all v1 artifacts before executing milestone edits.

Use the manifest's frozen local inputs for record generation. Compare their requirement IDs and unchanged baseline fields with the live research files. If differences exist, report them and use the declared v1 snapshot consistently; do not blend versions, silently update evidence, drop IDs or call drift unsupported. A decision to adopt changed inputs requires a new evidence revision with its own rationale. No network fetch or real-environment evidence is needed to complete this step.

Acceptance of M1.1 is a known intact input set and a recorded inspection result. It does not make the environment ready for engine or hardware testing.

### M1.2: specify the scope and representative experiments


Write `docs/prototype-scope.md` in plain language: purpose, synthetic-only boundary, all retained requirements, selected experiment IDs, expected benefit, explicitly deferred work, source/version limitations, budget state and handoff. It must say that an unselected requirement remains part of the product. Keep the 18 proposed collection-health companion families outside the 250 original metric IDs.

Write `docs/prototype-experiments.json` as the authoritative experiment registry, using PX01–PX12 below. Every entry has a title, purpose, `synthetic=true`, execution milestones, profile IDs, requirement IDs, association IDs, pinned source IDs or local baseline references, input specification, positive expected result, negative/missing-evidence result, limits of the claim, budget-record path and `execution_status=not_run`. No expected answer may be used as actual result evidence. “Not run” is an execution state; it differs from a specification being complete.

PX01 specifies native-family parsing across C01–C06, with E01–E10 as appropriate. Link L1-009, L2-011, L2-012, L2-022, L2-047 and L2-059. Include a vLLM histogram whose name ends in `_total`, TGI batch observations, direct TensorRT block gauges, Triton shared-pool scope and distinct Dynamo producers. Expect preservation of name, type, unit, labels and source boundary; malformed types or absent optional statistics yield explicit failures/unknown states, not invented zeros or events. Each producer variant uses its own pinned names. Later execution: Milestones 2–4.

PX02 specifies counter calculation, reset handling and the unresolved TGI arrival population. Link L1-001, L1-002, L1-005 and L1-006. A synthetic Triton success counter rising from 100 to 150 in ten virtual seconds yields five source-scoped successes/second. A reset must not emit a negative rate. A raw TGI counter is preserved but cannot be normalized into accepted requests or a canonical L1-005 arrival rate. E06–E08 provide source references; the counting gap is retained, not solved by the fixture. Later execution: Milestones 2–4.

PX03 specifies the limits of aggregate data. Link L1-025 and L1-027 with M16. Histograms with the same bucket totals but different possible event distributions cannot supply exact joint request goodput or exact variance. Expect an explicit unavailable/insufficient-input result; if a separate synthetic per-request record set is specified as a positive alternative, identify it as invented integration input, not endpoint data or an approved collector. Reference the frozen requirement/route records; new external sources are unnecessary. Later execution: Milestones 2, 4 and 7.

PX04 specifies framework/backend duplication and remote-worker attribution. Link L1-002 and L1-009, and associations M02, M03, M09, M11, M12 and M31. Supply distinct frontend/backend observations about the same ten synthetic logical requests. Expect ten logical requests within an explicitly selected counting boundary, not twenty. A worker reported by a frontend must not inherit the frontend's host; without worker-location evidence its host stays unknown. Use E02–E05 as applicable. Later execution: Milestones 2, 4 and 5.

PX05 specifies heterogeneous replica membership and device-memory calculation. Link L1-002 and L4-007–L4-010, with M09, M32, M33, M37, M38 and M40. Invent one replica with two symbolic GPU models and a replica-wide count of 100; expect 100, not 200. Memory observations of used/total 10/20 and 20/40 on the two devices each yield 0.5 occupancy, with units explicitly consistent. An incomplete member set must stay partial; a missing total prevents that device's occupancy result. These are invented device records based on the frozen requirements and metadata contract, not hardware schemas certified by E01–E10. Later execution: Milestones 2, 4 and 5.

PX06 specifies process/cgroup identity changes. Link L4-048 and L4-054 with M21–M26, M30 and M58. Reuse a synthetic PID with a new start identity and simulate changing cgroup membership. Expect historical observations to retain the old process/container association, and later observations to use the new incarnation only when evidence exists. Denied runtime metadata must not create a guessed container. Inputs are invented lifecycle records. Later execution: Milestones 2 and 5.

PX07 specifies NIC/fabric distinctions. Link L4-071 with M43–M53, M55 and M56. Invent an adapter supporting more than one transport, interface byte counters and a configured preference, while omitting operation-path evidence. Preserve inventory and source-scoped counters, but keep active transport and operation-to-NIC attribution unknown. Do not infer RoCE, InfiniBand or TCP use from capability or one control socket. These are symbolic metadata/counter records; exact physical hardware remains unselected. Later execution: Milestones 2 and 5.

PX08 specifies distributed timing with complete and incomplete evidence. Link L3-001, L3-007 and L3-008 with M13–M16. A complete two-rank group with fixture arrivals at 100 and 130 microseconds in one declared clock domain has arrival skew of 30 microseconds. A missing member, mismatched group incarnation or unknown inter-clock offset prevents an exact whole-group skew claim. Record supplied clock uncertainty and any bounded result separately. This tests arithmetic and identity logic against invented events, not real timing accuracy. Later execution: Milestones 2 and 7.

PX09 specifies stream overlap. Link L4-040 with its full baseline metadata dependency set. Stream A has intervals [0,10] and [20,30], and stream B has [10,20], using virtual microseconds and a stated [0,30] window. Expect a ten-microsecond gap on A and zero whole-device idle time over that covered window. Missing stream coverage prevents a whole-device conclusion. Invented device events do not qualify a real profiling path. Later execution: Milestones 2 and 7.

PX10 specifies local storage/replay behavior as a `pipeline_control` experiment with no invented original metric ID. Admit two explicitly identified records, inject interruption at a declared simulated write/commit boundary, and expect the stated final committed set of two unique records only where the declared recovery boundary protects them. Queries exclude uncommitted files; a pre-persistence loss case must report its actual expected loss rather than zero. Distinguish admitted, durable and query-visible stages. Storage mechanics remain a Milestone 6 experiment, not a production design. Later execution: Milestones 2 and 6.

PX11 specifies collection policy and visibility as a `collection_control` experiment, with M58 and C01–C10. A synthetic denied source stays denied, a disabled metric family stays disabled and a missing identity remains unknown. Forbidden activation, escalation or workload-mutation actions must be rejected by the later harness policy. A budget with required null limits cannot pass runtime acceptance. Keep private payloads and credential values out of invented retained records. This is a planned policy test, not authorization for any real access. Later execution: Milestones 2–5.

PX12 specifies freshness and bounded observation handling. Link L2-059 and M58, plus a separate `pipeline_control` assertion. Two equal latest-gauge snapshots do not prove there were no intervening updates. A synthetic bounded queue with capacity two and three offered records may admit two and explicitly reject one under its declared reject-new policy; it must not silently claim lossless delivery of all three. Keep source-update uncertainty, collection delay and later query visibility separate. Queue size and timings here are fixture values only. Later execution: Milestones 2, 4 and 6.

For every metric selected by a case, include all its baseline M-ID dependencies in the registry and track them as planned links, even when a case deliberately omits the evidence to test an unknown result. Supplemental association IDs above identify focus areas, not permission to discard other dependencies. For host/device/fabric cases use a `baseline_only_synthetic` evidence kind unless a new versioned upstream input is separately captured; never label arbitrary new records as matches to pinned live hardware schemas.

Acceptance of M1.2 is a complete registry of twelve reviewable specifications with consistent cross-references. The specification is complete when expected answers and limitations are explicit; fixture files and passing runtime outputs are not required or claimed.

### M1.3: create complete requirement and association tracking


Write `docs/prototype-status.json` with `schema_version=1`, the v1 evidence bundle and manifest checksum, environment ID, experiment-registry path, a `metrics` array of 250 records and an `associations` array of 58 records. Generate metrics from the frozen B01/B03 snapshots and associations from the 58 definition rows in B02. Check each table row begins with `| Mdd |`; do not count prose mentions as additional associations, and handle escaped pipes if present. Do not assume numeric continuity alone proves there are no duplicates or omissions.

For each metric, copy the complete B01 object into `baseline_requirement`; retain its stable `id` at record level. Copy route evidence separately as `research_mapping`, including route, source, dependencies, metadata dependencies, original source/mode, gaps and any source-review/pinned-evidence details. Add `selected_for_fixture`, `experiment_ids`, `prototype_state`, `evidence_references`, `remaining_unproven`, `synthetic_execution_status=not_run`, `deployed_schema_verification=not_run` and `capability_verification=not_run`.

For each association, retain `id` and the six B02 table cells in `baseline_association`; add the same selection, experiment and unproven/status fields, with `association_verification=not_run`. This is a tracking state, not an observation-level outcome. Later observed relations can be resolved, reported-only, ambiguous, stale or unknown as the enrichment contract requires; no global M-ID is verified because an invented example works.

At milestone completion, selected records use `prototype_state=fixture_planned`; unselected records use `not_prototyped`. Retain existing unresolved-source notes in `remaining_unproven` without replacing a selection state by a source route. Do not set `synthetic_verified`, `prototype_in_progress` or any real-verification success merely because the tracking generator or checker runs. Preserve L1-001 as U and L1-005's dependency; source interpretation caveats also remain visible on E rows such as L2-071.

The selected/unselected metric counts must sum to 250, and the corresponding association counts to 58. Compute and report those values from the actual chosen registry; do not invent them in advance. Every selected record must be linked to at least one case, and every linked record must be selected. Shared requirements can belong to several experiments without becoming duplicate requirement records. Keep generic control experiments outside the original metric arrays.

Acceptance of M1.3 is complete identity preservation and consistent bidirectional links, with all future execution and real validation still unperformed.

### M1.4: prepare experiment-specific budget records


Create `docs/prototype-budgets/PX01.json` through `PX12.json` by copying the root budget template into new records. Keep its null meaning and distinction between synthetic and real evidence. Set the experiment ID, `evidence_scope=synthetic_fixture`, selected profiles, requirement and association IDs, input version references, declared positive/negative cases, expected outcomes and measured quantity definitions. Point to the local environment record. The files are planning records, not executable engine configuration.

Use these explicitly proposed synthetic safety values unless a case needs a documented revision: at most 32 invented entities, 64 series per producer variant, one MiB of input text per variant, 256 bytes per label value, one in-flight read, a one-second local request timeout, at most ten requests/second, at most one MiB/second of planned transfer, 64 MiB of queued data, a proposed one-core CPU envelope, 256 MiB of process memory, 128 MiB of disposable disk, ten minutes of retention and a 60-second wall-clock scenario cap. Stop the case and report the exceeded limit; do not relax it automatically. PX12's deliberate queue test uses two records instead of the general queue allowance and reports one intentional rejection. Set explicit target paths and limits when future fixtures exist; until then mark them pending, never scan for targets.

These are proposed finite limits for small invented cases, not measured feasibility or an agreed production envelope. Enter proposed values in the safety planning fields, label their proposal status explicitly and leave all readiness flags false until the required agreements and later executable prerequisites exist. Do not invent owner names, agreement timestamps, baseline measurements or completed results. The implementing author may document a fixture-only choice in the decision log as such; this is not proof of user agreement. If a current acceptance field is required and unset, the record must continue to prevent an acceptance-run pass. Production overhead/freshness/timing/loss thresholds remain null.

For deterministic correctness, the proposed criteria require exact expected IDs/counts/decisions for each case, subject to explicitly stated floating-point tolerances only where the future representation needs them. Define denominators and observation windows. PX02's ten virtual seconds are data, not a ten-second blocking wait. PX08/PX09 virtual timestamps test computation, not physical resolution. PX10 recovery expectations apply only to the named simulated durability boundary. Mark real-inference impact as not applicable to a synthetic case only with a recorded reason; do not claim a real overhead budget passed.

Acceptance of M1.4 is twelve internally consistent planning records with explicit proposed safety values, known pending agreements and no false readiness. Agreement and actual run acceptance are separate from completing this specification milestone.

### M1.5: implement the planning-record checker


Extend the existing Rust validation package with the `aether-validate traceability` command. It must accept `--root PATH`, defaulting to the repository root derived from the tool's build-time manifest directory; the documented commands pass the root explicitly. Read inputs without modifying them. Return zero for valid planning records, nonzero for violations, and print concise errors identifying the file, experiment or L/M ID. Missing output files are failures, not instructions to create default-success data.

Validate the existing evidence bundle first. Then validate exact baseline metric ID equality, exact preservation of `baseline_requirement`, current route/dependency equality, all six association cells, no duplicates, all twelve unique experiment IDs, legal references, profile/source IDs, budget paths, synthetic-only claims, selected-state consistency, and each selected metric's complete metadata dependency set. Local references must exist inside the supplied root; a planned future fixture path is explicitly a non-existing specification field, not evidence that a fixture exists. Never fetch a remote URL to make a local reference pass.

Enforce that no record has real/deployed/capability success or synthetic execution success in this milestone. Validate that required-null acceptance budgets do not mark readiness true, that agreement fields have not been fabricated by a generator, and that every selected case has a purpose, input specification, expected positive/negative result, later milestone and limitation. Unknown source meanings and the absence of a real cluster must remain visible. A checker cannot determine truth of a human signature; it can reject missing or inconsistent agreement provenance and must not populate it.

Expose `pub fn check_evidence(root: &std::path::Path) -> Result<EvidenceSummary, Vec<Violation>>` and `pub fn check_traceability(root: &std::path::Path) -> Result<TraceabilitySummary, Vec<Violation>>`. `Violation` identifies the file, optional experiment/requirement ID and reason; summaries report counts without modifying files. `main` dispatches the two subcommands and exits nonzero on violations. Suggested helpers are `load_baselines`, `load_experiments`, `parse_association_rows` and `validate_selection_links`. Implement any deterministic record-generation helper in this crate too; it may write only the declared new planning outputs, must refuse destructive overwrites and must preserve manual notes on rerun. These are validation-tool interfaces, not production engine modules.

Use Rust unit/integration tests run by `cargo test --locked --manifest-path tools/aether-validation/Cargo.toml`. Tests operate on temporary copies of the needed valid files and never edit workspace records. Demonstrate rejection of a missing metric, duplicate M-ID, changed definition, bogus experiment/source ID, dropped metadata dependency, broken budget link, a synthetic success claim without runtime evidence and readiness true with a required-null limit. Include one valid unselected requirement retaining `not_prototyped` so the checker does not demand all 250 be exercised. Restore no expected hashes automatically. Each rejection must fail for its intended reason; file absence alone is not a substitute for exercising the validator.

Acceptance of M1.5 is a checker that distinguishes complete planning records from omissions and misleading claims. Its tests validate the traceability policy, not a mirror of the implementation's formatting choices.

### M1.6: run acceptance and capture evidence


Run both Rust validation commands and the Rust unit/integration tests. Record actual commands, exit codes, outputs and test count rather than inventing results. Verify baseline files have unchanged hashes and no source routes were modified. Ensure generic pipeline/control cases did not add original metric IDs and E/H/I/R/U were not mistaken for execution states.

Write `evidence/prototype/milestone-1.md` with input manifest hash, current planning-record hashes, rustc/Cargo identities, validation-tool Cargo.lock hash, observed environment differences, exact registry/selected counts, positive and negative checker results, scope limitations, pending agreements and the unexecuted real-source questions. Do not rewrite the frozen `evidence/inputs/v1/` material or add milestone results inside that old manifest. This new report is evidence of specification/traceability validation only.

Acceptance is a repeatable reader-facing report: all 250 metrics and 58 associations accounted for, all twelve experiment specifications linked, all selected cases with expected answers and budget planning records, all integrity/traceability checks passed, and no experiment or deployed capability falsely marked verified. A missing real cluster is a stated limit, not a Milestone 1 failure.

### M1.7: synchronize tracking and hand off


After acceptance, complete only the tasks actually finished in this plan's Progress and update its discoveries, decisions, outcomes and revision note. In root `EXECPLAN.md`, mark Milestone 1 complete only with the evidence path and concise result; leave Milestones 2–8 pending. Update the folder index's status to match. If acceptance fails or work stops early, retain unchecked items and describe the next concrete action in both plans.

The handoff to Milestone 2 is the experiment registry, selected IDs, proposed budgets, pinned input references, unresolved agreements and expected answers. Milestone 2 creates the actual invented files. Do not automatically start it or mark its fixture work done as part of this milestone's bookkeeping.

## Concrete Steps


All commands below run from `/home/mbellary/work/projects/DeepMon`. First inspect current state and the installed Rust toolchain:

    git status --short
    rustc -Vv
    cargo -V

The observed prerequisite result at the original plan creation was produced by the historical verifier, not by the pending Rust port:

    PASS: 10 pinned upstream inputs; 24 artifact checksums; 26/26 historical U rows reconciled.
    Documentation routes: E=88 R=35 H=100 I=26 U=1. Remaining U: L1-001; L1-005 inherits its semantic gap.
    Reference environment: aether-validation-synthetic-local-v1. Deployed schemas/capabilities: NOT VERIFIED.

Create the isolated validation package and evidence command in M1.1. Bootstrap its dependency lock with `cargo generate-lockfile --manifest-path tools/aether-validation/Cargo.toml` after choosing dependency versions. The first run may need dependency downloads; no package or command exists merely because it is listed. Then create the scope/registry, tracking JSON, twelve budget records and traceability command in M1.2–M1.5. Use these commands only after their corresponding code and lockfile exist:

    cargo run --locked --manifest-path tools/aether-validation/Cargo.toml -- evidence --root .
    cargo run --locked --manifest-path tools/aether-validation/Cargo.toml -- traceability --root .
    cargo test --locked --manifest-path tools/aether-validation/Cargo.toml
    git diff --check

Prescribe the normal traceability output to include the following facts, with actual selected counts added by the implementation:

    metric_requirements=250
    metadata_associations=58
    experiment_specifications=12
    missing_original_ids=0
    duplicate_ids=0
    synthetic_experiments_executed=0
    deployed_schemas_and_capabilities=not_verified
    milestone_1_traceability=pass

A deliberately corrupted temporary record must yield a nonzero exit and identify the precise inconsistency, such as `missing metric L1-001` or `PX11 acceptance readiness contradicts required unset limit`. Successful Rust tests must report their actual number and each tested failure category. These prescribed behaviors now have actual implementation evidence in `evidence/prototype/milestone-1.md`; the traceability command ran successfully with 23 selected metrics, 42 selected associations and 30 passing Rust integration tests.

This milestone compiles and tests the isolated Rust validation tool and may resolve its declared Cargo dependencies. It does not compile the inference-engine prototype, start a real engine, read device interfaces, call runtime sockets, run a fixture endpoint or write to remote storage.

## Validation and Acceptance


The earlier plan-authoring acceptance was that the shared index and this detailed plan exist, all required living sections are present, master and milestone progress agree, references point to real prerequisites or explicitly future artifacts, and baseline/frozen evidence files remain unchanged. That authoring acceptance did not check off M1.1–M1.7; their checkmarks now refer to actual execution evidence.

Execution acceptance requires every output specified in M1.2–M1.6, exact original ID sets, unchanged baseline objects and route counts, bidirectional links, all selected metadata dependencies, twelve complete experiment/budget records, passing Rust validation commands and unit/integration tests, a concrete evidence report and synchronized plan statuses. The reviewer must be able to inspect one selected case and trace it through requirement, native/invented inputs, expected result, scope limit, budget and future execution milestone. An unselected case must remain visible as `not_prototyped`.

The report must explicitly retain L1-001/L1-005 uncertainty, any E-row semantic caveats, independent framework/backend boundaries, remote ownership uncertainty and synthetic-only hardware/timing evidence. No prototype case can claim an active transport or physical device ownership without its invented supporting record, and no invented record proves the real relationship can be collected.

Milestone 1 can pass its planning checks while future runtime readiness remains false. Required numeric runtime thresholds and agreements must be resolved before their evaluated runs; real production thresholds can remain unset throughout this milestone. A real-source validation result cannot pass in this environment. The engine implementation remains a later task; the Rust validation-tool implementation belongs to this milestone.

## Idempotence and Recovery


Generate tracking records deterministically from declared input versions and stable IDs. Before regenerating an existing output, inspect whether it contains manual experiment notes or later status evidence; preserve those fields by ID or stop and report the conflict rather than overwrite them. Sort records by their original IDs. Repeating the checker must not change records, timestamps, selection or expected checksums.

Keep rejection tests in dedicated temporary directories. Do not mutate v1 evidence, the real catalog or live status files to demonstrate failures. If execution stops halfway, leave completed tasks checked with evidence and other tasks pending; rerun the existing integrity checker before resuming, then validate the partially created outputs. An incomplete JSON file is a failure, not an empty successful result.

Do not reset repository changes, erase untracked work, regenerate baseline hashes, enable missing source features or change live deployment settings. Any new source correction belongs in a separate review record with the original ID and reason; do not apply it silently to the frozen research baseline. New evidence versions require an explicit recorded revision.

## Artifacts and Notes


Historical plan-authoring verification returned: required plan sections/format/links passed for both plans; original metric references valid; baseline and frozen-evidence files unchanged; prerequisite evidence checker still reports ten inputs and 24 artifact checksums; seven detailed tasks and eight program milestones pending. Those were documentation checks at plan creation. Current execution acceptance supersedes that pending state: the Rust traceability command and thirty integration tests have passed, with evidence recorded in the milestone report.

Completed plan-navigation outputs are `milestone-execution-plans/README.md`, this plan and the updated master `EXECPLAN.md`. Completed Milestone 1 deliverables are `docs/prototype-scope.md`, `docs/prototype-experiments.json`, `docs/prototype-status.json`, `docs/prototype-budgets/PX01.json` through `PX12.json`, the isolated Rust package `tools/aether-validation/` with its Cargo.toml, Cargo.lock, source/tests, and `evidence/prototype/milestone-1.md`.

The v1 baseline identities are 250 metric IDs, 58 association IDs, twelve planned representative experiments, E:88/R:35/H:100/I:26/U:1 routes and environment `aether-validation-synthetic-local-v1`. Experiment PX IDs are distinct from source evidence E IDs, contract profile C IDs, local frozen input B IDs and original L/M IDs. Do not combine these namespaces into a requirement count.

Store milestone outputs outside frozen v1 evidence. The validation tool's Cargo.toml, Cargo.lock and Rust source/tests are Milestone 1 deliverables. Engine fixture payloads, the engine package/lockfile, host collectors and benchmarks remain later work. The index is a navigation/status summary, not a replacement for detailed progress evidence.

## Interfaces and Dependencies


Use UTF-8 Markdown and JSON with `schema_version=1` for the new registry/status records. The experiment registry top-level fields are `schema_version`, `evidence_bundle`, `environment_id` and `experiments`. Each experiment carries the fields defined in M1.2, plus `experiment_kind` equal to `metric_behavior`, `pipeline_control` or `collection_control`; mixed cases can carry separately named assertions with their own requirement links. The registry is the source of selection truth; the checker rejects disagreement with status records or budgets.

The status top-level fields are `schema_version`, `evidence_bundle`, `evidence_manifest_sha256`, `environment_id`, `experiment_registry`, `metrics` and `associations`. Preserve original objects/cells and separate selection, research route, prototype state and actual verification. A metric/association's `evidence_references` must distinguish pinned upstream source references, frozen local baseline references and future runtime evidence, which is empty at this milestone. No hardware source is certified by assigning it a software E ID.

Budget records preserve the root template shape, add `experiment_id` and `proposal_status`, and reference the same environment/evidence bundle. An `input_specification` may contain planned fixture identifiers or virtual values; an `evidence` list may contain only material that already exists. The checker must enforce that distinction. Agreement and result fields remain honest even when a planning document is complete.

The Rust tool shares one evidence-verification implementation between the evidence and traceability commands; traceability calls `check_evidence` with the same root. Rust tests must pass the temporary tree's root explicitly so they do not accidentally check the workspace instead. Use only the selected JSON/hash dependencies and isolated test support, pinned in the tool's Cargo.lock. Never shell out to a Python script, embed Python or add a second runtime language for generation or tests. Select a temporary-copy layout containing the required evidence and planning files; skip no input verification to make a mutation test pass. Retain the legacy Python verifier and recorded Python-version fields as historical provenance without requiring execution or changing frozen v1 evidence. New code remains validation tooling and does not prescribe production engine architecture.

At each stopping point, update this plan first with detailed task evidence, then the master with milestone status, evidence link and next unfinished action, then the folder index if its status changed. At final Milestone 1 acceptance, retain all prior decisions and explain revisions rather than erasing the pending history.

## Revision Notes


2026-11-01: created the detailed Milestone 1 execution plan and shared milestone-plan folder at the user's request. Defined complete requirement tracking, twelve representative experiment specifications, separate budget proposal/agreement states, meaningful traceability validation and synchronized master tracking. Only plan authoring and prerequisite integrity inspection are complete; execution tasks remain pending. Product requirements, source classifications, frozen evidence and production scope are unchanged.

2026-11-01 21:15 IST: validated the written plans and index, confirmed original metric references and unchanged baseline hashes, reran the existing evidence-integrity checker and synchronized actual planning completion in both plans. No M1 execution output was generated; M1.1 is the next pending task.

2026-11-01 21:24 IST: revised Milestone 1 to use Rust for evidence verification, planning-record generation, traceability validation and tests. Added the isolated validation-tool package and its dependency lock to M1.1, updated commands/interfaces, and separated its build from the later engine harness. Earlier Python-run results remain historical; no Python execution is required going forward. The port and all execution tasks remain pending; frozen evidence and product requirements are unchanged.

2026-11-01 21:39 IST: executed and accepted M1.1–M1.7 in Rust. Delivered scope/registry/status/twelve proposed budget files and the locked validation crate. Evidence and traceability pass; 30 integration tests, Clippy, format and unchanged-input checks pass. Captured logs and SHA-256 records in `evidence/prototype/milestone-1.md`. Synchronized master/index: Milestone 1 complete; Milestone 2 fixture creation is next and has not started. Runtime experiments, budget agreement and real validation remain pending.
