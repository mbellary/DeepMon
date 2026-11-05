# Project Aether — Phase 1 Product Specification

| Document control | Value |
|---|---|
| Revision | 0.8 — approval recorded; product baseline approved at 0.7 |
| Prepared | 5 November 2026, Asia/Kolkata |
| Status | Approved by the product owner; Phase 1 complete |
| Phase | 1 — Product Definition |
| Product owner and approval authority | User; named organizational delegates may be recorded later |
| Baseline | Full-baseline delivery; all OSS and custom LLM inference engines; direct Rust measurement without inference metric endpoints, feasibility pending |
| Next gate | Phase 2 — Architecture & Feasibility preparation, verification, and approval |

## 1. Purpose and governing scope rule

Aether is a proposed telemetry and analytical platform for distributed AI inference fleets. It will help operators understand inference behavior across requests, serving engines, distributed execution, and physical infrastructure, while protecting the workloads being observed. It will support both fresh operational state for routing consumers and retained telemetry for historical investigation.

This specification defines the overall product before selecting delivery increments. All 250 metric-family requirements across four layers and all 58 metadata associations in the existing workspace remain in the product baseline. A requirement with an unknown source, difficult implementation, unavailable environment, or unverified performance remains a requirement. None of those conditions authorizes its removal.

**Rule G-01 — scope preservation, applicable to every phase:** Define and retain the overall scope before proposing a subset. No phase may silently remove, narrow, defer, replace, or declare a requirement optional. Any proposed scope change must briefly describe what changes and why, and receive explicit product-owner approval. Record the decision in the revision history; no separate audit package is required. Full-baseline delivery is the agreed approach.

Full-baseline delivery is required: all 250 metric families and 58 metadata associations remain in the delivery commitment, subject to the explicit UI, alerting, and routing-policy exclusions below. Implementation may be sequenced and experiments may exercise representative examples, but neither changes the committed scope or establishes full product completeness. No MVP or reduced release baseline is selected.

A critical product requirement is to implement measurement of all 250 metric families and resolution of all 58 metadata associations directly within Aether's Rust engine, without subscribing to or consuming inference-engine metric endpoints. This changes the earlier endpoint-consumption assumption. The requirement is recorded in PR-16; feasibility is explicitly pending evaluation under D-10. It does not assert that all required semantics are observable externally.

The six-phase process is Product Definition; Architecture & Feasibility; Implementation Planning; Implementation & Component Verification; System Validation & Release Readiness; and Deployment & Operational Acceptance. Each phase must be defined, performed, verified, and explicitly approved before the next starts. This document prepares Phase 1 only. It does not authorize architecture execution, prototyping, implementation, or deployment.

## 2. Inputs, authority, and current evidence

Use this document for product-level scope, outcomes, and approval rules after approval. Existing detailed inventories remain the definitions of their individual requirements; this document incorporates them rather than duplicating hundreds of rows.

| Existing input | Role in this specification |
|---|---|
| [Product vision](../aether.md) | Motivation, two-loop concept, candidate architecture, and aspirational performance targets. Architecture statements are not automatically approved implementation decisions. |
| [Metric catalog](aether-metrics-catalog.md) and [structured catalog](aether-metrics-catalog.json) | Complete baseline of 250 metric families, their definitions, units, scopes, aggregation rules, and limitations. |
| [Metadata matrix](metadata-enrichment-matrix.md) | All 58 associations, evidence rules, temporal validity, and ambiguity handling. |
| [Source coverage matrix](metric-to-source-coverage-matrix.md) and [source recheck](source-mapping-recheck.md) | Historical source research and unresolved semantics. Endpoint routes do not satisfy PR-16 and must be reevaluated for direct Rust measurement. |
| [Native metrics baseline](engine-native-metrics-baseline.md) | Historical semantic reference for engine measurements; neither an exclusive engine list nor approval to consume their metric endpoints. |
| [Collection contract](zero-application-change-collection-contract.md) | Historical initial collection boundary and correctness rules. Its endpoint-based profiles are superseded by PR-16 for the new product direction; other access restrictions remain until explicitly changed. |
| [Budget template](collection-acceptance-budget-template.json) | Existing structure for environment-specific limits; unset values cannot establish acceptance. |
| [Validation baseline](validation-evidence-baseline.md) and [historical prototype plan](EXECPLAN.md) | Historical context requiring evidence reconciliation before reuse. |

Current workspace inspection found documentation and structured inventories. Historical documents reference prototype implementations, tests, manifests, and evidence bundles that are not present in this checkout. Their completion statements are historical claims, not independently verified results for this product specification. Documents also describe different historical progress points. This specification does not resolve that discrepancy by assuming either implementation success or failure.

No real cluster, deployed source schemas, production compatibility envelope, runtime benchmark, or release readiness is established here. Earlier documentation records that no existing cluster was available; current access and availability must be reconfirmed before real validation. No upstream source or version claim has been revalidated as part of authoring this document.

When inputs conflict, record the conflict and seek a decision under G-01. Do not use an older plan, source limitation, or convenient architecture to override the user's scope rule. The new phase gates govern future work; historical milestone acceptance does not satisfy a new phase gate.

## 3. Users, problems, and intended outcomes

| User | Problem to solve | Successful outcome |
|---|---|---|
| Inference operations engineer | Queueing, cache pressure, latency, and throughput appear in separate systems with incompatible meanings. | Inspect the relevant measurements together with their true boundaries, freshness, and source health. |
| Performance engineer | Application symptoms cannot reliably be related to host, GPU, or fabric conditions. | Compare historical intervals using justified identity relationships and explicit uncertainty. |
| Routing or serving-platform engineer | Load balancers make decisions from stale or unavailable node state. | Consume versioned, freshness-qualified state without waiting for analytical storage. |
| Infrastructure operator | Collectors can consume inference resources or require broad privileges. | Configure finite budgets and least necessary access; stop or degrade Aether without changing serving workloads. |
| Platform or data engineer | Retention and analytics become coupled to an expensive always-on database. | Retain portable columnar data and query it with independently provisioned compute. |

The product must answer: What changed? At which observed scope? When was it measured? How fresh and complete is it? What evidence connects it to a workload or resource? What is unavailable? It must not turn correlation into an unsupported causal conclusion.

Business outcomes are reduced investigation time, more trustworthy routing inputs, controlled telemetry cost, and bounded workload impact. Baselines and numeric improvement targets remain to be agreed; this draft makes no ROI or cost-saving guarantee.

## 4. Overall and long-term coverage

### 4.1 Four-layer metric baseline

All families retain their existing canonical IDs. A family can contain many series, components, directions, or reasons; collecting one series does not establish full family coverage.

