# Step 2: metadata enrichment and identity associations

Research date: 1 November 2026.

## Purpose

Define how Aether can associate the existing vLLM, SGLang and NVIDIA Dynamo observations with their software, host, container, accelerator and network context. This is a proposed enrichment contract and evidence matrix, not an implemented discovery system or a claim of validated deployment support.

Inputs are the [native metric baseline](engine-native-metrics-baseline.md) and [four-layer requirements catalog](aether-metrics-catalog.md). DeepInfer is outside this step. Exact software releases, runtime, orchestration, hardware and network configuration remain unspecified.

This step defines metadata sources and associations only. It does not add performance metrics, choose hardware, implement probes, set collection intervals or certify that an application-level metric is obtainable without an existing endpoint.

## 1. Enrichment rules

1. Preserve the original emitter identity, metric name, native labels, measurement boundary, units and observation interval. Add associations without overwriting those facts.
2. Distinguish the **emitter** that publishes a metric, the **subject** the metric describes, the **allocated resources** available to the workload, and the **resources observed participating** in execution. They can be different entities.
3. Attach a scalar identity only when the observation has that scope. A replica-wide observation can reference a set of GPUs; it must not be duplicated onto every GPU and then summed.
4. Native labels establish what the publisher reports. Verification of physical ownership or deployment membership needs a separate association, with supporting evidence.
5. Keep configured capability, allocation, observed activity and selected communication path separate. A permitted GPU or NIC is not proof that an operation used it.
6. Every association has validity times and provenance. A currently correct association is not automatically correct for historical measurements.
7. Missing evidence remains unknown or ambiguous. No hostname pattern, GPU ordinal, IP address, model name or process command alone establishes a distributed replica.

## 2. Source and resolution classifications

| Code | Meaning | What it establishes |
|---|---|---|
| N | Native metric label or info family | A value reported by that publisher, at that metric's scope |
| C | Configuration, registration or control-plane metadata | Declared intent, allocation or membership; source and generation must be recorded |
| H | Host, runtime or driver discovery | Local identities and relationships observable through supported interfaces |
| I | Existing runtime/library events, logs or topology integration | Runtime relationships when their records provide explicit identifiers |
| X | Not established generically | Requires an additional interface or evidence; leave unresolved |

Codes describe a source route, not a guarantee of implementation, availability or truthfulness. A dimension may have several routes. Conditional Kubernetes, Docker, containerd, NVIDIA and NCCL sources do not become universal dependencies.

Association outcomes are `resolved`, `reported_only`, `candidate`, `ambiguous`, `unknown`, `conflict`, `stale`, `permission_denied`, `unsupported`, or `not_applicable`. Do not encode any of these as an invented identity or a healthy zero. `Resolved` means the recorded evidence supports that specific relation; it does not assert causal attribution.

## 3. Identity keys proposed for Aether

These are design choices for this engine, not existing universal engine labels.

| Entity | Proposed key / scope | Reuse and lifecycle rule |
|---|---|---|
| Deployment environment | Explicit Aether installation/environment ID | Do not infer customer/cluster ownership from network names |
| Host | Provisioned host ID plus observed OS instance/boot identity | Hostname is a display attribute; cloned machine IDs need disambiguation |
| Process | Host ID + boot ID + host PID + process start identity | Revalidate around reads; use a stable process handle where available |
| Thread | Process key + host TID + thread lifetime | Scope event correlations to the process/thread incarnation |
| Container | Runtime endpoint identity + runtime namespace + full container ID + task incarnation | Runtime namespace is not Linux network namespace or Kubernetes namespace |
| Cgroup | Host/boot + cgroup version + hierarchy/mount identity + cgroup identity/lifetime | Paths and numeric IDs can be reused; preserve time-bounded membership |
| Network namespace | Host/boot + namespace identity + lifetime | Namespace handles/inodes need lifecycle protection; not globally permanent IDs |
| Metric emitter | Collector-assigned emitter ID + service/registry incarnation | URL alone is not identity; proxies and endpoint reuse require rebinding |
| Engine instance | Verified publisher/deployment instance ID with lifecycle | Preserve native engine index separately; it is not globally unique |
| Model replica | Explicit serving-replica/group ID with membership generation | Parent deployment, model name or number of GPUs alone is insufficient |
| GPU | Vendor-qualified physical device identity, host relation and validity | NVIDIA UUID when available; ordinal is local and mutable |
| GPU partition | Parent GPU identity + driver-reported instance identities + partition lifetime | MIG reconfiguration can reuse local instance IDs |
| NIC/function | Host + hardware/function identity + discovery lifetime | PCI address alone identifies location, not permanently installed hardware |
| NIC port | NIC/function key + physical port identity | Do not collapse multi-port or SR-IOV functions |
| Network interface | Host/boot + network namespace + ifindex + interface lifetime | Name, MAC and ifindex can change or be reused |
| RDMA port | Host + RDMA device identity/lifetime + port | Device names are host-local; associate with the correct hardware/function |
| Communicator/group | Runtime-scoped group identity + membership generation | Rank number alone is not unique; multiple groups coexist |

