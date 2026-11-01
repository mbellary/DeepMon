# Aether validation tooling

This Rust package verifies frozen research evidence and Milestone 1 planning records. It does not collect telemetry, start an engine, inspect hardware or certify deployed compatibility. Python is not required or invoked.

From the repository root:

    cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- evidence --root .
    cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- traceability --root .
    cargo test --locked --offline --manifest-path tools/aether-validation/Cargo.toml

`--offline` works with the dependencies already cached in the recorded environment. A fresh environment must obtain the locked dependencies through its normal authorized dependency access before offline builds can work. Use Rust 1.99.0 with the recorded target/compiler identity in `validation/first-environment-v1.json`; report drift rather than silently installing a toolchain.

The `generate` command creates the fifteen planning files from the frozen baseline and the twelve case specifications in `src/generate.rs`. It accepts `--root PATH`, writes only the declared `docs/` outputs, leaves byte-identical files unchanged and refuses to overwrite differing content. This protects manual notes and later evidence. When intentionally changing the plan or cases, review and edit the corresponding records; do not erase hand edits just to rerun generation.

`evidence` verifies manifest/artifact SHA-256, Git blob identities, index consistency, original requirements, historical-U reconciliation and synthetic-only qualification states. `traceability` first runs the same evidence checks, then checks requirement/association preservation, experiment links, dependency completeness, budget proposal states, unexecuted status and local paths. It intentionally validates Milestone 1's phase: later real/synthetic execution states will require an explicit validator/schema revision, not relabeling this milestone as runtime verification.

Results use exit code 0 for a valid specification, 1 for validation failure, and 2 for invalid command usage. A passing check is not a passing experiment. Unit/integration tests use temporary copies and never change frozen inputs or real planning records.

Dependencies are exact in Cargo.toml and resolved in Cargo.lock. SHA-1 is used only to compare upstream Git blob identities; SHA-256 protects evidence content. No network client, device interface, process-injection facility or production engine dependency is included.
