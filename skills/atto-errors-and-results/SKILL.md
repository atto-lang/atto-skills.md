---
name: atto-errors-and-results
description: Handle AttoLang Option, Result, typed Error, throw, raise, panic and trap. Use for fallible APIs, recovery, cancellation, supervisors, error propagation and diagnostics.
---

# Errors, Option/Result and recovery

Keep **ordinary values** separate from **exceptional control**. An `Option<T>` or `Result<T,E>` value is not automatically a thrown exception. A failed Result must not become a panic without an explicit operation.

## Contracts (TARGET unless backed by guest tests)

| Construct | Meaning |
| --- | --- |
| `throw Error` | Ordinary exception intercepted by nearest matching `try/catch` |
| `raise Error` | Bypasses ordinary catches toward authorized recovery |
| `panic Error` | Actor failure, supervisor CONTROL; cannot be caught by the actor |
| `trap expr` | Draft contract converts escaping ordinary throws to `Option<T>`; **not** panic/raise/cancellation |
| `defer` | Cleanup during eligible normal or exceptional exit |
| `?` | Context-sensitive early propagation target; verify typing/lowering |

Only an `Error`-derived value is valid for `throw`, `raise` and `panic` in the target. Error construction need not capture a stack trace; explicit trace capture should be bounded and permission-aware.

## Procedure

1. Inspect the function's declared return/effects and chosen compiler's actual lowering.
2. Prefer explicit `Result` admission/return values for recoverable failures; handle `Ok`/`Err`, or use tested propagation operators.
3. Preserve error causes and typed diagnostics; avoid using `unwrap()` for routine control flow because it may panic.
4. Treat cancellation separately from a catchable error; cleanup must not revive a cancelled/stale actor.
5. For actor panic, report through supervisor CONTROL and restart policy; do not swallow it inside receiver code.
6. Preserve exact-once cleanup and avoid converting a stronger failure into success in `finally`/`defer`.

**Trap ambiguity:** one design request describes `trap worker.send(msg)` producing `Result`, while the written language spec describes `trap expr` returning `Option`. This is unresolved. Do not silently promise both; use the send API's explicit admission Result until a verified amendment decides.

See [failure specification](https://github.com/atto-lang/atto-spec) and [types](https://github.com/atto-lang/atto-spec).
