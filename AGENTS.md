<!-- org-merge-review-policy:begin -->
> **Mandatory agent policy:** Read [atto-lang/.github/AGENTS.md](https://github.com/atto-lang/.github/blob/main/AGENTS.md) before editing, reviewing, or merging. **0–72h:** Codex/ChatGPT + Claude/Opus + `ORESoftware` + `the1mills` reviews. **72h–<240h:** one verified independent human/AI approval permits merging with passing tests and no conflicts. **240h+:** fresh post-240h reviews from all four again. Every phase requires green verified CI, current-head evidence, no holds/critical risks, and compliance with GitHub rules. Merge eligible PRs immediately; never reset age or bypass protections. More restrictive local safety rules remain effective.
<!-- org-merge-review-policy:end -->

# Instructions for AI coding agents

This repository is documentation for writing **AttoLang**, not the AttoLang compiler. Read this file before editing a skill or generating code from one.

1. Pick the smallest matching `skills/*/SKILL.md`; prefer progressive disclosure over loading every skill.
2. Treat [the draft language spec](https://github.com/ores-truffle-oreslang/attolang-language-spec.md) as intended semantics, **not** runtime proof. Check compiler parser/tests and actual executable `.atto` examples first.
3. Always distinguish `TARGET` / `PROPOSED` examples from source that compiled **and ran** on the exact compiler revision. A playground fixture may be supported only by its restricted backend.
4. Use `snake_case` function/method names, typed bindings, `Option`/`Result` instead of null, actors as specialized classes, ownership checks, bounded mailboxes and explicit supervisor CONTROL semantics.
5. Never turn a rejected program into misleading success by changing effect behavior, bypassing borrows, weakening actor-state confinement, dropping a pending test, or silently invoking a host interpreter on full AOT.
6. For namespace-based `declare actor` / `mtd`, sealed actor-state `handle_state`, actor `pipe`, supervisor syntax, generators, instrumentation scopes and new `using` constructs, check implementation status before writing code as runnable.
7. Security matters: do not reveal source from private repositories in public docs; do not export private code to another org just to use its CI minutes without explicit authorization.
8. On any edit, run `python3 scripts/validate_skills.py` and include the tests/evidence you actually executed; no fabricated CI claim.

The status matrix and evidence guidance live in [docs/FEATURE_STATUS.md](docs/FEATURE_STATUS.md).
