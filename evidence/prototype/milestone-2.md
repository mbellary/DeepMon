# Milestone 2 acceptance evidence

Completed on 1 November 2026, Asia/Calcutta (IST). Result: reproducible synthetic fixture construction and validation passed. No engine experiment, deployed schema, hardware capability or production budget is claimed verified.

## Delivered artifacts and behavior

The [fixture manifest](../../fixtures/prototype/manifest.json) indexes **12 parent groups, 36 variants and 82 payload files**. There are seven distinct positive endpoint producer variants: vLLM, SGLang, Dynamo, TGI, Triton, native TensorRT-LLM and Triton's TensorRT-LLM backend. Two deliberately malformed exposition payloads are rejected for their declared TYPE/histogram errors. Other negative cases are well-formed but have incomplete, denied, ambiguous or limited semantic evidence.

The files cover endpoint snapshots; counter reset/unresolved TGI admission semantics; aggregate counterexamples; frontend/backend overlap and remote subjects; heterogeneous GPUs; PID/cgroup changes; NIC capability versus active transport; complete/incomplete rank and clock evidence; stream overlap; simulated persistence boundaries, retries and losses; action denial/redaction; and bounded queue/freshness uncertainty. All timestamps and values are invented. Expected answers are authored literals and explanations, independently checked for simple consistency rather than produced by a future observability engine.

[Format notes](../../docs/prototype-fixture-format.md) specify the strict envelopes, provenance, virtual clocks, limits, finite exposition grammar and oracle/actual-result distinction. The [fixture-status ledger](../../docs/prototype-fixture-status.json) records authored groups and construction-validation evidence. Experiment execution remains not_run, and runtime acceptance readiness remains false. The existing Milestone 1 registry/status/budget files, all 250 metric requirements, all 58 associations and frozen v1 inputs remain unchanged.

Rust tooling now supplies `aether-validate fixtures` and non-destructive `author-fixtures`. Construction validation calls the existing evidence/traceability checks, validates exact registry/source/budget links, local path confinement, byte hashes, bounded payloads, clocks/incarnations, file indexing and independent oracle consistency. Intentionally invalid samples cannot bypass packaging, hash or provenance checks. No network, process, GPU/NIC or actual fault operation is invoked.

## Actual verification

Working directory: `/home/mbellary/work/projects/DeepMon`. These commands completed successfully with exit code 0:

    cargo run --quiet --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- evidence --root .
    cargo run --quiet --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- traceability --root .
    cargo run --quiet --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- fixtures --root .
    cargo test --locked --offline --manifest-path tools/aether-validation/Cargo.toml
    cargo clippy --locked --offline --manifest-path tools/aether-validation/Cargo.toml --all-targets -- -D warnings
    cargo fmt --manifest-path tools/aether-validation/Cargo.toml -- --check
    git diff --check

The fixture command reports:

    fixture_groups=12
    fixture_variants=36
    fixture_payload_files=82
    intentional_invalid_payloads=2
    missing_experiment_groups=0
    unindexed_payload_files=0
    synthetic_experiments_executed=0
    deployed_schemas_and_capabilities=not_verified
    runtime_acceptance_readiness=false
    milestone_2_fixture_validation=pass

All **70 Rust integration tests pass**: 40 fixture tests and all 30 original Milestone 1 regressions. The fixture tests cover all groups; actual negative-payload rejection and wrong-reason checks; corruption and missing artifacts; dropped dependencies and unknown sources; budget hashes; duplicate incarnations; clock units/offset context/timestamps and missing/cross-clock stream provenance; changed numeric/native/proof answers; malformed histograms; unsafe paths/symlinks; oversized/truncated inputs; real/captured provenance; false runtime/construction claims; forbidden fault commands; ledger links; sensitive-marker leakage; hidden queue rejections; incomplete stream/group exactness; CLI roots; and repeatable authoring/manual-note protection. Tests operate on isolated temporary copies, never the delivered bundle.

