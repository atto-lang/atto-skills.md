---
name: atto-ownership-and-memory
description: Review AttoLang ownership, borrowing and memory safety. Use for rt take, rt copy, borrows, shares, actor messages, pointer transport, lifetimes, arena memory and FFI safety.
---

# Ownership and memory safety

The source-level ownership model applies to ordinary values, actor state, tasks, closures, channels and transport. Managed backends must preserve the same observable rights rules as native LLVM.

## Operations (language TARGET)

| Source | Meaning |
| --- | --- |
| `rt take value` | Move exclusive ownership; source must become unusable |
| `rt copy value` | Duplicate only proven Copy types |
| `rt borrow value` | Scoped immutable loan |
| `rt borrow mut value` | Scoped exclusive mutable loan |
| `rt share value` | Explicitly permitted frozen/shared ownership |
| `rt ref` / `rt deref` / `rt unref` | Checked managed-handle operations |

~~~atto
val packet = make_packet()
val owned = rt take packet
// Any subsequent use of packet must fail a move check.
~~~

Illustrative: concrete value types, method signatures and backend admission must be checked before claiming compilation.

## Borrow-safety checklist

1. Track owner identity and initialization through control-flow joins and calls.
2. After `take`, reject use or double-drop of the old place.
3. Mutable borrow must be exclusive; ordinary borrows may not survive a conflicting mutation.
4. At `await`, `select`, task spawn, actor turn changes and cancellation, either prove pinning/lifetime/alias safety or reject the escaping loan.
5. Treat `critical-read` / `critical-write` actor-state loans as scoped and nonescaping. Never serialize mutable actor-state pointers or capture them in child tasks.
6. Validate transport **type, depth, bytes, ownership action, generation and capability** before publishing a message. `Sendable` is not identical to `Serializable`.
7. Never transport raw `Pointer<T>` addresses across isolates. An explicitly admitted copy/move/frozen-share representation is required.

## Runtime distinctions

LLVM targets zero tracing GC on supported native paths, but needs actual allocator and sanitizer evidence. Java/GraalVM targets low GC through arenas, not universal zero GC. Per-actor heaps are logical ownership domains, not automatically protected OS address spaces.

For changes: add negative compile fixtures (use-after-take, leaked borrows, wrong-domain ref, forged share) plus runtime races and native sanitizer jobs. See [ownership specification](https://github.com/ores-truffle-oreslang/attolang-language-spec.md/blob/main/spec/04-ownership-and-memory.md).
