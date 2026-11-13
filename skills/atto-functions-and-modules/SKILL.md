---
name: atto-functions-and-modules
description: Implement AttoLang functions, class methods, generics, namespaces and package imports. Use when resolving calls, writing APIs, refactoring modules or debugging name lookup.
---

# Functions, methods, modules, name lookup

## Free functions and methods

~~~atto
fnc add(int a, int b): int {
  return a + b;
}

define class Account as
  int id: 0

  pub balance_after(int deposit): int {
    return deposit;
  }
end
~~~

Free functions use `fnc`. Instance methods ordinarily omit `fnc` and have a receiver (`self` where required). Static methods use `static fnc`. For class constructor syntax, see the [playground class example](https://github.com/atto-lang/atto-playground/blob/main/examples/classes.atto). The exact `self`-omission rules and separately declared `pub mtd (Type receiver) name(...)` forms are evolving; use real compiler tests, not guessed desugaring.

## Call resolution checklist

1. Determine the current namespace, imported names, lexical bindings, receiver type, and method visibility.
2. Resolve the callable by nominal owner/name/arity first; reject ambiguous overloads rather than guessing structural shape.
3. If using named value arguments, write `argument_name: expression`, **not** `argument_name = expression`. Enforce no duplicates, no unknown labels, no positional after named arguments, and single lexical-order evaluation.
4. Named generic arguments use `T=Type` only for formals declared by the resolved type/function. Do not confuse generic parameters with storage metadata such as `<rank=2>`.
5. An implicit method call may be treated like `self.method()` only after the receiver and method are unambiguous and authorization/visibility checks succeed.
6. A method extension declared on an actor or class in a different namespace must not silently attach to that type. Check the active `declare actor`/`mtd` design and actual compiler support before generating that form.

## Modules and imports — TARGET example

~~~atto
define pub module math as
  pub fnc add(int a, int b) -> int ::
    return a + b
  end
end

import module math from "+lib/math"
~~~

This is **illustrative target syntax**, not universally compiled. The `+` prefix identifies a governed project root with `.atto.pkg.toml` in canonical new projects. Resolver rules must fence symlinks and package boundaries. Do not silently fall back from missing `.atto` sources to `.ores`/`.java`; treat legacy imports as explicit migration modes. Check the compiler's current implementation and its package configuration before editing imports.

References: [functions/effects](https://github.com/ores-truffle-oreslang/attolang-language-spec.md/blob/main/spec/03-functions-and-effects.md) and [modules/AOT](https://github.com/ores-truffle-oreslang/attolang-language-spec.md/blob/main/spec/09-modules-and-aot.md).