| Layer | Families | Full coverage includes |
|---|---:|---|
| L1 — Request and generation | 34 | Traffic and outcomes; request and generation latency; token accounting; service-level objectives; client-boundary measurements where supported by a qualifying source. |
| L2 — Serving engine, scheduling, and memory | 71 | Scheduling and queues; KV-cache capacity and usage; prefix-cache behavior; offload; speculative decoding; runtime memory; response cache; optional engine state; derived execution measures; runtime failures. |
| L3 — Distributed execution and network fabric | 58 | Collectives; pipeline behavior; mixture-of-experts execution; KV transfer; InfiniBand; Ethernet/RoCE; TCP transport; external switch telemetry. |
| L4 — Host, GPU, and infrastructure | 87 | GPU activity, memory, power, thermal behavior, reliability, interconnects, device tracing and topology; host CPU, scheduling, pressure, memory, storage, networking, lifecycle, and derived infrastructure measures. |
| **Total** | **250** | Complete current metric-family baseline; no families removed or assigned to an unapproved later release. |

These are the current finite requirements, not a claim to enumerate every future vendor field. Additional engine or hardware capabilities may expand the catalog through versioned product decisions. Existing research classifies routes as endpoint, derived, host/fabric, instrumentation/integration, or unresolved. Route counts are not a supported-coverage percentage.

### 4.2 Identity and contextual coverage

Preserve all M01–M58 associations from the metadata matrix, including workload/software, host/container, accelerator/topology, and NIC/fabric/software-stack relationships. Preserve the distinction between the publisher of a metric, its measured subject, allocated resources, and resources observed participating in execution.

Every association must retain evidence, provenance, validity interval, and resolution status. A distributed replica may refer to a set of GPUs; a replica-wide metric must not become one independent observation per GPU. Configuration establishes intent, not proof of actual execution. Ambiguous or conflicting identity remains explicit and queryable.

### 4.3 Engine, hardware, and environment coverage

The overall product concerns distributed LLM inference on Linux hosts with containerized serving and NVIDIA-class accelerators, consistent with existing requirements. H200 is a motivating example, not a selected exclusive hardware profile. Exact GPU, NIC, fabric, kernel, driver, runtime, orchestration, and serving-engine versions remain unspecified.

All open-source (OSS) LLM inference engines and custom engines are in product scope. Existing research into vLLM, SGLang, Dynamo, TGI, Triton, and TensorRT-LLM supplies examples, not a binding engine selection. llm-d is explicitly included as a core inference stack by product-owner direction. No specific engine, native naming scheme, endpoint, scheduler model, or runtime assumption may define the product boundary. Historical engine-specific deferrals, including DeepInfer, do not exclude an engine from this overall scope. Cache integrations, distributed libraries, host interfaces, and external fabric sources remain part of coverage where required by the catalog.

Engine-independent coverage requires extensible collection and semantic mapping for native, integrated, and custom sources. Each measurement retains its actual meaning; an engine without a matching field must show a coverage gap rather than a fabricated equivalent. Real compatibility is demonstrated per source/version/environment, separately from overall product scope. Representative qualification profiles must not become an exclusive engine list or an implicit scope reduction. No universal already-tested compatibility claim is made. Expansion beyond the existing Linux/NVIDIA hardware baseline requires a product decision rather than an inferred commitment.

#### 4.3.1 Inference engines and serving stacks in scope

This table names the engines and stacks currently identified for the overall product. It is an open inventory, not an exclusive support list, release subset, implementation claim, or compatibility certification. All rows remain subject to PR-16: direct Rust measurement and metadata resolution without consuming inference metric endpoints. Native metrics documented in the catalog are semantic references for evaluation, not the acquisition design.

