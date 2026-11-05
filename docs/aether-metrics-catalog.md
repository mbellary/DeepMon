# Aether: four-layer inference observability metric catalog

Research baseline: 1 November 2026. This is a requirements and source-mapping inventory for the proposed bare-metal Linux/Rust engine, not a claim that Aether currently implements the collectors.

## Scope and evidence

The user has specified distributed LLM inference, NVIDIA-class GPUs, containerized serving, Rust, and four observability layers. Exact serving-engine versions, GPU/NIC models, fabric configuration, runtime configuration and kernel versions have not been supplied. Consequently, deployment-specific availability, numeric collection intervals, overhead and alert thresholds remain **unconfirmed**. Nothing in this catalog assumes they are enabled or supported.

“All metrics” has no finite vendor-independent definition: arbitrary serving engines and driver versions can expose additional metrics. This catalog is a broad cross-layer inventory of metric families relevant to the specified product. It separates documented source mappings from explicitly proposed derived/conditional measurements. It is not an exhaustive dump of every field in every vendor API. Native source references are provided for extension. A complete deployed inventory requires an actual capability/export snapshot from the selected software and hardware versions.

Canonical IDs (L1-001 etc.) are proposed Aether identifiers. Native names are documented source mappings or named API/record fields, not promises of universal names. A family may expand into multiple components, directions, reasons, sensors, locations or priorities; those dimensions must be preserved. Exported Prometheus counter names can append `_total` to the documented family name; inspect HELP/TYPE in the actual endpoint. Histograms likewise expand into buckets/count/sum. None of those generated series is counted as a new metric family.

## Reading the tables

- **C**: counter; handle reset, wrap and identity changes before calculating rates.
- **G**: gauge; use time-weighted means for irregular observations.
- **H**: histogram/distribution, either natively exported or built from identified event records. Aether must not manufacture distributions from cumulative sums.
- **S**: state/bitset; aggregate as state durations or counts of transitions, never arithmetic sums of codes.
- **D**: explicitly derived metric; formula/inputs are stated. **OWN** indicates a proposed Aether definition, not an established vendor export.
- Collection modes are defined below. A source key links to primary documentation/source code.
- All numeric collection cadences remain unset. Event occurrence cadence, hardware sampling cadence, agent polling, upload batching, alert latency and query freshness are separate properties.
- Aggregations are ways to consume the metric, not alert thresholds. Tail percentiles require sufficient observations and compatible distributions; never average per-host percentiles.

## Collection and availability modes

- **A**: Engine endpoint: event/step-updated; host polls at a supported, unset interval. Endpoint must already be exposed and accessible; enabling optional metrics may require workload configuration.
- **H**: Host source: read counters/state at a supported, unset interval, or capture native events. Driver/kernel permissions and capability checks required.
- **I**: Profiling: supported process/library/device instrumentation; event-driven then buffered. May require injection, plugin loading, configuration or restart; not guaranteed host-only.
- **X**: Conditional: semantic events or integration are required; no universal native export verified. No collection interval or support assumed.
- **F**: External fabric integration: switch telemetry/exporter required; vendor-supported cadence unset.
- **D**: Derived: compute when validated inputs arrive over an explicit event/aggregation window. Inherits all input limitations.

## Inventory summary

| Layer | Metric families |
|---|---:|
| 1. Request and generation | 34 |
| 2. Serving engine, scheduling and memory | 71 |
| 3. Distributed execution and network fabric | 58 |
| 4. Host, GPU and infrastructure | 87 |
| **Total** | **250** |

## Layer 1: Request and generation


### Traffic

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L1-001 Requests received** | Count requests accepted at the defined server boundary | requests / C | model, replica | tgi_request_count | A / [TGI](https://huggingface.co/docs/text-generation-inference/reference/metrics) | rate, increase | Offered load; excludes traffic never reaching this boundary |
| **L1-002 Requests completed successfully** | Count successful completions | requests / C | model, replica, finish reason | vllm:request_success; nv_inference_request_success | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/), [TR](https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/user_guide/metrics.html) | rate, increase | Completion throughput; preserve source success semantics |
| **L1-003 Requests failed** | Count failures by reported reason | requests / C | model, replica, reason | nv_inference_request_failure | A / [TR](https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/user_guide/metrics.html) | rate, increase | Separate backend, canceled and rejected reasons |
| **L1-004 Admission rejections** | Count rejected admission attempts | attempts / C | model, replica, reason | vllm:admission_rejections | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | rate, increase | Attempts are not automatically distinct logical requests |
| **L1-005 Request arrival rate** | Received-request counter delta divided by elapsed time | requests/s / D | model, replica | requests_received | D / OWN | sum across disjoint replicas | Demand; match interval and counter resets |
| **L1-006 Request completion rate** | Successful-completion delta divided by elapsed time | requests/s / D | model, replica | requests_completed_successfully | D / OWN | sum across disjoint replicas | Delivered request throughput |
| **L1-007 Request failure fraction** | Failed terminal requests divided by all terminal requests | ratio / D | model, replica, interval | failed and successful terminal counts | D / OWN | ratio of summed counts | Only valid with mutually exclusive exhaustive terminal states |
| **L1-031 Corrupted generation requests** | Requests with source-reported NaNs in logits | requests / C | model, replica | vllm:corrupted_requests | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | rate, increase | Numerical failure signal; not a model-quality score |
| **L1-034 Inference item count** | Number of inference items including request batches | items / C | model, version | nv_inference_count | A / [TR](https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/user_guide/metrics.html) | rate, increase | Distinct from API requests and LLM tokens |

