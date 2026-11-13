---
name: atto-tasks-and-channels
description: Write AttoLang asynchronous tasks, await operations, typed channels and select loops. Use for generators of async work, actors spawning tasks, backpressure, scheduling and cancellation.
---

# Async tasks, channels, and scheduling

A `Task<T>` is a structured lightweight computation, not an actor. A `Future<T>` represents a value that may settle later; `await` suspends logical work rather than requiring one blocked OS thread per operation.

## Task TARGET (not certified across backends)

~~~atto
let Task<int> work = nb {
  return compute()
}
val result = await work
~~~

Confirm the specific compiler supports this form and distinguishes task creation from a Future. `do nb { ... }` is the proposed detached-handle/discard syntax but must **not** discard lifecycle or cancellation obligations.

## Channels and select TARGET

~~~atto
do select<int> over
  readch updates as value -> value
  after timeout -> 0
end
~~~

The arm grammar and timeout lowering are still targets; verify with compiler fixtures first. `nb-select` is a compound keyword, unlike arbitrary hyphenated names. Avoid a blocking `readch` inside a nonblocking selection arm unless a verified adapter explicitly allows it.

## Design rules

- Bound channel capacity and payload bytes; preserve FIFO/admission guarantees where the channel's contract promises them.
- Distinguish enqueue success, handling success, and completion. Check closure and cancellation races.
- Ownership/borrow checks must run on captures, `await`, `select`, task exit and message handoff.
- Tasks spawned inside actors must be attached to the actor's scope/scheduling domain as designed; they must not silently migrate into the supervisor CONTROL pool or acquire mutable actor-state access.
- `rt cooperate` provides a fairness checkpoint; actor scheduling uses reductions/time budgets, but a watchdog alone is not a hard preemption guarantee.
- Async I/O may be named without suffix or `_async`; a synchronous-style convenience API must end in `_blocking` and still suspend without parking the actor carrier.

See [async/scheduling](https://github.com/ores-truffle-oreslang/attolang-language-spec.md/blob/main/spec/06-async-and-scheduling.md) and [ownership](https://github.com/ores-truffle-oreslang/attolang-language-spec.md/blob/main/spec/04-ownership-and-memory.md).
