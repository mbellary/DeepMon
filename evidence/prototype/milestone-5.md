# Milestone 5: synthetic enrichment and lifecycle evidence

Milestone 5 is complete for the intermediate Rust prototype. It demonstrates scoped, evidence-based metadata association against invented inputs. It does not qualify live collection, physical device identity, inference overhead, timing accuracy, backward compatibility or production readiness. No existing cluster is available. Product requirements, all four layers, 250 metric families, 58 metadata associations and source eligibility remain unchanged.

The detailed tracking document is [the Milestone 5 execution plan](../../milestone-execution-plans/milestone-05-execplan.md). The [master plan](../../EXECPLAN.md) and [folder index](../../milestone-execution-plans/README.md) record completion and the next milestone. The [enrichment contract](../../docs/prototype-enrichment-contract.md) defines the implemented scope and limits.

## Implemented behavior

Typed entity keys distinguish correlation scope, incarnation and explicitly supplied host/boot/namespace facts. Symbolic locators remain symbolic. The immutable relationship graph preserves historical facts, deduplicates identical evidence, rejects conflicting evidence IDs and validates typed references. Resolution uses explicit compatible clocks and half-open validity intervals. Missing query time does not become a current timestamp. A query must match an explicitly keyed emitter or subject before it can enrich that measurement.

Original adapters execute publisher ambiguity and remote subjects (PX04), replica resource membership (PX05), process/container lifetime and denied visibility (PX06), NIC capability/configuration/use separation (PX07), and visibility/privacy/policy (PX11). A replica count of 100 linked to two allocated GPUs stays one measurement totaling 100. Allocation does not prove participating devices. Untimed relationships remain reported-only or ambiguous; missing ownership remains unknown. Unattached NIC capability facts, a configured preference and a TCP control socket do not establish an active payload path or loaded CUDA/NCCL versions.

The new `fixtures/enrichment-v1/` extension has seven cases and 16 files, authored with literal expected answers in Rust. It tests lifecycle endpoints, missing/single/multiple candidates, boot/namespace collisions, set membership/cardinality, membership history, NIC intent/use and denied/private/conflicting metadata. Original fixtures and oracles remain unchanged. Loading confines paths, rejects symlinks/unindexed files, checks provenance/roles/sizes/hashes and bounds typed records. Files are limited to 1 MiB; cases to 32 unique nodes and 32 unique relationships, 64 queries/measurements, and 64 initial/update records each. The deduplicating merge accepts at most 128 raw records. These limits are local prototype choices, not agreed runtime budgets.

## Results and assertion accounting

All 17 assigned original assertions match independent expected answers: seven newly exercised in M5 and ten existing checks rechecked. All 69 boundary checks pass across seven extension cases. These counts remain separate from the original 49 assertion identities.

The separate [current execution ledger](../../docs/prototype-enrichment-execution-status.json) retains all 49 original assertions: 31 matched, two expected rejections, one proof verified, one queue assertion deferred to M6, and 14 later-stage assertions not run. Prior-only evidence is linked to the accepted M4 ledger and is explicitly not rerun by the association verifier. M4's historical ledger is unchanged.

The [association facet ledger](../../docs/prototype-enrichment-coverage.json) retains all 58 original definitions and their source hash. It reports 23 exercised synthetic facets and 35 entries unexercised in M5. A facet check does not establish complete association support. Physical identity, live acquisition and the remaining facets are unverified. M21/M33/M35/M38 remain unexercised rather than treating generic unknown output as evidence of their implementation.

## Reproduction and verification

Run from `/home/mbellary/work/projects/DeepMon`:

    cargo test --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml
    cargo test --locked --offline --manifest-path tools/aether-validation/Cargo.toml
    cargo fmt --manifest-path prototypes/aether-poc/Cargo.toml -- --check
    cargo clippy --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml --all-targets -- -D warnings
    cargo run --locked --offline --manifest-path prototypes/aether-poc/Cargo.toml -- verify-milestone --root . --milestone 5

All 161 tests pass: 32 new enrichment tests, four prior client unit tests, 25 prior harness tests, 30 prior metric tests, and 70 unchanged validation tests. There are zero final failures or skipped socket tests. New enrichment scenarios are file-only and use virtual time. Existing HTTP regressions used the approved execution path permitting owned loopback sockets. No real endpoint was contacted. `milestone-5/full-harness-tests.txt`, `enrichment-tests.txt` and `validation-tests.txt` record actual execution. Formatting, locked offline check and strict Clippy pass.

Six planned demonstrations ran twice: publisher ambiguity, process lifecycle, resource membership, fabric context, time boundary and stage-5 verification. The `publisher`, `process`, `resources`, `fabric`, `boundary` and `verify-milestone` pairs are byte-identical. `determinism.txt` records comparison success. `m4-regression.json` is byte-identical to M4's accepted `verify-milestone-final-1.json`.

`examples/author_enrichment_fixtures.rs` authored 16 files and an identical rerun wrote zero; conflicting existing files are refused. `examples/record_enrichment_evidence.rs` validates the saved report, provenance and original IDs before writing protected separate ledgers. Identical reruns succeed. They index functional evidence, not signed attestations or independently measured performance.

## Preservation and findings

Before live edits, accepted M4 and required predecessor inputs were preserved at original relative paths under `milestone-5/inputs/milestone-4/`. All 405 indexed predecessor file hashes pass. M4's original 260-file acceptance list passes against that root and predecessor fixture validation passes. M2's original 100-artifact list and the original research baseline pass against live files. Baseline source/traceability/fixture checks pass. The input index records the starting Git revision separately from three pre-existing uncommitted planning changes, plus the observed local environment. No environment drift from the M4 synthetic reference was observed.

The first new fabric test exposed integer byte-count coercion through floating-point decoding. The adapter now preserves `serde_json::Number`; the independent expected integer was retained. The initial failure log is kept separately from final passing results. An unrelated-query guard prevents attaching a correct association to the wrong measurement. One prior unsupported-stage test now uses stage 6 because stage 5 is implemented; original source remains preserved in the predecessor archive.

No dependency or lockfile change was required. All implementation, fixture helpers and tests use Rust; no Python was used. Existing Cargo dependencies and the validation-tool source remain unchanged. Initial/draft diagnostic artifacts are retained by name; final acceptance uses the saved final demonstration pairs and current ledgers. Output sanitation retains known safe facts and removes the synthetic sensitive marker; archived original fixtures preserve historical input bytes.

`milestone-5/artifacts.sha256` indexes current source, new fixtures/docs, this report and evidence inputs/logs. It excludes build output, itself and the living master/detailed plans/index so later tracking changes do not rewrite acceptance. Historical hash lists are preserved unchanged and verified in their corresponding archived roots.

Runtime acceptance remains `blocked_unagreed_budget`, readiness false, and real verification `not_run`. No whole-PX runtime pass, physical microsecond accuracy, zero overhead or production claim follows. DeepInfer remains deferred, and initial vLLM/SGLang/Dynamo review does not exclude relevant other engines.

The next implementation milestone is M6: bounded queue behavior, a local journal with a documented acknowledgement boundary, compatible Arrow/Parquet/DataFusion libraries, committed-file SQL queries and simulated storage/restart failures. It must test accepted/rejected/replayed counts and avoid duplicate logical batches within the explicitly tested local durability boundary. It is not yet implemented.
