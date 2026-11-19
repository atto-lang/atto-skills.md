---
name: atto-actors-and-state
description: Build AttoLang actors with mailboxes and state confinement. Use for actor definitions, spawn/send, receive, critical read/write, declared actors and actor message safety.
---

# Actors, state, and mailboxes

An actor is a special nominal type with identity, mailbox, lifecycle, supervision, and owned memory. It is **not** an OS thread or ordinary class.

## Two separate syntax levels

**Playground legacy/limited example:** [actors.atto](https://github.com/atto-lang/atto-playground/blob/main/examples/actors.atto) demonstrates `define actor`, `spawn`, `receive`, and `send`. Its direct `self.total` mutation does **not** establish compliance with the newer sealed-state contract. Do not copy that pattern into a compiler/profile that enforces sealed state.

**Sealed-state TARGET example** (verify before use):

~~~atto
define actor Counter as
  state {
    let int count: 0
  }

  receive(Increment message) -> void ::
    self.handle_state(critical-write |s| -> {
      s.count = s.count + message.amount
    })
  end

  read_count() -> int ::
    return self.handle_state(critical-read |s| -> s.count)
  end
end
~~~

## Invariants for generated actors

- State is sealed; direct external state access, leaked state references, or writes outside `critical-write` fail compilation.
- `critical-read` supports nonmutating loans; `critical-write` grants exclusive mutation. Actor turns may be concurrent by default; `define serial actor` is a target for at most one active turn.
- Do not `await` or spawn a task while holding an unproven state loan. Tasks must not inherit actor-state authority just because they share a worker pool.
- A bounded `send(message)` is **admission**, not completion; honor its actual typed result/failed-send ownership semantics. Use explicit `get_done()`/other supported completion protocol when needed.
- A message crossing actors must be `Sendable` under the selected transport; serialization is a separate `Serializable` concern.
- Panic is sent to the supervisor CONTROL path, not caught in the actor's own DATA handler.
- New namespace-based `declare actor Name implements ...;` and `pub mtd (Name ...)` are a requested design; **do not assume every backend lowers them**.

When implementing actors, write positive send/read tests and negative tests for unguarded writes, receiver forgery, stolen ownership, closed/full inbox, stale generation, and cancellation.

See [actor semantics](https://github.com/atto-lang/atto-spec) and [shared ownership](https://github.com/atto-lang/atto-spec).
