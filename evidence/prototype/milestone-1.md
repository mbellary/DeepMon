# Milestone 1 acceptance evidence

Completed on 1 November 2026, Asia/Calcutta (IST). Result: specification and traceability acceptance passed. This report does not claim execution of the twelve prototype experiments, deployed-schema compatibility or hardware capabilities.

## Delivered behavior

The isolated Rust package at `tools/aether-validation/` implements `aether-validate evidence`, `traceability` and a non-destructive `generate` command. No Python interpreter or subprocess was used. The evidence verifier checks the existing v1 bundle directly; record generation and validation use its frozen B01/B02/B03/B05/B06 inputs. The working research files and frozen evidence were not rewritten.

The [scope](../../docs/prototype-scope.md), [experiment registry](../../docs/prototype-experiments.json), [complete status file](../../docs/prototype-status.json) and twelve records under `docs/prototype-budgets/` now exist. The registry is the selection authority, with original requirement objects, mapping/dependency evidence and all six association table cells preserved separately from selection and execution state.

The checker reports:

    metric_requirements=250
    metadata_associations=58
    experiment_specifications=12
    selected_metrics=23
    unselected_metrics=227
    selected_associations=42
    unselected_associations=16
    missing_original_ids=0
    duplicate_ids=0
    synthetic_experiments_executed=0
    deployed_schemas_and_capabilities=not_verified
    runtime_acceptance_readiness=false
    milestone_1_traceability=pass

All four layers are represented. Unselected requirements remain `not_prototyped`; selected ones are `fixture_planned`. Original source-route totals remain E:88, R:35, H:100, I:26, U:1. The historical 26 U rows remain individually reconciled in the frozen ledger. L1-001 remains semantically unresolved, L1-005 inherits its counting gap, and source-specific caveats such as L2-071 remain in the preserved mappings.

## Validation performed

Working directory: `/home/mbellary/work/projects/DeepMon`. Each command completed with exit code 0:

    cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- evidence --root .
    cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- traceability --root .
    cargo test --locked --offline --manifest-path tools/aether-validation/Cargo.toml
    cargo clippy --locked --offline --manifest-path tools/aether-validation/Cargo.toml --all-targets -- -D warnings
    cargo fmt --manifest-path tools/aether-validation/Cargo.toml -- --check
    sha256sum -c /tmp/aether-m1-inputs.sha256
    git diff --check

The Rust test suite passed **30 integration tests**, with zero failures. Tests include valid evidence and unselected records; missing metrics/files; duplicate association IDs; changed definitions/routes/cells; bogus experiment/source references; dropped metadata dependencies; broken budget links; malformed JSON; false synthetic/deployed verification; invented agreement; false readiness; missing criteria; invalid finite durations; generator byte stability and manual-note protection; CLI failures; escaped table pipes; input/output symlink escapes; and rehashed-but-inconsistent metadata/environment states. Test corruption is confined to temporary trees. These are tool correctness tests, not the twelve future telemetry experiments.

The generator initially created fifteen planning files. A second run created zero files and retained byte-identical outputs. A temporary modified record was refused rather than overwritten. No fixture payload, fixture server or engine package was created.

Captured logs: [evidence check](milestone-1/evidence-check.txt), [traceability check](milestone-1/traceability-check.txt), [Rust tests](milestone-1/rust-tests.txt), [Clippy](milestone-1/clippy.txt), [format check](milestone-1/format.txt), [unchanged input checks](milestone-1/baseline-unchanged.txt) and [toolchain identities](milestone-1/toolchain.txt). The zero-byte format log means the command succeeded without diagnostics. Expected baseline digests are retained in `milestone-1/baseline-before.sha256` so preservation can be checked without the temporary source file.

## Version and integrity record

Input bundle: `aether-source-evidence-v1`; environment: `aether-validation-synthetic-local-v1`. The Rust verifier checked ten pinned upstream files and 24 checksummed artifacts. Compiler, Cargo, kernel and libc identities match the recorded reference: Rust 1.99.0, Cargo 1.99.0, x86_64 Linux 7.0.0-38-generic and glibc 2.43. No observed drift or dependency download was needed. Dependencies were resolved from the local cache and locked before evaluated runs: serde 1.0.228, serde_json 1.0.149, sha2 0.10.9, sha1 0.10.6 and tempfile 3.27.0 (tests).

Git base at inspection was `d412dd11a16667f7323cea227c7f0d099d49123b`; delivered working-tree files are identified by [artifacts.sha256](milestone-1/artifacts.sha256), including manifest, Cargo.lock, Rust source/tests and all fifteen planning records. This is not a claim that those changes were committed at that base. Verify the listed bytes with:

    sha256sum -c evidence/prototype/milestone-1/artifacts.sha256
    sha256sum -c evidence/prototype/milestone-1/baseline-before.sha256

## Decisions and remaining limits

Validation tooling is separate from the engine prototype. The twelve safety envelopes remain `proposed_not_agreed`, owner/agreement fields remain unset, and every runtime-readiness flag is false. Three deterministic-correctness criteria per case carry proposed exact-outcome rules; actual acceptance limits remain null. Production CPU/freshness/timing/loss budgets remain unset. The proposed 100-ms poll period, two-retry cap and one-second backoff cap are explicit small synthetic planning choices; they establish no product interval or performance guarantee.

The actual measurements here are evidence integrity and planning-tool behavior. No real inference service, GPU/NIC, fabric, container-runtime control socket, kernel probe, library injection or cloud storage was accessed. No microsecond capture, inference overhead, driver/kernel compatibility, source freshness, production recovery or durability was measured. Existing Python material is historical and was not required for this implementation.

## Handoff

Milestone 1 is complete for its defined specification/traceability scope. **Milestone 2 is next: create reproducible synthetic fixtures** for the approved registry, including source/version references, known expected answers, missing-evidence cases and bounded faults. Budget agreement/readiness remains distinct from fixture authoring and must be resolved before evaluated runtime runs. Milestone 3 later creates the separate engine harness. Milestones 2–8 have not started.
