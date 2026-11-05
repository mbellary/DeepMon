# Aether initial zero-application-change collection contract

Version: 0.1. Defined on 1 November 2026 for prototype preparation. Implementation, real-system qualification and numeric performance budgets are pending.

## Purpose and status

Define what Aether may read, what it may change, which sources are eligible first, and what evidence is required before collection can pass acceptance. This is a collection boundary and evaluation process. It does not finalize the production engine architecture or start implementation or deployment.

The current executable plan remains limited to intermediate Rust prototypes and synthetic fixtures. Every source below is a proposed initial collection interface, not an implemented adapter or certified compatibility claim. Real source access is a later qualification activity; this document does not authorize accessing a customer environment. No real-deployment GPU, NIC, fabric, kernel, CUDA, NCCL, serving release or container runtime has been selected or assumed. The later [versioned validation baseline](validation-evidence-baseline.md) records the observed local OS/toolchain for synthetic experiments only; it does not select a production compatibility range.

The [250-family requirements catalog](aether-metrics-catalog.md), [58 metadata associations](metadata-enrichment-matrix.md) and four observability layers remain intact. The [source recheck](source-mapping-recheck.md) identifies E:88, R:35, H:100, I:26 and U:1 routes. Those counts do not define a product support percentage. An initial collection subset does not remove the remaining requirements.

## Meaning of zero application change

A qualifying collection run requires no changes to serving code, application container contents or images, model pipelines, launch arguments, environment variables, deployment manifests, serving libraries, model configuration or workload lifecycle. Aether runs separately on the host in the intended product. Its own installation, configuration, credentials, storage directories and host access are explicit operational prerequisites, not changes to application code and not automatically granted permissions.

Reading an existing metric endpoint may involve polling. This initial path collects the source's published aggregates; it cannot reveal unexported events or make the source update faster. Fine-grained eBPF and device/library instrumentation remain product requirements to investigate later. A microsecond timestamp in a fixture does not demonstrate microsecond measurement accuracy.

There are three distinct outcomes: eligible by this contract, tested with a synthetic fixture, and verified on a specific real deployment. Record them separately. Never label a documented route or a passing fixture as verified real support.

## Initial source set

Profiles identify interfaces, not mandatory components on every host. Only explicitly configured targets and fields are eligible. Nothing below authorizes network scanning, workload inspection beyond the listed paths, or automatic feature activation. Every runtime profile starts disabled until its target, read permissions and finite resource limits have been configured. In the current prototype plan, all interfaces are simulated.