### Latency

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L1-008 Server time to first token** | Elapsed server time to first output token | s / H | model, replica | vllm:time_to_first_token_seconds | A / [VD](https://docs.vllm.ai/en/latest/design/metrics/) | merged histogram p50/p95/p99 | Not client-observed latency |
| **L1-009 Server request latency** | Elapsed time to terminal server completion | s / H | model, replica | sglang:e2e_request_latency_seconds; tgi_request_duration | A / [SG](https://docs.sglang.io/docs/references/production_metrics), [TGI](https://huggingface.co/docs/text-generation-inference/reference/metrics) | merged histogram p50/p95/p99 | Specify admission and terminal boundaries |
| **L1-010 Stream output gap** | Elapsed time between emitted output events | s / H | model, replica | vllm:inter_token_latency_seconds | A / [VD](https://docs.vllm.ai/en/latest/design/metrics/) | p50/p95/p99, max with raw events | An output event may contain multiple tokens |
| **L1-011 Request time per output token** | Decode elapsed time divided by output tokens minus one | s/token / H | model, replica, request class | sglang:request_time_per_output_token_seconds | A / [SG](https://docs.sglang.io/docs/references/production_metrics) | request-weighted percentiles | Undefined for fewer than two tokens; retain native exclusion rules |
| **L1-012 Request inference time** | Elapsed time attributed to request execution | s / H | model, replica | tgi_request_inference_duration | A / [TGI](https://huggingface.co/docs/text-generation-inference/reference/metrics) | p50/p95/p99 | Source-defined; do not equate with GPU busy time |
| **L1-013 Validation time** | Elapsed request-validation time | s / H | model, replica | tgi_request_validation_duration | A / [TGI](https://huggingface.co/docs/text-generation-inference/reference/metrics) | p50/p95/p99 | Separates validation from scheduling |
| **L1-014 Triton request handling time** | Accumulated server request-handling duration | us / C | model, model version | nv_inference_request_duration_us | A / [TR](https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/user_guide/metrics.html) | delta sum / delta count | Mean only; cumulative duration cannot yield percentiles |

### Tokens

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L1-015 Input token count per request** | Prompt length for each observed request | tokens / H | model, replica | vllm:request_prompt_tokens | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | p50/p95/p99, sum | Workload shape; tokenizer and multimodal semantics matter |
| **L1-016 Output token count per request** | Generated length for each observed request | tokens / H | model, replica | vllm:request_generation_tokens | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | p50/p95/p99, sum | Compare distributions with latency |
| **L1-017 Requested output limit** | Reported generation cap per request | tokens / H | model, replica | tgi_request_max_new_tokens | A / [TGI](https://huggingface.co/docs/text-generation-inference/reference/metrics) | p50/p95/p99 | Demand envelope, not actual generated work |
| **L1-018 Generated token total** | Count generated output tokens | tokens / C | model, replica | sglang:generation_tokens_total | A / [SG](https://docs.sglang.io/docs/references/production_metrics) | rate, increase | Deduplicate logical outputs across parallel workers |
| **L1-019 Prefill token total** | Count source-defined prompt tokens processed | tokens / C | model, replica | sglang:prompt_tokens_total | A / [SG](https://docs.sglang.io/docs/references/production_metrics) | rate, increase | May differ from uncached computed tokens |
| **L1-020 Output token throughput** | Generated-token delta divided by elapsed time | tokens/s / D | model, replica | generated_token_total | D / OWN | sum disjoint logical counters | Primary delivered token throughput |
| **L1-021 Input token throughput** | Prefill-token delta divided by elapsed time | tokens/s / D | model, replica | prefill_token_total | D / OWN | sum disjoint logical counters | Preserve cached versus computed token distinction |
| **L1-022 Context limit occupancy** | Current sequence tokens divided by its applicable context limit | ratio / D | sequence, model | current sequence length and validated model limit | X / OWN | distribution, maximum | Requires sequence-state events; request length alone is insufficient |
| **L1-032 Requested completion multiplicity** | Number of completions requested per input | completions / H | model, replica | vllm:request_params_n | A / [VL](https://github.com/vllm-project/vllm/blob/main/vllm/v1/metrics/loggers.py) | distribution | One API request may produce several outputs |
| **L1-033 Cached prompt tokens by tier** | Count logical prompt tokens supplied from cache by tier | tokens / C | model, replica, tier | vllm:prompt_tokens_cached_by_source | A / [VL](https://github.com/vllm-project/vllm/blob/main/vllm/v1/metrics/loggers.py) | rate, increase | Keep logical cached tokens separate from transfers and local reuse |

### SLO

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L1-023 Latency SLO compliance** | Eligible completed requests satisfying a defined latency objective / eligible completions | ratio / D | model, service, policy | request timing and explicit SLO policy | D / OWN | ratio of summed counts | No default threshold assumed |
| **L1-024 Token delivery SLO compliance** | Eligible measured gaps satisfying a defined delivery objective / eligible gaps | ratio / D | model, service, policy | token or output-event gaps and policy | D / OWN | ratio of summed counts | Keep token and output-event policies distinct |
| **L1-025 Request goodput** | Successful requests satisfying all selected objectives divided by elapsed time | requests/s / D | model, service, policy | joined success and timing records | D / OWN | sum disjoint requests | Requires joint request records; marginal histograms are insufficient |
| **L1-026 Output token goodput** | Output tokens from qualifying requests divided by elapsed time | tokens/s / D | model, service, policy | request qualification plus output counts | D / OWN | sum disjoint outputs | Define qualification and cancellation treatment explicitly |
| **L1-027 Latency variability** | Standard deviation or coefficient of variation of a selected latency population | s or ratio / D | model, replica, interval | raw observations or mergeable moments | D / OWN | stddev; CV when mean > 0 | Cannot reconstruct from quantiles alone |

### Client boundary

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L1-028 Client-observed first-token latency** | Client request start to first token received | s / H | client cohort, model | client or gateway timing records | X / OWN | p50/p95/p99 | Not observable universally from a bare-metal host agent |
| **L1-029 Client-observed request latency** | Client request start to terminal response received | s / H | client cohort, model | client or gateway timing records | X / OWN | p50/p95/p99 | Includes transport; keep separate from server latency |
| **L1-030 Timed-out or aborted requests** | Count explicit timeout or abort outcomes at a named boundary | requests / C | model, boundary, reason | supported gateway/engine terminal events | X / OWN | rate, increase | TCP disconnect alone does not establish a semantic timeout |

## Layer 2: Serving engine, scheduling and memory


### Scheduling

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L2-001 Running requests** | Requests participating in execution | requests / G | engine instance | vllm:num_requests_running | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | last, max, time-weighted mean | Engine-level value; do not sum duplicated TP exports |
| **L2-002 Waiting requests** | Requests queued for processing | requests / G | engine instance | vllm:num_requests_waiting; tgi_queue_size | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/), [TGI](https://huggingface.co/docs/text-generation-inference/reference/metrics) | last, max, time-weighted mean | Use same boundary when comparing with request queue latency |
| **L2-003 Waiting by reason** | Queued requests partitioned by source-defined reason | requests / G | engine instance, reason | vllm:num_requests_waiting_by_reason | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | last, max | Reason labels are version-specific |
| **L2-004 Prefill requests** | Context-phase requests reported for an iteration | requests / G | engine instance | trtllm_num_context_requests | A / [RT](https://github.com/NVIDIA/TensorRT-LLM/blob/main/tensorrt_llm/metrics/collector.py) | last, max, mean | TensorRT-LLM collector prefix must be verified in deployed build |
| **L2-005 Decode requests** | Generation-phase requests reported for an iteration | requests / G | engine instance | trtllm_num_generation_requests | A / [RT](https://github.com/NVIDIA/TensorRT-LLM/blob/main/tensorrt_llm/metrics/collector.py) | last, max, mean | Separate prefill/decode populations |
| **L2-006 Paused requests** | Source-reported paused request count | requests / G | engine instance | trtllm_num_paused_requests | A / [RT](https://github.com/NVIDIA/TensorRT-LLM/blob/main/tensorrt_llm/metrics/collector.py) | last, max | Not universally equivalent to swapped-to-host requests |
| **L2-007 Scheduled requests** | Requests selected for an iteration | requests / G | engine instance | trtllm_num_scheduled_requests | A / [RT](https://github.com/NVIDIA/TensorRT-LLM/blob/main/tensorrt_llm/metrics/collector.py) | distribution, maximum | Iteration selection rather than total active requests |
| **L2-008 Queue residence time** | Time spent awaiting execution | s / H | model, engine instance | vllm:request_queue_time_seconds | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | p50/p95/p99 | Request waiting duration; polling queue depth cannot reproduce it |
| **L2-009 Prefill duration** | Request time attributed to prefill | s / H | model, engine instance | vllm:request_prefill_time_seconds | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | p50/p95/p99 | Source phase timing may include more than device computation |
| **L2-010 Decode duration** | Request time attributed to decode | s / H | model, engine instance | vllm:request_decode_time_seconds | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | p50/p95/p99 | Overlaps among concurrently served requests |
| **L2-011 Engine iteration token count** | Tokens scheduled/processed in one engine step | tokens / H | engine instance | vllm:iteration_tokens_total | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | p50/p95/p99 | Native suffix says total but documented type is histogram |
| **L2-012 Current sequence batch size** | Sequences in current batch | sequences / G | engine instance | tgi_batch_current_size | A / [TGI](https://huggingface.co/docs/text-generation-inference/reference/metrics) | last, max, mean | Keep distinct from tokens per iteration |
| **L2-013 Next batch size** | Sequences selected for next batch | sequences / H | engine instance | tgi_batch_next_size | A / [TGI](https://huggingface.co/docs/text-generation-inference/reference/metrics) | p50/p95/p99 | Scheduler packing behavior |
| **L2-014 Batch token capacity** | Source-reported token capacity of current batch | tokens / G | engine instance | tgi_batch_current_max_tokens | A / [TGI](https://huggingface.co/docs/text-generation-inference/reference/metrics) | last, max | Capacity is not actual tokens executed |
| **L2-015 Batch forward duration** | Time in batch model forward pass | s / H | engine instance, phase | tgi_batch_forward_duration | A / [TGI](https://huggingface.co/docs/text-generation-inference/reference/metrics) | p50/p95/p99 | Host-measured backend interval unless documented otherwise |
| **L2-016 Batch token decode duration** | Time in the source's token-decoding stage | s / H | engine instance, method | tgi_batch_decode_duration | A / [TGI](https://huggingface.co/docs/text-generation-inference/reference/metrics) | p50/p95/p99 | Do not rename as GPU autoregressive execution duration |
| **L2-017 Batch filter duration** | Time filtering batches and sending generated tokens | s / H | engine instance, phase | tgi_batch_filter_duration | A / [TGI](https://huggingface.co/docs/text-generation-inference/reference/metrics) | p50/p95/p99 | CPU/output processing overhead |
| **L2-018 Batch inference calls** | Batch inference invocations, with success as separate outcome | calls / C | engine instance, phase | tgi_batch_inference_count; tgi_batch_inference_success | A / [TGI](https://huggingface.co/docs/text-generation-inference/reference/metrics) | rate, increase | Use outcome dimension only when populations align |
| **L2-019 Preemption total** | Engine preemption events | events / C | engine instance | vllm:num_preemptions | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | rate, increase | Reason and recovery behavior depend on engine |
| **L2-020 Preemptions per request** | Number of preemptions experienced by a request | events / H | model, engine instance | vllm:request_num_preemptions | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | p50/p95/p99 | max reveals repeatedly disrupted requests |
| **L2-057 Backend execution count** | Number of batch executions by backend | executions / C | model, version | nv_inference_exec_count | A / [TR](https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/user_guide/metrics.html) | rate, increase | One execution may serve several requests |
| **L2-058 Backend input/inference/output cumulative duration** | Accumulated time by backend processing stage | us / C | model, version, stage | nv_inference_compute_input_duration_us; nv_inference_compute_infer_duration_us; nv_inference_compute_output_duration_us | A / [TR](https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/user_guide/metrics.html) | delta time / delta stage count | Preserve stage boundaries and cache exclusions; no percentiles from sums |
| **L2-059 Iteration duration** | Latest source-reported engine iteration latency | s / G | engine instance | TensorRT-LLM collector suffix iteration_latency_seconds | A / [RT](https://github.com/NVIDIA/TensorRT-LLM/blob/main/tensorrt_llm/metrics/collector.py) | last, distribution only with every iteration | Polling a latest-value gauge misses intermediate iterations |
| **L2-069 Backend pending requests** | Requests enqueued but not yet executing in backend | requests / G | model, version | nv_inference_pending_request_count | A / [TR](https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/user_guide/metrics.html) | last, max | Different queue boundary from engine waiting-request gauges |
| **L2-070 Batch inference duration** | Elapsed source-defined batch inference interval | s / H | engine instance | tgi_batch_inference_duration | A / [TGI](https://huggingface.co/docs/text-generation-inference/reference/metrics) | p50/p95/p99 | Not interchangeable with individual forward/decode/filter components |

### KV cache

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L2-021 KV-cache occupancy** | Used capacity divided by allocatable capacity in one cache pool | ratio / G | engine instance, pool | vllm:kv_cache_usage_perc; sglang:token_usage | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/), [SG](https://docs.sglang.io/docs/references/production_metrics) | last, max, time-weighted mean | Retain source allocation semantics; not framebuffer occupancy |
| **L2-022 KV blocks used** | Allocated or pinned blocks as defined by source | blocks / G | engine instance, pool | trtllm_kv_cache_used_blocks | A / [RT](https://github.com/NVIDIA/TensorRT-LLM/blob/main/tensorrt_llm/metrics/collector.py) | last, max | Do not merge different block sizes or pinned/evictable definitions |
| **L2-023 KV blocks free** | Available blocks reported by source | blocks / G | engine instance, pool | trtllm_kv_cache_free_blocks | A / [RT](https://github.com/NVIDIA/TensorRT-LLM/blob/main/tensorrt_llm/metrics/collector.py) | last, min | Immediate allocation headroom |
| **L2-024 KV block capacity** | Maximum blocks in cache pool | blocks / G | engine instance, pool | trtllm_kv_cache_max_blocks | A / [RT](https://github.com/NVIDIA/TensorRT-LLM/blob/main/tensorrt_llm/metrics/collector.py) | last, change events | Cache capacity; not total HBM |
| **L2-025 Tokens per KV block** | Configured tokens represented by one block | tokens/block / G | engine instance, pool | trtllm_kv_cache_tokens_per_block | A / [RT](https://github.com/NVIDIA/TensorRT-LLM/blob/main/tensorrt_llm/metrics/collector.py) | last | Required when comparing block counts |
| **L2-026 Prefix cache queried tokens** | Tokens considered for prefix-cache lookup | tokens / C | engine instance, tier | vllm:prefix_cache_queries | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | rate, increase | Token denominator, not request denominator |
| **L2-027 Prefix cache hit tokens** | Tokens satisfied by prefix cache | tokens / C | engine instance, tier | vllm:prefix_cache_hits | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | rate, increase | Avoid adding overlapping local/external accounting |
| **L2-028 Prefix cache hit fraction** | Hit-token delta divided by queried-token delta | ratio / D | engine instance, tier | matched prefix hits and queries | D / OWN | ratio of sums | Undefined when no queried tokens |
| **L2-029 Computed prefill tokens per request** | New KV tokens computed excluding cache reuse | tokens / H | model, engine instance | vllm:request_prefill_kv_computed_tokens | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | sum, percentiles | Distinguishes logical input length from compute work |
| **L2-030 Cache block lifetime** | Allocation-to-eviction interval | s / H | engine instance, pool | vllm:kv_block_lifetime_seconds | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | p50/p95/p99 | Optional sampled cache metrics |
| **L2-031 Cache block idle before eviction** | Idle interval preceding eviction | s / H | engine instance, pool | vllm:kv_block_idle_before_evict_seconds | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | p50/p95/p99 | Optional sampled cache metrics |
| **L2-032 Cache block reuse gap** | Interval between observed accesses to a block | s / H | engine instance, pool | vllm:kv_block_reuse_gap_seconds | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | p50/p95/p99 | Sampling and finite access history affect completeness |
| **L2-060 External prefix-cache queries and hits** | Tokens looked up and hit in an external cache | tokens / C | engine instance, outcome | vllm:external_prefix_cache_queries; vllm:external_prefix_cache_hits | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | rate, ratio of matching deltas | Connector-dependent; avoid overlap with tier totals |
| **L2-061 Hot-buffer hits and misses** | Device hot-buffer lookup outcomes | lookups / C | engine instance, outcome | vllm:hisparse_cache_hits; vllm:hisparse_cache_misses | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | rate, matching hit fraction | HiSparse-only; not whole prefix-cache hit ratio |

### Offload

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L2-033 Host KV occupancy** | Occupied fraction of the configured host cache pool | ratio / G | engine instance, host pool | vllm:hisparse_host_cache_usage_perc | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | last, max | HiSparse only; evictable blocks count as free |
| **L2-034 Pending cache transfers** | Outstanding cache page transfers | transfers / G | engine instance | vllm:hisparse_pending_page_transfers | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | last, max | Configured connector only |
| **L2-035 Host-to-device KV bytes** | Bytes restored from host KV storage | bytes / C | engine instance | vllm:hisparse_host_to_device_bytes | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | rate, increase | Configured connector only; not all PCIe traffic |
| **L2-036 Offload blocks restored** | Blocks loaded back onto GPU | blocks / C | engine instance, backend | vllm:simple_kv_offload_load_blocks_total | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | rate, increase | SimpleCPUOffloadConnector only |
| **L2-037 Pending offload stores** | Blocks awaiting store completion/release | blocks / G | engine instance, backend | vllm:simple_kv_offload_pending_store_blocks | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | last, max | Connector-specific logical blocks |
| **L2-038 Pinned offload blocks** | Offload blocks unavailable for eviction | blocks / G | engine instance, backend | vllm:simple_kv_offload_used_blocks | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | last, max | Not total warm-cache residency |
| **L2-062 Offload store decisions** | Eager-mode store admissions by outcome | decisions / C | engine instance, outcome | vllm:simple_kv_offload_save_outcomes_total | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | rate, increase | Lazy-mode stores not classified by this family |

### Speculation

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L2-039 Accepted speculative tokens by position** | Accepted draft tokens grouped by draft position | tokens / C | engine instance, position | vllm:spec_decode_num_accepted_tokens_per_pos | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | rate, increase | Speculative decoding only |
| **L2-040 Draft steps configured** | Currently active speculative step setting | steps / G | engine instance | sglang:spec_num_steps | A / [SG](https://docs.sglang.io/docs/references/production_metrics) | last | Configuration, not actual completed work |
| **L2-041 Draft tokens configured** | Currently active draft-token setting | tokens / G | engine instance | sglang:spec_num_draft_tokens | A / [SG](https://docs.sglang.io/docs/references/production_metrics) | last | Configuration, not accepted output |
| **L2-042 Draft acceptance fraction** | Accepted draft-token count / proposed draft-token count | ratio / D | engine instance, interval | matched draft proposal and acceptance records | X / OWN | ratio of sums | Requires verified denominator; excludes bonus tokens |
| **L2-043 Accepted tokens per speculation step** | Accepted draft tokens / completed speculation steps | tokens/step / D | engine instance | matched accepted tokens and step records | X / OWN | ratio of sums | Requires completed-step events; configured steps are not denominator |
| **L2-063 Draft proposal count** | Completed speculative draft opportunities | drafts / C | engine instance | vllm:spec_decode_num_drafts | A / [VS](https://github.com/vllm-project/vllm/blob/main/vllm/v1/spec_decode/metrics.py) | rate, increase | Conditional on enabled speculation; counter export may append total |
| **L2-064 Proposed draft tokens** | Number of draft token proposals | tokens / C | engine instance | vllm:spec_decode_num_draft_tokens | A / [VS](https://github.com/vllm-project/vllm/blob/main/vllm/v1/spec_decode/metrics.py) | rate, increase | Do not count as delivered output |
| **L2-065 Accepted draft tokens** | Number of accepted draft tokens | tokens / C | engine instance | vllm:spec_decode_num_accepted_tokens | A / [VS](https://github.com/vllm-project/vllm/blob/main/vllm/v1/spec_decode/metrics.py) | rate, increase | Keep bonus tokens separate |
| **L2-066 Mean acceptance length including bonus** | One plus accepted draft-token delta divided by draft delta | tokens/draft / D | engine instance | matched speculative counters | D / [VS](https://github.com/vllm-project/vllm/blob/main/vllm/v1/spec_decode/metrics.py) | ratio over interval plus one | Source convention includes bonus; differs from accepted draft tokens per step |
| **L2-071 Skipped speculative tokens per request** | Source-reported speculated/skipped token quantity | tokens / H | model, engine instance | tgi_request_skipped_tokens | A / [TGI](https://huggingface.co/docs/text-generation-inference/reference/metrics) | distribution | Preserve source meaning; not universally accepted draft-token count |

### Runtime memory

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L2-044 Engine GPU memory allocated** | GPU memory attributed by engine runtime | bytes / G | engine instance | nv_trt_llm_runtime_memory_metrics{memory_type=gpu} | A / [TB](https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/tensorrtllm_backend/README.html) | last, max | Allocator accounting differs from total device usage |
| **L2-045 Engine CPU memory allocated** | CPU memory attributed by engine runtime | bytes / G | engine instance | nv_trt_llm_runtime_memory_metrics{memory_type=cpu} | A / [TB](https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/tensorrtllm_backend/README.html) | last, max | Not total process resident memory |
| **L2-046 Engine pinned memory allocated** | Page-locked host memory allocated by engine | bytes / G | engine instance | nv_trt_llm_runtime_memory_metrics{memory_type=pinned} | A / [TB](https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/tensorrtllm_backend/README.html) | last, max | Distinct from total host memory |
| **L2-047 Triton pinned pool capacity and use** | Total and used bytes in server pinned pool | bytes / G | server, component | nv_pinned_memory_pool_total_bytes; nv_pinned_memory_pool_used_bytes | A / [TR](https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/user_guide/metrics.html) | last, max | Two components; server-wide rather than per model |

### Response cache

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L2-048 Response-cache hits and misses** | Response lookup outcomes | lookups / C | model, outcome | nv_cache_num_hits_per_model; nv_cache_num_misses_per_model | A / [TR](https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/user_guide/metrics.html) | rate, increase | Whole-response cache, not prefix KV cache |
| **L2-049 Response-cache operation duration** | Cumulative cache hit or miss processing time | us / C | model, outcome | nv_cache_hit_duration_per_model; nv_cache_miss_duration_per_model | A / [TR](https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/user_guide/metrics.html) | delta time / delta count | Mean only from cumulative sums |

### Optional engine state

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L2-050 Engine sleep state** | Reported awake/offload/discard state | state / S | engine instance | vllm:engine_sleep_state | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | last, time in state | Source is labeled state gauges; preserve state dimensions |
| **L2-051 LoRA request activity information** | Adapter-associated activity information reported by engine | info / S | engine instance, adapter | vllm:lora_requests_info | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | last | Inspect labels and native representation; do not interpret info-gauge value as request count |
| **L2-052 Multimodal cache lookups and hits** | Cached-item outcomes for multimodal preprocessing | items / C | engine instance, outcome | vllm:mm_cache_queries; vllm:mm_cache_hits | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | rate; hit/query ratio | Item-weighted; multimodal workloads only |

### Derived execution

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L2-053 Batch sequence occupancy** | Scheduled sequences / configured sequence capacity | ratio / D | engine instance, iteration | scheduled sequences and applicable capacity | D / OWN | distribution, mean | Not an optimization target by itself; token limits may bind first |
| **L2-054 Scheduler gap** | Next iteration start minus previous iteration end | us / H | engine instance, execution lane | matched iteration boundary events | X / OWN | p50/p95/p99, max | Requires verified boundaries; continuous overlap can invalidate simple subtraction |
| **L2-055 Estimated FLOP throughput** | Rate of engine-estimated floating-point operations | FLOP/s / D | GPU, engine instance | sglang:estimated_flops_per_gpu_total | A / [SG](https://docs.sglang.io/docs/references/production_metrics) | rate | Optional MFU metrics; model estimate rather than hardware measurement |
| **L2-056 Estimated model-memory traffic** | Rates of engine-estimated read and write bytes | bytes/s / D | GPU, direction | sglang:estimated_read_bytes_per_gpu_total; sglang:estimated_write_bytes_per_gpu_total | A / [SG](https://docs.sglang.io/docs/references/production_metrics) | rate by direction | Not measured HBM bandwidth |

### Runtime failures

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L2-067 GPU allocation failures** | Count explicit failed allocation outcomes | events / C | engine instance, allocator, reason | supported allocator failure events; no universal export verified | X / OWN | rate, increase | Not inferable solely from low free memory |
| **L2-068 Model load and warmup duration** | Observed start-to-ready interval by lifecycle phase | s / H | model, replica, phase | supported lifecycle phase events | X / OWN | p50/p95/p99 | Includes startup only when exact boundaries are available |

## Layer 3: Distributed execution and network fabric


### Collectives

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L3-001 Collective duration** | Recorded duration of matched collective operation | us / H | communicator, rank, operation | coll_exec_time_us; coll_timing_source | I / [NCCL](https://github.com/NVIDIA/nccl/blob/master/plugins/profiler/inspector/README.md) | p50/p95/p99 by size | Keep host and GPU timing sources separate |
| **L3-002 Collective message size** | Payload size recorded for collective | bytes / H | communicator, rank, operation | coll_msg_size_bytes | I / [NCCL](https://github.com/NVIDIA/nccl/blob/master/plugins/profiler/inspector/README.md) | distribution | Preserve per-rank versus collective size definition |
| **L3-003 Collective algorithm bandwidth** | Source-reported algorithmic bandwidth | GB/s / G | communicator, rank, operation | coll_algobw_gbs | I / [NCCL](https://github.com/NVIDIA/nccl/blob/master/plugins/profiler/inspector/README.md) | distribution by message size | Not physical wire throughput |
| **L3-004 Collective bus bandwidth** | Source-normalized collective bus bandwidth | GB/s / G | communicator, rank, operation | coll_busbw_gbs | I / [NCCL](https://github.com/NVIDIA/nccl/blob/master/plugins/profiler/inspector/README.md) | distribution by message size | Normalization differs by collective type |
| **L3-005 Point-to-point duration** | Recorded send/receive operation duration | us / H | communicator, peer, operation | p2p_exec_time_us | I / [NCCL](https://github.com/NVIDIA/nccl/blob/master/plugins/profiler/inspector/README.md) | p50/p95/p99 | Requires supported P2P profiler records |
| **L3-006 Collective invocation rate** | Observed operation count divided by elapsed time | operations/s / D | communicator, operation | matched collective records | D / OWN | count/time | Mark incomplete when profiler drops records |
| **L3-007 Rank arrival skew** | Latest minus earliest entry timestamp for one matched operation | us / D | communicator, sequence | rank entry times, clock mapping | X / OWN | distribution, max | No reliable result when clock uncertainty exceeds skew |
| **L3-008 Rank completion skew** | Latest minus earliest completion for one matched operation | us / D | communicator, sequence | rank completion times, clock mapping | X / OWN | distribution, max | Separate timing sources and missing ranks |
| **L3-009 Rank duration imbalance** | Maximum rank duration divided by median for one matched operation | ratio / D | communicator, sequence | complete matched rank durations | D / OWN | distribution, max | Median must be positive; does not identify root cause |
| **L3-010 Communication interval coverage** | Union of observed communication intervals divided by observation interval | ratio / D | rank, device, interval | complete device communication intervals | X / OWN | interval union | Avoid summing overlapping collective durations |
| **L3-011 Compute-communication overlap** | Intersection of compute and communication interval unions / communication union | ratio / D | rank, device, interval | classified device execution intervals | X / OWN | interval intersections | Requires device timing and classification; not host API timings |
| **L3-012 Collective timeout/abort count** | Explicit collective failure outcomes | events / C | communicator, rank, reason | supported library error/status records | X / OWN | rate, increase | No universal host-only semantic counter |
| **L3-057 Collective sequence completeness** | Observed matched ranks / expected ranks for operation | ratio / D | communicator, sequence | verified membership and captured rank records | D / OWN | per-operation ratio | Missing records and failed ranks require separate diagnosis |

### Pipeline

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L3-013 Pipeline stage duration** | Execution duration for a matched stage/microbatch | us / H | replica, pipeline stage, microbatch | supported stage boundary events | X / OWN | p50/p95/p99 | No universal engine-native identifier established |
| **L3-014 Pipeline stage handoff delay** | Consumer-ready time minus matched producer completion | us / H | stage pair, microbatch | matched producer/consumer events | X / OWN | p50/p95/p99 | Requires clock mapping and definition of ready |
| **L3-015 Pipeline bubble fraction** | Unscheduled time while stage is expected to participate / eligible time | ratio / D | stage, execution interval | stage schedule and execution intervals | X / OWN | interval unions | Needs explicit eligible-time definition; not equivalent to GPU idle |

### MoE

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L3-016 Tokens assigned per expert** | Count router assignments per expert | assignments / C | MoE layer, expert | supported router assignment records | X / OWN | distribution, rate | Top-k routing can create multiple assignments per token |
| **L3-017 Expert assignment imbalance** | Maximum assignments / mean across eligible experts | ratio / D | MoE layer, routing group | complete assignment counts including zero-load experts | X / OWN | max/mean | No assignments yields undefined ratio |
| **L3-018 Expert execution duration** | Execution interval for expert batch | us / H | MoE layer, expert, batch | supported expert execution events | X / OWN | p50/p95/p99 | Requires expert semantics; generic kernel name may be insufficient |
| **L3-019 Expert dispatch/combine duration** | Duration of dispatch or combine stage | us / H | MoE layer, phase | supported dispatch/combine boundaries | X / OWN | p50/p95/p99 | May mix communication and compute; preserve phase |
| **L3-020 Expert overflow or dropped assignments** | Count explicit router capacity overflow/drop outcomes | assignments / C | MoE layer, expert, reason | supported routing outcomes | X / OWN | rate, increase | Only applicable if implementation has this behavior |

### KV transfer

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L3-021 KV-transfer size** | Bytes recorded per connector transfer | bytes / H | replica pair, connector | vllm:nixl_bytes_transferred | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | distribution, sum | NIXL connector only; source metadata needed for peer mapping |
| **L3-022 KV-transfer duration** | Elapsed connector transfer duration | s / H | connector, replica pair | vllm:nixl_xfer_time_seconds | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | p50/p95/p99 | Connector scope; not all network traffic |
| **L3-023 KV-transfer posting time** | Time spent posting transfer work | s / H | connector | vllm:nixl_post_time_seconds | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | p50/p95/p99 | Separates submission overhead from transfer |
| **L3-024 KV-transfer failures** | Explicit failed connector transfers | transfers / C | connector, reason | vllm:nixl_num_failed_transfers | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | rate, increase | Includes handshake/notification failures in documented source |
| **L3-025 Expired KV requests** | Requests whose KV lease expired | requests / C | prefill instance, connector | vllm:nixl_num_kv_expired_reqs | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | rate, increase | Not the same population as transfer failures |
| **L3-026 Late completion notifications** | Completion notifications after request tracking expired | notifications / C | connector, remote rank | vllm:nixl_num_notifications_after_expiry | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | rate, increase | One request may produce notifications from multiple ranks |
| **L3-058 KV-transfer descriptor count** | Descriptors recorded per connector operation | descriptors / H | connector | vllm:nixl_num_descriptors | A / [V](https://docs.vllm.ai/en/latest/usage/metrics/) | distribution | NIXL only; descriptor count is not transfer bytes |

### InfiniBand

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L3-027 Port transmit/receive data** | Port payload data counters by direction | bytes / C | RDMA device, port, direction | port_xmit_data; port_rcv_data | H / [IB](https://kernel.googlesource.com/pub/scm/linux/kernel/git/torvalds/linux/+/8c13415c8a4383447c21ec832b20b3b283f0e01a/Documentation/ABI/stable/sysfs-class-infiniband) | rate, increase | Raw IB data units require conversion; commonly four octets |
| **L3-028 Port transmit/receive packets** | Port packet counters by direction | packets / C | RDMA device, port, direction | port_xmit_packets; port_rcv_packets | H / [IB](https://kernel.googlesource.com/pub/scm/linux/kernel/git/torvalds/linux/+/8c13415c8a4383447c21ec832b20b3b283f0e01a/Documentation/ABI/stable/sysfs-class-infiniband) | rate, increase | Not per-workload without additional attribution |
| **L3-029 Port transmit wait** | Source-defined intervals waiting to transmit | native ticks / C | RDMA device, port | port_xmit_wait | H / [IB](https://kernel.googlesource.com/pub/scm/linux/kernel/git/torvalds/linux/+/8c13415c8a4383447c21ec832b20b3b283f0e01a/Documentation/ABI/stable/sysfs-class-infiniband) | delta; convert only with verified units | Do not label raw ticks as microseconds |
| **L3-030 Port receive errors** | Received packets with reported errors | packets / C | RDMA device, port | port_rcv_errors | H / [IB](https://kernel.googlesource.com/pub/scm/linux/kernel/git/torvalds/linux/+/8c13415c8a4383447c21ec832b20b3b283f0e01a/Documentation/ABI/stable/sysfs-class-infiniband) | rate, increase | Retain source counter width and reset behavior |
| **L3-031 Port transmit discards** | Packets discarded on transmit | packets / C | RDMA device, port | port_xmit_discards | H / [IB](https://kernel.googlesource.com/pub/scm/linux/kernel/git/torvalds/linux/+/8c13415c8a4383447c21ec832b20b3b283f0e01a/Documentation/ABI/stable/sysfs-class-infiniband) | rate, increase | Congestion or policy context required |
| **L3-032 Link down events** | Recorded transitions into down state | events / C | RDMA device, port | link_downed | H / [IB](https://kernel.googlesource.com/pub/scm/linux/kernel/git/torvalds/linux/+/8c13415c8a4383447c21ec832b20b3b283f0e01a/Documentation/ABI/stable/sysfs-class-infiniband) | increase | Correlate with configured topology |
| **L3-033 Link recovery events** | Successful link error recoveries | events / C | RDMA device, port | link_error_recovery | H / [IB](https://kernel.googlesource.com/pub/scm/linux/kernel/git/torvalds/linux/+/8c13415c8a4383447c21ec832b20b3b283f0e01a/Documentation/ABI/stable/sysfs-class-infiniband) | increase | Recovery count, not lost packet count |
| **L3-034 Symbol errors** | Reported physical-lane symbol errors | errors / C | RDMA device, port | symbol_error | H / [IB](https://kernel.googlesource.com/pub/scm/linux/kernel/git/torvalds/linux/+/8c13415c8a4383447c21ec832b20b3b283f0e01a/Documentation/ABI/stable/sysfs-class-infiniband) | rate, increase | Hardware capability dependent |
| **L3-035 Receive physical/relay errors** | Receive errors classified by physical or switch-relay cause | errors / C | RDMA device, port, class | port_rcv_remote_physical_errors; port_rcv_switch_relay_errors | H / [IB](https://kernel.googlesource.com/pub/scm/linux/kernel/git/torvalds/linux/+/8c13415c8a4383447c21ec832b20b3b283f0e01a/Documentation/ABI/stable/sysfs-class-infiniband) | rate, increase | Preserve classes; no inferred packet-level cause |
| **L3-036 Constraint errors** | Transmit/receive filtering or partition constraint failures | errors / C | RDMA device, port, direction | port_xmit_constraint_errors; port_rcv_constraint_errors | H / [IB](https://kernel.googlesource.com/pub/scm/linux/kernel/git/torvalds/linux/+/8c13415c8a4383447c21ec832b20b3b283f0e01a/Documentation/ABI/stable/sysfs-class-infiniband) | rate, increase | Configuration and partition membership investigation |
| **L3-054 Local link integrity and buffer overrun errors** | Port error counts separated by class | errors / C | RDMA device, port, class | local_link_integrity_errors; excessive_buffer_overrun_errors | H / [IB](https://kernel.googlesource.com/pub/scm/linux/kernel/git/torvalds/linux/+/8c13415c8a4383447c21ec832b20b3b283f0e01a/Documentation/ABI/stable/sysfs-class-infiniband) | rate, increase | Availability and saturation depend on hardware |
| **L3-055 Management VL drops** | Packets dropped on management virtual lane | packets / C | RDMA device, port | VL15_dropped | H / [IB](https://kernel.googlesource.com/pub/scm/linux/kernel/git/torvalds/linux/+/8c13415c8a4383447c21ec832b20b3b283f0e01a/Documentation/ABI/stable/sysfs-class-infiniband) | rate, increase | Management-plane indicator, not user collective packet loss |

### Ethernet/RoCE

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L3-037 PFC pause frames** | Priority flow-control frames sent/received | frames / C | NIC, port, priority, direction | tx_prio[p]_pause; rx_prio[p]_pause | H / [NIC](https://networking-docs.nvidia.com/doca/archive/3-2-1/doca-telemetry-service-guide) | rate, increase | Driver-specific names; relevant priority mapping required |
| **L3-038 PFC pause duration** | Source-reported paused time per priority | native time / C | NIC, port, priority, direction | priority pause-duration counter; driver-specific | H / [NIC](https://networking-docs.nvidia.com/doca/archive/3-2-1/doca-telemetry-service-guide) | delta after verified unit conversion | Pause direction semantics must be retained |
| **L3-039 Priority traffic bytes** | Physical-port bytes by priority and direction | bytes / C | NIC, port, priority, direction | rx_prio[p]_bytes; tx_prio[p]_bytes | H / [NIC](https://networking-docs.nvidia.com/doca/archive/3-2-1/doca-telemetry-service-guide) | rate, increase | Physical-port scope; not necessarily container scope |
| **L3-040 Receive buffer exhaustion** | Adapter receives with insufficient software buffers | events / C | NIC, port | rx_out_of_buffer | H / [NIC](https://networking-docs.nvidia.com/doca/archive/3-2-1/doca-telemetry-service-guide) | rate, increase | Not equivalent to every type of packet drop |
| **L3-041 Priority buffer discards** | Receive discards caused by priority buffer shortage | packets / C | NIC, port, priority | rx_prio[p]_discards | H / [NIC](https://networking-docs.nvidia.com/doca/archive/3-2-1/doca-telemetry-service-guide) | rate, increase | Availability depends on adapter/driver |
| **L3-042 Handled congestion notifications** | CNP packets handled by RDMA reaction point | packets / C | RDMA device, port | rp_cnp_handled | H / [ROCE](https://docs.nvidia.com/networking/display/bfswtroubleshooting/doca-services.pdf) | rate, increase | Congestion response; not itself a failure |
| **L3-043 Ignored congestion notifications** | CNP packets ignored by RDMA reaction point | packets / C | RDMA device, port | rp_cnp_ignored | H / [ROCE](https://docs.nvidia.com/networking/display/bfswtroubleshooting/doca-services.pdf) | rate, increase | Investigate congestion-control configuration |
| **L3-044 RNR retry events** | Reported receiver-not-ready retry outcomes | events / C | RDMA device, supported scope | rnr_nak_retry_err | H / [ROCE](https://docs.nvidia.com/networking/display/bfswtroubleshooting/doca-services.pdf) | rate, increase | Counter semantics differ from final retry exhaustion |
| **L3-045 Completion queue errors** | Completion records with errors, separated from flush errors | errors / C | RDMA device, supported scope | resp_cqe_error; resp_cqe_flush_error | H / [ROCE](https://docs.nvidia.com/networking/display/bfswtroubleshooting/doca-services.pdf) | rate, increase | Preserve requester/responder and error classes |
| **L3-046 RDMA remote access errors** | Requester or responder reports remote access failure | errors / C | RDMA device, role | req_remote_access_errors; resp_remote_access_errors | H / [ROCE](https://docs.nvidia.com/networking/display/bfswtroubleshooting/doca-services.pdf) | rate, increase | Not a congestion metric |
| **L3-047 RDMA local length errors** | Responder reports local length violation | errors / C | RDMA device | resp_local_length_error | H / [ROCE](https://docs.nvidia.com/networking/display/bfswtroubleshooting/doca-services.pdf) | rate, increase | Buffer/protocol correctness investigation |
| **L3-056 RDMA invalid request errors** | Requester reports remote invalid request | errors / C | RDMA device, role | req_remote_invalid_request | H / [ROCE](https://docs.nvidia.com/networking/display/bfswtroubleshooting/doca-services.pdf) | rate, increase | Protocol/access issue rather than bandwidth saturation |

### TCP transport

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L3-048 TCP retransmissions** | Segments retransmitted by kernel TCP | segments / C | network namespace; connection if traced | Tcp:RetransSegs | H / [TCP](https://docs.kernel.org/networking/snmp_counter.html) | rate, increase | Does not observe RDMA retransmissions |
| **L3-049 TCP connection establishment failures** | Failed active/passive establishments | events / C | network namespace | Tcp:AttemptFails | H / [TCP](https://docs.kernel.org/networking/snmp_counter.html) | rate, increase | Not equivalent to application request failure |
| **L3-050 TCP resets** | Resets sent by kernel TCP | segments / C | network namespace | Tcp:OutRsts | H / [TCP](https://docs.kernel.org/networking/snmp_counter.html) | rate, increase | Separate normal teardown from application impact |
| **L3-051 TCP RTT** | Kernel TCP smoothed round-trip estimate | us / G | connection, peer | TCP_INFO tcpi_rtt; supported socket hooks | X / OWN | distribution, last | TCP-specific estimate; not one-way fabric latency |

### External switches

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L3-052 Switch queue occupancy** | Reported switch egress/ingress queue depth | bytes or cells / G | switch, port, queue | vendor-specific switch telemetry; native ID unverified | F / OWN | last, max, distribution | Requires switch integration; not available from host alone |
| **L3-053 Switch congestion/drop counters** | Reported switch drops, ECN marks and PFC outcomes by class | events / C | switch, port, queue, class | vendor-specific switch counters; native ID unverified | F / OWN | rate, increase | Preserve vendor semantics; do not merge unlike counters |

## Layer 4: Host, GPU and infrastructure


### GPU activity

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L4-001 GPU busy fraction** | Source-defined interval with at least one executing kernel | ratio / G | GPU | nvmlDeviceGetUtilizationRates.gpu | H / [NV](https://docs.nvidia.com/deploy/nvml-api/latest/api/group__nvmlDeviceQueries.html) | last, time-weighted mean | Hardware sample period limits resolution |
| **L4-002 SM activity** | Interval activity averaged across multiprocessors | ratio / G | GPU or supported partition | DCGM profiling field 1002 | H / [DP](https://docs.nvidia.com/datacenter/dcgm/latest/learn/modules/profiling.html) | mean, min, max | Active warps may be waiting, not computing |
| **L4-003 SM occupancy** | Observed resident-warp occupancy | ratio / G | GPU or supported partition | DCGM profiling field 1003 | H / [DP](https://docs.nvidia.com/datacenter/dcgm/latest/learn/modules/profiling.html) | mean, min, max | Not equivalent to SM activity or achieved FLOP throughput |
| **L4-004 Tensor pipeline activity** | Observed tensor execution pipeline activity | ratio / G | GPU or supported partition | DCGM profiling field 1004 | H / [DP](https://docs.nvidia.com/datacenter/dcgm/latest/learn/modules/profiling.html) | mean, min, max | Architecture and counter compatibility dependent |
| **L4-005 Device-memory activity** | Cycles with memory-interface read/write activity | ratio / G | GPU | DCGM profiling field 1005 | H / [DP](https://docs.nvidia.com/datacenter/dcgm/latest/learn/modules/profiling.html) | mean, min, max | Not percentage of peak memory bandwidth |
| **L4-006 FP64/FP32/FP16 pipeline activity** | Activity by supported floating-point pipeline class | ratio / G | GPU, pipeline | DCGM profiling fields 1006/1007/1008 | H / [DP](https://docs.nvidia.com/datacenter/dcgm/latest/learn/modules/profiling.html) | mean by pipeline | Counter support/multiplexing is GPU-specific |

### GPU memory

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L4-007 Framebuffer used** | Reported allocated device memory | bytes / G | GPU or supported partition | nvmlDeviceGetMemoryInfo used | H / [NV](https://docs.nvidia.com/deploy/nvml-api/latest/api/group__nvmlDeviceQueries.html) | last, max | Not solely model weights or KV cache |
| **L4-008 Framebuffer free** | Reported available device memory | bytes / G | GPU or supported partition | nvmlDeviceGetMemoryInfo free | H / [NV](https://docs.nvidia.com/deploy/nvml-api/latest/api/group__nvmlDeviceQueries.html) | last, min | Driver-reserved/accounting behavior matters |
| **L4-009 Framebuffer total** | Reported usable framebuffer capacity | bytes / G | GPU or supported partition | nvmlDeviceGetMemoryInfo total | H / [NV](https://docs.nvidia.com/deploy/nvml-api/latest/api/group__nvmlDeviceQueries.html) | last | Denominator must match used/free API version |
| **L4-010 Framebuffer occupancy** | Used framebuffer divided by matching total | ratio / D | GPU or supported partition | framebuffer used and total | D / OWN | last, max | Not KV-cache occupancy |

### GPU power/thermal

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L4-011 GPU power draw** | Device-reported power reading | W / G | GPU | nvmlDeviceGetPowerUsage | H / [NV](https://docs.nvidia.com/deploy/nvml-api/latest/api/group__nvmlDeviceQueries.html) | last, mean, max | Sensor update/averaging depends on hardware |
| **L4-012 GPU enforced power limit** | Active enforced power cap | W / G | GPU | nvmlDeviceGetEnforcedPowerLimit | H / [NV](https://docs.nvidia.com/deploy/nvml-api/latest/api/group__nvmlDeviceQueries.html) | last, changes | Compare with requested/configured cap separately |
| **L4-013 GPU energy consumed** | Reported accumulated energy | J / C | GPU | nvmlDeviceGetTotalEnergyConsumption | H / [NV](https://docs.nvidia.com/deploy/nvml-api/latest/api/group__nvmlDeviceQueries.html) | increase, rate | Availability and reset boundary depend on device |
| **L4-014 GPU and memory temperature** | Reported temperature by supported sensor | degrees C / G | GPU, sensor | DCGM_FI_DEV_GPU_TEMP; DCGM_FI_DEV_MEMORY_TEMP | H / [DF](https://docs.nvidia.com/datacenter/dcgm/latest/dcgm-api/dcgm-api-field-ids.html) | last, max | Missing memory sensor is not zero temperature |
| **L4-015 GPU SM/memory clocks** | Current reported clock frequency | MHz / G | GPU, clock domain | DCGM_FI_DEV_SM_CLOCK; DCGM_FI_DEV_MEM_CLOCK | H / [DF](https://docs.nvidia.com/datacenter/dcgm/latest/dcgm-api/dcgm-api-field-ids.html) | last, min, max | Sampling may miss transient clock changes |
| **L4-016 Clock event reasons** | Active source-defined clock-limiting reason mask | bitset / S | GPU, reason | nvmlDeviceGetCurrentClocksEventReasons | H / [NV](https://docs.nvidia.com/deploy/nvml-api/latest/api/group__nvmlDeviceQueries.html) | state changes, time in reason | Preserve overlapping reasons; do not sum enum values |
| **L4-017 Performance state** | Reported device performance state | state / S | GPU | DCGM_FI_DEV_GPU_PSTATE | H / [DF](https://docs.nvidia.com/datacenter/dcgm/latest/dcgm-api/dcgm-api-field-ids.html) | last, time in state | State is not utilization |
| **L4-018 Power/thermal violation duration** | Accumulated violation time by cause | ns / C | GPU, reason | DCGM_FI_DEV_POWER_VIOLATION; DCGM_FI_DEV_THERMAL_VIOLATION | H / [DF](https://docs.nvidia.com/datacenter/dcgm/latest/dcgm-api/dcgm-api-field-ids.html) | increase, fraction of interval | Counter support and semantics must be checked |
| **L4-087 Fan speed** | Reported fan duty/relative speed | percent / G | GPU, fan | DCGM_FI_DEV_FAN_SPEED | H / [DF](https://docs.nvidia.com/datacenter/dcgm/latest/dcgm-api/dcgm-api-field-ids.html) | last, max | Passive-cooled server GPUs may not expose local fans |

### GPU reliability

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L4-019 Xid error events** | Driver-reported error events grouped by Xid code | events / C | GPU, Xid code | driver event/log stream; Xid catalog | H / [XID](https://docs.nvidia.com/deploy/xid-errors/latest/index.html) | increase by code | Latest DCGM Xid field is a code, not event count |
| **L4-020 Volatile corrected/uncorrected ECC** | ECC totals since source reset boundary | errors / C | GPU, severity, location | DCGM_FI_DEV_ECC_SBE_VOL_TOTAL; DCGM_FI_DEV_ECC_DBE_VOL_TOTAL | H / [DF](https://docs.nvidia.com/datacenter/dcgm/latest/dcgm-api/dcgm-api-field-ids.html) | increase | Retain volatile versus persistent scope |
| **L4-021 Persistent ECC totals** | Aggregate ECC counts by severity/location | errors / C | GPU, severity, location | DCGM_FI_DEV_ECC_SBE_AGG_TOTAL; DCGM_FI_DEV_ECC_DBE_AGG_TOTAL | H / [DF](https://docs.nvidia.com/datacenter/dcgm/latest/dcgm-api/dcgm-api-field-ids.html) | increase, current total | Architecture-specific error granularity |
| **L4-022 Retired pages** | Pages retired by error class | pages / C | GPU, reason | DCGM_FI_DEV_PAGE_RETIRED_SBE_TOTAL; DCGM_FI_DEV_PAGE_RETIRED_DBE_TOTAL | H / [DF](https://docs.nvidia.com/datacenter/dcgm/latest/dcgm-api/dcgm-api-field-ids.html) | increase, current total | Legacy mechanism; generation-specific |
| **L4-023 Row remapping count** | Rows remapped by correction class | rows / C | GPU, reason | DCGM_FI_DEV_ROW_REMAP_CORRECTABLE_TOTAL; DCGM_FI_DEV_ROW_REMAP_UNCORRECTABLE_TOTAL | H / [DF](https://docs.nvidia.com/datacenter/dcgm/latest/dcgm-api/dcgm-api-field-ids.html) | increase, current total | Supported architectures only |
| **L4-024 Pending/failed memory repair** | Source-reported retirement or remapping state | boolean / S | GPU, repair type | DCGM pending/failed repair fields | H / [DF](https://docs.nvidia.com/datacenter/dcgm/latest/dcgm-api/dcgm-api-field-ids.html) | last, transitions | Do not treat missing support as healthy |
| **L4-025 Health status** | Reported passive health result by subsystem | state / S | GPU, subsystem | DCGM health watch results | H / [DH](https://docs.nvidia.com/datacenter/dcgm/latest/user-guide/feature-overview.html) | last, transitions | Health watch configuration is required |
| **L4-084 Remap spare capacity state** | Source-reported remaining memory-row spare state | state / S | GPU, bank class | DCGM bank-remap availability fields | H / [DF](https://docs.nvidia.com/datacenter/dcgm/latest/dcgm-api/dcgm-api-field-ids.html) | last, transitions | Architecture-specific; capacity classes are not linear percentages |

### GPU interconnect

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L4-026 PCIe transmit/receive throughput** | Source-reported data rate by direction | bytes/s / G | GPU, direction | DCGM_FI_PROF_PCIE_TX_BYTES; DCGM_FI_PROF_PCIE_RX_BYTES | H / [DP](https://docs.nvidia.com/datacenter/dcgm/latest/learn/modules/profiling.html) | mean, max | Verify field units despite BYTES suffix |
| **L4-027 PCIe replay count** | Reported transaction replays | events / C | GPU | DCGM_FI_DEV_PCIE_REPLAY_TOTAL | H / [DF](https://docs.nvidia.com/datacenter/dcgm/latest/dcgm-api/dcgm-api-field-ids.html) | rate, increase | Recovery activity; not proof of request impact |
| **L4-028 PCIe negotiated generation/width** | Current link generation and lane count | generation or lanes / G | GPU, component | DCGM_FI_DEV_PCIE_LINK_GEN; DCGM_FI_DEV_PCIE_LINK_WIDTH | H / [DF](https://docs.nvidia.com/datacenter/dcgm/latest/dcgm-api/dcgm-api-field-ids.html) | last, changes | Compare with supported maximum under load |
| **L4-029 NVLink transmit/receive throughput** | Source-reported traffic rate by direction | bytes/s / G | GPU or supported link, direction | DCGM NVLink profiling fields | H / [DP](https://docs.nvidia.com/datacenter/dcgm/latest/learn/modules/profiling.html) | mean, max | Aggregate versus per-link scope depends on generation |
| **L4-030 NVLink state** | Reported active/down/disabled/unsupported status | state / S | GPU, link | DCGM link-state API | H / [NL](https://docs.nvidia.com/datacenter/dcgm/latest/learn/core-services/topology-and-links.html) | last, transitions | Disabled or unsupported links are not automatically faults |
| **L4-031 NVLink CRC errors** | Reported FLIT/data CRC errors | errors / C | GPU, supported link, class | DCGM generation-specific NVLink error fields | H / [NL](https://docs.nvidia.com/datacenter/dcgm/latest/learn/core-services/topology-and-links.html) | rate, increase | Legacy classes are not universal on Blackwell+ |
| **L4-032 NVLink replay/recovery errors** | Reported replay or recovery counters | events / C | GPU, supported link, class | DCGM generation-specific NVLink error fields | H / [NL](https://docs.nvidia.com/datacenter/dcgm/latest/learn/core-services/topology-and-links.html) | rate, increase | Use generation-appropriate counter set |
| **L4-033 NVLink malformed/overrun/discard errors** | Reported newer-generation error classes | errors / C | GPU, error class | DCGM NVLink Error Detail fields | H / [NE](https://docs.nvidia.com/datacenter/dcgm/latest/reference/command-line-reference/dcgmi/dcgmi-nvlink.html) | rate, increase | Blackwell+ support; source field availability required |
| **L4-034 NVLink symbol/effective BER** | Reported bit error rates by supported class | ratio / G | GPU, class | DCGM generation-specific BER fields | H / [NE](https://docs.nvidia.com/datacenter/dcgm/latest/reference/command-line-reference/dcgmi/dcgmi-nvlink.html) | last, max | Do not sum BER across links |
| **L4-035 NVSwitch fatal/nonfatal health** | Reported switch health outcomes | state / S | switch, subsystem | DCGM NVSwitch health results | H / [DH](https://docs.nvidia.com/datacenter/dcgm/latest/user-guide/feature-overview.html) | last, transitions | Requires supported switch visibility |
| **L4-083 PCIe maximum supported generation/width** | Supported maximum link capability | generation or lanes / G | GPU, component | DCGM_FI_DEV_PCIE_MAX_LINK_GEN; DCGM_FI_DEV_PCIE_MAX_LINK_WIDTH | H / [DF](https://docs.nvidia.com/datacenter/dcgm/latest/dcgm-api/dcgm-api-field-ids.html) | last | Capability is a comparison baseline, not current bandwidth |

### Device tracing

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L4-036 CUDA host API duration** | Host exit time minus entry time for observed CUDA API | us / H | process, thread, API | supported uprobes or CUPTI API activities | I / [CU](https://docs.nvidia.com/cupti/main/main.html) | p50/p95/p99 | Not GPU kernel execution duration |
| **L4-037 GPU kernel execution duration** | Device end timestamp minus start timestamp | us / H | GPU, context, stream, kernel | CUPTI kernel start/end | I / [CK](https://docs.nvidia.com/cupti/api/structCUpti__ActivityKernel9.html) | p50/p95/p99 by kernel | Profiling integration required |
| **L4-038 GPU kernel queued-to-start delay** | Device start minus supported queued timestamp | us / H | GPU, context, stream, kernel | CUPTI queued/start timestamps | I / [CK](https://docs.nvidia.com/cupti/api/structCUpti__ActivityKernel9.html) | p50/p95/p99 | Optional timestamps; not always populated |
| **L4-039 GPU kernel submitted-to-start delay** | Device start minus supported submitted timestamp | us / H | GPU, context, stream, kernel | CUPTI submitted/start timestamps | I / [CK](https://docs.nvidia.com/cupti/api/structCUpti__ActivityKernel9.html) | p50/p95/p99 | Distinguish submission from host API return |
| **L4-040 GPU stream execution gap** | Next start minus previous end on same stream | us / H | GPU, context, stream | complete kernel/activity sequence | I / OWN | distribution, max | A stream gap does not prove device-wide idle |
| **L4-041 Observed device idle fraction** | Interval not covered by any captured device activity / interval | ratio / D | GPU, context coverage | union of complete kernel/copy activity intervals | I / OWN | interval union | Only label device-wide when all contexts/engines are covered |
| **L4-042 Kernel launch rate** | Observed kernel count divided by elapsed time | launches/s / D | GPU, process, kernel | CUPTI kernel records | I / [CK](https://docs.nvidia.com/cupti/api/structCUpti__ActivityKernel9.html) | count/time | CUDA Graphs and missing records affect coverage |
| **L4-043 Memory-copy duration** | Device copy end minus start | us / H | GPU, direction, stream | CUPTI memcpy start/end | I / [CA](https://docs.nvidia.com/cupti/api/group__CUPTI__ACTIVITY__API.html) | p50/p95/p99 | Classify H2D/D2H/D2D/P2P separately |
| **L4-044 Memory-copy bytes** | Bytes recorded for copy operation | bytes / C | GPU, direction | CUPTI memcpy bytes | I / [CA](https://docs.nvidia.com/cupti/api/group__CUPTI__ACTIVITY__API.html) | increase, rate | Counts explicit copies; excludes kernel HBM traffic |
| **L4-045 Memory-copy effective bandwidth** | Recorded copy bytes divided by device copy duration | bytes/s / D | GPU, direction, copy class | matched copy byte and timing records | I / OWN | size-stratified distribution | Do not equate per-copy rate with sustained link utilization |
| **L4-046 CUDA synchronization API duration** | Host duration of synchronization API | us / H | process, thread, API | CUPTI runtime/driver API records | I / [CU](https://docs.nvidia.com/cupti/main/main.html) | p50/p95/p99 | May include waiting for prior work; not pure barrier cost |

### Host CPU

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L4-047 CPU time by mode** | Accumulated CPU accounting by execution mode | s / C | host, CPU, mode | /proc/stat cpu fields | H / [PROC](https://docs.kernel.org/filesystems/proc.html) | rate by mode | iowait is not precise per-task blocked time |
| **L4-048 Process CPU time** | Accumulated user/system CPU time | s / C | process identity | /proc/PID/stat utime/stime | H / [PROC](https://docs.kernel.org/filesystems/proc.html) | rate | Normalize clock ticks; identity includes start time |
| **L4-049 Host runnable and blocked tasks** | Current runnable/blocked task counts | tasks / G | host, state | /proc/stat procs_running/procs_blocked | H / [PROC](https://docs.kernel.org/filesystems/proc.html) | last, max | Snapshot counts; not historical scheduler latency |
| **L4-050 Context switches** | Host or process context-switch counts | switches / C | host or process, class | /proc/stat ctxt; /proc/PID/status | H / [PROC](https://docs.kernel.org/filesystems/proc.html) | rate | Host/process counters have different scopes |
| **L4-051 Thread runqueue wait** | Time runnable but awaiting CPU | ns / C | thread or CPU | /proc/PID/schedstat; /proc/schedstat | H / [SCHED](https://docs.kernel.org/scheduler/sched-stats.html) | increase, mean per timeslice | CONFIG_SCHEDSTATS and format/version dependent |
| **L4-052 CPU scheduling latency** | Runnable transition to scheduled execution | us / H | process, thread, CPU | sched wakeup/switch event correlation | H / [TRACE](https://www.kernel.org/doc/html/latest/trace/events.html) | p50/p95/p99, max | Custom eBPF derivation; include preempted runnable tasks |
| **L4-053 Off-CPU duration** | Deschedule-to-reschedule interval | us / H | thread, prior state | sched_switch event pairs | H / [TRACE](https://www.kernel.org/doc/html/latest/trace/events.html) | p50/p95/p99 | Includes sleeping and waiting; not all time is contention |
| **L4-054 Cgroup CPU usage** | Accumulated cgroup CPU time | us / C | cgroup | cpu.stat usage_usec/user_usec/system_usec | H / [CG](https://docs.kernel.org/admin-guide/cgroup-v2.html) | rate | Do not add parent and descendant totals |
| **L4-055 Cgroup CPU throttling** | Quota-throttled periods and duration | periods or us / C | cgroup, component | cpu.stat nr_throttled/throttled_usec | H / [CG](https://docs.kernel.org/admin-guide/cgroup-v2.html) | increase, rate | Quota enforcement; not all scheduler waiting |

### Host pressure

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L4-056 CPU pressure** | Source-reported runnable resource stall time | us / C; ratio / G | host or cgroup | cpu.pressure some/full where supported | H / [PSI](https://docs.kernel.org/accounting/psi.html) | delta total / elapsed; native averages | Keep some/full separate; host CPU full availability differs |
| **L4-057 Memory pressure** | Time tasks are stalled on memory pressure | us / C; ratio / G | host or cgroup | memory.pressure some/full | H / [PSI](https://docs.kernel.org/accounting/psi.html) | delta total / elapsed; native averages | Not equivalent to memory occupancy |
| **L4-058 I/O pressure** | Time tasks are stalled on I/O resources | us / C; ratio / G | host or cgroup | io.pressure some/full | H / [PSI](https://docs.kernel.org/accounting/psi.html) | delta total / elapsed; native averages | Not a device latency histogram |

### Host memory

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L4-059 Host memory available** | Kernel estimate of memory available without swapping | bytes / G | host | /proc/meminfo MemAvailable | H / [PROC](https://docs.kernel.org/filesystems/proc.html) | last, min | Prefer explicit kernel estimate over total-minus-free |
| **L4-060 Process resident memory** | Resident process memory | bytes / G | process identity | /proc/PID/status VmRSS | H / [PROC](https://docs.kernel.org/filesystems/proc.html) | last, max | Shared pages may be counted in multiple processes |
| **L4-061 Cgroup memory usage** | Memory charged to cgroup | bytes / G | cgroup | memory.current | H / [CG](https://docs.kernel.org/admin-guide/cgroup-v2.html) | last, max | Includes more than anonymous process RSS |
| **L4-062 Cgroup memory limit events** | Reported low/high/max/oom/oom_kill events by class | events / C | cgroup, reason | memory.events | H / [CG](https://docs.kernel.org/admin-guide/cgroup-v2.html) | increase | Preserve hierarchy versus memory.events.local |
| **L4-063 Cgroup swap usage** | Swap charged to cgroup | bytes / G | cgroup | memory.swap.current | H / [CG](https://docs.kernel.org/admin-guide/cgroup-v2.html) | last, max | Not KV offload accounting |
| **L4-064 Minor/major page faults** | Process page-fault counts by class | faults / C | process identity, class | /proc/PID/stat minflt/majflt | H / [PROC](https://docs.kernel.org/filesystems/proc.html) | rate, increase | Major faults imply storage work; not always swap |
| **L4-065 NUMA memory placement** | Cgroup pages/bytes accounted by NUMA node and category | native pages or bytes / G | cgroup, NUMA node, category | memory.numa_stat | H / [CG](https://docs.kernel.org/admin-guide/cgroup-v2.html) | last, distribution | Placement does not directly measure remote access latency |
| **L4-082 Host swap input/output** | Pages moved into/out of swap | pages / C | host, direction | /proc/vmstat pswpin/pswpout | H / [PROC](https://docs.kernel.org/filesystems/proc.html) | rate, increase | Convert with actual page size when reporting bytes |

### Host storage

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L4-066 Block read/write bytes** | Completed block I/O data by direction | bytes / C | block device, direction | /sys/block/DEV/stat sectors read/written | H / [BLOCK](https://docs.kernel.org/block/stat.html) | rate, increase | Convert documented sectors; layered devices can double-count |
| **L4-067 Block I/O operations** | Completed reads/writes by direction | operations / C | block device, direction | /sys/block/DEV/stat completed operations | H / [BLOCK](https://docs.kernel.org/block/stat.html) | rate | Distinguish operations from merged requests |
| **L4-068 Block I/O service time** | Accumulated read/write completion time | ms / C | block device, direction | /sys/block/DEV/stat read/write ticks | H / [BLOCK](https://docs.kernel.org/block/stat.html) | delta time / delta completed operations | Mean only; overlapping operations accumulate time |
| **L4-069 In-flight block I/O** | Currently issued incomplete I/O operations | operations / G | block device | /sys/block/DEV/stat in_flight | H / [BLOCK](https://docs.kernel.org/block/stat.html) | last, max | Snapshot may miss brief queue buildup |
| **L4-070 Cgroup I/O traffic** | Bytes and operations charged by direction | bytes or operations / C | cgroup, device, component | io.stat rbytes/wbytes/rios/wios | H / [CG](https://docs.kernel.org/admin-guide/cgroup-v2.html) | rate, increase | Do not add parent and descendant totals |

### Host networking

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L4-071 Interface transmitted/received bytes** | Interface data counters by direction | bytes / C | interface, direction | rtnl_link_stats64 tx_bytes/rx_bytes | H / [NET](https://docs.kernel.org/networking/statistics.html) | rate, increase | Boundary differs from physical-port wire bytes |
| **L4-072 Interface transmitted/received packets** | Interface packet counters by direction | packets / C | interface, direction | rtnl_link_stats64 tx_packets/rx_packets | H / [NET](https://docs.kernel.org/networking/statistics.html) | rate, increase | Offload and interface layering affect interpretation |
| **L4-073 Interface errors and drops** | Reported errors/drops by direction and class | events / C | interface, reason | rtnl_link_stats64 rx/tx errors and dropped | H / [NET](https://docs.kernel.org/networking/statistics.html) | rate, increase | Do not sum overlapping aggregate/detail counters |
| **L4-074 NIC link state and speed** | Reported operational state and negotiated speed | state or bit/s / S,G | interface | netlink/ethtool supported link data | H / [NET](https://docs.kernel.org/networking/statistics.html) | last, transitions | Unknown speed is not zero-speed traffic |

### Lifecycle

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L4-075 Process/container restarts** | Observed lifecycle transitions counted by identity | events / C | host, workload identity | runtime lifecycle events plus process start identities | H / OWN | increase, rate | Requires discovery history; PID reuse must be handled |
| **L4-076 Expected versus observed rank membership** | Observed participating ranks and missing members of a known replica | ranks / G | replica, communicator | verified expected topology and live member identities | X / OWN | last, changes | Expected count must come from configuration or library metadata |

### Derived infrastructure

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L4-077 Parallel-group clock dispersion** | Maximum minus minimum clock within matched comparable devices | MHz / D | replica, clock domain | aligned current clocks and verified membership | D / OWN | range, distribution | Compare equivalent hardware and workload phase |
| **L4-078 Observed GPU activity imbalance** | Dispersion of matched interval activity across a defined group | ratio / D | replica, parallel group | aligned device activity and membership | D / OWN | range, CV | Diagnostic correlation; does not establish a straggler cause |
| **L4-079 Output tokens per GPU-second** | Logical output tokens / allocated GPU-seconds in same interval | tokens/GPU-s / D | replica, hardware class | logical outputs and verified allocation timeline | D / OWN | ratio of sums | Include all assigned parallel ranks; cannot infer allocation from activity |
| **L4-080 Energy per output token** | Matched GPU energy increase / logical output tokens | J/token / D | replica, boundary | energy counters and logical outputs | D / OWN | ratio of sums | Undefined with zero outputs; GPU-only unless other power is measured |
| **L4-081 Measured FLOP or bandwidth efficiency** | Measured or estimated throughput / matching documented theoretical limit | ratio / D | GPU, datatype, operation class | explicit throughput source and verified peak specification | D / OWN | ratio with matching precision | Keep estimated and hardware-measured sources separate; no default peak assumed |

### GPU topology health

| ID / metric | Definition or calculation | Unit / type | Scope | Native source / required inputs | Mode / source | Aggregation | Diagnostic use / limits |
|---|---|---|---|---|---|---|---|
| **L4-085 GPU fabric readiness** | Reported fabric initialization/health state | state / S | GPU, fabric | DCGM fabric health/probe status | H / [DH](https://docs.nvidia.com/datacenter/dcgm/latest/user-guide/feature-overview.html) | last, transitions | Requires platform-specific fabric service support |
| **L4-086 IMEX domain status** | Reported inter-node memory-exchange state | state / S | domain, host | DCGM IMEX status fields | H / [DH](https://docs.nvidia.com/datacenter/dcgm/latest/user-guide/feature-overview.html) | last, transitions | Relevant only when IMEX is deployed |

## Cross-layer context: metadata, not additional utilization metrics

Attach only identities supported by evidence. Retain unknown values explicitly; do not infer model/rank/tenant ownership from a PID alone.

| Context | Required meaning |
|---|---|
| Cluster and host | Stable cluster identity, host identity and boot ID. |
| Process | Host PID, start time and relevant namespace IDs; PID alone is reusable. |
| Container and cgroup | Runtime container identity, cgroup identity and validity interval. |
| Workload | Model identity/version, engine/version, replica and deployment identities when reported or configured. |
| Parallelism | TP/PP/DP/EP/CP role and rank only when established; communicator ID and operation sequence for collective matching. |
| Device | GPU UUID, PCI identity, MIG GPU/compute instance when applicable; no assumption of one process per GPU. |
| Fabric | NIC identity, port, priority, peer and switch path only when known; CPU/GPU/NIC NUMA placement is topology metadata. |
| Request/event correlation | Request, batch, iteration, microbatch, expert and stream IDs belong to detailed records where available, not labels on every metric. |
| Time | Event time, observation time, monotonic clock domain, wall-clock mapping and uncertainty, plus measurement interval. |
| Provenance | Native metric/field, unit conversion, software/driver version, reported/inferred classification and confidence. |
| Measurement validity | Supported/unsupported/permission-denied/stale/missing/dropped/partial status. Missing and unsupported are not zero. |

## Correctness rules for the complete catalog

1. Declare the timing boundary. Server TTFT, client TTFT, host API duration, GPU execution, stream gaps and network round trips describe different intervals.
2. Declare the counting population. Logical outputs must not be counted once per TP rank; token assignments in MoE are not unique input tokens; cache hits can be token-, block-, item- or request-weighted.
3. Preserve native counter scope. Do not add parent/child cgroups, layered interfaces, aggregate/detail errors, or overlapping cache counters.
4. Preserve device generation and capability. DCGM profiling metrics are interval measurements; metric multiplexing and tool conflicts affect availability. GPU architecture, partitioning and drivers determine which fields exist.
5. Mark conditional semantic metrics honestly. Expert routing, stage boundaries, request abort reasons and expected replica membership cannot be recovered universally through generic host probes.
6. Join distributed events by communicator/sequence or verified semantic IDs, not nearby timestamps alone. Cross-host deltas must include clock uncertainty. PTP support is not proof of achieved accuracy.
7. Do not infer cause from one metric. Long collectives, high memory activity, high queue depth or low SM activity are evidence to correlate, not standalone root-cause verdicts.
8. Keep telemetry source precision separate from engine polling. Faster reads can return the same underlying sample. Counter polling cannot reconstruct the timing of events within its interval.
9. Missing SLOs, cost inputs, peak-throughput specifications and expected topology remain unspecified. Derived quantities needing them remain unavailable until supplied.
10. Histogram bucket boundaries must resolve the target latency range. Existing coarse histograms cannot retrospectively reveal sub-millisecond tails.

## Aether measurement-health companion inventory

These are proposed internal engine measurements needed to trust all four layers. They are not a fifth customer-workload layer and are not vendor-native exports.

| Metric family | Unit / type | Definition / purpose |
|---|---|---|
| Collector capability state | state / S | Supported, disabled, permission denied, unavailable or incompatible per collector/source. |
| Captured, retained and dropped records | records / C | Separate counts by collector and reason; report only stages with observable accounting. |
| Correlation coverage | ratio / D | Records mapped to required verified identities / eligible retained records. |
| Source freshness | s / G | Time since last valid source observation, using an appropriate clock mapping. |
| Clock mapping uncertainty | ns / G | Measured/estimated uncertainty per clock-domain mapping; label unknown when unbounded. |
| Ring-buffer capacity/use | bytes or records / G | Configured capacity and occupied amount per buffer; do not mix units. |
| Queue residence time | s / H | Time records spend in each Aether stage. |
| Sampling/retention fraction | ratio / D | Retained observations / eligible observed observations when denominator is known. |
| Agent CPU and resident memory | s / C; bytes / G | Engine resource cost, excluding inference processes. |
| Local spool capacity/use | bytes / G | Durable pending telemetry and configured spool limit. |
| Oldest uncommitted record age | s / G | Age of oldest locally accepted record not durably committed remotely. |
| Upload attempts/retries/failures | operations / C | Object writes by outcome, with retry attempt counts separated from unique objects. |
| Upload and manifest commit duration | s / H | Data upload and query-visibility commit timings measured separately. |
| Persisted data volume and file count | bytes or files / C | Successfully committed data volume/file count, distinct from attempts. |
| Query-visible freshness | s / G | Age/watermark of committed, discoverable data, with per-source coverage. |
| Query duration and failures | s / H; queries / C | Interactive/analytical engine health by query class and outcome. |
| Compaction backlog and duration | bytes / G; s / H | Pending eligible bytes and time per compaction job. |
| Alert evaluation/delivery delay | s / H | Delay from qualifying observation/window to evaluation and delivery. |

## What is still required to certify deployment completeness

The catalog can be reviewed without choosing these values. A claim of complete collection cannot be certified until the following are known: serving engines and versions; NVIDIA GPU models and partition/MPS configuration; CUDA/DCGM/NVML/CUPTI/NCCL versions; Linux kernel/cgroup/runtime versions; NIC/driver/firmware models; InfiniBand/RoCE/TCP and switch access; permitted instrumentation methods; exposed endpoints and optional metrics; model architecture/parallelism; and explicit SLO definitions.

Future adapters must enumerate their native fields and compare them with this catalog. Any unmatched native metric should be reviewed rather than silently discarded. Source lists change; linked latest/main documents are research references, not pinned compatibility contracts.

## Primary sources

- **V**: [vLLM production metrics](https://docs.vllm.ai/en/latest/usage/metrics/)
- **VD**: [vLLM metric semantics](https://docs.vllm.ai/en/latest/design/metrics/)
- **VL**: [vLLM metric collector implementation](https://github.com/vllm-project/vllm/blob/main/vllm/v1/metrics/loggers.py)
- **VS**: [vLLM speculation metrics implementation](https://github.com/vllm-project/vllm/blob/main/vllm/v1/spec_decode/metrics.py)
- **SG**: [SGLang production metrics](https://docs.sglang.io/docs/references/production_metrics)
- **TGI**: [TGI metric reference](https://huggingface.co/docs/text-generation-inference/reference/metrics)
- **TR**: [Triton metrics](https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/user_guide/metrics.html)
- **RT**: [TensorRT-LLM metric collector](https://github.com/NVIDIA/TensorRT-LLM/blob/main/tensorrt_llm/metrics/collector.py)
- **TB**: [TensorRT-LLM Triton backend](https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/tensorrtllm_backend/README.html)
- **NCCL**: [NCCL Inspector](https://github.com/NVIDIA/nccl/blob/master/plugins/profiler/inspector/README.md)
- **NENV**: [NCCL profiler configuration](https://docs.nvidia.com/deeplearning/nccl/user-guide/docs/env.html)
- **DF**: [DCGM field identifiers](https://docs.nvidia.com/datacenter/dcgm/latest/dcgm-api/dcgm-api-field-ids.html)
- **DP**: [DCGM profiling](https://docs.nvidia.com/datacenter/dcgm/latest/learn/modules/profiling.html)
- **DH**: [DCGM health](https://docs.nvidia.com/datacenter/dcgm/latest/user-guide/feature-overview.html)
- **NL**: [DCGM topology and links](https://docs.nvidia.com/datacenter/dcgm/latest/learn/core-services/topology-and-links.html)
- **NE**: [DCGM NVLink error reference](https://docs.nvidia.com/datacenter/dcgm/latest/reference/command-line-reference/dcgmi/dcgmi-nvlink.html)
- **NV**: [NVML device queries](https://docs.nvidia.com/deploy/nvml-api/latest/api/group__nvmlDeviceQueries.html)
- **XID**: [NVIDIA Xid reference](https://docs.nvidia.com/deploy/xid-errors/latest/index.html)
- **CK**: [CUPTI kernel activity](https://docs.nvidia.com/cupti/api/structCUpti__ActivityKernel9.html)
- **CA**: [CUPTI Activity API](https://docs.nvidia.com/cupti/api/group__CUPTI__ACTIVITY__API.html)
- **CU**: [CUPTI usage and deployment](https://docs.nvidia.com/cupti/main/main.html)
- **IB**: [Linux InfiniBand ABI](https://kernel.googlesource.com/pub/scm/linux/kernel/git/torvalds/linux/+/8c13415c8a4383447c21ec832b20b3b283f0e01a/Documentation/ABI/stable/sysfs-class-infiniband)
- **NIC**: [NVIDIA NIC telemetry](https://networking-docs.nvidia.com/doca/archive/3-2-1/doca-telemetry-service-guide)
- **ROCE**: [NVIDIA RDMA counters](https://docs.nvidia.com/networking/display/bfswtroubleshooting/doca-services.pdf)
- **NET**: [Linux interface statistics](https://docs.kernel.org/networking/statistics.html)
- **TCP**: [Linux protocol counters](https://docs.kernel.org/networking/snmp_counter.html)
- **CG**: [Linux cgroup v2](https://docs.kernel.org/admin-guide/cgroup-v2.html)
- **PROC**: [Linux proc filesystem](https://docs.kernel.org/filesystems/proc.html)
- **PSI**: [Linux pressure stall information](https://docs.kernel.org/accounting/psi.html)
- **SCHED**: [Linux scheduler statistics](https://docs.kernel.org/scheduler/sched-stats.html)
- **TRACE**: [Linux event tracing](https://www.kernel.org/doc/html/latest/trace/events.html)
- **BLOCK**: [Linux block statistics](https://docs.kernel.org/block/stat.html)
- **PTP**: [Linux PTP clocks](https://cdn.kernel.org/doc/html/latest/driver-api/ptp.html)