| Engine or serving stack | Product-scope role | Basis and evaluation considerations |
|---|---|---|
| vLLM | Core LLM inference engine in scope. | Named throughout the [metric catalog](aether-metrics-catalog.md), including request/generation, scheduler, KV-cache, and transfer semantics. Evaluate direct measurement of these boundaries. |
| SGLang | Core LLM inference engine in scope. | Named in the catalog for request latency, token accounting, cache, speculation, and estimated execution measures. Preserve its distinct measurement meanings. |
| NVIDIA Dynamo | Core distributed inference framework/serving stack in scope. | Included in the [native metrics baseline](engine-native-metrics-baseline.md). Separate framework/component observations from the underlying inference engine; do not double-count the same work. |
| llm-d | Core distributed inference framework/serving stack in scope, explicitly requested by the product owner. | The [official project](https://github.com/llm-d/llm-d) describes a Kubernetes-native distributed inference stack. Evaluate stack-level and underlying model-server boundaries separately; llm-d inclusion does not bind Aether to one backend or authorize Aether to own routing policy. Direct-measurement feasibility is pending. |
| Hugging Face Text Generation Inference (TGI) | LLM serving engine in scope. | Named in the catalog for request, validation, token, queue, and batching measurements. Existing request-counting ambiguities remain evaluation items rather than assumed equivalents. |
| NVIDIA Triton Inference Server | Inference serving platform in scope for LLM deployments. | Named in the catalog for request/inference/backend counts, cumulative timing, response cache, and server memory pools. Preserve server/model/backend measurement boundaries. |
| NVIDIA TensorRT-LLM | LLM inference runtime/engine in scope, including native and Triton-backed deployment forms. | Named in the catalog for iteration, scheduling, KV-block, speculation, and memory semantics. Native runtime and Triton backend evidence must remain distinguishable. |
| DeepInfer (historically named) | Retained in the open engine inventory; no historical deferral excludes it from overall scope. | Mentioned in existing product/research context. Exact project identity, interfaces, and applicable semantics still require confirmation; no integration or qualification is claimed. |
| Other OSS LLM inference engines | All are in scope, including engines not yet individually listed. | Add named entries as discovered without treating the current research list as a restriction. Assess semantic applicability and direct observability without narrowing the full baseline. |
| Custom LLM inference engines | All custom engines are in scope. | Evaluate custom execution, scheduling, cache, and identity boundaries under the same product requirements; do not assume a standard endpoint, framework, event schema, or instrumentation facility. |

Engine/stack distinctions describe measurement boundaries, not different scope priority. Supporting a framework does not automatically establish support for every backend/version combination, and evaluating a backend alone does not establish framework-level coverage. Required observations, associations, and unresolved gaps must be explicit for each tested combination. Cache libraries, communication libraries, GPU telemetry APIs, and storage/query components remain dependencies or observation sources rather than being mislabeled as inference engines.

### 4.4 Collection coverage and the application-change boundary

The product now requires direct Rust measurement and association resolution rather than consumption of inference-engine metric endpoints. Candidate inputs for evaluation include host accounting, device APIs, network counters, raw events/probes, permitted library/device instrumentation, topology/identity evidence, and external fabric observations where required. These are candidate mechanisms, not proven substitutes for engine-internal semantics.

The historical collection contract defined C01–C10, including endpoint-based C01–C06. Those endpoint profiles no longer establish an acceptable acquisition path for PR-16. Non-endpoint C07–C10 remain candidate interfaces, not complete coverage. Existing restrictions on probe attachment, application changes, privileges, and content access remain in force pending explicit approval; the new measurement direction does not itself authorize instrumentation or expanded access.

Zero application change means no change to serving code, images, libraries, launch arguments, environment variables, deployment manifests, model pipelines, configuration, or workload lifecycle. Aether installation and access configuration are separate operational prerequisites.

Some long-term measurements may need instrumentation that conflicts with this boundary. They remain required investigations, with the conflict visible. Phase 2 must identify whether an acceptable source exists. If it does not, the product owner must decide whether to authorize a separately labeled integration mode, revise a requirement, or retain it as unresolved. No one may silently claim coverage, enable instrumentation, or weaken the zero-change promise to close the gap.

### 4.5 Product capability coverage beyond metrics

The overall product also includes source configuration and qualification, both telemetry paths, bounded buffering and recovery, analytical retention and queries, consumer integration, self-observation, security controls, installation and upgrades, and operational support. These are specified below. The four-layer inventory alone is not a complete product definition.

### 4.6 🗺️ High-Level Product Architecture

The following diagram is reproduced unchanged from the [product vision](../aether.md#🗺️-high-level-product-architecture). It shows the proposed separation between reactive routing and asynchronous analytical telemetry, together with the host, routing plane, object storage, and query responsibilities.

```text
                          ┌────────────────────────────────────────────────────────┐
                          │            BARE-METAL HOST COMPUTE NODE                │
                          │                                                        │
                          │        🚀 aether_agent Hardware Ingress Daemon         │
                          │      (100ms Kernel eBPF & libnvidia-dcgm Loops)        │
                          └───────────────┬────────────────────────┬───────────────┘
                                          │                        │
         [Observability Path]             │                        │ [Reactive Routing Path]
         Raw Telemetry Structs            │                        │ Zero-Copy 24-Byte Casting Buffer
                                          ▼                        ▼
     ┌───────────────────────────────────────────┐      ┌───────────────────────────────────────────┐
     │     🧠 aether_common (Library)            │      │      ⚡ aether_router (Edge Utility)      │
     ├───────────────────────────────────────────┤      ├───────────────────────────────────────────┤
     │ ▪ Lock-free MPMC ring-buffer ingestion.   │      │ ▪ Immediate bitfield packing layer.       │
     │ ▪ Columnar Apache Arrow matrix staging.   │      │ ▪ Non-blocking UDP/QUIC network ship.     │
     │ ▪ Background async Parquet serialization. │      │ ▪ Direct wire stream to load balancers.   │
     └────────────────────┬──────────────────────┘      └────────────────────┬──────────────────────┘
                          │                                                  │
                          ▼ (Asynchronous S3 Offload)                        ▼ (<100ms Cluster Broadcast)
             ┌───────────────────────────┐                      ┌───────────────────────────┐
             │ Immutable Lakehouse       │                      │ Replicated Routing Plane  │
             │ (S3/MinIO Parquet Assets) │                      │ (Valkey / Local DashMaps) │
             └────────────┬──────────────┘                      └───────────────────────────┘
                          │
                          ▼
             ┌───────────────────────────┐
             │ aether_query analytical   │
             │ (On-Demand DataFusion SQL)│
             └───────────────────────────┘
```

This is the product vision's high-level architecture, pending Phase 2 feasibility review and approval. Its named technologies, 100 ms collection loops, 24-byte messages, zero-copy claims, and sub-100 ms broadcasts remain proposed design details or targets. The diagram does not replace the full coverage baseline in Sections 4.1–4.5 or resolve the collection-boundary conflicts in Section 4.4 and D-02. Detailed interfaces, topology, access, and performance budgets remain Phase 2 work under G-01.

### 4.7 Architecture groups

The following group definitions, core components, and testing isolation boundaries are reproduced from [aether.md](../aether.md#architecure-groups), with heading levels adjusted for this specification. The group identifiers used below are **G1** (`aether_agent`), **G2** (`aether_common`), **G3** (`aether_router`), and **G4** (`aether_query`). These are architecture groups, distinct from the four observability layers: a layer describes what is measured; a group describes responsibility for processing and exposing it.

To ensure Project Aether can be engineered with absolute predictability, the unbundled database and telemetry architecture must be separated into four decoupled architectural groups.
Each group acts as an independent system tier with a cleanly defined, mockable boundary interface, allowing you to implement and comprehensively test them in total isolation.

#### 4.7.1 Group 1: The Local Data Ingress & Driver Plane (aether_agent)

This group is responsible for the raw, high-frequency (100ms) non-blocking data extraction from local physical and isolated virtual boundaries.
- Core Components:
  - eBPF System Probes: Pure-Rust kernel space filters (Aya) intercepting sys_write/sys_sendto to measure userspace network-stream latency (TTFT/ITL).
  - Hardware C-FFI Wrappers: Non-blocking unsafe bindings to libnvidia-dcgm reading raw H200 HBM3e and tensor pipe telemetry.
  - cgroup Topology Scanner: Async runtime interface mapping local process namespaces (/sys/fs/cgroup) to container metadata.
- Testing Isolation Boundary: Mock out system calls and the local container API via trait primitives to assert that collection loops handle a simulated firehose safely without blocking.

#### 4.7.2 Group 2: The High-Performance Memory Pipeline (aether_common)

This group owns the zero-copy orchestration layer, turning fast telemetry signals into compressed, structurally rigid database files.
- Core Components:
  - Lock-Free Queue Plane: A multi-threaded crossbeam::queue::ArrayQueue MPMC ring buffer providing nanosecond collection decoupling.
  - In-Memory Arrow Stage: Columnar builders and matrix compaction loops implementing std::mem::take for memory optimization.
  - Local Parquet Compactors: Double-buffered asynchronous workers converting RecordBatch streams into highly dense .parquet blocks.
- Testing Isolation Boundary: Expose a simple data injection stream function. You can run multi-threaded fuzz testing and memory sanitizers to guarantee zero allocations, locks, or resource leaks under peak-load stress without having any live drivers installed.

#### 4.7.3 Group 3: The Sub-Second Telemetry Plane (aether_router)

This group processes the sub-second reactive state loop that tells your replicated layer of load balancers exactly where to route active inference requests.
- Core Components:
  - Binary Packing Layer: Fixed-width 24-byte bitfield layout translation logic ensuring heap-free byte casting.
  - State Cluster Bridge: A stateless, parallel client client utilizing Valkey glide-core or fred streams and short-TTL hashes.
  - Load Balancer Mirror: Local read-only cluster caches sitting inside concurrent, lock-free structures like dashmap.
- Testing Isolation Boundary: Abstract Valkey behind a local Docker container or mock socket provider. You can assert that the load balancer performs nanosecond O(1) local table updates and context-routing evaluations using purely pre-baked mock payload streams.

#### 4.7.4 Group 4: The Stateless Query Lakehouse (aether_query)

This group acts as the on-demand analytical compute tier, unlocking sub-second SQL transformations straight over cheap cloud objects.
- Core Components:
  - Stateless DataFusion Runtime: Embedded configurations of SessionContext optimized for SIMD execution on bare-metal CPUs.
  - Provider Object Storage Client: Dynamically registered table connectors built over the agnostic object_store library.
  - Predicate Pushdown Evaluator: File layout sorting configurations designed to scan only required block partitions.
- Testing Isolation Boundary: Completely testable locally by point-mounting DataFusion to a local directory of pre-generated Parquet logs, bypassing MinIO or remote infrastructure setups entirely during verification.

#### 4.7.5 Placement, handoffs, and interpretation

As described in the source document's bare-metal engine section, the host engine contains G1 and G2 plus G3's outbound edge worker. G3 also includes the shared state bridge and load-balancer mirrors outside that host worker. G4 provides the analytical query tier and object-storage integration. These logical groups are not four mandatory standalone services or four identical deployment units.

G1 supplies observations and source/identity evidence. G2 buffers and prepares analytical records. G3 packs and distributes reactive state to consumers. G4 exposes retained data for analytical queries. Acquisition, preservation, and interpretation of metadata span these handoffs; no group may discard the evidence or uncertainty required by the product baseline.

The quoted source descriptions include proposed technologies and performance assertions. Their inclusion preserves the architecture definitions; it does not verify zero allocations, zero-copy behavior, lock freedom, nanosecond execution, measurement accuracy, or any timing guarantee. Probe activation remains subject to Section 4.4. Phase 2 must reconcile those assertions with the functional and nonfunctional requirements and the approved collection boundary.

Upload responsibility is intentionally shared between G2's local persistence/output boundary and G4's provider-object-storage integration until D-09 resolves the execution location, acknowledgment boundary, and retry ownership. G3's state distribution must similarly define a real broadcast/consumer contract. The mappings below assign product responsibilities without silently settling those design questions or reducing any of the 250/58 requirements.

### 4.8 Critical requirement: direct Rust measurement and metadata resolution

Aether must implement the full 250-family measurement baseline and all 58 metadata associations within its Rust engine. Reading precomputed inference metrics from an engine's HTTP/Prometheus endpoint, subscription, or equivalent exported metric feed does not satisfy this requirement. Relaying the same precomputed metrics through an exporter, proxy, or intermediate store does not make them direct measurements.

Direct implementation means that Aether acquires the underlying permitted observations, implements measurement boundaries, counters, timings, distributions, and calculations, and resolves metadata from supporting evidence. Host/kernel/device interfaces can supply raw observations or device counters; the requirement does not mean replacing operating-system or hardware measurement facilities. Concrete source eligibility and any source-owned aggregation must be justified during evaluation. A Rust client that merely reads an inference metric is insufficient.

The existing catalog's measurement meanings, required dimensions, and scope remain the baseline. Published native names are historical semantic references, not required runtime inputs. Aether must not label socket timing as token timing, hardware memory use as KV allocation, or an externally estimated queue as an engine's actual queue without proving equivalence. Measurements dependent on internal scheduler, token, cache, collective, or request-lifecycle information require explicit observability analysis. Their implementability is not assumed.

All OSS and custom inference engines remain in scope. Engine-specific instrumentation or knowledge may be necessary to obtain equivalent semantics; any such dependency must be disclosed and evaluated rather than becoming a binding single-engine architecture. Zero application change, bounded overhead, prohibited-content rules, and minimum-access constraints remain concurrent requirements. Where they conflict, D-02 and D-10 require a product decision before design approval. No endpoint fallback, semantic weakening, silent omission, or reduced release baseline is authorized.

Phase 2 must evaluate the entire inventory in the existing Technical Design, with a compact per-ID mapping for all 250 families and all 58 associations: required meaning/scope, proposed raw evidence, Rust measurement/resolution method, permissions/integration needs, semantic limitations, validation method, performance implications, and feasibility status. Reuse and annotate the existing inventories rather than creating another standalone specification. Status must distinguish proposed, demonstrated under named conditions, unresolved, and conflicting with another requirement. A representative experiment does not close untested IDs.

The evaluation must identify externally observable measurements, those needing internal semantic events, and those lacking a justified direct route. Use independent reference truth and defined error bounds to test correctness and attribution. Explicitly evaluate all four architecture groups, the combined host CPU/memory envelope, source-to-consumer latency, cardinality, privacy, and loss/recovery implications. An unresolved route remains a visible gap in full-baseline delivery. Changing the measurement objective or authorizing a special integration requires product-owner agreement, summarized in the revision history.

## 5. Product workflows

1. **Establish collection readiness.** An operator selects explicit targets, fields, identities, credentials, destinations, and finite budgets. Aether validates configuration and reports available, unsupported, denied, and unknown capabilities. Invalid required configuration prevents the affected operation from starting.
2. **Observe an inference problem.** An engineer retrieves queue/cache/latency and infrastructure observations for an interval. Results retain measurement scope, timestamps, association evidence, and gaps, allowing comparison without fabricating missing relationships.
3. **Consume reactive state.** A routing consumer receives node-state updates independently of analytical uploads. It can identify missing fields, expired state, incompatible versions, source restarts, and update ordering. Routing policy remains the consumer's responsibility unless separately approved.
4. **Investigate history.** An engineer queries a defined retained dataset by time, source, workload, host, or resource. Results show completeness and persistence/visibility status and avoid duplicate logical observations after replay.
5. **Operate through failure.** During source disappearance, storage outage, queue exhaustion, or restart, Aether stays within its configured envelope, reports degradation and known loss, and recovers within its declared durability boundary.
6. **Manage lifecycle.** An operator installs, configures, upgrades, rolls back, and removes Aether through documented procedures. These operations do not silently reconfigure inference workloads or delete retained telemetry.

## 6. Functional product requirements

All requirements below are baseline requirements. Verification methods describe future evidence, not tests performed during document preparation. Detailed L/M definitions remain normative within their inventories. The group column maps every requirement to the Section 4.7 groups. “Leads” identifies the principal functional responsibility, not sole accountability; each contributing group must preserve the requirement at its own boundary. Cross-group acceptance remains an end-to-end obligation.

| ID | Required product behavior | Acceptance evidence | Architecture group responsibility |
|---|---|---|---|
| PR-01 | Maintain every L1–L4 family and M01–M58 association with separate requirement, implementation, qualification, and release-disposition states. | Inventory reconciliation finds no missing/duplicate IDs; every release claim maps to actual evidence and approved scope. | G1–G4: shared coverage traceability; G1 source qualification, G2 transformation/persistence, G3 reactive support, G4 query support. Product owner governs release disposition; no runtime group approves scope. |
| PR-02 | Collect from explicitly configured eligible sources and expose capabilities and access failures without changing workloads. | Valid source cases collect expected data; denied/disabled/unsupported cases report the correct state and perform no prohibited mutation. | G1 leads source collection, capability discovery, and access reporting; G2/G3 preserve collection status downstream; G4 exposes retained qualification/quality evidence. |
| PR-03 | Preserve producer, native name/labels, type, units, measured scope, source incarnation, observation times, and quality state. | Known-input comparisons preserve exact integer counts and declared numeric tolerances; required provenance survives query/export. | G1 captures native semantics and provenance; G2 preserves them in analytical records; G3 retains the identity, time, and quality needed by its state contract; G4 preserves them in query/export results. |
| PR-04 | Calculate catalog-derived metrics only from compatible inputs and windows; handle reset, wrap, missing denominators, and identity changes. | Independent expected answers match; invalid or absent inputs produce explicit unavailable/invalid results rather than plausible values. | G2 and G4 apply validated analytical calculations at their respective processing/query boundaries; G1 supplies reset/incarnation and input evidence; G3 applies the same rules to any derived reactive field. |
| PR-05 | Resolve metadata only with supporting evidence and temporal validity, retaining ambiguity and conflicts. | All association cases satisfy their evidence rules; reused IDs and historical joins do not acquire incorrect identities. | G1 obtains identity evidence; G2 preserves associations and validity; G3 uses only justified reactive subject identity; G4 applies temporally valid analytical joins and exposes unresolved states. |
| PR-06 | Deliver a freshness-qualified reactive state path independent of analytical storage availability. | With analytical storage unavailable, valid routing updates remain within the approved reactive envelope; stale/missing states are visible. | G3 leads independent reactive delivery; G1 supplies fresh observations; G2/G4 analytical backpressure or outages must not block that path. |
| PR-07 | Define a consumer contract with subject identity, schema/version, observation age, validity, restart and ordering semantics. | Duplicate, delayed, reordered, incompatible, and expired updates produce defined outcomes in every supported consumer configuration. | G3 leads state schema, ordering, expiry, and consumer behavior; G1 supplies source identity/timing/restart evidence; G2/G4 retain consistent subject/version meanings where data is shared. |
| PR-08 | Retain analytical telemetry in portable columnar objects with independent query compute and no mandatory central operational time-series database. | Stored data can be read by the supported query interface; remote upload interruption has defined local behavior. Arrow/Parquet/object storage/DataFusion remain design candidates until Phase 2 approval. | G2 leads columnar preparation and local output; G4 owns analytical storage/query integration. Shared upload/persistence boundary remains D-09; G1 supplies records and G3 remains independent. |
| PR-09 | Query by supported dimensions and time, preserving declared aggregation semantics, quality states, and dataset visibility boundaries. | Reference queries match independent truth, including restarts, gaps, conflicting metadata, and retries. Query errors and incomplete results are explicit. | G4 leads query execution and result semantics; G2 supplies correctly structured/committed data; G1 supplies provenance; G3 provides reactive-history/status inputs only where retained under the approved contract. |
| PR-10 | Bound memory, disk, processing work, concurrency, cardinality, requests, and network traffic. | At/beyond-limit scenarios remain within the approved envelope or stop safely with observable rejection/degradation; no uncontrolled growth. | G1–G4 each enforce their local resource/admission limits; G1/G2/G3 edge work also meets the combined host envelope, and all handoffs have bounded backlog behavior. |
| PR-11 | Declare persistence acknowledgments, replay identity, recovery, and overflow policy; account for known pending, rejected, lost, committed, and query-visible observations. | Crash/outage/retry tests reconcile unique observations at named boundaries; durable acknowledgment is never issued before its persistence condition. | G2 leads local persistence, overflow, and replay accounting; G4 leads remote commit/query-visibility accounting; G1 provides observation identity; G3 defines separate transient-state retry/expiry semantics without implying durable delivery. |
| PR-12 | Expose Aether health: collection failures, age/lag, queue/spool use, loss/rejections, upload/query status, and resource consumption. | Injected faults produce the correct externally observable health state even if an optional collector fails. | G1–G4 expose their own health and stage counts; G1 reports source failures, G2 queue/spool and loss, G3 routing lag/expiry, and G4 upload/query visibility and failures. |
| PR-13 | Support validated configuration and explicit enable/disable controls with effective configuration revision and secret-safe diagnostics. | Invalid settings fail clearly; configuration changes are traceable; disabling Aether work leaves the workload unchanged. | G1–G4 validate and report configuration at their boundaries, isolate disabled/failing work, and preserve effective revision; host controls cover G1/G2/G3 edge work. |
| PR-14 | Provide installation, upgrade, rollback, shutdown, retention, and removal behavior appropriate to each supported deployment profile. | Representative lifecycle checks preserve declared compatibility and data guarantees; recovery/removal behavior is documented and verified. | G1–G4 participate in version-compatible lifecycle procedures for their deployment units; G2/G4 preserve declared data/retention boundaries and G3 handles mixed-version/restarted consumers. |
| PR-15 | Protect credentials and telemetry access, restrict collection targets, and reject prohibited content. | Unauthorized access is denied; configured sensitive fields are excluded; credentials and workload content do not leak through retained data or diagnostics. | G1 limits source access and filters prohibited content before retention/transmission; G2 protects local data; G3 secures state publication/consumption; G4 controls storage/query access and credentials. |
| PR-16 | Critical: implement all 250 metric families and all 58 metadata associations directly in the Rust engine without subscribing to or consuming inference-engine metric endpoints or equivalent precomputed metric feeds. Feasibility must be evaluated before the design is approved. | Phase 2 evaluates every L/M ID with a justified direct observation/resolution route or an explicit unresolved conflict; evaluation alone is not implementation acceptance. Product acceptance requires demonstrated semantic correctness, association validity, and endpoint-independent operation within approved access and resource budgets for the complete baseline. | G1 leads direct evidence acquisition, event measurement, and identity discovery; G2 implements/preserves required aggregation and analytical records; G3 consumes Aether-measured reactive fields; G4 exposes correct analytical calculations and metadata joins. G1–G4 jointly evaluate observability, semantics, overhead, and coverage; no group may substitute engine metric ingestion. |

PR-16 governs the acquisition interpretation of PR-02–PR-05 and the historical source definitions: preserve native evidence when available, and otherwise retain Aether measurement names, methods, boundaries, provenance, and uncertainty. Existing endpoint mappings are research inputs to reassess, not fulfillment of the direct-measurement requirement.

The routing consumer interface and analytical query access are required product surfaces. User-facing UI, dashboards, alerting (including alert rules, evaluation, and notification delivery), and routing-policy ownership are out of scope by product-owner decision D-03. Aether supplies telemetry, health/status, and freshness-qualified state through programmatic interfaces; external systems own presentation, alerting, destination selection, and routing decisions. G3's state packing, distribution, and load-balancer mirror integration remain in scope. These exclusions apply throughout this specification, including any alert/SLO or routing-policy wording inherited from source catalogs or the reproduced architecture descriptions; the underlying telemetry measurements remain in scope.

## 7. Correctness, freshness, and failure semantics

Missing, unsupported, denied, invalid, stale, and healthy-zero data are different outcomes. Source timestamps, collection timestamps, and query visibility times must remain distinguishable. If a source has no trustworthy update time, Aether must expose that uncertainty rather than equating the latest poll with a new measurement.

Preserve counter populations and resets. Do not infer request events from aggregate histograms, derive percentiles from cumulative sums, average node percentiles, add overlapping parent/child counters, or equate GPU framebuffer usage with KV-cache occupancy. Raw collection and semantic support are separate claims.

An observation accepted into memory is not necessarily durable. Persistence guarantees must specify process-crash and host/power-loss boundaries separately. Retries preserve logical observation identity; separate observations with equal values remain separate. A bounded buffer cannot guarantee unlimited lossless outage survival. Each supported configuration must declare a finite outage envelope and what happens when capacity is exhausted.

A failed collector must not trigger workload restarts, source feature activation, privilege expansion, or host/device changes. Optional source failure must be isolated from unaffected collection. Reactive consumers must receive an explicit stale/unavailable outcome rather than indefinitely treating last-known values as current. Aether does not own routing policy or make inference destination selections; consumers own those decisions.

## 8. Nonfunctional requirements and measurement targets

The group column identifies contributors to each measurement boundary. Shared targets apply to the combined relevant deployment footprint; they are not separate allowances for each group. Remote analytical services have their own budgets, while any Aether work co-located with inference is included in host-impact accounting.

The product aims for very low host overhead and predictable resource use. Rust is the intended implementation language from the existing baseline. Lock-free queues, thread pinning, fixed-width casting, and specific libraries are proposed implementation choices to evaluate in Phase 2, not product-level proof of latency or safety.

| ID | Requirement or target | Measurement and approval treatment | Architecture group responsibility |
|---|---|---|---|
| NFR-01 | Protect inference performance. | Compare baseline and Aether-enabled runs using the same named workload; report throughput and latency percentiles, uncertainty, repetitions, and per-host outliers. Numeric allowable degradation requires approval before acceptance testing. | G1/G2/G3 edge work share host-impact responsibility; G3 remote and G4 contribute endpoint/network/storage contention where their operation affects the workload. |
| NFR-02 | Original host CPU target: less than 0.5% of total host CPU capacity. | Measure all Aether host processes/threads and attributable collection overhead against an explicit CPU-capacity denominator; also report core equivalents and peaks. A fleet mean cannot hide overloaded hosts. Feasibility remains unproven. | Combined G1 + G2 + G3 outbound edge worker, including attributable collection overhead; include G4 or other Aether work if co-located on the inference host. |
| NFR-03 | Original host memory target: less than 64 MB under peak load. | Retain the original target; Phase 2 must define MB units, RSS/other accounted memory, library/worker scope, cache treatment, sampling, startup/steady/burst conditions, and upper bound. Any changed target requires G-01 approval. | Combined G1 + G2 + G3 outbound edge worker and their host-resident dependencies; any co-located G4/Aether work is included rather than excluded by group name. |
| NFR-04 | Original reactive objective: under 100 ms from a relevant source change to routing-state update. | Budget source update/detection, collection, processing, transport, and consumer application separately. Define percentile versus hard maximum, clocks, uncertainty, and exception conditions before acceptance. A 100 ms polling interval alone cannot satisfy this complete budget. | G1 + G3 end to end: source detection through consumer application; G2/G4 must not introduce analytical dependencies into this path. |
| NFR-05 | Original analytical efficiency objective: at least 10× compression against text/JSON logging. | Fix semantically equivalent inputs, encoding, metadata overhead, dataset size, and comparison formula before evaluation. Report measured ratio; do not generalize to arbitrary data. | G2 leads encoding/compression; G4 validates retained-object and query-readable equivalence; G1 supplies comparable raw inputs and metadata. |
| NFR-06 | Original sub-second analytical query objective. | Define query set, data volume/age, selectivity, concurrency, compute allocation, cold/warm conditions, result completeness, and percentile. Query latency and data freshness are separate budgets. | G4 leads query latency/concurrency; G2 data layout and G4 storage access jointly affect performance and completeness. |
| NFR-07 | Finite scale and recovery envelopes. | Qualify host/cluster count, source count, observations/s, series/cardinality, bursts, retention, outage duration, replay time, and concurrent consumers/queries. Required numeric profiles remain to be established. | G1–G4: source/target load, bounded buffering, consumer fan-out, retained-data/query scale, and recovery at every stage. |
| NFR-08 | Explicit collection and analytical freshness. | Qualify source cadence, observation age, poll delays, upload/commit lag, and query visibility separately; unknown source freshness cannot pass a measurement-to-consumer claim. | G1 source/collection age; G3 reactive state age; G2 batch/local persistence lag; G4 upload/commit/query visibility. Shared timestamps and uncertainty must survive handoffs. |
| NFR-09 | Secure, compatible operation. | Qualify minimum access, credential handling, access isolation, transport/storage protection, version compatibility, and patch/update behavior per supported environment. | G1–G4: minimum access, protected data/credentials, interface compatibility, isolation, and lifecycle controls at each boundary. |

These are retained targets and required measurement definitions, not accepted guarantees. The vision's broader statements such as zero interference, zero scale cost, infinite observability, or universal zero allocation must not be published as verified capabilities. Phase 2 must identify where allocation/copying is prohibited, merely bounded, or unavoidable and provide evidence. Failure to meet a retained target triggers a product decision, not a silent threshold adjustment.

PR-16 requires reevaluating NFR-01–NFR-09 for direct measurement rather than endpoint polling: instrumentation and semantic reconstruction must be included in host overhead, memory, timing accuracy, compatibility, and access measurements. Prior endpoint-based estimates or prototype results cannot establish these budgets.

Every acceptance budget needs a named owner, finite limits, workload/environment, metric/formula, aggregation, measurement window, reference truth, uncertainty, and evidence. An unset value is not unlimited, not automatically inapplicable, and not a pass. Product targets must be approved before evaluation; exploratory calibration requires finite safety limits and cannot retroactively qualify a changed threshold.

## 9. Security, data governance, and operational boundaries

Aether collects telemetry and justified identity context. It must not collect prompts, responses, tensor or KV contents, credentials, or process environments. Sensitive labels require explicit allowlists/filtering before retention. Diagnostic data is subject to the same controls.

Targets, methods, paths, fields, and devices must be configured or discovered through an explicitly approved discovery mechanism. No blanket root access, privileged container, runtime control socket, kernel capability, network scan, or workload namespace access is assumed. Probe attachment and instrumentation need their own approved access and impact contracts before activation.

D-07 is out of scope by product-owner direction: this product specification does not define an organizational access model, data-ownership policy, organizational/tenant isolation policy, retention/deletion governance, or compliance program. These are not pending Phase 1 decisions or deferred Phase 2 deliverables. Existing technical requirements for least-necessary collection access, credential/content protection, bounded storage, and safe recovery remain as specified in PR-10–PR-15 and NFR-07–NFR-09; technical storage limits do not establish an organizational retention policy.

## 10. Verification and evidence model

Track coverage on separate axes: requirement retained; source route researched; implemented; synthetic behavior verified; real environment qualified; and approved for a named release. A fixture pass cannot become a real-support claim. A metric family with missing required dimensions is partial even if a native value was read successfully. Applicable and inapplicable conditions must be justified, not used to hide missing coverage.

| Scenario group | Required observable result | Requirements traced |
|---|---|---|
| Nominal collection and history | Known records retain values, types, times, identity evidence, and expected query results. | PR-02–PR-05, PR-08–PR-09 |
| Missing or invalid sources | Disabled, absent, malformed, oversized, denied, and timed-out inputs produce explicit states with bounded work. | PR-02–PR-03, PR-10, PR-12–PR-13 |
| Identity and semantic boundaries | Counter resets, PID/source reuse, scope conflicts, ambiguous joins, and overlapping populations do not corrupt logical results. | PR-03–PR-05, PR-09 |
| Reactive state faults | Duplicate, delayed, reordered, expired, and restarted producers cannot silently appear fresh; analytical outage does not block the reactive path. | PR-06–PR-07, NFR-04, NFR-08 |
| Capacity and persistence faults | Queue/disk limits, storage interruptions, crash/restart, retries, and replay follow declared policies with reconciled counts. | PR-10–PR-12, NFR-07 |
| Workload impact and scale | Real baseline/enabled comparisons meet approved per-profile limits without hiding adverse hosts or conditions. | NFR-01–NFR-08 |
| Security and lifecycle | Denied access, invalid credentials, sensitive fields, version changes, upgrade and rollback behave as specified. | PR-13–PR-15, NFR-09 |
| Direct measurement and endpoint independence | With inference metric endpoints unavailable and no equivalent metric feed supplied, Aether produces measurements and associations matching independent truth under the declared integration conditions; unresolved IDs prevent full-baseline acceptance. | PR-16, PR-02–PR-05, NFR-01–NFR-09 |
| Coverage and scope check | All 250/58 IDs remain visible; claims and release dispositions match evidence and explicit approvals. | G-01, PR-01 |

The Release Record will eventually hold the evidence summary; raw evidence can be linked rather than copied into new documents. No runtime tests or product acceptance results are claimed by this draft.

## 11. Decisions and risks requiring disposition

This table keeps open decisions and resolved product directions together. D-01, D-03, D-04, and the D-07 exclusion reflect the user's explicit decisions; the remaining recommendations are proposals. Changes are summarized in the revision history without a separate audit process.

| ID | Decision / risk | Disposition and owner | Status / deadline / effect |
|---|---|---|---|
| D-01 | Overall baseline versus release scope | Product owner requires full-baseline delivery, retaining all 250 metric families and 58 metadata associations, with the explicit D-03 product-surface exclusions. No reduced MVP or release subset is authorized. | Resolved by user direction; governs every phase. Work may be sequenced without reducing the committed scope. |
| D-02 | Direct/fine-grained measurement versus zero application change | Retain both objectives. Technical lead evaluates whether raw observations can establish every required semantic boundary and association without changing workloads. Product owner approves any exception or requirement change. | Evaluate together with PR-16/D-10 in Phase 2; resolve blocking conflicts before approving its design. |
| D-03 | User-facing UI, alerting, and routing-policy ownership | Product owner excludes all three. Programmatic telemetry/query access, self-health reporting, reactive state distribution, and consumer mirror integration remain in scope; external consumers own UI, alerting, and routing policy. | Resolved by user direction; applies to all groups and phases, including inherited catalog/diagram wording. |
| D-04 | Inference-engine scope and qualification | Product owner includes all OSS LLM inference engines and custom engines; no specific inference-engine assumption is binding. Technical design must accommodate different source interfaces and semantics. | Engine scope resolved by user direction. Phase 2 still defines representative version/environment qualification profiles and access; these cannot narrow engine scope. |
| D-05 | Numeric resource, latency, scale, retention, and recovery criteria | Product owner retains original targets; technical lead proposes complete measurement profiles and feasibility evidence. Infrastructure owner agrees real-run safety limits. | Phase 2 exit requires approved finite acceptance profiles or an approved product revision; implementation must not invent them. |
| D-06 | Historical evidence absent or inconsistent here | Technical lead recovers and verifies original artifacts or labels them unavailable; do not regenerate historical evidence as if original. | Before any historical result is used to justify a Phase 2 decision or later gate. |
| D-07 | Access model, data ownership, isolation, and retention | Out of scope by product-owner direction. Organizational governance requirements in this item are excluded; existing technical collection/data-protection safeguards and finite storage/recovery requirements remain unchanged. | Resolved: excluded from scope. No Phase 1 exit dependency and no Phase 2 discovery obligation. |
| D-08 | Budget, delivery horizon, and support expectations | Product owner defines commercial/time constraints, operational ownership, and required support/availability expectations. Do not translate missing dates into reduced scope. | Before Phase 3 planning; technical design must surface implications earlier. |
| D-09 | Original routing/storage topology and uploader ownership differ across inputs | Technical lead resolves direct versus bridged state distribution, broadcast semantics, and host/upload/query responsibilities. Preserve product behavior while comparing designs. | Phase 2 design gate; no transport, 24-byte layout, Valkey topology, or crate boundary is approved here. |
| D-10 | Direct Rust implementation of the full 250/58 baseline without inference metric endpoints | Required by user direction. Technical lead evaluates every metric family and metadata association for direct observability, semantic fidelity, engine independence, integration/permissions, privacy, and resource impact. Existing endpoint routes cannot count as satisfying PR-16. | Product direction recorded; feasibility unresolved. Phase 2 must present the complete evaluation and obtain disposition of blocking gaps before design approval; no endpoint fallback or scope reduction is implied. |

A missing real environment blocks real qualification, not documentation or synthetic investigation. An unresolved requirement blocks a completeness claim. If no feasible source meets a required behavior, retain the gap and request a product decision rather than marking it supported or silently dropping it.

## 12. Phase handoff, change control, and artifacts

Phase 2 receives this approved specification, the existing detailed inventories, the decision register, and the collection boundary. It must cover the whole product architecture, including long-term extension points and unresolved requirements, before proposing bounded experiments or implementation increments. It must map every product requirement and all catalog areas to an architectural responsibility or explicit unresolved item.

The Technical Design must include the complete PR-16/D-10 direct-measurement evaluation before treating any source route as viable. The prior endpoint-ingestion approach is not the default design.

The Technical Design must define interfaces, deployment responsibilities, timing/resource budgets, source compatibility, failure behavior, persistence/query semantics, security, and feasibility evidence. Experimental subsets must name what they prove and what they leave unverified. Phase 3 can sequence only the approved scope. Later implementation, validation, and deployment cannot use a partial test suite or successful rollout to redefine product completeness.

Keep documentation lean: this Product Specification, one Technical Design, one Delivery Plan/work-item set, one Release Record, and one Deployment Runbook. Reuse the existing catalogs and collection contract as detailed references. Record changes and phase approvals briefly in the artifact's revision history. Add separate files only when they carry necessary executable data, evidence, or information that cannot be maintained clearly in the existing artifact.

Change tracking is limited to a concise revision-history entry with the revision, date, what changed, and why. Scope changes still require product-owner agreement, and each phase still requires verification and approval before the next begins. No strict audit process, separate audit document, detailed change dossier, or additional sign-off workflow is required. Product tests, measurement provenance, and validation evidence remain necessary to demonstrate behavior; they are not a document-change audit.

## 13. Phase 1 verification and approval gate

The product owner explicitly approved Phase 1 on 5 November 2026. The approved product baseline is revision 0.7; revision 0.8 records this approval without changing product scope or requirements. Phase 1 is complete. Later-phase feasibility and validation obligations remain open at their assigned gates.

| Exit criterion | Current status |
|---|---|
| Overall four-layer coverage defined before release sequencing | Accepted: 250 families retained across all four layers. |
| Contextual and association coverage preserved | Accepted: all 58 associations incorporated without changing their definitions. |
| Product users, workflows, functional behavior, and measurement semantics defined | Accepted as the Phase 1 product baseline. |
| Direct measurement direction and collection boundaries | Accepted: PR-16 covers all 250/58 IDs; inference metric endpoint consumption superseded. Feasibility and zero-application-change/access conflicts remain for Phase 2 evaluation. |
| Full-baseline delivery and explicit exclusions | Confirmed by user direction: full 250/58 baseline; UI, alerting, routing-policy ownership, and D-07 excluded. G-01 applies to all phases. |
| Targets distinguished from evidence and guarantees | Accepted: targets remain unproven; measurement-profile decisions are assigned to Phase 2. |
| Phase 1 product decisions dispositioned | D-01, D-03, and D-04 resolved; D-07 resolved as out of scope. Direct-measurement feasibility, representative qualification, and detailed technical budgets remain assigned to Phase 2. |
| Later-phase open questions assigned owners and resolution gates | Accepted as assigned in Section 11; no feasibility or validation requirement waived. |
| Product-owner approval of revision and permitted Phase 2 scope | Approved by the user on 5 November 2026: revision 0.7 is the Phase 1 baseline. Phase 2 may proceed within that scope; its own approval gate remains required. |

Phase approval is recorded in the revision history. Approval of the product baseline does not establish technical feasibility, production compatibility, performance compliance, or approval of a Phase 2 design.

Phase 1 approval: **Approved by the user on 5 November 2026 for revision 0.7. Phase 1 complete.** D-01, D-03, and D-04 are resolved; D-07 is excluded. Phase 2 preparation has not started in this approval-recording update.

## 14. Revision history

| Revision | Date | Change |
|---|---|---|
| 0.1 | 5 November 2026 | Prepared the full product baseline and long-term coverage before release selection; incorporated the 250/58 inventories, scope-preservation rule for all phases, product requirements, acceptance model, decisions, and approval gate. Existing source documents were left unchanged. |
| 0.2 | 5 November 2026 | Added the High-Level Product Architecture diagram unchanged from aether.md, with its proposed-design status and relationship to full product coverage clarified. |
| 0.3 | 5 November 2026 | Added all four architecture groups and source-defined components/testing boundaries; mapped all 15 functional and nine nonfunctional requirements to group responsibilities, including shared host budgets and unresolved upload ownership. No requirement or scope was removed. |
| 0.4 | 5 November 2026 | Recorded user decisions: full-baseline delivery (D-01); UI, alerting, and routing-policy ownership excluded (D-03); all OSS and custom LLM inference engines in scope with no binding engine assumption (D-04). Updated related scope and phase-gate text; simplified change tracking to revision history while retaining phase verification and approval. |
| 0.5 | 5 November 2026 | Added critical PR-16: implement all 250 metric families and 58 associations directly in Rust without inference metric endpoint consumption. Added full-inventory Phase 2 feasibility evaluation (D-10), reconciled historical endpoint assumptions, and retained zero-application-change, full scope, and resource constraints pending evaluation. |
| 0.6 | 5 November 2026 | Added the open inference-engine/serving-stack inventory, including llm-d as a core stack, catalog-named engines, other OSS engines, and custom engines. Preserved full scope and PR-16 endpoint-independent evaluation; distinguished stack and backend measurement boundaries. |
| 0.7 | 5 November 2026 | Recorded the user's D-07 out-of-scope decision, removed its Phase 1 exit dependency and Phase 2 discovery obligation, and aligned the governance boundary. Existing technical safeguards remain; Phase 1 awaits approval of this revision. |
| 0.8 | 5 November 2026 | Recorded the user's explicit Phase 1 approval of baseline revision 0.7 and marked Phase 1 complete. No scope or requirement changes; Phase 2 retains its separate verification and approval gate. |