| Profile | Initial interface and observations | Access prerequisite and boundary | Initial qualification status |
|---|---|---|---|
| C01 | Existing vLLM Prometheus endpoint: native names, HELP/TYPE, labels, counters, gauges and histograms from the baseline | Configured address, read credentials if required, exact deployed schema. Preserve engine index and outcome semantics. | Documented; fixtures and real verification pending. [vLLM reference](https://docs.vllm.ai/en/latest/usage/metrics/) |
| C02 | Existing SGLang Prometheus endpoint, including already emitted cache families | Metrics must already be enabled and reachable; do not add `--enable-metrics`. Preserve scheduler and tokenizer boundaries. | Documented; fixtures and real verification pending. [SGLang reference](https://docs.sglang.io/docs/references/production_metrics) |
| C03 | Existing Dynamo frontend, component, router and endpoint-picker metrics, each as a separate emitter | Explicit endpoints and component identities; backend pass-through must retain the original producer. Do not configure ports or enable components. | Documented; fixtures and real verification pending. [Dynamo reference](https://docs.dynamo.nvidia.com/dynamo/reference/observability/metrics-catalog) |
| C04 | Existing TGI `/metrics`, including original batch/request families | Keep TGI measurement meanings. L1-001 is raw-only until accepted-request counting is verified; L1-005 remains blocked. L2-071 retains its interpretation caveat. | Documented; fixtures and real verification pending. [TGI reference](https://huggingface.co/docs/text-generation-inference/reference/metrics) |
| C05 | Existing Triton metrics, including backend counts, cumulative timing, shared pinned pool and response cache | Inspect deployed version and enabled families. Response cache is not KV-prefix cache; pinned pool is server-wide. | Documented; fixtures and real verification pending. [Triton reference](https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/user_guide/metrics.html) |
| C06 | Existing TensorRT-LLM native collector and Triton TensorRT-LLM backend metrics, as distinct schemas | Native iteration fields need already active statistics and populated keys; do not enable them. Backend memory evidence includes an archived schema, requiring build verification. | Documented conditionally; fixtures and real verification pending. [Native collector](https://github.com/NVIDIA/TensorRT-LLM/blob/main/tensorrt_llm/metrics/collector.py), [backend evidence](https://docs.nvidia.com/deeplearning/triton-inference-server/archives/triton-inference-server-2520/user-guide/docs/tensorrtllm_backend/README.html#triton-metrics) |
| C07 | Bounded host reads: `/proc/stat`, `/proc/meminfo`, selected `/proc/PID/stat` and `/proc/PID/status`; existing cgroup v2 CPU, memory, pressure and I/O accounting files | Read only allowlisted hosts, processes and cgroups. Record mount/namespace, PID lifetime and controller availability. Do not enable controllers or remount filesystems. | Selected initial host interface; field and kernel qualification pending. [procfs](https://docs.kernel.org/filesystems/proc.html), [cgroup v2](https://docs.kernel.org/admin-guide/cgroup-v2.html) |
| C08 | NVIDIA device inventory and selected NVML getters: UUID/name, available partition identity, framebuffer used/free/total, utilization, power, enforced power limit and cumulative energy when supported | Existing driver/library/device access; calls from an explicit getter allowlist only. No setter, reset, mode change, profiling activation or new GPU service. | Selected initial device interface; hardware/driver/partition qualification pending. [NVML queries](https://docs.nvidia.com/deploy/nvml-api/latest/api/group__nvmlDeviceQueries.html) |
| C09 | Existing NIC/interface inventory and standard Linux link statistics; readable RDMA device/port identity and counters | Explicit interface/device/port selection, current namespace and read-only netlink/sysfs access as available. Do not change links, counters or driver settings. Driver-specific fields require separate schema verification. | Selected initial network interface; NIC/driver/fabric qualification pending. [Linux interface statistics](https://docs.kernel.org/networking/statistics.html), [RDMA ABI](https://kernel.googlesource.com/pub/scm/linux/kernel/git/torvalds/linux/+/8c13415c8a4383447c21ec832b20b3b283f0e01a/Documentation/ABI/stable/sysfs-class-infiniband) |
| C10 | Configured identity manifest and native info families; approved host files for boot/process/device identity | Operator-supplied deployment/resource relations must retain provenance and validity. Report configuration as intent; it does not prove active GPU/NIC use. | Metadata contract defined; association fixtures and real verification pending. [Enrichment rules](metadata-enrichment-matrix.md) |

C01–C06 admit the existing endpoint routes identified in the source matrix when their fields are actually emitted. They do not require collecting every optional family or claiming all its dimensions. A finite family/label allowlist and limits are required for each experiment. C07–C09 deliberately select a small set of host/device/network interfaces; the remaining H routes remain pending, not rejected.

Initial metric traceability: C07 includes L4-047–L4-050, L4-054–L4-064 and L4-070 where the named files provide the required fields. C08 includes L4-001, L4-007–L4-013; L4-010 is calculated from matching memory inputs. C09 includes L4-071–L4-073 and readable L3 RDMA counters individually verified against the matrix; other fields, such as link speed, are not promised by generic interface statistics. These ranges are candidates, not evidence that every field is available on one host.

No standalone LMCache, Dynamo KVBM or SGLang HiCache adapter has been qualified. Cache metrics already emitted through an admitted engine endpoint may be read at their documented boundary. A separately configured existing Prometheus endpoint from any additional producer is eligible for the same raw parser evaluation; retain unmapped families as explicitly unmapped observations only if permitted by its allowlist. Assign requirement IDs and semantic support only after source/schema review. A source is never rejected merely because its engine has not yet been reviewed. DeepInfer remains deferred by user direction.

Direct Docker/containerd control sockets and Kubernetes API discovery are outside the initial active profiles. Their future metadata value is retained; broad runtime access must not be implied by a “read-only” intention. Explicit discovery targets and deployment manifests supplied as data can be used without those connections.

## Permitted actions

| Action | Initial policy |
|---|---|
| Read configured existing metric endpoints | Allowed within target, method/path, credential, timeout, payload, concurrency and request-rate limits; no redirects to unlisted targets. |
| Read allowlisted host accounting/inventory files or query interfaces | Allowed with existing access; bound reads and validate the source namespace. No broad process-memory, environment or filesystem traversal. |
| Use existing NVML getters and read NIC/RDMA counters | Allowed only for the configured fields and devices; record unsupported fields or denied access. No automatic privilege escalation. |
| Configure/start/stop/restart Aether itself | Permitted as an Aether-only operation in an explicitly authorized future installation. The prototype may start only its own harness and fixture services. |
| Write Aether records, journals, diagnostics and result files | Only to configured Aether directories or explicitly authorized output destinations, with finite retention/space limits. Current plan uses local simulated storage. |
| Set Aether collection intervals or reduce its load | Allowed within the run manifest; changes must be recorded. Reduced collection marks freshness/coverage degraded and cannot silently pass the original budget. |
| Enable an engine metric flag, optional statistic, tracing mode or new exporter | Outside this contract. An owner may establish it separately before a later run; Aether cannot claim that activation was zero application change. |
| Change a serving image, library, pipeline, launch argument, environment variable, config or deployment manifest; restart a workload | Not permitted by the initial contract. |
| Inject a library, use `LD_PRELOAD`, attach CUPTI/NCCL plugins, enable a profiler or read application memory | Not permitted in initial profiles. Existing or later approved instrumentation remains separately evaluated. |
| Attach eBPF/uprobes, enter workload namespaces, open runtime control sockets or change host tracing/kernel settings | Not permitted in initial profiles. Later host-only probes need a named probe contract, minimum access and measured overhead; they remain within the broader research requirements. |
| Change GPU/NIC settings, reset devices/counters, change MIG layout, tune fabric or query switch management systems | Not permitted in initial profiles. Switch telemetry requires a separate external source contract. |

“Allowed” describes policy for an explicitly configured run; it is not authorization to deploy now. Proposed access should use the least privilege demonstrated sufficient for that interface. No blanket root, privileged-container, runtime-socket or kernel-capability requirement is approved. If access fails, report it rather than widening privileges. Permission requirements must be recorded per qualified build/interface, not guessed as one universal list.

## Discovery and qualification record

For each target, record a stable source ID and incarnation, producer and schema/build evidence, endpoint/path or device/interface identity, enabled optional features, authentication/access method without secret values, namespace, allowed fields, maximum payload and label cardinality, timeouts, concurrency, polling schedule and resource limits. An address alone does not identify the measured subject. Capture the exact observed exposition/schema hash and source references; unavailable version evidence stays unknown.

For hardware, retain model, driver/firmware evidence and partition/link mode when readable. A NIC model does not establish whether a particular operation used Ethernet, RoCE or InfiniBand; record physical capability, configured mode and observed use separately. Kernel/API availability is discovered per interface. Backward compatibility is an eventual qualification objective, not a guarantee inferred from using Rust or a new driver.

A field has separate states for presence, freshness, valid type/units, semantic mapping and metadata association. Use the existing metadata outcomes for associations. Collection also needs `not_configured`, `not_enabled`, `unmapped`, `parse_error`, `timeout`, `oversize` and `budget_exceeded` states. An HTTP error or invalid payload must not become a healthy zero. Unknown or denied hardware attribution does not invalidate an otherwise valid engine-scoped raw observation.

## Data, timing and failure behavior

Preserve producer, native family and labels, HELP/TYPE evidence, native and normalized units, observed subject scope, collector/source incarnation, collection start/end, available source timestamps and freshness evidence. If the source supplies no timestamp or update cadence, retain that uncertainty. Separate wall-clock timestamps used to compare hosts from monotonic time used for local elapsed durations.

Preserve counters and detect resets or incarnation changes before calculating rates. Histogram buckets do not provide individual request events, and repeated equal snapshots can be valid separate observations. Do not merge Dynamo framework and backend observations as independent counts of the same logical work. Replica-wide observations may reference a GPU set but must not be multiplied into a count per GPU. Apply enrichment only with the evidence and time validity required by M01–M58.

Bound response bytes, parse work, target count, series count, label sizes, in-flight work, memory, disk and outbound traffic. Oversized/malformed input must yield an explicit rejected/partial status with bounded diagnostics. Do not silently retain an arbitrary prefix as a complete scrape. Do not collect prompts, responses, tensor/KV contents, process environments or credentials. Filter configured sensitive labels before retention and keep credential values out of diagnostics.

When a source disappears, back off within configured limits and report the interval without observations. Mark last-known data stale; do not fabricate samples. Disable a failing optional profile independently. If a freshness or overhead limit is exceeded, reduce/stop the affected Aether work according to the recorded policy and mark the run degraded or failed. Never repair the workload or enable missing telemetry automatically.

## Acceptance-budget process

A budget is a measurable limit attached to a defined source profile, environment, workload and measurement window. No numeric overhead, freshness, resolution or loss promise has been agreed. The companion [budget template](collection-acceptance-budget-template.json) uses `null` for unset limits; null means not agreed, not unlimited and not a passing result. This template is a planning record, not an implemented configuration format.

1. Define the experiment before running it: source/field set, versions or fixture references, workload shape, offered metric volume/cardinality, duration, warmup, repetitions, baseline/control method, fault cases and measurement tools. Synthetic values must be labeled synthetic.
2. Identify the user-visible tolerances and resource envelope. The product owner and, for real systems, workload/infrastructure owner agree the numbers; the implementer proposes them with baseline evidence. Record the decision and rationale before the evaluated run. Do not infer them from marketing goals or choose them after seeing results.
3. Specify every measurement's formula, unit, population, percentile/window, reference truth and uncertainty. Name which profile/run it applies to. A budget may be explicitly not applicable only with a recorded reason and owner; missing measurement capability cannot silently become not applicable.
4. Fix finite execution safety limits before even a calibration run. Exploratory runs can measure candidate budgets but receive an exploratory result, not an acceptance pass. A synthetic experiment may select fixture-only limits without making production promises.
5. Run baseline and collector-enabled cases under comparable conditions, retaining raw results and variation. For eventual real testing, include endpoint-serving overhead and inference impact, not only Aether CPU. For host-wide or cluster-wide claims measure the relevant hosts/ranks and execution boundary; one process average is insufficient.
6. Apply the predeclared decision rules. Record `pass`, `fail`, `inconclusive`, `not_run` or justified `not_applicable` per criterion. Any required unset budget, missing reference, incomplete fault case or uncertainty overlapping the limit prevents an overall pass. Overall acceptance requires every required criterion to pass; a changed limit or source profile requires a new manifest and reevaluation.

The initial budget register must cover the following independent quantities:

| Criterion | Measurement to declare | Acceptance limit status |
|---|---|---|
| Aether CPU | Process CPU time divided by elapsed time, in core equivalents; distribution/window and peak method | Unset |
| Memory | RSS/high-water memory plus host-attributable buffers/page cache where measurable; allocator/queue peaks | Unset |
| Local I/O and storage | Read/write bytes and operations per interval, retained bytes, disk high-water mark and maximum outage capacity | Unset |
| Network and endpoint impact | Request rate/concurrency/bytes, endpoint response latency/error change, upload bandwidth where relevant | Unset |
| Inference impact | Relative throughput loss and latency increase at named TTFT/TPOT/e2e percentiles versus paired baseline, with variation | Unset; real workload evaluation is outside current prototype scope |
| Source freshness | Native publication/update delay when a truth reference exists; otherwise explicitly unknown | Unset |
| Collection freshness | Source update or known fixture change to observation capture; separate configured poll period and missed polls | Unset |
| Query freshness | Capture to durable acknowledgment, commit and query visibility as separate stages | Unset |
| Timing accuracy and resolution | Error versus reference, clock uncertainty and resolvable event separation; endpoint aggregates cannot qualify event visibility | Unset; real event-path proof deferred |
| Loss | Missing observations at named boundaries and missing source events only where source truth exists | Unset numeric tolerance; accounting completeness is required |
| Duplication and replay | Extra record IDs and wrong logical aggregates after retry/restart; valid repeated snapshots remain separate | Exact fixture results required; no universal production guarantee |
| Attribution and semantics | Correct/rejected associations, type/unit fidelity, counter reset handling and calculation results against truth | Exact fixture results required; unresolved identities must remain unresolved |
| Recovery and backpressure | Recovery time, queue/disk behavior, explicit loss/rejection and query results under named faults | Unset time/capacity limits; expected fixture outcomes must be explicit |

Different budgets may be needed per host, profile, collector mix, normal load, burst load and fault condition. Declare the aggregation rule in advance: averaging quiet nodes must not hide an overloaded host. Budgets are conditional evidence for the tested combination, never a single universal “zero overhead” number.

## Loss and durability boundary

Name the units being counted. A scrape attempt, exposition, metric observation, histogram event and inference request are different things. A source histogram does not expose individual events, so successful scrape delivery cannot prove event completeness. Latest-value gauge updates missed between reads are a source-observation limitation, not agent-delivery loss.

Track read attempts/results, parsed observations, admitted observations, rejected observations, durable acknowledgments, committed/query-visible records, replay duplicates and known pending work separately. For each stage and record ID, account for input as successful output, explicit rejection/loss or outstanding work. Report attempt failures even when no number of missing source observations can be known.

“Admitted” means accepted into Aether's bounded processing path; it does not mean durable. A durable acknowledgment is permitted only after the declared experiment's persistence mechanism has met its tested boundary. Record that boundary explicitly, including process crash versus host/power failure. Query-visible means included in the selected committed snapshot. Prototype storage mechanics will be selected in the storage milestone; this contract does not prescribe their production design.

Retries must preserve observation identity so the same accepted record does not multiply after replay. Independently captured snapshots need distinct identities even when values match. Recovery checks compare committed unique IDs and logical results with expected records under each declared failure. A crash can lose volatile admitted records within the declared boundary; report them rather than claiming end-to-end zero loss. Unknown loss remains unknown.

## Required acceptance scenarios

Before a prototype profile passes, its fixtures must show normal parsing and enrichment, missing/disabled optional families, unsupported fields, denied access, malformed/oversized input, bounded cardinality, timeouts/source disappearance, stale data, counter reset/restart, conflicting/reused identities, framework/backend duplication risk and retry behavior. Exercise storage interruption, full queue/disk and restart once the storage path exists. Every scenario needs expected states/counts; not every scenario applies to every profile.

Only fault the prototype's own services and synthetic inputs in the current plan. Do not kill customer processes, reboot hosts, saturate real fabrics or fill shared disks. Future real fault tests need their own environment and run authorization.

Real-system qualification later requires versioned endpoints and hardware/interface snapshots, baseline-versus-enabled evidence, the agreed budgets, permission observations and recovery/loss results within the stated boundary. Passing synthetic fixtures establishes only behavior against those inputs.

## Remaining decisions and next step

The contract now defines initial source eligibility, allowed actions, failure semantics and the budget-setting process. Concrete deployed targets, exact source versions, field allowlists, hardware/driver/kernel combinations, permissions and numeric run limits remain unset. No live collection run may be marked ready while its required manifest or finite safety limits are missing. Resolving those real-system inputs is not a prerequisite for synthetic prototyping.

The next step is prototype Milestone 1: select representative fixtures from these eligible profiles and create tracking entries for all 250 requirements and 58 associations. Choose explicit synthetic limits and expected outcomes for each selected experiment. Production collection and numeric production guarantees remain unapproved and untested.

Version 0.1 evidence addendum, 1 November 2026: source inputs are now pinned in `evidence/manifest-v1.json`, historical U rows are reconciled in `evidence/reconciliation-v1.json`, and the first synthetic reference environment is recorded in `validation/first-environment-v1.json`. The user confirmed no existing cluster. Deployed schemas/capabilities, fixture execution and numeric budgets remain pending; no action policy or product requirement changes.
