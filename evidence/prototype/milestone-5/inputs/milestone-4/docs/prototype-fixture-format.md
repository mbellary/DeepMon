# Synthetic fixture interchange format — version 1

This local interchange format is for Milestone 2 construction checks and future Rust experiments. All payloads are invented. No field establishes live engine compatibility, real hardware support, production architecture or executed experiment behavior. The original twelve PX groups and 250/58 requirement tracking remain unchanged.

`fixtures/prototype/manifest.json` is the only fixture file index. It identifies bundle `aether-synthetic-fixtures-v1`, schema 1, `synthetic=true`, the source manifest/registry/status hashes, environment `aether-validation-synthetic-local-v1` and twelve fixture groups. Each group copies its registry ID/link sets, source evidence kind, claim limits and budget reference/hash. Variants have stable IDs and local artifact references, one JSON input envelope, optional endpoint text and exactly one independent expected-answer file.

Artifact paths are relative to `fixtures/prototype`, with a role, UTF-8 media type, exact length and SHA-256. Only manifest.json and README.md are allowed unindexed control files. No path can traverse upward or resolve outside the fixture root. Files/variants/groups are unique; shared files require identical declarations. No URL is retrieved or used as an execution target.

JSON inputs use schema 1, `synthetic=true`, their variant ID, explicit microsecond clock-domain/uncertainty metadata and a `records` array. Records use typed `kind` tags for endpoint snapshots, counters, identities, links, memory, rank events, intervals, stage/fault descriptions and policy/queue inputs. Identity records include entity/incarnation and validity windows. All clocks are virtual, integer and nonnegative, with offset bounds explicitly present or unknown. Intervals are half-open. Faults are finite data actions, never shell commands or live file/device operations.

Expected-answer envelopes have schema 1, `synthetic=true`, the same variant ID, `experiment_execution_status=not_run`, an assertion list and optional proof material. Assertions identify target, expected value, unit, state, supporting record IDs and reasoning. Numbers and states are independently authored literals; future engine output must not generate its own expected answers. Construction checks verify simple arithmetic and consistency without running the future observability algorithm.

A variant can declare a specific intentionally invalid endpoint payload with an allowlisted expected error code. The validator must actually reject the payload for that error; this never permits missing files, wrong hashes, unsafe paths, forged provenance or invalid packaging. Semantically incomplete inputs remain well-formed inputs with unknown/unavailable oracle states, not malformed files.

The endpoint validator supports only the generated finite exposition subset: HELP/TYPE declarations, scalar samples and histograms with quoted labels, cumulative buckets, count/sum and finite sample values. It is not a general production Prometheus parser. Native-family declarations reference exact E01–E10 input definitions. All values, aliases, timestamps, optional fixture labels and test bucket boundaries are invented; no native label or configured bucket set is guaranteed unless separately documented. Missing optional families are absent, not zero.

Every variant retains `experiment_execution_status=not_run`. The separate `docs/prototype-fixture-status.json` hashes the manifest and links groups/variants/files. Its authored and construction-validation verdicts are distinct from experiment execution, budget agreement and real-source verification. The manifest does not hash this ledger, avoiding circular checksums.

Budget files remain proposed/not agreed and readiness stays false. Existing Milestone 1 records and frozen source inputs are not edited; evolving Rust tool code has a verified predecessor snapshot under `evidence/prototype/milestone-2/inputs/milestone-1/`. Construction validation and payload tests do not start an engine, connect to a network, inspect a device or replay actual faults.

Group association lists declare all inherited requirements, not that every relation is observed. An unmentioned or unsupported relation remains unknown by default; an explicit oracle can resolve only its supplied invented evidence. All source/claim limits still apply even in positive variants.

PX10 stage booleans are supplied hypothetical persistence-boundary facts for the future fault adapter and construction oracle. They are not observations from executed writes. A later storage implementation must produce and verify its own actual record/visibility results against these authored expectations. No payload outcome is marked as actual engine evidence here.

