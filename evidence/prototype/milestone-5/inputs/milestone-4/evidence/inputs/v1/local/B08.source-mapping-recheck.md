# Source-mapping recheck after the engine-scope correction

Reviewed: 1 November 2026. Status: documentation recheck completed; no engine, host or collector was run.

The mistake was treating the first three reviewed engines as the only eligible sources. That rule has been removed. A requirement can use its original TGI, Triton or TensorRT-LLM source even if vLLM, SGLang or Dynamo has no equivalent. This does not promise compatibility with every engine release.

All 26 rows previously marked U (unresolved) were rechecked against their original sources. Twenty-five now have E (documented endpoint) routes. L1-001 remains U because the raw counter's counting boundary differs from the accepted-inference-request boundary that the requirement asks for. Ten nearby mappings were also corrected; four changed from calculated to direct endpoint routes.

All 250 requirement IDs, definitions and required dimensions remain unchanged. All 58 metadata associations remain unchanged. DeepInfer remains deferred. Prototype milestones have not started.

## Route counts

| Classification | Before | After |
|---|---:|---:|
| E: documented endpoint | 59 | 88 |
| R: calculated from inputs | 39 | 35 |
| H: host/device/fabric collector | 100 | 100 |
| I: deeper integration | 26 | 26 |
| U: unresolved mapping | 26 | 1 |
| Total requirements | 250 | 250 |

These are collection-route classifications, not implemented or supported metric counts. E means a documented acquisition path exists for the stated source. It does not establish every required dimension, optional feature, release or update interval.

## Evidence used

