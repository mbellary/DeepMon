

# Project Aether: Executive & Product Vision

Project Aether delivers an unbundled, ultra-low-overhead telemetry engine and lakehouse architecture engineered specifically for real-time AI inference fleets (such as NVIDIA H200 clusters). Standard observability stacks introduce intolerable CPU cache pollution, lock contention, and garbage collection pauses that directly steal compute cycles from AI workloads. Aether fundamentally redefines this space by splitting telemetry into two completely decoupled data loops right at the bare-metal kernel boundary: an instant, heap-free path for cluster routing and a heavy-duty, zero-copy asynchronous path for analytical auditing.

## 🎯 Strategic Imperatives


### 1. Zero AI Workload Interference

- The Problem: Traditional monitoring daemons compete for host CPU registers, trigger memory allocations, and introduce scheduling jitter, degrading token-generation latency.
- The Aether Approach: The bare-metal host engine is written entirely in pure, unmanaged Rust utilizing custom lock-free primitives and thread-affinity pinning. By operating with zero heap allocations during the critical path, Aether guarantees that 99% of host CPU cycles and memory bandwidth remain dedicated to raw inference execution.

### 2. Microsecond Routing Agility

- The Problem: Dynamic inference routing demands instantaneous cluster state visibility (e.g., KV cache exhaustion, sudden queue spikes). Waiting for a centralized database to scrape nodes introduces staleness, leading to sub-optimal traffic distribution.
- The Aether Approach: The Sub-Second Reactive Loop bypasses all storage layers. Nodes stream tightly packed, 24-byte bitfields directly from kernel memory out to replicated load balancers, enabling O(1) routing updates at sub-100ms frequencies.

### 3. Infinite Observability at Zero Scale Cost

- The Problem: Storing high-cardinality, high-frequency infrastructure metrics in classic time-series databases results in astronomical storage costs and severe query degradation over time.
- The Aether Approach: The Asynchronous Analytical Loop groups raw telemetry directly into memory-optimized Apache Arrow batches, flattening them straight into compressed Parquet files on cheap, serverless object storage (S3/MinIO). Compute is decoupled entirely from storage, unlocking sub-second SQL queries via an on-demand DataFusion engine.

## 🗺️ High-Level Product Architecture

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

## 🛠️ The Core Bare-Metal Architectural Pillars


### ⚡ The Fast Path: Sub-Second Reactive Loop

To protect the sub-100ms routing budget, aether_agent intercepts high-frequency system events and passes a zero-copy 24-byte casting buffer directly to aether_router. By bypassing the main batching queues, this pipeline ensures that critical cluster telemetry reaches your load-balancing layer with absolute minimum latency, preventing any artificial jitter in traffic distribution.

### 📊 The Steady Path: Asynchronous Analytical Loop

For long-term compliance, security, and performance tracking, aether_agent pushes deeper, high-cardinality telemetry records directly into aether_common's lock-free MPMC ring buffers. A background worker steadily drains this queue, compacting raw events into columnar Apache Arrow batches before flushing them as compressed Parquet blocks to localized disk or remote object storage.

## 📈 Success Metrics

- Host Resource Ceiling: The entire bare-metal runtime engine must consume < 0.5% of total host CPU cycles and maintain a flat, deterministic memory footprint of < 64MB RAM under peak load.
- Routing Update Frequency: End-to-end cluster state propagation (from a hardware metric shift on a host node to a routing table update inside the load balancer) must happen in < 100ms.
- Analytical Storage Efficiency: Achieve a minimum 10x data compression ratio by shifting telemetry from raw operational structs into structured, columnar Parquet files compared to conventional text/JSON-based logging agents.




# System Architecture

Here is the end-to-end data workflow for Project Aether. This workflow traces how telemetry moves through the four independent architectural groups—separating into the Sub-Second Reactive Loop (for routing) and the Asynchronous Analytical Loop (for the serverless lakehouse).