Aether-generated opaque IDs may represent these composite keys internally. This document does not require exposing the entire key as labels on every metric.

## 4. Software and workload dimensions

The evidence column is the condition for resolving an association. If it is not satisfied, use the unresolved outcome indicated rather than guessing.

| ID | Dimension | Route / source | Join key and cardinality | Evidence required | Unresolved behavior |
|---|---|---|---|---|---|
| M01 | Environment / cluster identity | C: explicit installation or verified scheduler cluster ID | Installation → many hosts/emitters | Configured scope; remote collectors remain in their own environment | Unknown cluster; never derive from hostname suffix |
| M02 | Emitter identity and location | H/C: discovered endpoint plus verified process/service registry | Emitter incarnation → local process, proxy or remote service | Listener ownership/registration at collection time; address alone insufficient | Record collection target; subject location unknown |
| M03 | Emitting software family | N/C/H: native namespaces, version info, executable/package metadata | Emitter → one or several producers | Confirm metric producer; Dynamo can pass through engine families | Preserve separate producer claims; classify uncertain fingerprints as candidates |
| M04 | Software version/build | C/H/N: manifest, package/build info or exposed version | Producer instance → version/build facts | Running artifact/build provenance, not image tag text alone | Version unverified; no release-support claim |
| M05 | Served model name | N: `model_name` or `model` on relevant families | Observation → publisher-reported model name | Label definition and API alias behavior from matching build | Keep original spelling and source; no automatic cross-emitter merge |
| M06 | Model revision / weights / quantization | C/N: explicit deployment/configuration facts | Engine instance → model artifact/config revision | Revision/checksum or verified configuration tied to the instance | Unknown revision; display name is insufficient |
| M07 | Engine role | N/C: SGLang `engine_type`, Dynamo component/worker role, verified vLLM config | Instance/observation → role at valid time | Family-specific role semantics; frontend view may differ from worker topology role | Retain native role, canonical role unknown |
| M08 | Engine index | N: vLLM `engine` | Emitter incarnation + native index → engine member | Explicit mapping to a running engine member | Store index; never reinterpret as GPU index or TP rank |
| M09 | Serving replica | C/I: engine group registration, scheduler assignment or explicit configured ID | Replica generation ↔ many engine processes/hosts | Exact serving-group membership; deployment owner is not enough | Replica unknown; do not group all same-model workers |
| M10 | Deployment owner | C: runtime labels or optional orchestrator owner references | Workload instance → owner UID/generation | Verified container/task-to-workload association and owner chain | Retain labels as reported; owner relationship unknown |
| M11 | Framework worker identity | N/C: Dynamo worker/discovery ID | Framework scope + worker ID + incarnation → worker subject | Match publisher/family encoding and discovery registration | Keep raw ID and scope; no decimal/hex conversion without schema evidence |
| M12 | Remote worker location | C/I: framework discovery or scheduler inventory | Worker incarnation → endpoint → remote process/host | Valid discovery record and target-side identity evidence | Worker may remain known while host is unknown; do not use frontend host |
| M13 | TP / PP / DP / EP / CP ranks | N/C/I: available labels and runtime group metadata | Group generation + rank kind/value → members | Rank scope and process/device mapping for that build | Keep native rank; physical mapping unknown |
| M14 | Parallel-group membership/size | C/I: runtime or scheduler group records | Group generation → member set, many-to-many across groups | Explicit complete membership, including distinction between world and subgroups | Partial membership flagged; no missing-rank claim without expected set |
| M15 | Communicator identity | I: existing communication-library registration/profiler records | Runtime/process scope + communicator generation | Identifier lifetime and cross-rank group matching | Unknown communicator; do not join by coincident timestamps |
| M16 | Request/batch/iteration identity | N/I where existing events or exemplars provide identifiers | Source-scoped event IDs; requests ↔ batches can be many-to-many | Explicit semantic records and matching rules | Aggregate endpoint histogram cannot be expanded into request events |
| M17 | Logical expert / physical placement | C/I: explicit router and placement records | Model layer + logical expert ↔ replicas/GPU over placement epochs | Expert identity and placement-change history | No generic expert attribution from PID or GPU activity |
| M18 | Cache provider / tier / pool | N/C: native source/pool labels and verified connector config | Engine/cache-service instance → provider and local pool IDs | Preserve logical KV, host, storage, response and tokenizer-cache meanings | Provider/tier unknown; do not assume LMCache/KVBM/HiCache |
| M19 | Cache service peer/storage target | C/I: connector/service registration or explicit transfer metadata | Transfer/cache service → peer set over time | Actual service/operation identity, not merely a configured URL | Configured target retained as intent; actual target unknown |
| M20 | Tenant / team / service ownership | C: explicitly administered identity mapping | Workload/replica → approved ownership scope | User-selected authoritative metadata source and conflict rules | Unknown ownership; arbitrary labels are not authorization evidence |

