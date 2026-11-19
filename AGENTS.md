# Instructions for AI coding agents

This repository is documentation for writing **AttoLang**, not the AttoLang compiler. Read this file before editing a skill or generating code from one.

1. Pick the smallest matching `skills/*/SKILL.md`; prefer progressive disclosure over loading every skill.
2. Treat [the canonical language spec](https://github.com/atto-lang/atto-spec) as the current normative AttoLang contract, **not** runtime proof. Check each versioned proposal's status, compiler parser/tests and actually executed `.atto` programs.
3. Always distinguish `TARGET` / `PROPOSED` examples from source that compiled **and ran** on the exact compiler revision. A playground fixture may be supported only by its restricted backend.
4. Use `snake_case` function/method names, typed bindings, `Option`/`Result` instead of null, actors as specialized classes, ownership checks, bounded mailboxes and explicit supervisor CONTROL semantics.
5. Never turn a rejected program into misleading success by changing effect behavior, bypassing borrows, weakening actor-state confinement, dropping a pending test, or silently invoking a host interpreter on full AOT.
6. For namespace-based `declare actor` / `mtd`, sealed actor-state `handle_state`, actor `pipe`, supervisor syntax, generators, instrumentation scopes and new `using` constructs, check implementation status before writing code as runnable.
7. Security matters: do not reveal source from private repositories in public docs; do not export private code to another org just to use its CI minutes without explicit authorization.
8. On any edit, compile and run `scripts/validate_skills.rs` with `rustc --edition=2021`, and run its Rust unit tests using `rustc --edition=2021 --test`. Include only tests/evidence actually executed; no fabricated CI claim.

The status matrix and evidence guidance live in [docs/FEATURE_STATUS.md](docs/FEATURE_STATUS.md).