```text
                       [ RAW SYSTEM & INFRASTRUCTURE SIGNAL SOURCE ]
                                             │
                                             ▼
                 ┌───────────────────────────────────────────────────────┐
                 │  Group 1: Local Data Ingress & Driver Plane (Daemon)  │
                 └───────────────────────────┬───────────────────────────┘
                                             │
                                             ├──────────────────────────────────────────────┐
                    (24-Byte Binary Packets) │                                              │ (Raw Telemetry Event Structs)
                                             ▼                                              ▼
                 ┌───────────────────────────────────────┐      ┌───────────────────────────────────────────────────────┐
                 │  Group 3: Sub-Second Telemetry Plane  │     │    Group 2: High-Performance Memory Pipeline          │
                 └───────────────────┬───────────────────┘      └───────────────────────────┬───────────────────────────┘
                                     │                                                      │
                       (Valkey Redis │ Protocol Streams)                                    │ (Optimized Columnar Parquet Chunks)
                                     ▼                                                      ▼
                 ┌───────────────────────────────────────┐      ┌───────────────────────────────────────────────────────┐
                 │      Replicated Load Balancers        │     │             S3 / MinIO Storage Plane                  │
                 │   (Adaptive O(1) Local Routing)       │     │               (Structured Data Lake)                  │
                 └───────────────────────────────────────┘      └───────────────────────────┬───────────────────────────┘
                                                                                            │
                                                                                            │ (Parquet Block Predicate Streams)
                                                                                            ▼
                                                                ┌───────────────────────────────────────────────────────┐
                                                                │       Group 4: Stateless Query Lakehouse              │
                                                                │          (DataFusion Analytics / UI)                  │
                                                                └───────────────────────────────────────────────────────┘
```

## 🏎️ The Sub-Second Reactive Loop (Routing Path)

This path handles cluster-wide routing dynamics at a &lt;100ms frequency, transforming raw system calls into routing tables.

### Step 1: Ingress Sampling & Extraction

- Processing Component: Group 1 (eBPF Kernel Probes & DCGM C-FFI Worker Loops).
- Input Interface: Direct OS boundaries: /proc/stat, eBPF socket events, and raw GPU system pointers.
- Output Interface: H200Telemetry Struct (A pre-allocated, un-nested Rust data tuple containing node_id, kv_cache_pct, queue_depth, avg_decode_ms, ttft_ms, and host_ctx_switches).

### Step 2: Serialization & Binary Packing

- Processing Component: Group 3 (Binary Packing Layer).
- Input Interface: H200Telemetry Struct.
- Output Interface: 24-Byte Raw Byte Slice (&[u8]). Safe, memory-aligned, zero-allocation byte block.

### Step 3: Telemetry Plane Cluster Broadcast

- Processing Component: Group 3 (State Cluster Bridge).
- Input Interface: 24-Byte Raw Byte Slice (&[u8]) over a non-blocking UDP socket or QUIC stream.
- Output Interface: Valkey Stream Entry (XADD) & Hash Registry (HSET) with 5s TTL.

### Step 4: Load Balancer Mirror Update

- Processing Component: Group 3 (Load Balancer Mirror Background Thread).
- Input Interface: Valkey Cluster replication stream (XREADGROUP / Pub-Sub).
- Output Interface: In-Memory Cluster State Table (dashmap::DashMap<u16, H200NodeState>) sitting directly inside the memory space of each replicated load balancer.
The Load Balancer consumes this local map in nanoseconds via O(1) lookups to route incoming user prompts to the optimal H200 node.

## 📊 The Asynchronous Analytical Loop (Observability Path)

This path processes rich, high-cardinality monitoring rows into an immutable, unbundled lakehouse.

### Step 1: Memory Decoupling & Queue Ingestion

- Processing Component: Group 2 (Lock-Free Queue Plane).
- Input Interface: Non-blocking try_push() event calls fired by the Group 1 daemon collection threads.
- Output Interface: crossbeam::queue::ArrayQueue<TelemetryRow> internal ring buffer sequence.

### Step 2: Columnar Compaction

- Processing Component: Group 2 (In-Memory Arrow Stage Background Worker).
- Input Interface: Raw TelemetryRow payloads pulled systematically out of the ArrayQueue.
- Output Interface: Apache Arrow RecordBatch. Pre-allocated columnar format blocks using std::mem::take to clear arrays.

### Step 3: Parquet Edge Storage Rotation

- Processing Component: Group 2 (Local Parquet Compactor).
- Input Interface: Arrow RecordBatch blocks triggered upon hitting time boundaries (1 minute) or rows boundaries (500k rows).
- Output Interface: Snappy-Compressed .parquet Byte Buffers (bytes::Bytes) written to local disk cache or prepared for S3 offloading.