Native label routes above inherit the family-specific definitions and version uncertainties recorded in the Step 1 baseline. Configuration routes are proposed association methods, not assertions that every engine exports such an interface.

## 5. Host and container dimensions

| ID | Dimension | Route / source | Join key and cardinality | Evidence required | Unresolved behavior |
|---|---|---|---|---|---|
| M21 | Host / OS / CPU architecture | H: host inventory and OS interfaces | Host incarnation → inventory facts | Collector knows whether it observes the actual host or an isolated namespace | Record observed environment; do not call it host-wide if visibility is restricted |
| M22 | Boot identity | H: OS boot identity | Host → successive boot incarnations | Observed boot change invalidates local ephemeral identities | Do not join PIDs/interface IDs across unknown boot boundaries |
| M23 | Process and parent | H: procfs/process APIs | Process key → parent process incarnation | Start identity, host PID scope and race-aware reads | Parent exit/reparenting handled historically; ancestry alone does not prove engine membership |
| M24 | Linux namespaces | H: process namespace handles/mappings | Process → namespace identities over lifetime | Verify PID/net/mount namespace roles separately | Namespace identity unknown where inaccessible |
| M25 | Process-to-cgroup | H: procfs and supported cgroup interfaces | Process lifetime → cgroup memberships over time | Hierarchy/version and observed movement recorded | Do not resolve by substring match on a cgroup path |
| M26 | Container-to-process | H/C: runtime task/inspect API plus host PID/cgroup verification | Container task incarnation → many process incarnations | Runtime endpoint/namespace, full ID and live task mapping | Runtime inaccessible: retain verified process/cgroup context without inventing container |
| M27 | Container image/build | C: runtime metadata and artifact digest | Container incarnation → image artifact | Digest identifies artifact; running files may differ through mounts/updates | Image tag is reported metadata, not proof of running library versions |
| M28 | Kubernetes pod / namespace | C: optional runtime/CRI and Kubernetes metadata | Container full ID → pod UID + cluster scope | Verified object IDs; namespace name alone not unique across clusters | Not applicable without Kubernetes; no mandatory dependency |
| M29 | Orchestrator owner chain | C: optional owner references | Pod UID → ReplicaSet/Deployment/StatefulSet or other owner UIDs | Explicit valid owner references and object generation | Owner known does not automatically resolve a distributed serving replica |
| M30 | Endpoint-to-publisher process | H/C: listener/socket ownership and service registration | Net namespace + endpoint/listener incarnation ↔ process set | Shared listeners, proxies and multiprocess collectors accounted for | Multiple candidates retained; no first-PID-wins rule |
| M31 | Publisher-to-execution workers | C/I: engine/framework registry or explicit process topology | Publisher instance → worker/process/host set | Group registration and remote-worker membership | API listener ownership does not establish worker ownership |

