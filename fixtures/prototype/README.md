# Invented observability fixtures v1

All inputs and expected answers are synthetic. No engine, host, device, fault adapter or network is executed here. Inspect manifest.json for twelve parent PX groups, local input/endpoint/expected files, byte hashes, source references and negative variants. Full format: ../../docs/prototype-fixture-format.md.

All timestamps are virtual microseconds with clock evidence; intervals are half-open. Oracle counterexamples are kept out of aggregate-only runtime inputs. Native family names follow pinned source definitions, but all values, aliases and test bucket boundaries are invented. Malformed exposition is useful test input only when its specific rejection is actually checked. Hardware identities are symbolic.

Run cargo run --locked --offline --manifest-path tools/aether-validation/Cargo.toml -- fixtures --root . from the workspace. A construction pass is not an engine-experiment pass, real schema/capability verification or runtime budget agreement. Source/registry/status/budget inputs remain accepted Milestone 1 bytes.