### Step 4: Serverless Offloading & Storage

- Processing Component: Group 4 (Provider Object Storage Client).
- Input Interface: Local Parquet byte files payload.
- Output Interface: Immutable remote storage assets formatted under the Hive Partition standard directly within MinIO/S3:
```text
s3://telemetry-lake/cluster_id=X/year=YYYY/month=MM/day=DD/hour=HH/uuid_metrics.parquet
```

### Step 5: On-Demand Execution

- Processing Component: Group 4 (Stateless DataFusion Runtime).
- Input Interface: Vectorized SQL query input submitted by an observability application or dashboard service.
- Output Interface: Standard Row/Column Output Matrix derived using predicate pushdowns, streaming only the requested file slices into CPU SIMD registers.
Now that the explicit inputs and outputs are locked down across the entire pipeline, let me know:
- Which group's input-to-output data contract interfaces should we code first in Rust?
- Would you like to map out the isolated unit tests to verify that Group 2 correctly transforms a stream of TelemetryRow structs into an Arrow RecordBatch without memory allocations?


# Architecure Groups

To ensure Project Aether can be engineered with absolute predictability, the unbundled database and telemetry architecture must be separated into four decoupled architectural groups.
Each group acts as an independent system tier with a cleanly defined, mockable boundary interface, allowing you to implement and comprehensively test them in total isolation.

## Group 1: The Local Data Ingress & Driver Plane (aether_agent)

This group is responsible for the raw, high-frequency (100ms) non-blocking data extraction from local physical and isolated virtual boundaries.
- Core Components:
  - eBPF System Probes: Pure-Rust kernel space filters (Aya) intercepting sys_write/sys_sendto to measure userspace network-stream latency (TTFT/ITL).
  - Hardware C-FFI Wrappers: Non-blocking unsafe bindings to libnvidia-dcgm reading raw H200 HBM3e and tensor pipe telemetry.
  - cgroup Topology Scanner: Async runtime interface mapping local process namespaces (/sys/fs/cgroup) to container metadata.
- Testing Isolation Boundary: Mock out system calls and the local container API via trait primitives to assert that collection loops handle a simulated firehose safely without blocking.

## Group 2: The High-Performance Memory Pipeline (aether_common)

This group owns the zero-copy orchestration layer, turning fast telemetry signals into compressed, structurally rigid database files.
- Core Components:
  - Lock-Free Queue Plane: A multi-threaded crossbeam::queue::ArrayQueue MPMC ring buffer providing nanosecond collection decoupling.
  - In-Memory Arrow Stage: Columnar builders and matrix compaction loops implementing std::mem::take for memory optimization.
  - Local Parquet Compactors: Double-buffered asynchronous workers converting RecordBatch streams into highly dense .parquet blocks.
- Testing Isolation Boundary: Expose a simple data injection stream function. You can run multi-threaded fuzz testing and memory sanitizers to guarantee zero allocations, locks, or resource leaks under peak-load stress without having any live drivers installed.

## Group 3: The Sub-Second Telemetry Plane (aether_router)

This group processes the sub-second reactive state loop that tells your replicated layer of load balancers exactly where to route active inference requests.
- Core Components:
  - Binary Packing Layer: Fixed-width 24-byte bitfield layout translation logic ensuring heap-free byte casting.
  - State Cluster Bridge: A stateless, parallel client client utilizing Valkey glide-core or fred streams and short-TTL hashes.
  - Load Balancer Mirror: Local read-only cluster caches sitting inside concurrent, lock-free structures like dashmap.
- Testing Isolation Boundary: Abstract Valkey behind a local Docker container or mock socket provider. You can assert that the load balancer performs nanosecond O(1) local table updates and context-routing evaluations using purely pre-baked mock payload streams.

## Group 4: The Stateless Query Lakehouse (aether_query)

This group acts as the on-demand analytical compute tier, unlocking sub-second SQL transformations straight over cheap cloud objects.
- Core Components:
  - Stateless DataFusion Runtime: Embedded configurations of SessionContext optimized for SIMD execution on bare-metal CPUs.
  - Provider Object Storage Client: Dynamically registered table connectors built over the agnostic object_store library.
  - Predicate Pushdown Evaluator: File layout sorting configurations designed to scan only required block partitions.