Host primitives: [Linux procfs](https://docs.kernel.org/filesystems/proc.html), [cgroup v2](https://docs.kernel.org/admin-guide/cgroup-v2.html). Runtime/control-plane routes: [Docker API](https://docs.docker.com/reference/api/engine/), [containerd namespaces](https://containerd.io/docs/main/namespaces/), [Kubernetes owner references](https://kubernetes.io/docs/concepts/overview/working-with-objects/owners-dependents/). These interfaces support discovery facts; the proposed join and ambiguity rules are Aether design decisions.

## 6. Accelerator and topology dimensions

The NVIDIA routes below apply only when NVIDIA hardware and supported interfaces are present. They are examples for that backend, not a claim of vendor-independent accelerator support.

| ID | Dimension | Route / source | Join key and cardinality | Evidence required | Unresolved behavior |
|---|---|---|---|---|---|
| M32 | GPU model / vendor | H: supported device inventory | Physical GPU identity → model/vendor | Driver-reported inventory and host relation | Unknown/unsupported device; do not infer from model being served |
| M33 | GPU UUID / PCI identity | H: device UUID and PCI inventory | Physical device ↔ current host PCI function | Both identities observed in same valid inventory generation | Keep ordinal only as a local display attribute |
| M34 | Visible GPU ordinal mapping | C/H: process/container device exposure and driver inventory | Process view + ordinal → device identity set | Resolve remapping in that process view; environment ordering alone may be incomplete | Candidate allocation only; ordinal zero is not host GPU zero |
| M35 | MIG GPU/compute instance | H: partition-capable driver API | Parent GPU → partition incarnation | Driver-reported parent/child relation and reconfiguration history | Parent known may coexist with unknown partition; never substitute silently |
| M36 | MPS/shared-device mode | H/C/I: supported driver/service metadata | Device ↔ server/client process set | Explicit sharing-mode evidence and client mapping where available | Process attribution partial; server PID is not automatically every client's identity |
| M37 | Allocated GPU set | C/H: device allocation records and runtime exposure | Replica/process/container → many allocated devices | Verified allocation/exposure; preserve allocated versus visible distinction | Record scope and candidates; no execution claim |
| M38 | Observed process-to-GPU use | H/I: supported process/device records | Process incarnation ↔ GPU/partition during observation | Reconcile PID namespace, process lifetime and sharing mode | Unknown or partial membership when driver records are unavailable |
| M39 | Rank-to-GPU mapping | C/I plus M13/M38 | Group generation + rank → process/device association | Rank identity and explicit worker device selection | Native rank retained; no mapping by rank modulo GPU count |
| M40 | Replica GPU composition | C/H/I from verified members | Replica generation → device set and hardware-model set | Complete member/allocation mapping | Partial composition retained; no single GPU-model label for heterogeneous group |
| M41 | NUMA / PCIe locality | H: device/sysfs topology | Host device/function ↔ NUMA/PCI ancestry | Verified local topology in same host inventory | Unknown locality is not NUMA node zero |
| M42 | NVLink / NVSwitch adjacency | H/I: supported device/fabric topology | Device/link/switch identities → time-bounded adjacency | Topology plus actual link state when queried | Physical adjacency does not prove selected communication path |

Primary inventory interfaces: [NVML device queries](https://docs.nvidia.com/deploy/nvml-api/latest/api/group__nvmlDeviceQueries.html), [NVML MIG management](https://docs.nvidia.com/deploy/nvml-api/latest/api/group__nvmlMultiInstanceGPU.html), [NVIDIA system/process reporting](https://docs.nvidia.com/deploy/nvidia-smi/), [Linux PCI sysfs](https://docs.kernel.org/PCI/sysfs-pci.html), [DCGM topology](https://docs.nvidia.com/datacenter/dcgm/latest/learn/core-services/topology-and-links.html). A supported inventory association does not imply supported process-level utilization or complete concurrent execution attribution.

## 7. NIC, fabric and software-stack dimensions

| ID | Dimension | Route / source | Join key and cardinality | Evidence required | Unresolved behavior |
|---|---|---|---|---|---|
| M43 | Network interface identity | H: netlink/sysfs in relevant namespace | Net namespace + ifindex + lifetime → interface | Namespace, link creation/deletion and rename accounted for | Interface name alone insufficient |
| M44 | NIC model / vendor / function | H: PCI IDs, supported driver inventory and device database | Interface/RDMA function → hardware identity | Driver/device relation; distinguish physical function, virtual function and virtual interface | Raw IDs retained if model decoding unavailable |
| M45 | NIC port / negotiated mode | H: supported port and RDMA inventory | NIC/function + port → link facts | Port-specific state/mode; do not generalize across all ports on a card | Unknown mode is not TCP fallback |
| M46 | RDMA device-to-netdev | H: supported RDMA/netdev inventory | RDMA device/port ↔ netdev/function | Explicit association including VFs, representors and multiple ports | Keep separate inventories until link is verified |
| M47 | Fabric capability | H/C: supported adapter/port capability | Port → capability set | Reported support, separate from configuration and use | Capability only; no traffic-path conclusion |
| M48 | Port link layer | H: RDMA `link_layer` and netdev type | RDMA port → Ethernet/InfiniBand link-layer fact | Per-port source and observation time | Ethernet RDMA capability alone is not proof this workload uses RoCE |
| M49 | Workload-configured NIC/fabric | C: targeted documented transport configuration | Workload/group → allowed/preferred interfaces/transport | Versioned interpretation of settings | Retain as intent, not active use |
| M50 | Observed selected transport | I/H: existing library/connection records that identify the operation path | Communicator/connection/operation → selected transport | Evidence distinguishes bootstrap, control and payload paths; supports direction/phase changes | Active data transport unknown; do not infer from one open TCP socket |
| M51 | GPU-to-NIC topology | H: shared host PCI/NUMA/device topology | GPU ↔ candidate NIC/port set | Hardware locality relationship | Candidate adjacency only; not NIC actually used by collective |
| M52 | Operation-to-NIC attribution | I: transport records with explicit endpoint/device identifiers | Operation/connection → NIC port/function set | Runtime identifiers plus verified device mapping | Leave operation unattributed; do not attach all NICs on host |
| M53 | Remote peer identity | C/I/H: verified peer registry and connection metadata | Connection/path → remote endpoint incarnation | NAT, proxies, namespace and endpoint reuse handled | Peer address retained separately from remote host/workload identity |
| M54 | Switch / path / rail identity | C/I: optional fabric controller or switch topology | Port/path generation → switch/rail relations | External topology and routing evidence; active paths may change | Unknown fabric path; host cannot generically recover switch queues/topology |
| M55 | Driver / firmware versions | H: device and driver inventory | Device/function incarnation → reported versions | Actual loaded driver and device firmware facts | Package installed on disk is not proof of loaded version |
| M56 | CUDA / communication-library versions | C/H/I: running artifact metadata and existing runtime version reports | Workload process/library instance → version/build | Distinguish host driver capability from loaded runtime, and multiple libraries/processes | Unknown workload version; `nvidia-smi` CUDA display is not proof of installed runtime |
| M57 | Cache connector/library version | C/H/I: artifact and existing component registration | Engine/cache instance → connector build/config | Bind version evidence to actual loaded/deployed component | Do not infer connector from metric prefix or image tag alone |
| M58 | Collector visibility/capability | H: discovery attempt result | Source/collector + host/process scope → status | Report source access and version/capability limitations | Partial visibility explicit; never equate access denial with no workload/device |

Interface and RDMA definitions: [Linux netdev ABI](https://github.com/torvalds/linux/blob/master/Documentation/ABI/testing/sysfs-class-net), [InfiniBand/RDMA sysfs ABI](https://github.com/torvalds/linux/blob/master/Documentation/ABI/stable/sysfs-class-infiniband). Transport configuration: [NCCL environment variables](https://docs.nvidia.com/deeplearning/nccl/user-guide/docs/env.html). CUDA compatibility context: [NVIDIA compatibility documentation](https://docs.nvidia.com/deploy/cuda-compatibility/latest/). These sources establish interfaces/capability distinctions; the operation-level joins remain conditional until the actual runtime supplies sufficient evidence.

## 8. Engine-specific association paths

| Engine/framework | Existing native anchor | Required association chain | What must remain separate |
|---|---|---|---|
| vLLM | Emitter + `model_name` + native `engine` where present | Publisher incarnation → verified engine index/member → worker processes → replica/group → allocated/observed GPUs | API process versus workers; engine index versus rank/device; aggregate logical tokens versus per-worker execution |
| SGLang request metrics | Emitter + request-level model/role labels | Request publisher → scheduler/worker group using explicit engine membership | Request histogram is not rank-specific merely because scheduler metrics have rank labels |
| SGLang scheduler metrics | Emitter + model/role + supported parallel-rank labels | Scoped group/rank → worker process → selected device | Rank identity can remain valid while GPU association is unknown; optional priority totals overlap |
| Dynamo frontend | Frontend emitter + model + relevant stage/phase labels | Frontend remains the emitter; logical model/serving-topology relation resolved through registration | Frontend host hardware is not the backend hardware for TTFT |
| Dynamo worker-view gauges | Frontend emitter + native worker ID + DP rank/worker role | Scoped subject-worker identity → discovery record → target engine/process/host → device set | Keep publisher frontend and subject remote worker as distinct IDs |
| Dynamo component/backend | Hierarchy labels and family-specific model/worker facts | Runtime namespace/component/endpoint → emitting worker incarnation → verified underlying engine instance | Backend-native and framework observations retain separate producer and timing boundaries |
| Dynamo router | Router emitter + family-specific model/phase/worker dimensions | Routing observation → selected worker only when actual record provides selection identity | Aggregated router histogram without worker label cannot be partitioned into individual workers |

All existing-label facts inherit Step 1's version qualifications, including the Dynamo documentation mismatch. A model name shared across emitters is a candidate relationship, not a join key proving they are the same deployment.

## 9. Time, evidence and conflicting metadata

Every resolved or candidate relation should retain these fields. Field names are proposed Aether schema names, not mandatory metric labels.

| Field | Meaning |
|---|---|
| `subject_id`, `relation`, `object_id` | Typed association, e.g. process uses GPU, or frontend reports worker |
| `valid_from`, `valid_to` | Supported validity interval; open/unknown endpoints represented explicitly |
| `observed_at`, `last_confirmed_at` | When Aether learned/reconfirmed the relation; not automatically the time it began |
| `evidence_source`, `source_record_id` | Interface/record supplying evidence and an audit reference |
| `source_version`, `schema_version` | Versioned interpretation of the evidence |
| `resolution_state`, `resolution_reason` | Resolved/candidate/conflict/etc.; no unsupported probability score |
| `native_value`, `canonical_value` | Preserve source representation; canonical value only where rules justify it |
| `membership_generation` | Changes when replica, communicator, partition or resource membership changes |
| `measurement_scope`, `coverage` | Emitter/subject/process/rank/replica scope and complete/partial/unknown coverage |

Resolution policy:

- A new metadata observation is not backdated to process start unless evidence establishes that interval.
- A conflicting source is retained, not silently replaced by whichever update arrived last. Reported configuration and observed behavior can legitimately differ.
- A periodic snapshot supplies an observation and bounds on change time, not a precise lifecycle event. Unknown transition intervals stay uncertain.
- Historical events join to relations valid at event time, with clock-domain/uncertainty handling. Collection time is not a substitute for event time.
- A cumulative metric snapshot can include observations from earlier membership generations. Do not relabel its entire historical count with current hardware. Compute deltas only with valid scope/reset/membership handling; transition-spanning intervals remain ambiguous when they cannot be separated.
- Remote worker, endpoint, process, cgroup, GPU-partition and network-interface reuse all require lifecycle handling.
- Read only the targeted metadata required for identity. Do not ingest raw process environments, authentication strings or arbitrary command lines as metric labels.

## 10. Cardinality and storage placement

| Information | Proposed representation | Reason |
|---|---|---|
| Original native labels | Preserved on original observations | Avoid losing source semantics |
| Stable emitter/subject/group IDs | References on relevant observations | Enable joins without copying full inventories |
| GPU/NIC model, PCI topology, firmware | Versioned dimension records | Shared metadata with temporal validity |
| Replica GPU/NIC membership | Explicit membership relations | Many-to-many sets must not multiply metric values |
| Model/team/tenant aliases | Separately administered dimension records | Display and ownership meanings have different authority |
| Request, batch, iteration, communicator/expert IDs | Detailed event/correlation records where available | Avoid unbounded labels on every aggregate metric |
| Arbitrary user labels | Preserved under source-qualified names with explicit selection policy | Avoid collisions with canonical identities and uncontrolled expansion |

This is a logical data contract. Choosing a persistent metadata format, indexes or storage service is outside this step; none is implied by the table.

## 11. Worked association examples

The symbols below are illustrative relationship shapes, not facts about the user's deployment.

**Replica-wide metric:** observation O is published by API emitter E and describes verified replica R. R has verified member devices {G1, G2}. Store one O associated with R, plus membership edges R→G1 and R→G2. A GPU-model filter can find matching replicas through membership, but must not turn O into two independent measurements. If membership is partial, the result carries partial coverage.

**Dynamo remote-worker gauge:** frontend F on host HF publishes a gauge labeled with worker W. The emitter remains F/HF. Only a verified discovery and target-side join can associate W with host HW and device set GW. F's own GPU/NIC inventory is not attached as W's execution context.

**Unknown network selection:** host H contains a RoCE-capable port and an engine exposes TTFT. This supports a host inventory relationship, not a statement that TTFT used RoCE or that a collective stalled on that port. Configuration can add an intended transport; operation/runtime evidence is still required for actual-path attribution.

## 12. Validation requirements for a future implementation

These are acceptance scenarios to implement later, not tests run during this documentation step.

| Scenario | Required result |
|---|---|
| PID reused after process exits | Old measurements remain associated with the old process incarnation |
| Process moves cgroup or container task restarts | Membership has bounded validity; no historical overwrite |
| API/metrics endpoint is proxied or shared | Publisher mapping remains explicit or ambiguous; it is not assigned to an arbitrary worker |
| Same model name in separate deployments | No merge without verified deployment/replica relationship |
| Same rank/index appears on two replicas | Scope/generation prevents cross-replica join |
| Container remaps GPU ordinals | Container ordinal maps through verified device identity |
| MIG reconfiguration or MPS sharing | Partition/client limitations remain explicit |
| Dynamo reports a remote worker | Collector host and subject-worker host stay distinct |
| Replica uses multiple or heterogeneous GPUs | Single aggregate observation retains set-valued membership without duplicated counts |
| Multiple NICs, VFs or bootstrap/payload paths | Capability/topology does not become operation-use attribution |
| Resource mapping changes within a histogram/counter interval | No retrospective precise split without sufficient evidence |
| Runtime/driver/control-plane access denied | Source status shows restricted visibility; workload is not reported absent |
| Contradictory metadata or discovery gap | Conflict/unknown state survives until sufficient evidence resolves it |

## Completion and proposed next step

Completed: 58 dimension/association entries, source classifications, scoped identity keys, engine-specific joins, temporal/provenance rules, cardinality handling and future validation scenarios.

Suggested next step, not performed here: build the **metric-to-source coverage matrix** against the 250-family requirements catalog. For each requirement, classify whether it is available from an existing endpoint, derivable from existing observations, requires a new host/device/fabric collector, or requires additional application/library integration. Record the exact source, required dimensions from this document, permissions/configuration dependencies and unresolved evidence. This will establish Aether's incremental collection work before implementation or hardware-specific validation.
