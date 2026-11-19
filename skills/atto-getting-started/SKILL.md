---
name: atto-getting-started
description: Write and run first AttoLang programs. Use for hello world, .atto files, compiler selection, basic functions, CLI or playground setup and checking generated code.
---

# Getting started with AttoLang

AttoLang is an actor-first, statically typed systems language. New code is `.atto`, rather than a host-language or migration-era source spelling. Function and method names use `snake_case`.

## First program — grounded in the playground fixture

~~~atto
fnc add(int a, int b): int {
  return a + b;
}

pub fnc main(): int {
  val x: int = 40;
  stdio.println(add(x, 2));
  return add(x, 2);
}
~~~

This uses the form in [the checked-in playground hello example](https://github.com/atto-lang/atto-playground/blob/main/examples/hello.atto). Do not infer that the playground's limited Wasm compiler supports every language feature.

## Agent workflow

1. Identify the repository, compiler backend, target profile, and known-good source revision. Look for actual `.atto` fixtures and tests.
2. Prefer the local compiler's tested syntax. `fnc name(T arg): R { ... }` is a safer first choice than target-only `:: ... end` free-function bodies.
3. Find build and test commands in the actual repo; do **not** invent global `atto build` or `atto run` options. The `atto-cli` repo describes a desired CLI, not universal executable coverage.
4. For playground work, use the documented `npm run verify` in [atto-playground](https://github.com/atto-lang/atto-playground) and check which subset the vendored Rust-to-Wasm compiler accepts.
5. Compile **and execute** the produced guest when possible. Report compiler revision, invocation, exit code/output, and whether checks were actually run.

## Compatibility and scope

- `val` is immutable; `let` is mutable; `const` requires a compile-time constant. Never use `null` to represent optional data.
- `: T` and `-> T` are accepted return *targets*, but support differs by backend and body style.
- The [canonical language specification](https://github.com/atto-lang/atto-spec) includes versioned normative contracts and separately marked TARGET proposals. It is not a certificate that a feature executes.
- Use the [backend/conformance skill](../atto-backends-and-testing/SKILL.md) before asserting support or parity.