- Testing Isolation Boundary: Completely testable locally by point-mounting DataFusion to a local directory of pre-generated Parquet logs, bypassing MinIO or remote infrastructure setups entirely during verification.


# Bear-Metal Rust Engine Process


Architectural data flow running inside the bare-metal engine process:

```text
                            ┌────────────────────────────────────────────────────────┐
                            │                🚀 aether_agent Daemon                  │
                            │                                                        │
                            │  [Raw Kernel/GPU Extraction Loops (100ms Frequency)]    │
                            └───────────────┬────────────────────────┬───────────────┘
                                            │                        │
               (Raw Event Structs)          │                        │ (Zero-Copy 24-Byte Casting Buffer)
     ┌──────────────────────────────────────┘                        └──────────────────────────────────────┐
     ▼                                                                                                      ▼
┌───────────────────────────────────────────────┐                                      ┌───────────────────────────────────────────────┐
│     🧠 aether_common (Library Crate)          │                                      │      ⚡ aether_router [Edge Utility]           │
├───────────────────────────────────────────────┤                                      ├───────────────────────────────────────────────┤
│ ▪ Absorbs high-frequency bursts via lock-free │                                      │ ▪ Consumes memory-aligned, packed bitfields.  │
│   MPMC ring buffers.                          │                                      │                                               │
│ ▪ Compacts telemetry into memory Arrow frames.│                                      │ ▪ Ships byte blocks instantly via non-blocking│
│ ▪ Flushes compressed .parquet to disk/S3.     │                                      │   UDP/QUIC wire protocols to load balancers.  │
└───────────────────────────────────────────────┘                                      └───────────────────────────────────────────────┘
          [ Observability Path Loop ]                                                             [ Sub-Second Routing Path Loop ]

          [ Analytical Loop Plane ]                                                               [ Fast-Loop Plane ]


```
To build the bare-metal Rust engine running directly on the host machines (housing the CPUs and GPUs), we need to extract the edge-resident runtime groups from the architecture.
The bare-metal host engine is comprised of Group 1 and Group 2, running alongside a lightweight outbound worker from Group 3. 


Here are the specific groups making up the local bare-metal host engine:

## 🛠️ The Bare-Metal Host Engine Core Groups


### 1. Group 1: The Local Data Ingress & Driver Plane (The Hardware Extraction Layer)

- Why it runs on the bare-metal host: It requires direct, low-level access to kernel space and hardware driver APIs to read metrics at a 100ms frequency.
- Host-Specific Execution:
  - CPU / Kernel: Loads pure-Rust eBPF programs into the host kernel using Aya to track network stream latency directly on the host's network interfaces.
  - GPU: Links directly to the host's physical NVIDIA driver via a non-blocking C-FFI wrapper around libnvidia-dcgm to pull H200 HBM3e VRAM caching metrics and Tensor Pipe telemetry.
  - Host OS Topology: Reads /sys/fs/cgroup natively on the host machine to accurately map hardware performance directly back to active container process namespaces.

### 2. Group 2: The High-Performance Memory Pipeline (The Zero-Copy Local Multiplexor)

- Why it runs on the bare-metal host: It acts as the local data stabilizer. If network connectivity drops or the downstream database experiences backpressure, this pipeline ensures the host doesn’t drop raw hardware metrics.
- Host-Specific Execution:
  - CPU Optimization: Spawns a dedicated thread pool pinned to specific host CPU cores (using thread affinity) to manage the lock-free MPMC ArrayQueue.
  - Memory Footprint: Operates completely in-memory on the host machine, building Apache Arrow RecordBatch structures and encoding them into .parquet blocks using strict zero-allocation primitives to prevent the engine from starving the local AI inference workloads of CPU/RAM.

### 3. Group 3: The Outbound Edge Worker (The Fast-Path Transmitter)

- Why it runs on the bare-metal host: While the State Cluster Bridge (the database itself) lives centrally, a tiny, high-performance bit-packing utility from Group 3 must run on the host.
- Host-Specific Execution: It runs directly inside the host daemon's memory loop, casting raw telemetry structs into fixed 24-byte packet slices and pushing them instantly via a non-blocking UDP/QUIC socket out of the host machine to the load-balancing plane.


