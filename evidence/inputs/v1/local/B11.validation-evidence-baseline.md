# Aether versioned evidence and first validation baseline

Baseline: v1, 1 November 2026. Status: documentation inputs pinned and historical mappings reconciled; first local reference environment recorded. Deployed schemas and capabilities remain unverified.

The user confirmed that no existing cluster is available. The first validation environment is therefore the inspected local workspace for future synthetic tests. No inference engine, cluster, GPU/NIC access or production compatibility is assumed. The prototype milestones have not started.

## What is complete and what remains

| Work | Completed | Still pending |
|---|---|---|
| Versioned evidence inputs | Ten upstream files captured at seven exact repository commits; Git blob hashes verified and local SHA-256 checksums recorded | These are documentation/code inputs, not real endpoint exports or a tested release combination |
| The 26 historical U rows | All 26 reconciled individually against pinned source inputs; 25 endpoint routes and one remaining semantic gap | Deployed schemas, activation, scope, attribution and capabilities for every row |
| First validation environment | Actual local OS, kernel, architecture, libc and Rust/Cargo/Python identities recorded in a versioned manifest | Synthetic fixture manifests, dependency lockfile, finite run limits and test execution |
| First real deployment environment | Availability explicitly recorded as absent | A later test environment or authentic versioned captures; no real support claim is possible yet |

All 250 metric requirements and 58 metadata associations remain unchanged. Initial source selection is not an exclusive product-support list. DeepInfer remains deferred.

## Frozen source inputs

The authoritative file index is [source-metadata.json](evidence/inputs/v1/source-metadata.json). Each entry contains the exact repository commit, upstream path, Git blob SHA-1, local SHA-256, byte count and snapshot location. Files were fetched from the commit, not merely downloaded from a moving `main` link. Their Git blob hashes were recomputed from the local bytes and matched the upstream API values.

