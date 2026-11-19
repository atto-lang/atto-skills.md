---
name: atto-ownership-and-memory
description: Review AttoLang ownership, borrowing and memory safety. Use for take!, copy!, copy!!, borrow!, share!, actor messages, pointer transport, lifetimes, arena memory and FFI safety.
---

# Ownership and memory safety

The source-level ownership model applies to ordinary values, actor state, tasks, closures, channels and transport. Managed backends must preserve the same observable rights rules as native LLVM.

## Operations (language TARGET)

| Source | Meaning |
| --- | --- |
| `take! value` | Move unique ownership; the original owned place becomes unusable |
| `copy! value` | Shallow copy where type/alias permissions allow it; not deep transport isolation |
| `copy!! value` | Deep graph copy of reachable mutable data with independently owned storage, where supported |
| `borrow! value` | Scoped shared read-only loan; ordinary nonprimitive arguments may borrow implicitly under the current target contract |
| `borrow! mut value` or `fnc f(Foo x)` called as `f(mut x)` | Exclusive temporary write loan, subject to checked effects and writable authority |
| `share! value` | Frozen read-only shared ownership; no implicit mutable aliases or locks |
| `ref!` / `deref!` / `unref!` | Checked handle operations; do not infer raw-pointer privileges |

~~~atto
val packet = make_packet()
val owned = take! packet
// Any subsequent use of packet must fail a move check.
~~~

Illustrative: concrete value types, method signatures and backend admission must be checked before claiming compilation.

## Borrow-safety checklist

1. Track owner identity and initialization through control-flow joins and calls.
2. After `take!`, reject use or double-drop of the old place.
3. Mutable borrow must be exclusive; ordinary borrows may not survive a conflicting mutation.
4. At `await`, `select`, task spawn, actor turn changes and cancellation, either prove pinning/lifetime/alias safety or reject the escaping loan.
5. Treat `critical-read` / `critical-write` actor-state loans as scoped and nonescaping. Never serialize mutable actor-state pointers or capture them in child tasks.
6. Validate transport **type, depth, bytes, ownership action, generation and capability** before publishing a message. `Sendable` is not identical to `Serializable`.
7. Never transport raw `Pointer<T>` addresses across isolates. An explicitly admitted copy/move/frozen-share representation is required.

## Runtime distinctions

LLVM targets zero tracing GC on supported native paths, but needs actual allocator and sanitizer evidence. Java/GraalVM targets low GC through arenas, not universal zero GC. Per-actor heaps are logical ownership domains, not automatically protected OS address spaces.

These are **language target rules**: confirm the selected compiler's exact guest-level implementation. Compare [ownership and synchronized guards](https://github.com/atto-lang/atto-spec/blob/main/language/shared-ownership-and-guards-v1.md) and [implicit argument borrowing](https://github.com/atto-lang/atto-spec/blob/main/language/implicit-argument-borrow-v1.md). For changes: add negative compile fixtures (use-after-take, leaked borrows, wrong-domain ref, forged share) plus runtime races and native sanitizer jobs. See [ownership specification](https://github.com/atto-lang/atto-spec).
