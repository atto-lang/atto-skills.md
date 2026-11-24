---
name: atto-parameters-and-arguments
description: Audit optional and named AttoLang parameters, Option type sugar, call binding and defaults. Use when designing function signatures, compiler diagnostics or overload and argument resolution.
---

# Optional and named parameters and arguments

**Status: TARGET / proposed until proven at an exact compiler revision.** Do not claim these spellings pass a particular backend merely because the parser accepts related syntax. New Atto sources use `.atto`, not `.ores`.

## Two independent meanings of `?`

- A question mark on a **formal parameter name** (`y?`) means callers may omit that argument. It does not change the declared value type.
- A question mark on a **type** (`int?`) is shorthand for `Option<int>`. It does not by itself make an argument optional at the call site.
- A default (`= 10`) applies only when the caller omits that optional formal.
- An optional parameter without a default needs an Option-valued binding or another explicit omission rule; do not invent an uninitialized value.

## TARGET: both signature orientations

~~~atto
fnc calculate(
  x: int,
  y?: int = 10,
  z?: int?
): int {
  return x + y;
}

pub fnc main(): int {
  return calculate(x: 20, y: 22);
}
~~~

The corresponding type-first form is also part of the requested design:

~~~atto
fnc calculate(
  int x,
  int y? = 10,
  int? z?
): int {
  return x + y;
}
~~~

The same call should resolve to the same parameter mapping in either orientation; syntax acceptance, inferred types, and runtime defaults must each be tested. For a missing `z?: int?`, use the actual backend's Option/None semantics rather than manufacturing a second Option layer.

## Diagnostics and evaluation checklist

1. Match named arguments by the **formal name**, not their textual order; reject duplicates and unknown names.
2. Reject a missing required `x`, surplus positional values and a positional argument following a named one.
3. Evaluate every supplied argument exactly once, in source order, before applying defaults. Defaults must not accidentally evaluate an explicit argument again.
4. Type-check a supplied `int?` against `Option<int>` and a supplied plain `int` against `int`; `name?` is not a type conversion.
5. Reject duplicate formal names after normalization of `?`, ambiguous overloads, and unsafe defaults that capture out-of-scope bindings.
6. Check both the native compiler entry point and actual guest execution. A formatter round-trip or a host parser-only pass does not prove correct argument binding.

## Negative conformance cases — TARGET

~~~atto
// missing required argument: reject
calculate(y: 22);

// duplicate named argument: reject
calculate(x: 20, x: 21);

// unknown label: reject
calculate(x: 20, unknown: 5);

// positional after named: reject
calculate(x: 20, 22);
~~~

These are design-time diagnostics, not asserted runnable fixtures. Before promoting this skill's status, add pass/fail `.atto` fixtures for each signature orientation, omitted/defaulted/explicit `Option` parameters, named evaluation order, overload collisions, and cross-backend execution evidence.

See [functions and modules](../atto-functions-and-modules/SKILL.md) for broader call resolution, and the [feature status policy](../../docs/FEATURE_STATUS.md) for evidence requirements.
