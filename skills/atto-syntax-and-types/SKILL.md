---
name: atto-syntax-and-types
description: Author AttoLang source syntax and static types. Use for bindings, expressions, conditions, classes, symbols, ADTs, generics, pattern matching and diagnostics.
---

# AttoLang syntax and types

## Basic syntax

Use UTF-8 `.atto`, case-sensitive keywords, `//` comments, and `snake_case` names. Do not extrapolate Java-like declarations to AttoLang.

~~~atto
fnc classify(int n): int {
  if n < 0 then
    return -1;
  elif n eq 0 then
    return 0;
  else
    return 1;
  fi
}
~~~

This form comes from the [playground conditions fixture](https://github.com/atto-lang/atto-playground/blob/main/examples/conditions.atto). Conditions require `bool`; there is no arbitrary truthiness. In current grammar, `,` means AND and `|` means OR **inside conditions**. `eq`/`neq` compare values; `==`/`!=` denote reference identity on objects where admitted, and `is` is a type/identity test. Check backend admission. Do not confuse `=` assignment with equality.

## Bindings and nominal types

- `val x: int = 42;`: immutable binding.
- `let x: int = 42;`: mutable binding (confirm backend support).
- `const`: compile-time constant obligation.
- Ordinary class fields without a binding qualifier default to immutable `val` under [class-field defaults v2](https://github.com/atto-lang/atto-spec/blob/main/language/class-field-binding-defaults-v2.md); explicit `let` is required for writable state. Cross-backend guest execution still needs evidence.
- Classes are nominal; same field shape does not automatically imply assignability.
- A class is **not** an actor; an actor has lifecycle, inbox, and sealed-state requirements.
- Use `Option<T>`/`Result<T,E>` rather than inventing null values or unchecked exceptions.
- Generic type binding uses `T=int` inside `<...>`; named **value** arguments use `label: expr` and have separate resolution rules.
- Symbol literals use `#ready` or `#"with spaces"`, not `:ready` or unquoted `#with spaces`.

## Matching

~~~atto
fnc sign(int n): int {
  match n over
    on 0 -> { return 0; }
    on _ -> { return n > 0 ? 1 : -1; }
  end
}
~~~

Canonical `on`-arm forms and other matching variants depend on compiler revision. Guarded patterns, enums/ADTs, `is type` binding, `match<T>` and exhaustive semantics are language targets until tested in the selected backend. Never add an implicit default arm or silently discard duplicate/unreachable arms.

## Hard errors to preserve

Reject assignment as a condition (`if x = y then`), use-before-initialization, incompatible nominal types, mutation through `val`, unsupported syntactic sugar, ambiguous generics, and nonexhaustive match where exhaustion is required.

See [syntax](https://github.com/atto-lang/atto-spec), [types](https://github.com/atto-lang/atto-spec), and [observed grammar](https://github.com/atto-lang/atto-spec).
