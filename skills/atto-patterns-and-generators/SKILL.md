---
name: atto-patterns-and-generators
description: Write AttoLang match and switch patterns, generators, async generators and iterators. Use when creating branching code, exhaustiveness tests, yield, async streams and type/value matching.
---

# Matching, generators, and iterators

## Matching from an existing example

~~~atto
fnc fizz(int n): string {
  match n over
    on 15 -> { return "FizzBuzz"; }
    on 3 -> { return "Fizz"; }
    on 5 -> { return "Buzz"; }
    on _ -> { return "-"; }
  end
}
~~~

This shape appears in [playground examples/match.atto](https://github.com/atto-lang/atto-playground/blob/main/examples/match.atto). A static match must preserve source arm order/guard behavior and diagnose missing cases where exhaustiveness is required. Avoid mixing `match`, `switch`, `select`, and Rust-like `match { ... }` syntax.

## Type and value patterns

- Value equality uses `eq` (or admitted `==` alias); `is` can denote identity or type tests depending on grammar context.
- `if animal is type Dog dog then ... fi` is documented as a type-test pattern, but validate the binder semantics on the actual compiler.
- `match<int>`, ADT destructuring, enum methods, typed switch arms, and duplicate-arm diagnostics require backend-specific acceptance tests.
- No implicit truthiness: a match guard or `if` condition must be boolean under the language's static rules.

## Generators and async generators

The grammar describes `generator` / `yield` and `async` / `await`, but exact syntax, return types and lowering vary by backend. **Do not invent runnable `generator fnc`, `generator*`, or `async generator` signatures** without finding a compiled source fixture in the selected repo.

When implementing a generator or iterator:
1. Establish producer type, yielded element type, ownership of each element and terminal result.
2. Test resume/yield/exhaustion, cancellation, panics and error propagation.
3. Ensure borrows do not outlive suspended frames or escape actor-state critical sections.
4. Check downstream channel backpressure and exactly-once cleanup on early break.
5. Add positive compile+execute cases and negative type/lifetime cases for every backend being claimed.

See [lexical grammar](https://github.com/ores-truffle-oreslang/attolang-language-spec.md/blob/main/spec/16-lexical-and-grammar-reference.md) and [async scheduling](https://github.com/ores-truffle-oreslang/attolang-language-spec.md/blob/main/spec/06-async-and-scheduling.md).
