# Prototype experiment scope — Milestone 1

This is a local Rust proof-of-concept program using invented inputs. It retains all 250 metric requirements across four layers and all 58 metadata associations. There is no existing cluster. No real engines, devices, runtime sockets, kernel probes, external storage or customer workloads are accessed. Production architecture is not selected. DeepInfer remains deferred.

The authoritative registry is [prototype-experiments.json](prototype-experiments.json); [prototype-status.json](prototype-status.json) preserves every baseline object and research route. 23 metrics and 42 associations are selected for planned fixtures; the remaining 227 metrics and 16 associations remain not_prototyped, not unsupported. Selection is bidirectional and dependencies remain visible.

## Planned cases

| ID | Purpose | Later execution milestones |
|---|---|---|
| PX01 | Native families and producer boundaries | [2,3,4] |
| PX02 | Counter rates, resets and unresolved arrivals | [2,4] |
| PX03 | Insufficient aggregate evidence | [2,4,7] |
| PX04 | Framework/backend duplication and remote subject | [2,4,5] |
| PX05 | Replica GPU set and memory occupancy | [2,4,5] |
| PX06 | Process and cgroup incarnation changes | [2,5] |
| PX07 | NIC capability is not observed transport | [2,5] |
| PX08 | Distributed rank skew and clock uncertainty | [2,7] |
| PX09 | Stream gap versus whole-device idle | [2,7] |
| PX10 | Storage admission, commit and replay | [2,6] |
| PX11 | Visibility, privacy and action policy | [2,3,4,5] |
| PX12 | Latest-gauge freshness and bounded admission | [2,4,6] |

## Inputs and limits

Use aether-source-evidence-v1 and aether-validation-synthetic-local-v1. Software references E01–E10 are exact repository snapshots, not a deployed stack. Host/device/fabric/semantic records are explicitly baseline-only synthetic unless new evidence is separately frozen. All six named serving sources are eligible; reviewing three engines first does not exclude others. Cache providers not independently verified remain unknown.

L1-001 retains U because raw TGI request_count does not establish accepted inference requests; L1-005 inherits the gap. L2-071's raw endpoint exists but its skipped/speculated-token interpretation remains conditional. Current routes E:88/R:35/H:100/I:26/U:1 are not implementation or support counts. The historical 26 U rows are distinct from 26 I integration rows.

The twelve budget records are proposals, not agreed runtime limits. Correctness limits reference exact named outcomes; numeric production budgets and owner agreements remain unset. Readiness flags remain false; all experiment results and real verification remain not_run. The additional 100-ms proposed poll period, two retries and one-second capped backoff are small synthetic planning choices, not resolution/freshness guarantees. PX12 has a two-record reject-new queue assertion. Sensitive values and real prompt/response data are excluded.

## Acceptance and handoff

Milestone 1 validates planning records only. Run the Rust evidence and traceability commands and cargo tests under tools/aether-validation. A specification pass does not mean its experiment passed. Milestone 2 will create invented endpoint, metadata, event and fault payloads with expected answers. Milestone 3 adds the separate engine harness. Later source qualification, actual GPU/NIC/fabric support, backwards compatibility, microsecond acquisition, inference overhead, production durability and release remain unproven. Generic pipeline/policy checks and 18 proposed companion families never increase the original 250 metric IDs.