[TGI metrics](https://huggingface.co/docs/text-generation-inference/reference/metrics) documents the named families and `/metrics`. [TGI router source](https://github.com/huggingface/text-generation-inference/blob/main/router/src/server.rs) shows request-count increments before validation rejection and in several handlers.

[Triton metrics](https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/user_guide/metrics.html) documents request/backend counts, cumulative timings, pinned pools and response-cache families. [TensorRT-LLM collector source](https://github.com/NVIDIA/TensorRT-LLM/blob/main/tensorrt_llm/metrics/collector.py) defines the `trtllm_` prefix, gauge registrations, conditional iteration-stat updates and milliseconds-to-seconds conversion. The [archived Triton 25.02 backend example](https://docs.nvidia.com/deeplearning/triton-inference-server/archives/triton-inference-server-2520/user-guide/docs/tensorrtllm_backend/README.html#triton-metrics) documents the runtime memory family. The archived example is evidence for that schema, not every later release.

At the original review, the rolling documentation and GitHub `main` references were not pinned. The later [versioned evidence baseline](validation-evidence-baseline.md) captures exact commits and checksums, with all 26 historical U rows in a separate reconciliation ledger. This pins research inputs; no deployed version or compatibility matrix has been verified. The JSON companion retains prior routes, units, conditions and links to the pinned evidence.

## Every previously unresolved row

The source keys below link to the evidence above: TGI = TGI metric reference; TGI_CODE = router source; TR = Triton reference; RT = native TensorRT-LLM collector; TB_ARCHIVE = archived backend example. “E” remains conditional on feature availability and version checks.

| ID | Requirement | Result | Documented source | Evidence | Meaning or remaining limit |
|---|---|---|---|---|---|
| L1-001 | Requests received | U → U | tgi_request_count (raw endpoint counter exists; accepted-inference-request equivalence unverified) | TGI, TGI_CODE | Router increments before some validation failures and in multiple API handlers; verify the selected API/admission boundary and multiplicity. No engine is excluded. |
| L1-013 | Validation time | U → E | tgi_request_validation_duration | TGI | Preserve source validation phase and histogram buckets; no individual validation events are reconstructed. |
| L2-012 | Current sequence batch size | U → E | tgi_batch_current_size | TGI | Current source batch; not all running requests or every iteration. |
| L2-013 | Next batch size | U → E | tgi_batch_next_size | TGI | Distribution of next batch sizes, not a current-size gauge. |
| L2-014 | Batch token capacity | U → E | tgi_batch_current_max_tokens | TGI | Current batch token capacity; not global KV capacity. |
| L2-015 | Batch forward duration | U → E | tgi_batch_forward_duration | TGI | Keep method labels; source phase timing is not a GPU kernel trace. |
| L2-016 | Batch token decode duration | U → E | tgi_batch_decode_duration | TGI | Preserve the source decode-stage meaning; do not equate to autoregressive GPU execution time. |
| L2-017 | Batch filter duration | U → E | tgi_batch_filter_duration | TGI | Keep source filter/send phase and method labels. |
| L2-018 | Batch inference calls | U → E | tgi_batch_inference_count; tgi_batch_inference_success | TGI | Keep total and successful calls separate, including method labels; not user request counts. |
| L2-070 | Batch inference duration | U → E | tgi_batch_inference_duration | TGI | Batch inference duration; not a complete per-iteration event stream. |
| L2-071 | Skipped speculative tokens per request | U → E | tgi_request_skipped_tokens | TGI | Documentation calls these speculated tokens; validate skipped-token interpretation in the selected backend before semantic normalization. |
| L1-014 | Triton request handling time | U → E | nv_inference_request_duration_us | TR | Cumulative request handling time, including cached requests; not a latency distribution. |
| L1-034 | Inference item count | U → E | nv_inference_count | TR | Batch elements, excluding cached requests; not generated tokens. |
| L2-047 | Triton pinned pool capacity and use | U → E | nv_pinned_memory_pool_total_bytes; nv_pinned_memory_pool_used_bytes | TR | Triton pool across models; not per-model allocator memory. Documented starting with 24.01. |
| L2-048 | Response-cache hits and misses | U → E | nv_cache_num_hits_per_model; nv_cache_num_misses_per_model | TR | Whole-response cache; requires response caching. Not KV prefix reuse. |
| L2-049 | Response-cache operation duration | U → E | nv_cache_hit_duration_per_model; nv_cache_miss_duration_per_model | TR | Cumulative response-cache timings; compatible count deltas permit means, not quantiles. |
| L2-057 | Backend execution count | U → E | nv_inference_exec_count | TR | Backend executions exclude cached requests; not tokens or requests. |
| L2-058 | Backend input/inference/output cumulative duration | U → E | nv_inference_compute_input_duration_us; nv_inference_compute_infer_duration_us; nv_inference_compute_output_duration_us | TR | Backend phase totals exclude cached requests; no per-request timeline or percentiles. |
| L2-069 | Backend pending requests | U → E | nv_inference_pending_request_count | TR | Triton enqueue-to-backend-execution boundary; not a universal engine scheduler queue. |
| L2-004 | Prefill requests | U → E | trtllm_num_context_requests | RT | Conditional iteration snapshot; not every iteration, and not an interchangeable SGLang queue population. |
| L2-005 | Decode requests | U → E | trtllm_num_generation_requests | RT | Conditional iteration snapshot; not every iteration, and not an interchangeable SGLang queue population. |
| L2-007 | Scheduled requests | U → E | trtllm_num_scheduled_requests | RT | Conditional iteration snapshot; not every iteration, and not an interchangeable SGLang queue population. |
| L2-059 | Iteration duration | U → E | trtllm_iteration_latency_seconds | RT | Updated from iterLatencyMS / 1000; latest gauge can miss iterations between reads. Not a duration histogram or complete event stream. |
| L2-044 | Engine GPU memory allocated | U → E | nv_trt_llm_runtime_memory_metrics{memory_type="gpu"} | TB_ARCHIVE | Backend runtime allocation boundary; not whole-device usage, process RSS or Triton shared pinned pool. Archived schema requires deployed-build verification. |
| L2-045 | Engine CPU memory allocated | U → E | nv_trt_llm_runtime_memory_metrics{memory_type="cpu"} | TB_ARCHIVE | Backend runtime allocation boundary; not whole-device usage, process RSS or Triton shared pinned pool. Archived schema requires deployed-build verification. |
| L2-046 | Engine pinned memory allocated | U → E | nv_trt_llm_runtime_memory_metrics{memory_type="pinned"} | TB_ARCHIVE | Backend runtime allocation boundary; not whole-device usage, process RSS or Triton shared pinned pool. Archived schema requires deployed-build verification. |

## Nearby mappings corrected

These ten rows were checked because the same scope mistake displaced an original source or introduced an unnecessary conversion. The original provenance fields are preserved in the JSON.

| ID | Route change | Source correction |
|---|---|---|
| L1-002 | R → E | nv_inference_request_success; vllm:request_success (finished_reason policy required) |
| L1-003 | R → E | nv_inference_request_failure |
| L1-009 | E → E | tgi_request_duration; vllm:e2e_request_latency_seconds; sglang:e2e_request_latency_seconds; dynamo_frontend_request_duration_seconds |
| L1-012 | E → E | tgi_request_inference_duration; vllm:request_inference_time_seconds |
| L1-017 | E → E | tgi_request_max_new_tokens; vllm:request_params_max_tokens |
| L2-006 | E → E | trtllm_num_paused_requests; sglang:num_paused_reqs (different pause cause) |
| L2-022 | R → E | trtllm_kv_cache_used_blocks |
| L2-023 | R → E | trtllm_kv_cache_free_blocks |
| L2-024 | E → E | trtllm_kv_cache_max_blocks |
| L2-025 | E → E | trtllm_kv_cache_tokens_per_block |

For L1-002 and L1-003, Triton provides direct counters; other engines retain their own outcome policies. L1-006 and L1-007 remain calculations over matching terminal populations. For L2-022 and L2-023, TensorRT-LLM provides block gauges, so conversion from token slots is not necessary. Configured capacities and other engines' page/block sizes must remain separately identified.

## What remains unresolved

L1-001 is a semantic gap, not an engine exclusion. The raw TGI counter is collectable, but the router increments it before some rejected requests and at multiple API call sites. We cannot claim it counts each accepted inference request exactly once. Verification must trace the selected API path through admission and count multiplicity using a fixed version. Until then, retain the raw series and do not normalize it to accepted requests. L1-005, the accepted-request arrival rate, inherits this gap.

For L2-071, the raw source has an E route, but documentation calls its observations speculated tokens. The catalog's skipped-token interpretation remains conditional on backend verification. For L2-047, the E route is server-wide; a per-model split cannot be invented. These limitations remain visible in the matrix rather than being hidden by the endpoint classification.

TensorRT-LLM iteration gauges need an active collector, iteration statistics (`enable_iter_perf_stats`) and the relevant fields. A registered default gauge is not evidence that its field is supported or fresh. Reading a latest-value gauge cannot recover all iterations. TGI histograms likewise cannot recover individual events. Existing settings must be inspected; this review does not authorize enabling features or changing workloads.

## Audit boundary and verification

All 250 source-provenance records were screened for the scope error, including all original TGI/TR/RT/TB references and integration routes. The focused primary-source recheck covers the 36 rows listed here. No other route classification changed. Explanatory wording on integration rows was corrected to avoid implying an exclusive engine list. Host/device/fabric routes still identify separate collectors, and integration routes still identify missing event or semantic records. This is not a new claim that all possible sources for those remaining routes have been exhaustively researched.

The Markdown and JSON matrices contain the same 250 IDs and route totals. The original catalog files, metadata matrix, initial three-engine baseline and uploaded planning instructions were left byte-for-byte unchanged. No requirements were removed or renamed. No Rust implementation, fixture or real-environment compatibility test was created.

The execution plan records this research follow-up as complete. All eight prototype milestones remain pending. The next step is Milestone 1: define the prototype experiment boundary and create tracking entries for every requirement and metadata association. Any selected synthetic examples remain experiments, not a reduction of the product requirements.

2026-11-01: linked the versioned source bundle and historical-U reconciliation ledger. The documentation outcomes and route counts are unchanged; deployed schema/capability checks remain unperformed.
