---
name: atto-supervision
description: Design and audit AttoLang actor supervisors, actor lifecycles and failure recovery. Use for define supervisor, restart trees, panics, CONTROL notices and actor isolation.
---

# Supervision and actor failure

## Mental model

A supervisor owns the controlling lifecycle of child actors, and in the hierarchy TARGET may own child supervisors. A failed child sends a bounded, authenticated **CONTROL** notice to its immediate parent; ordinary messages travel over DATA. A full DATA inbox must not prevent reporting terminal failure. A user actor cannot catch its own panic and resume.

`define supervisor ...`, nested supervisor policies, and `on_*` lifecycle APIs appear in the draft specification, but **the declaration/lowering is not universally implemented**. Do not invent runnable syntax for policies or hook parameters.

## Procedure for supervisor code

1. Locate an actual supervisor implementation and tests on the target backend.
2. Define parent/child identity, incarnation generation, quotas, restart policy and escalation behavior.
3. Separate CONTROL from bounded DATA; authenticate sender and generation; publish terminal notice at most once.
4. On restart, fence the old generation, in-flight callbacks, tasks, timers and message capabilities. Treat `get_done()` as completion only after resources physically retire.
5. Test supervisor failure, nested restart, stale control replay, full inbox, worker starvation, cancellations and host-fatal boundaries. Do not call actor confinement an OS sandbox.

## Failure semantics

- `throw Error` is ordinary exceptional control (nearest matching `try/catch`).
- `raise Error` bypasses ordinary catches to an authorized recovery boundary.
- `panic Error` ends the actor turn/incarnation and reports via supervisor CONTROL. Never swallow it with `catch` or `trap`.
- Root/main unhandled failure yields a non-success result, with exact process exit code determined by profile.
- A supervisor restart is **not** recovery of the failed actor's same stack frame.

Read [supervisor hierarchies](https://github.com/ores-truffle-oreslang/attolang-language-spec.md/blob/main/spec/13-supervisor-hierarchies.md), [failure](https://github.com/ores-truffle-oreslang/attolang-language-spec.md/blob/main/spec/07-errors-and-failure.md), and [runtime guarantees](https://github.com/ores-truffle-oreslang/attolang-language-spec.md/blob/main/spec/18-runtime-guarantees.md).