The initial Rust authoring run created 85 files: 82 payloads, manifest, README and the separate ledger. Before advancing the ledger, a repeat authoring run created zero files and retained identical bytes. The ledger was then updated only with actual construction-check log evidence; authoring must not reset this later state and will refuse differing files. Repeated read-only fixture checks pass with identical verdict output.

Logs: [fixture construction](milestone-2/fixtures-check.txt), [final ledger-aware fixture check](milestone-2/fixtures-final-check.txt), [Rust tests](milestone-2/rust-tests.txt), [Clippy](milestone-2/clippy.txt), [format check](milestone-2/format.txt), [source evidence](milestone-2/evidence-check.txt), [traceability](milestone-2/traceability-check.txt), [baseline preservation](milestone-2/baseline-unchanged.txt) and [M1 planning preservation](milestone-2/milestone-1-planning-unchanged.txt). The ledger references the first construction log's SHA-256; the final check validates that reference without any circular manifest dependency.

## Predecessor and current version evidence

Before changing the shared Rust tool, the accepted Milestone 1 source/input bytes were copied under `evidence/prototype/milestone-2/inputs/milestone-1/`. The [input index](milestone-2/input-index.json) maps originals to snapshot paths. The original M1 artifact hash list verifies against that predecessor root; see [verification log](milestone-2/predecessor-verification.txt). Current evidence and traceability commands also pass against the preserved predecessor inputs: [evidence](milestone-2/predecessor-evidence.txt), [traceability](milestone-2/predecessor-traceability.txt).

The old M1 hash list/report was not regenerated from changed code. The live tool intentionally adds fixture modules, commands and tests, and exposes the existing protected output writer for reuse. Earlier evidence/traceability/generation behavior remains covered by the thirty passing regressions. Original M1 planning records and Cargo dependency versions are unchanged. No Python runtime is invoked and no additional dependency or download was needed.

The inspected Git base was `31c550bcc36831204994fc8c99072d598af6c8cf`; this does not assert that the delivered working-tree changes were committed there. Actual source, dependency lock, format/ledger and fixture bytes are listed in the new [M2 artifact hash index](milestone-2/artifacts.sha256). Compiler/Cargo/kernel/libc identities in [toolchain.txt](milestone-2/toolchain.txt) match the original local reference; no environment drift was observed. The manifest's pinned registry/status/source/budget hashes make the input versions explicit.

To verify current M2 artifacts from the workspace:

    sha256sum -c evidence/prototype/milestone-2/artifacts.sha256
    sha256sum -c evidence/prototype/milestone-1/baseline-before.sha256

To verify historical M1 delivered bytes, change directory to its preserved predecessor root and run its original `evidence/prototype/milestone-1/artifacts.sha256` there. Applying that old list to intentionally revised live Rust source is not an appropriate current-version test.

## Scope limits and handoff

There is no existing cluster. Source mappings remain documentation routes, with L1-001/L1-005 and other stated semantic caveats unresolved. Symbolic hardware/metadata and hypothetical clocks do not qualify actual source discovery, drivers, CUDA/NCCL compatibility, fabric attribution, microsecond acquisition, throughput impact or production durability. Proposed runtime budgets remain unagreed. The finite construction parser is not the eventual production ingestion adapter.

PX10 stage facts are authored hypothetical boundary inputs for a future fault adapter, not executed storage outcomes. Counterexample worlds in expected files are kept out of aggregate-only runtime inputs. Missing association evidence stays unknown even when its requirement ID is present. The twelve experiments still need an actual harness and later behavior comparisons against these oracles.

**Next implementation milestone: Milestone 3 — build the isolated Rust prototype harness and loopback fixture server.** It should consume this manifest, validate fixture selection, expose invented endpoint data locally, preserve provenance and report unsupported/unknown behavior. It must remain separate from the validation crate and must not connect to real engines or devices. Milestones 3–8 have not started.
