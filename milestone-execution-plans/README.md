# Milestone execution plans

Keep all milestone execution plans in this folder. The [master execution plan](../EXECPLAN.md) tracks the whole prototype program; each milestone plan tracks that milestone's detailed work, evidence, decisions and remaining tasks. Follow [PLANS.md](../PLANS.md) when writing or updating a plan.

Use `milestone-NN-execplan.md`, with a two-digit milestone number. Add a link here when creating a future plan; do not create empty plans or mark future milestones started merely to populate the folder.

| Milestone | Plan | Current status |
|---|---|---|
| 1 — Prototype scope and complete requirement tracking | [Milestone 1 execution plan](milestone-01-execplan.md) | Complete — specification/traceability accepted; [evidence](../evidence/prototype/milestone-1.md) |
| 2 — Reproducible synthetic fixtures and expected answers | [Milestone 2 execution plan](milestone-02-execplan.md) | Complete — fixture construction accepted; [evidence](../evidence/prototype/milestone-2.md) |
| 3 — Isolated Rust harness and local fixture server | [Milestone 3 execution plan](milestone-03-execplan.md) | Complete — synthetic harness accepted; [evidence](../evidence/prototype/milestone-3.md) |
| 4 — Native metric parsing and justified calculations | [Milestone 4 execution plan](milestone-04-execplan.md) | Complete — scoped functional checks accepted; [evidence](../evidence/prototype/milestone-4.md); M5 enrichment/lifecycle next; runtime acceptance blocked |

Update the per-milestone plan's Progress, Surprises & Discoveries, Decision Log, Outcomes & Retrospective and Revision Notes at each stopping point. Mirror significant status changes and the next unfinished task in the master plan. A plan being ready is different from the milestone being complete. Mark a milestone complete only after its outputs and acceptance checks exist and pass.

Use Rust for validation tools, generation helpers, tests and engine work. Milestone 1 bootstraps the separate `tools/aether-validation/` crate; historical Python evidence tooling is not a runtime prerequisite.

Scope remains intermediate prototypes and synthetic fixtures. This folder does not approve production architecture, real-cluster access or deployment.
