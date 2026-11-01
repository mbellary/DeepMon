# Evidence integrity verification v1

Performed on 1 November 2026 in the local workspace. This verifies research-input integrity, not inference engine behavior, synthetic fixture behavior or real hardware capabilities.

Command:

    python3 scripts/check-evidence-inputs.py

Observed output:

    PASS: 10 pinned upstream inputs; 24 artifact checksums; 26/26 historical U rows reconciled.
    Documentation routes: E=88 R=35 H=100 I=26 U=1. Remaining U: L1-001; L1-005 inherits its semantic gap.
    Reference environment: aether-validation-synthetic-local-v1. Deployed schemas/capabilities: NOT VERIFIED.

An isolated temporary copy of the evidence and environment directories passed the same checker. Appending deliberate extra bytes to its E06 snapshot then caused a nonzero exit and an artifact checksum mismatch. The workspace's frozen snapshots were not modified by that test.

Additional checks confirmed that the original catalog Markdown/JSON, metadata association matrix, original three-engine baseline and uploaded PLANS were unchanged; the live Markdown/JSON source matrices agree on all 250 route IDs; local document links resolve; and all eight prototype milestones remain pending. Repository whitespace checking passed.

No endpoint was queried, source feature enabled, GPU/NIC inspected, cluster provisioned, fixture created or Rust package built. No deployed-schema or hardware-capability claim follows from these integrity checks.