| ID | Evidence input | Exact upstream version |
|---|---|---|
| E01 | vLLM production metrics documentation | [8530a778f78dc7cd9a31e108ebc035c4b27ceb3c](https://github.com/vllm-project/vllm/blob/8530a778f78dc7cd9a31e108ebc035c4b27ceb3c/docs/usage/metrics.md) |
| E02 | vLLM metric collector | Same commit as E01; exact path and blob in the index |
| E03 | SGLang production metrics documentation | [e2982985110fb432fd8ddba5f0147a5004fd03ab](https://github.com/sgl-project/sglang/blob/e2982985110fb432fd8ddba5f0147a5004fd03ab/docs/docs/references/production_metrics.mdx) |
| E04 | SGLang metric collector | Same commit as E03; exact path and blob in the index |
| E05 | Dynamo metrics catalog source | [6bcaabcb39cbf98ef048a6ef3145825b18118f3e](https://github.com/ai-dynamo/dynamo/blob/6bcaabcb39cbf98ef048a6ef3145825b18118f3e/docs/fern/pages/reference/observability/metrics-catalog.mdx) |
| E06 | TGI metrics documentation | [b4adbf2f6e2e721280bd0ea5f91d70f7d033f5ed](https://github.com/huggingface/text-generation-inference/blob/b4adbf2f6e2e721280bd0ea5f91d70f7d033f5ed/docs/source/reference/metrics.md) |
| E07 | TGI router request-count implementation | Same commit as E06; exact path and blob in the index |
| E08 | Triton metrics documentation source | [44b22e89d167bb25d016e59fda7d42da9bb36a3f](https://github.com/triton-inference-server/server/blob/44b22e89d167bb25d016e59fda7d42da9bb36a3f/docs/user_guide/metrics.md) |
| E09 | TensorRT-LLM native metric collector | [5d86481f60bdb137c1a2ecd283125f8d944c3635](https://github.com/NVIDIA/TensorRT-LLM/blob/5d86481f60bdb137c1a2ecd283125f8d944c3635/tensorrt_llm/metrics/collector.py) |
| E10 | Triton TensorRT-LLM backend metric examples | [4a2a42a9f66e287c0eeb2dd12ade7fccef787506](https://github.com/triton-inference-server/tensorrtllm_backend/blob/4a2a42a9f66e287c0eeb2dd12ade7fccef787506/README.md) |

These commits pin research inputs, not the engine binaries to deploy. They are independent repositories and do not establish an interoperable software stack. The Dynamo source snapshot is not asserted to be the published v1.6.1 page or to resolve its previously recorded label-page version mismatch. Earlier rolling/archived references remain historical provenance; this bundle does not silently replace their meaning with new release claims. E10 independently confirms the memory-family example previously supported by the archived backend documentation; it does not prove a running backend emits it.

This first bundle freezes the software inputs needed for the endpoint/reconciliation experiments. It is not an exhaustive archive of the catalog's hardware, kernel, profiler and fabric references. Before a later fixture relies on one of those references, capture its exact version/content in a new bundle revision and add the required checks. Do not use the absence of an archived reference to remove its requirement.

## Reconciliation of all 26 historical unresolved mappings

The complete machine-readable ledger is [reconciliation-v1.json](evidence/reconciliation-v1.json). It records every historical U ID, its current route, source names, input IDs, types/units, conditions and remaining limits. Every named family in those rows was found in the pinned inputs. Name presence is a provenance check, not a semantic or capability test.

| Pinned inputs | Historical U requirement IDs | Current documentation result |
|---|---|---|
| E06 and E07 | L1-001 | Raw TGI counter exists; accepted-inference-request counting remains unresolved |
| E06 | L1-013; L2-012–L2-018; L2-070–L2-071 | Ten documented endpoint routes; preserve phase meanings and the L2-071 interpretation caveat |
| E08 | L1-014, L1-034; L2-047–L2-049; L2-057–L2-058; L2-069 | Eight documented endpoint routes; preserve request/item/execution, cumulative-time, shared-pool and response-cache boundaries |
| E09 | L2-004–L2-005; L2-007; L2-059 | Four conditional native collector routes; iteration fields must be supplied and updated |
| E10 | L2-044–L2-046 | Three backend runtime-memory routes; deployed schema still unverified |

The current whole-catalog route totals remain E:88, R:35, H:100, I:26, U:1. The historical 26 U rows must not be confused with the separate 26 I integration rows. One remaining U does not mean only one unverified capability remains: all proposed routes still need implementation and appropriate real validation.

L1-001 remains U because the pinned router increments `tgi_request_count` before some validation failures and at multiple API handlers. This is a counting-boundary issue, not a missing engine equivalent. L1-005 inherits the unresolved accepted-request population. L2-071 has a raw endpoint route, but normalizing its source-reported speculated tokens to the catalog's skipped-token meaning still needs backend verification. No requirement definition was rewritten to eliminate a gap.

## First validation environment

The versioned environment record is [first-environment-v1.json](validation/first-environment-v1.json). It pins the observed local reference fingerprint, not a portable machine image or a production hardware baseline.

| Item | Recorded selection |
|---|---|
| Environment ID | `aether-validation-synthetic-local-v1` |
| OS | Ubuntu 26.04.1 LTS, x86_64 |
| Kernel | `7.0.0-38-generic` |
| libc | glibc 2.43 |
| Rust | 1.99.0; compiler commit `b940084d7eb6a299eb4bfeb8e34901bc051e7ac4`; target `x86_64-unknown-linux-gnu` |
| Cargo | 1.99.0; reported build `5f94df478`, dated 2026-08-27 |
| Python | 3.14.4, for evidence inspection |
| Validation inputs | Future synthetic fixtures derived from the pinned source bundle; no engine connections |
| Runtime boundary | Later loopback-only fixture services and disposable local storage; remote storage simulated |
| Hardware access | No actual GPU/NIC queries, kernel probes or runtime control sockets |

The observed rustup alias is `stable`; that alias is not a reproducibility pin. Use the exact compiler identity above for the first harness and verify it before running. No toolchain was installed or switched. If the environment drifts, record a new environment revision and compare results; do not overwrite v1 as if it had always matched. Rust dependencies and their `Cargo.lock` will be selected in the harness milestone. This task does not choose production dependencies or create an engine package.

This environment can support later parser, identity, calculation, bounded-buffer and recovery experiments against invented inputs. It cannot establish real engine compatibility, actual host/device discovery, driver/kernel backward compatibility, microsecond capture, inference overhead or fabric behavior. Real GPU/NIC models, fabric, driver, CUDA/NCCL and engine runtime versions remain unselected. No image digest or package lock exists for this host reference; portable environment provisioning remains pending.

## What real schema and capability verification must contain

A real-source evidence record must identify the target/incarnation and actual engine build or image digest, capture window, sanitized raw endpoint export with HELP/TYPE/native labels, feature/configuration evidence and the namespace/permissions used. A hardware capability claim additionally needs the actual GPU/NIC model, driver/firmware, partition/port/link mode, getter/counter support and freshness evidence. Keep emitter location separate from measured subject location.

Record field presence, valid type/units, meaning, active feature, observed updates, permissions and attribution as separate outcomes. An absent field does not prove unsupported; a registered zero gauge does not prove that statistics are active. Capturing a schema at one instant cannot certify continuing access, overhead, source-event completeness or durability. Existing capture files, if supplied later, must retain their real origin and must not be relabeled synthetic.

Until those inputs exist, each ledger row retains `deployed_schema_verification=not_run` and `capability_verification=not_run`. No new cluster is provisioned under the current synthetic-only plan. A separate future decision must identify an available test environment or authentic captures; absence of that environment does not block synthetic prototype preparation.

## Integrity and update procedure

The [evidence manifest](evidence/manifest-v1.json) lists source snapshots, reconciliation, environment and frozen copies of the local planning/research inputs with SHA-256 hashes. Its companion checksum covers the manifest bytes. The snapshots preserve upstream content, including copyright/license headers where present; they are reference material, not Aether implementation.

Run from the workspace:

    python3 scripts/check-evidence-inputs.py

The checker verifies file hashes, upstream Git blob hashes, local input copies, all 26 historical IDs against the coverage baseline, route totals, evidence links and native-name checks. It does not contact a cluster, inspect devices or certify compatibility. A changed or missing file must fail, not cause automatic regeneration of expected hashes. New source content, environments or revised requirements require a new bundle/revision with a recorded reason; preserve v1 as historical evidence.

## Next step

Prototype Milestone 1 can now select experiments using this pinned source bundle and local reference environment. It still must create the complete 250-requirement/58-association tracking file, fixture choices, expected outcomes and experiment-specific safety/acceptance limits. No prototype milestone, fixture, benchmark or deployed capability check is marked complete by this documentation work.