The author-fixtures command initially writes an authored/not-run ledger. Once actual construction-validation evidence is added, rerunning authoring must not reset it; differing existing files are protected and cause an explicit refusal. Use the read-only fixtures command for repeated validation.

## Exact envelope fields

All structures below reject unknown envelope fields. Fields listed as optional still appear as null when absent in authored JSON. Record bodies are tagged by `kind` and checked for the fields relevant to that tag. The Rust definitions are in `tools/aether-validation/src/fixtures.rs`; consumers must preserve the distinction between inputs, oracles and actual results.

| Structure | Required fields and types |
|---|---|
| Manifest | `schema_version`: integer 1; `bundle_id`: string; `synthetic`: true; `source_manifest_sha256`, `registry_sha256`, `status_sha256`: strings; `environment_id`: string; `fixtures`: group array |
| Group | `fixture_id`, `experiment_id`, `evidence_kind`, `invented_fields`, `limits_of_claim`, `budget_path`, `budget_sha256`: strings; `requirement_ids`, `association_ids`, `profile_ids`, `pinned_source_ids`, `local_baseline_ids`: unique string arrays; `variants`: array |
| Variant | `id`, `purpose`, `intent`, `experiment_execution_status`: strings; `expected_payload_error`: string or null; `artifacts`: array. Intent is positive/negative; execution is not_run. |
| Artifact | `path`, `role`, `media_type`, `sha256`: strings; `bytes`: nonnegative integer. Roles are input/expected/endpoint; media types application/json or text/plain. |
| Input | `schema_version`: 1; `synthetic`: true; `variant_id`: string; `clocks`: array; `records`: array |
| Clock | `domain`: string; `unit`: microseconds; `uncertainty_us`: nonnegative integer; `offset_bounds_us`: ordered pair of signed integers or null for unknown |
| Oracle | `schema_version`: 1; `synthetic`: true; `variant_id`: string; `experiment_execution_status`: not_run; `assertions`: array; `proof`: structured JSON or null |
| Assertion | `kind`, `target`, `unit`, `state`, `reason`: strings; `record_ids`: string array referring to input records; `expected_value`: JSON. An unknown assertion must use null and unknown state. |

The bounds are 12 groups, 2–16 variants/group, 2–3 artifacts/variant, at most 1 MiB/artifact, 64 records/input, 32 identity records/input, eight clock domains/input, 32 assertions/oracle, 64 endpoint samples/variant, 256 bytes/label value, 256 total fixture files including the two control files and directory nesting depth eight. These are fixture-construction limits, not production recommendations. No runtime acceptance budget becomes agreed because a construction limit is enforced.

The finite endpoint grammar accepts UTF-8 lines with HELP/TYPE, scalar samples and histogram bucket/count/sum samples. Quoted label values in this subset do not contain commas, embedded quotes, braces or escapes. Exemplars, summaries, timestamps and arbitrary exporter text are outside this construction grammar. Counter/gauge samples here are finite and nonnegative. Histogram bucket bounds increase, cumulative counts do not decrease, the +Inf bucket matches integer count, and count/sum declarations are complete. These checks are deliberately narrower than production metric collection.

Oracle kinds cover raw native observations, payload rejection, rate, memory occupancy, logical/replica counts, group skew/durations, stream gap/device coverage, committed/visible IDs, queue accounting, sanitization, explicit quality states, counterexample proofs and unknown outcomes. Native sample values and simple arithmetic are cross-checked against supplied inputs; this is consistency of authored data, not execution of a collector or serving engine. Exact numeric fixtures use representable integer/binary-rational results or the same declared decimal quantity; no hidden numeric tolerance is added.

The ledger stores `schema_version`, `synthetic`, manifest/registry/status hashes, false runtime readiness, unrun real verification and an `entries` array. Each entry has experiment/fixture IDs, variant/file lists, authored state, construction-validation state and evidence, and `experiment_execution_status=not_run`. A construction pass records a path/hash of the actual check log. That log attests bundle construction only. The manifest never depends on the ledger or the log, preventing hash cycles.
