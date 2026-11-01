# Step 1: native metrics and dimensions — vLLM, SGLang and NVIDIA Dynamo

Research date: 1 November 2026.

## Purpose and boundary

Establish what the software reviewed first already exposes before designing Aether enrichment or additional host instrumentation. This document complements [the four-layer requirements catalog](aether-metrics-catalog.md). It does not change that catalog into a claim of implemented or universally available collection.

The inventory groups related native metric families. It distinguishes the emitting process, native dimensions, optional features, and measurement boundaries. It is a documentation baseline, not a scrape of a running deployment or an exhaustive inventory of every plugin, development branch and library. No deployment versions, enabled flags, GPU models, NIC models or fabric configurations have been assumed.

Only this step is performed here. Hardware enrichment, hardware/fabric collectors, gap prioritization and Rust implementation are subsequent steps.

Reviewing vLLM, SGLang and Dynamo first does not limit Aether to those engines. No exclusive product-support list has been agreed; all 250 requirements remain.

## Evidence baseline

| Reference | Evidence used | Version limitation |
|---|---|---|
| [V1: vLLM production metrics](https://docs.vllm.ai/en/latest/usage/metrics/) | Native families, types and feature gates | Rolling `latest` documentation; installed release not supplied |
| [V2: vLLM metric semantics](https://docs.vllm.ai/en/latest/design/metrics/) | Request/token timing and aggregation distinctions | Rolling `latest` documentation |
| [V3: vLLM collector](https://github.com/vllm-project/vllm/blob/main/vllm/v1/metrics/loggers.py) | Collector implementation reference | `main` is not a pinned deployed build |
| [V4: vLLM speculation collector](https://github.com/vllm-project/vllm/blob/main/vllm/v1/spec_decode/metrics.py) | Draft and accepted-token families | Feature- and version-dependent |
| [S1: SGLang production metrics](https://docs.sglang.io/docs/references/production_metrics) | Endpoint setup, examples and request TPOT semantics | Rolling documentation; installed release not supplied |
| [S2: SGLang collector](https://github.com/sgl-project/sglang/blob/main/python/sglang/srt/observability/metrics_collector.py) | Native collector definitions | `main` is not a pinned deployed build |
| [S3: SGLang scheduler reporting](https://github.com/sgl-project/sglang/blob/main/python/sglang/srt/managers/scheduler_components/metrics_reporter.py) | Scheduler measurement/reporting context | Build and feature flags affect reporting |
| [A1: NVIDIA AIPerf server metric reference](https://docs.nvidia.com/aiperf/server-metrics/ai-perf-server-metrics-reference) | Cross-check of engine families and per-family dimensions | Page identified itself as AIPerf v0.13.0; it is a consumer reference, not proof of the installed engine version |
| [D1: Dynamo metrics catalog](https://docs.dynamo.nvidia.com/dynamo/reference/observability/metrics-catalog) | Frontend, component, router and endpoint-picker families | Page identified itself as Latest v1.6.1 when inspected; do not substitute the `dev` catalog |
| [D2: Dynamo labels](https://docs.dynamo.nvidia.com/dynamo/reference/observability/metric-labels) | Origins of labels and their scopes | This linked page identified itself as v1.4.2 when inspected. Treat details not corroborated by D1 as historical context, not a v1.6.1 compatibility guarantee |

The visible Dynamo catalog and label pages reported different versions. That inconsistency is retained explicitly rather than silently presenting a single verified schema. Family-specific labels confirmed by D1 are used below; a deployed exposition is required to certify the complete set.

## Reading the inventory

- C = counter; G = gauge; H = histogram. A row listing several families is a group, not one combined metric.
- Names retain the spelling used by the referenced source. Prometheus counters can expose a `_total` sample suffix; AIPerf normalizes some counter family names by removing that suffix. Never strip `_total` indiscriminately: `vllm:iteration_tokens_total` is documented as a histogram.
- H does not mean raw events are exported. An endpoint histogram provides aggregate buckets/count/sum, not the original event sequence or request IDs.
- A metric may be absent until enabled, until matching traffic occurs, or until a worker exists. Absence is not a zero measurement.
- A latest-value gauge cannot reconstruct every underlying iteration or the exact maximum between polls.
- Labels listed as common are not guaranteed on every family. Config/info metrics and subsystem collectors have exceptions.
- CPU/GPU hardware measurements, attached external-exporter series and scraper labels must not be attributed to the engine merely because they appear in the same dashboard.

## 1. Endpoints and emitting boundaries

| Software / emitter | Interface | Availability condition | Observation boundary |
|---|---|---|---|
| vLLM API server | Prometheus `/metrics` on its configured serving endpoint | Accessible metrics endpoint; individual families can be optional | Engine/API observations and engine-indexed statistics |
| SGLang server | Prometheus `/metrics` on configured server endpoint | `--enable-metrics`; further flags for optional features | Request/tokenizer and scheduler collectors have different scopes |
| Dynamo frontend | `/metrics` on its HTTP port | Running frontend and enabled/reachable serving endpoint | Framework frontend, request pipeline and frontend-local worker views |
| Dynamo backend component | `/metrics` on `DYN_SYSTEM_PORT` | System port must be configured in the inspected stable catalog | Framework worker/RPC handling plus backend metrics where exposed |
| Dynamo router | Frontend HTTP endpoint or standalone router system endpoint | Depends on deployment and router mode | Routing observations, not a replacement for engine scheduler metrics |
| Dynamo endpoint picker | Dedicated metrics endpoint | EPP deployment; its metrics port must not be disabled | Gateway endpoint-selection/response observations |

Source: V1, S1, D1. Ports are discovery inputs, not hard-coded Aether assumptions. No existing endpoint means host-only collection cannot simply retrieve its semantic metrics.

## 2. Native dimension sets

These symbols shorten the tables; they are not proposed Aether labels.

| Symbol | Documented native dimensions | Scope and qualifications |
|---|---|---|
| V | `model_name`, `engine` | Common vLLM V1 metric dimensions in A1; `engine` is an engine index, not a GPU UUID or universal TP rank |
| S-request | `model_name`, `engine_type` | Common SGLang request/tokenizer dimensions in A1; older examples in S1 show fewer labels |
| S-scheduler | `model_name`, `engine_type`, `tp_rank`, `pp_rank`, `moe_ep_rank` | Scheduler dimensions in A1; add `dp_rank` when present and configured extra labels where supported |
| D-frontend | `model` on most frontend families | Frontend-local registration; do not attach component hierarchy labels automatically |
| D-component | `dynamo_namespace`, `dynamo_component`, `dynamo_endpoint` | Hierarchy-registered component metrics in D1; frontend-local families are exceptions |
| D-worker-view | `worker_id`, `dp_rank`, `worker_type` | D1 confirms these for frontend per-worker load/timing gauges; emitted on frontend rather than standalone router |
| D-router | Family-specific runtime/component and routing dimensions | Use the actual family exposition; not one universal schema. D1 explicitly documents `model`, `phase`, `worker_type` or `router_id` on particular groups |

Additional dimensions documented in the sources:

| Software | Native dimension | Applies to |
|---|---|---|
| vLLM | `reason` | Reason-specific waiting/admission families where emitted |
| vLLM | `source` | Prompt-token accounting by source |
| vLLM | `finished_reason` | Request completion/outcome family |
| vLLM | `sleep_state` | Engine sleep-state family |
| vLLM | Draft position | Speculative position-specific family; verify exact label key in the build |
| vLLM | Cache configuration / adapter-list labels | Config/info families; not labels on all request metrics |
| SGLang | `priority` | Enabled priority scheduling; total and per-priority series must not be added together |
| SGLang | `mode` | Token/graph-use categories |
| SGLang | `category`, `num_prefill_ranks` | Execution accounting and DP-cooperation families where defined |
| SGLang | `cache_source` | Source-specific cached token counters |
| SGLang | `stage` | Per-stage request-latency histogram |
| SGLang | `layer` | Documented expert-placement distribution family |
| SGLang | Configured extra labels | User configuration; their presence or accuracy is not assumed |
| Dynamo | `stage`, `phase` | Frontend pipeline requests; routing phase on selected router families |
| Dynamo | `error_type` | Component error family |
| Dynamo | `endpoint`, `request_type` | Frontend cancellation family; API endpoint differs from RPC hierarchy endpoint |
| Dynamo | `migration_type`, `outcome` | Applicable request-migration families |
| Dynamo | `policy_class`, `reason` | Router queue and backpressure families |
| Dynamo | `status`, `event_type` | Router KV-index event accounting |

Source: A1, V1–V4, S1–S3, D1. D2 explains historical label origins; it is not used to guarantee uncorroborated current values.

## 3. vLLM native metric groups

V denotes the common dimensions above. Additional keys or exceptions are stated. Availability refers to source-documented emission, not an installed Aether adapter.

| Group | Native families | Type | Dimensions | Availability / boundary |
|---|---|---|---|---|
| Request outcomes | `vllm:request_success` | C | V + `finished_reason` where emitted | Preserve finish-reason semantics; the name alone does not establish an application-level success policy |
| Admission / numerical outcomes | `vllm:admission_rejections`, `vllm:corrupted_requests` | C | V; reason-specific labels where defined | Numerical-check reporting is optional; inspect actual configuration |
| Token totals | `vllm:prompt_tokens`, `vllm:generation_tokens` | C | V | Logical token accounting; do not multiply by TP workers |
| Prompt source accounting | `vllm:prompt_tokens_by_source`, `vllm:prompt_tokens_cached`, `vllm:prompt_tokens_cached_by_source` | C | V + source/tier where defined | Preserve overlapping aggregate and breakdown populations |
| Request duration | `vllm:e2e_request_latency_seconds` | H | V | Engine/server boundary, not client-observed network latency |
| Request phase duration | `vllm:request_queue_time_seconds`, `vllm:request_prefill_time_seconds`, `vllm:request_decode_time_seconds`, `vllm:request_inference_time_seconds` | H | V | Distinct engine phases; not device kernel timelines |
| First token | `vllm:time_to_first_token_seconds` | H | V | Native first-output timing definition applies |
| Stream gap and request TPOT | `vllm:inter_token_latency_seconds`, `vllm:request_time_per_output_token_seconds` | H | V | Output-event gap and request-average TPOT are different populations |
| Request sizes / parameters | `vllm:request_prompt_tokens`, `vllm:request_generation_tokens`, `vllm:request_max_num_generation_tokens`, `vllm:request_params_max_tokens`, `vllm:request_params_n` | H | V | Requested limits are not actual generated work |
| Running / queued requests | `vllm:num_requests_running`, `vllm:num_requests_waiting`, `vllm:num_requests_waiting_by_reason` | G | V; `reason` on breakdown | Engine scheduling state |
| Async cache waiting | `vllm:num_requests_kv_fetch_by_stage` | G | V + native stage labels | Async KV-load path only |
| Preemption | `vllm:num_preemptions`, `vllm:request_num_preemptions` | C / H respectively | V | Total events versus per-request distribution |
| Iteration workload | `vllm:iteration_tokens_total` | H | V | Tokens per step; histogram despite its suffix |
| Device KV allocation | `vllm:kv_cache_usage_perc` | G | V | Ratio; different from total GPU memory occupancy |
| Local prefix reuse | `vllm:prefix_cache_hits`, `vllm:prefix_cache_queries` | C | V | Token-weighted counts |
| External prefix reuse | `vllm:external_prefix_cache_hits`, `vllm:external_prefix_cache_queries` | C | V | Connector-dependent |
| Computed prefill work | `vllm:request_prefill_kv_computed_tokens` | H | V | Excludes cached prefill tokens |
| Multimodal reuse | `vllm:mm_cache_hits`, `vllm:mm_cache_queries` | C | V | Item-weighted, feature-dependent |
| Cache residency | `vllm:kv_block_lifetime_seconds`, `vllm:kv_block_idle_before_evict_seconds`, `vllm:kv_block_reuse_gap_seconds` | H | V | Optional sampled KV-cache metrics |
| Speculation | `vllm:spec_decode_num_drafts`, `vllm:spec_decode_num_draft_tokens`, `vllm:spec_decode_num_accepted_tokens`, `vllm:spec_decode_num_accepted_tokens_per_pos` | C | V + position on position family | Speculation enabled; source V4 supplies families omitted from some short docs |
| NIXL transfer accounting | `vllm:nixl_bytes_transferred`, `vllm:nixl_num_descriptors`, `vllm:nixl_post_time_seconds`, `vllm:nixl_xfer_time_seconds` | H | Connector dimensions must be inspected | NIXL connector; engine wrapper measurements, not NIC counters |
| NIXL failures / expiry | `vllm:nixl_num_failed_notifications`, `vllm:nixl_num_failed_transfers`, `vllm:nixl_num_kv_expired_reqs`, `vllm:nixl_num_notifications_after_expiry` | C | Connector dimensions must be inspected | Failure totals overlap; expiry and notifications have different units |
| HiSparse state | `vllm:hisparse_host_cache_usage_perc`, `vllm:hisparse_pending_page_transfers` | G | Connector-specific | HiSparse enabled; host cache differs from GPU KV allocation |
| HiSparse traffic / reuse | `vllm:hisparse_cache_hits`, `vllm:hisparse_cache_misses`, `vllm:hisparse_host_to_device_bytes` | C | Connector-specific | HiSparse enabled |
| HiSparse host residency | `vllm:hisparse_host_block_lifetime_seconds`, `vllm:hisparse_host_block_idle_before_evict_seconds`, `vllm:hisparse_host_block_reuse_gap_seconds` | H | Connector-specific | Optional sampled host-pool metrics |
| CPU/disk offload | `vllm:simple_kv_offload_load_blocks_total`, `vllm:simple_kv_offload_save_outcomes_total` | C | Connector outcome labels where defined | SimpleCPUOffloadConnector; store outcome classification is mode-dependent |
| CPU/disk offload state | `vllm:simple_kv_offload_pending_store_blocks`, `vllm:simple_kv_offload_used_blocks`, `vllm:simple_kv_offload_info` | G | Connector configuration / outcome labels | Info gauge is metadata; evictable warm blocks differ from pinned blocks |
| Estimated per-GPU work | `vllm:estimated_flops_per_gpu_total`, `vllm:estimated_read_bytes_per_gpu_total`, `vllm:estimated_write_bytes_per_gpu_total` | C | V | Requires MFU metrics; estimates rather than hardware counters |
| Sleep / adapters / config | `vllm:engine_sleep_state`, `vllm:lora_requests_info`, `vllm:cache_config_info` | G / info | State, adapter or config labels; common-set exceptions | LoRA/config families depend on build and configuration |

Sources: V1 for the production list; V2 for timing semantics; V3/V4 for collector definitions; A1 for common labels/config-info cross-checks. Endpoints may also contain library/runtime metrics outside the vLLM namespace.

## 4. SGLang native metric groups

S-request and S-scheduler are separate native label sets. The names below use family notation; counter wire samples commonly append `_total`.

| Group | Native families | Type | Dimensions | Availability / boundary |
|---|---|---|---|---|
| Logical token totals | `sglang:prompt_tokens`, `sglang:generation_tokens` | C | S-request | Request-level token accounting |
| Cached token sources | `sglang:cached_tokens` | C | S-request + `cache_source` | Source/tier and total may overlap |
| Request outcomes | `sglang:num_requests`, `sglang:num_aborted_requests`, `sglang:num_so_requests` | C | S-request | Completed/aborted/structured-output populations differ |
| First token / request duration | `sglang:time_to_first_token_seconds`, `sglang:e2e_request_latency_seconds` | H | S-request | S1 specifies different first-token endpoint semantics for streaming/non-streaming |
| Output gaps / TPOT | `sglang:inter_token_latency_seconds`, `sglang:request_time_per_output_token_seconds` | H | Request labels; TPOT also `is_streaming` in S1 | TPOT excludes specified short/aborted/prefill-worker cases |
| Request lengths | `sglang:prompt_tokens_histogram`, `sglang:uncached_prompt_tokens_histogram`, `sglang:generation_tokens_histogram` | H | S-request | Uncached input is distinct from logical input length |
| Load API duration | `sglang:get_loads_duration_seconds` | H | S-request | Load-endpoint handling, not inference duration |
| Current generation throughput | `sglang:gen_throughput` | G | S-scheduler | Reported window; not an event-by-event trace |
| Execution token accounting | `sglang:realtime_tokens`, `sglang:dp_cooperation_realtime_tokens` | C | S-scheduler + `mode`; cooperation adds `num_prefill_ranks` | Keep logical output totals separate from execution-work counts |
| Running / waiting / grammar queue | `sglang:num_running_reqs`, `sglang:num_queue_reqs`, `sglang:num_grammar_queue_reqs` | G | S-scheduler; priority when enabled | Total and priority breakdowns overlap |
| Decode context workload | `sglang:decode_sum_seq_lens`, `sglang:num_used_tokens` | G | S-scheduler | Different state quantities; not interchangeable |
| Pool utilization | `sglang:token_usage`, `sglang:full_token_usage`, `sglang:swa_token_usage`, `sglang:mamba_usage` | G | S-scheduler | Model/pool-dependent; combined token_usage is not a simple sum |
| Full-attention pool slots | `sglang:kv_available_tokens`, `sglang:kv_evictable_tokens`, `sglang:kv_used_tokens` | G | S-scheduler | Free, evictable and actively used slots |
| SWA pool slots | `sglang:swa_available_tokens`, `sglang:swa_evictable_tokens`, `sglang:swa_used_tokens` | G | S-scheduler | Hybrid-SWA models only |
| SSM pool slots | `sglang:mamba_available_tokens`, `sglang:mamba_evictable_tokens`, `sglang:mamba_used_tokens` | G | S-scheduler | Relevant hybrid-SSM models only |
| Prefix reuse | `sglang:cache_hit_rate` | G | S-scheduler | Reported ratio/window; not automatically a cumulative token hit fraction |
| Retraction state / totals | `sglang:num_retracted_reqs`; `sglang:num_retracted_requests`, `sglang:num_retracted_input_tokens`, `sglang:num_retracted_output_tokens` | G; C | S-scheduler | Current state differs from cumulative events/work |
| Paused state | `sglang:num_paused_reqs` | G | S-scheduler | Async weight synchronization path |
| Queue and phase timing | `sglang:queue_time_seconds`, `sglang:per_stage_req_latency_seconds` | H | S-scheduler; stage on phase histogram | Stage meanings depend on unified/disaggregated mode |
| Disaggregated queues | `sglang:num_prefill_bootstrap_queue_reqs`, `sglang:num_prefill_inflight_queue_reqs`, `sglang:num_decode_prealloc_queue_reqs`, `sglang:num_decode_transfer_queue_reqs`, `sglang:pending_prealloc_token_usage` | G | S-scheduler | Prefill/decode disaggregation only |
| KV transfer timing / size | `sglang:kv_transfer_latency_ms`, `sglang:kv_transfer_speed_gb_s`, `sglang:kv_transfer_total_mb`, `sglang:kv_transfer_alloc_ms`, `sglang:kv_transfer_bootstrap_ms` | H | S-scheduler | Mixed native units; normalize explicitly when comparing |
| Disaggregation failures | `sglang:num_bootstrap_failed_reqs`, `sglang:num_transfer_failed_reqs`, `sglang:num_prefill_retries` | C | S-scheduler | Requests/retries are not physical packet errors |
| Speculation ratios | `sglang:spec_accept_rate`, `sglang:spec_accept_length`, `sglang:spec_num_steps`, `sglang:spec_num_draft_tokens` | G | Scheduler/config-dependent | Configured proposal sizes differ from realized accepted length |
| Speculation verification | `sglang:spec_verify_calls` | C | S-request in A1 | Enabled speculation only |
| Scheduling / graph state | `sglang:utilization`, `sglang:fwd_occupancy`, `sglang:new_token_ratio`, `sglang:is_cuda_graph` | G | S-scheduler | Engine-reported values; do not substitute for DCGM SM occupancy |
| Graph execution categories | `sglang:cuda_graph_passes` | C | S-scheduler + `mode` | Graph/non-graph execution classes |
| Forward execution accounting | `sglang:forward_execution_seconds`, `sglang:dp_cooperation_forward_execution_seconds` | C | S-scheduler + `category`; cooperation adds `num_prefill_ranks` | Measurement/device-timer configuration matters; not a complete kernel timeline |
| Routing-key distributions | `sglang:num_unique_running_routing_keys`; `sglang:routing_key_running_req_count`, `sglang:routing_key_all_req_count` | G; H | S-scheduler | Keys in active versus active+waiting populations |
| Estimated device work | `sglang:estimated_flops_per_gpu`, `sglang:estimated_read_bytes_per_gpu`, `sglang:estimated_write_bytes_per_gpu` | C | Scheduler dimensions in A1 | MFU flag required; wire samples have `_total` in S1 |
| HiCache host pool | `sglang:hicache_host_used_tokens`, `sglang:hicache_host_total_tokens` | G | S-scheduler | Hierarchical cache enabled |
| Storage cache movement | `sglang:prefetched_tokens`, `sglang:backuped_tokens`; `sglang:prefetch_pgs`, `sglang:backup_pgs`, `sglang:prefetch_bandwidth`, `sglang:backup_bandwidth` | C; H | Collector-specific; inspect actual labels | Configured storage feature; not universal NIC traffic |
| LoRA slots | `sglang:lora_pool_slots_used`, `sglang:lora_pool_slots_total`, `sglang:lora_pool_utilization` | G | S-scheduler | LoRA enabled |
| Streaming sessions | `sglang:num_streaming_sessions`, `sglang:streaming_session_held_tokens` | G | S-scheduler | Session feature enabled |
| Grammar compilation | `sglang:grammar_compilation_time_seconds`, `sglang:grammar_schema_count`, `sglang:grammar_ebnf_size` | H | Scheduler/collector-specific | Structured-output path |
| Grammar outcomes | `sglang:num_grammar_cache_hit`, `sglang:num_grammar_aborted`, `sglang:num_grammar_timeout`, `sglang:num_grammar_total` | C | Scheduler/collector-specific | Structured-output path |
| Grammar traversal | `sglang:grammar_tree_traversal_time_avg`, `sglang:grammar_tree_traversal_time_max` | H | Scheduler/collector-specific | Source records a distribution of per-observation averages/maxima |
| Prefill-delayer behavior | `sglang:prefill_delayer_wait_forward_passes`, `sglang:prefill_delayer_wait_seconds`; `sglang:prefill_delayer_outcomes` | H; C | S-scheduler; outcome labels on counter | Optional scheduling policy |
| Expert placement | `sglang:eplb_gpu_physical_count` | H | S-scheduler + `layer` | EPLB-related distribution; not a per-expert token assignment trace |
| Startup/configuration | `sglang:engine_load_weights_time`, `sglang:page_size`, `sglang:num_pages`, `sglang:context_len`, `sglang:startup_available_gpu_memory_gb` | G | S-scheduler in A1 | Startup facts/current config; not continuous free GPU-memory measurements |

Sources: S1 for endpoint/request semantics; S2/S3 for collectors/reporting; A1 for the grouped field/dimension cross-check. Other optional collectors and later builds may add more families; this table does not certify all flags or collector subclasses.

## 5. Dynamo native metric groups

This section records Dynamo's framework metrics. Backend-native `vllm:*` and `sglang:*` remain owned by the backend engine and are represented in the preceding sections. Collecting both is appropriate; adding both counters as if they were disjoint user requests is not.

| Group | Native families | Type | Native dimensions / scope | Availability / boundary |
|---|---|---|---|---|
| Frontend request flow | `dynamo_frontend_active_requests`, `dynamo_frontend_stage_requests` | G | D-frontend; stage family adds `stage`, `phase` | Frontend-local; handler lifetime versus pipeline occupancy |
| Frontend request/output counts | `dynamo_frontend_requests_total`, `dynamo_frontend_output_tokens_total` | C | Model where documented | Framework API accounting |
| Frontend timing | `dynamo_frontend_request_duration_seconds`, `dynamo_frontend_time_to_first_token_seconds`, `dynamo_frontend_inter_token_latency_seconds` | H | D-frontend | Frontend boundary, distinct from backend engine timing |
| Frontend sequence/reuse size | `dynamo_frontend_input_sequence_tokens`, `dynamo_frontend_output_sequence_tokens`, `dynamo_frontend_cached_tokens` | H | D-frontend | Logical sequence and reported reuse observations |
| Tokenizer cache outcomes | `dynamo_frontend_tokenizer_cache_hits_total`, `dynamo_frontend_tokenizer_cache_misses_total` | C | D1 describes cross-model totals | Tokenizer cache, not GPU KV cache |
| Tokenizer cached/new tokens | `dynamo_frontend_tokenizer_cache_cached_tokens_total`, `dynamo_frontend_tokenizer_cache_uncached_tokens_total` | C | `model` | Active tokenizer-cache path; missing series can mean disabled |
| Client state | `dynamo_frontend_disconnected_clients` | G | Verify actual family exposition | A gauge; not a cumulative disconnect-event count |
| Migration events / limit | `dynamo_frontend_model_migration_total`, `dynamo_frontend_model_migration_max_seq_len_exceeded_total` | C | `model`; migration_type on applicable family | Migration support/configuration required |
| Migration timing | `dynamo_frontend_model_migration_duration_seconds` | H | `model`, `migration_type`, `outcome` | Recovery interval, not full restarted request latency |
| Frontend cancellations | `dynamo_frontend_model_cancellation_total` | C | `model`, `endpoint`, `request_type` | Framework-detected cancellation |
| Frontend overload responses | `dynamo_frontend_model_rejection_total` | C | `model`, `endpoint` | Client-facing overload response count |
| Worker load view | `dynamo_frontend_worker_active_decode_blocks`, `dynamo_frontend_worker_active_prefill_tokens` | G | D-worker-view | Frontend-local worker observations |
| Worker latest timing/size | `dynamo_frontend_worker_last_time_to_first_token_seconds`, `dynamo_frontend_worker_last_input_sequence_tokens`, `dynamo_frontend_worker_last_inter_token_latency_seconds` | G | D-worker-view | Latest observations, not latency histograms |
| Routable model state | `dynamo_frontend_model_ready` | G | `model` | Current complete routable-topology state |
| Registered model capacity | `dynamo_frontend_model_total_kv_blocks`, `dynamo_frontend_model_max_num_seqs`, `dynamo_frontend_model_max_num_batched_tokens`, `dynamo_frontend_model_context_length`, `dynamo_frontend_model_kv_cache_block_size`, `dynamo_frontend_model_migration_limit` | G | `model` | D1 states first registered worker's configuration; not cluster-total capacity |
| Component request flow | `dynamo_component_inflight_requests`, `dynamo_component_requests_total`, `dynamo_component_request_duration_seconds` | G / C / H respectively | D-component | RPC/component boundary; one user request can cross several components |
| Component payload | `dynamo_component_request_bytes_total`, `dynamo_component_response_bytes_total` | C | D-component | Framework payload accounting, not NIC wire bytes |
| Component errors / cancellation | `dynamo_component_errors_total`, `dynamo_component_cancellation_total` | C | D-component; error_type on errors | Worker cancellation signal is not proof the engine aborted |
| Runtime age | `dynamo_component_uptime_seconds` | G | Inspect actual family labels | Runtime age, not worker hardware uptime |
| Worker admission | `dynamo_rejection_request_total`, `dynamo_engine_request`, `dynamo_request_queue` | C / G / G respectively | Worker-side family labels require exposition check | Documented hard-concurrency-cap path |
| Router request counts | `dynamo_component_router_requests_started_total`, `dynamo_component_router_requests_total` | C | D-router | D1 stable catalog: KV mode populated; non-KV mode registered zero |
| Router timing | `dynamo_component_router_time_to_first_token_seconds`, `dynamo_component_router_inter_token_latency_seconds` | H | D-router | Router-level TTFT and reported average ITL; mode-dependent |
| Router request sizes | `dynamo_component_router_input_sequence_tokens`, `dynamo_component_router_output_sequence_tokens` | H | D-router; input includes `model`, `phase` | Routing attempts need not equal unique API requests |
| Router predicted cache reuse | `dynamo_component_router_kv_hit_rate` | H | D-router | Prediction at selection, not observed physical cache hit |
| Routing tradeoffs | `dynamo_component_router_non_max_overlap_selections_total`, `dynamo_component_router_overlap_blocks_lost` | C / H | `worker_type` on these families | Eligible prefill worker-selection comparisons |
| Token reuse funnel | `dynamo_component_router_kv_best_eligible_cached_prefix_tokens_total`, `dynamo_component_router_kv_selected_cached_prefix_tokens_total`, `dynamo_component_router_kv_worker_reused_tokens_total` | C | Router/phase dimensions from actual exposition | Best/selected estimates versus worker reports; reports arrive later |
| Routing decision cost | `dynamo_router_overhead_block_hashing_ms`, `dynamo_router_overhead_indexer_find_matches_ms`, `dynamo_router_overhead_seq_hashing_ms`, `dynamo_router_overhead_scheduling_ms`, `dynamo_router_overhead_total_ms` | H | `router_id`; frontend-local | KV-routing frontend only; milliseconds |
| Router queue state | `dynamo_frontend_router_queue_pending_requests`, `dynamo_frontend_router_queue_pending_isl_tokens`, `dynamo_frontend_router_queue_pending_cached_tokens` | G | `model`, `worker_type`, `policy_class` | Queue threshold/policy configuration required |
| Router queue rejection | `dynamo_frontend_router_queue_backpressure_total` | C | Queue labels + `reason` | Configured-limit rejections |
| Cache-index event processing | `dynamo_component_kv_cache_events_applied` | C | `status`, `event_type` plus applicable runtime labels | Requires relevant policy, published KV events and consumption |
| Registered but nonfunctional counter | `dynamo_component_ckf_mutation_total` | C | `outcome` | D1 documents always zero; not evidence of successful mutations |
| Gateway observed cache reuse | `dynamo_epp_cached_tokens` | H | `model` from served pool | EPP; depends on response usage accounting being present |
| Legacy frontend families | `dynamo_frontend_inflight_requests`, `dynamo_frontend_queued_requests` | G | Legacy family dimensions | Deprecated in inspected stable catalog; do not add to replacements |

Source: D1. D1 excludes engine-native pass-through and upstream NIXL from its own field catalog. Operator metrics, planner, KVBM-specific and other separately documented emitters are not silently treated as frontend/backend metrics here. Their deployment/version applicability remains separate. Development documentation differs from the inspected stable catalog; development-only additions have not been merged into this baseline.

## 6. What the native dimensions establish

| Dimension | vLLM baseline | SGLang baseline | Dynamo baseline |
|---|---|---|---|
| Served model | `model_name` | `model_name` | Commonly `model`; backend-native names remain separate |
| Engine identity/role | Engine index on common V1 series | `engine_type` | Namespace/component/endpoint and worker views |
| Parallel rank | No universal TP/PP/GPU identity established by common V dimensions | Scheduler families include documented TP/PP/EP ranks; DP conditional | DP rank on specified worker-view families; backend engine labels retained separately |
| Request phase | Separate phase metric families | Stage metrics and engine role | `stage`/`phase` on specified families |
| Cache source | Selected source/connector families | Selected cache-source and pool families | Predicted/reported reuse at framework boundaries |
| GPU model / GPU UUID | Not established as common labels by inspected core schema | Not established as common labels by inspected core schema | Not established as common labels by inspected framework schema |
| NIC model / fabric type | Not established as common labels by inspected core schema | Not established as common labels by inspected core schema | Not established as common labels by inspected framework schema |
| Host/container/pod | Do not assume external scrape labels are native | Do not assume external scrape labels are native | Runtime namespace differs from Kubernetes namespace; collector-added labels are separate |

“Not established as common labels” does not mean unsupported hardware or an assertion that no optional collector/custom label can expose the attribute. It means we have not found a universal native dimension contract in the inspected metric families.

## 7. Interpretation constraints established in this step

1. Retain source metric, emitter, measurement boundary and all native labels before any future normalization.
2. Do not collapse Dynamo frontend/router/component TTFT into one interchangeable engine metric, or subtract unrelated histogram percentiles to estimate routing overhead.
3. Do not sum request/token counts across software boundaries or duplicated parallel-rank exports without an explicit counting policy.
4. Do not interpret `request_success` solely from its name. Verify finished-reason values and how errors/aborts contribute before defining Aether success/failure metrics.
5. Preserve token-weighted versus request-weighted cache/latency measurements, predicted versus reported reuse, and latest gauges versus distributions.
6. All three provide useful software observations. Their core metric schemas do not constitute a universal GPU/NIC/fabric inventory contract.
7. This is a source-documentation comparison. Exact installed-release family names, labels, optional flags, histogram buckets and emission cadence remain unverified until an actual versioned export is inspected.

## Completion and proposed next step

Completed: the documentation baseline for existing native metric groups, dimensions, emitting boundaries and feature gates across vLLM, SGLang and Dynamo.

Suggested next step, not performed here: create a metadata-enrichment matrix. For each desired identity/dimension (host, process, container, GPU UUID/model, replica, parallel rank, NIC and fabric), identify whether it is already native, available through software configuration/discovery, requires host discovery, or remains unknown. Include the join key, cardinality and evidence needed to associate it with an engine metric. Hardware purchases and collector implementation are not prerequisites for that design step.
