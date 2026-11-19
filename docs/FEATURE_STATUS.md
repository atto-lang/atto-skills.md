# AttoLang feature-status and source-grounding policy

**Documented baseline: 2026-11-12. This page is a navigation policy, not a live support matrix.**

## Evidence labels

- **EXAMPLE**: checked-in sample from a named repo; its support is bounded to that repo/revision.
- **TARGET**: specified or requested semantics; not assumed compiler-ready.
- **PROPOSED**: spelling/behavior still in design or contradictory contracts.
- **IMPLEMENTED (backend + SHA)**: actual compiler + runtime guest execution and diagnostics at exact revisions.
- **VERIFIED PARITY**: identical source and semantics independently executed for every backend claimed.

For dates, revisions and actual status, search the target compiler repo's current tests, PRs and workflows; this file must not be treated as a freshness guarantee.

## Useful grounding

| Area | Source | Evidence caveat |
| --- | --- | --- |
| Intro scalar code, basic functions/conditions | [atto-playground/examples](https://github.com/atto-lang/atto-playground/tree/main/examples) | Limited Rust compiler-to-Wasm subset; inspect exact fixture |
| Type syntax / lexical grammar | [current language spec](https://github.com/atto-lang/atto-spec) | Revision-scoped grammar, many target-only forms |
| Sealed actor state | [actor semantics spec](https://github.com/atto-lang/atto-spec) | TARGET, not automatically proven by legacy playground actor |
| Tasks / select | [task and scheduling spec](https://github.com/atto-lang/atto-spec) | TARGET syntax; actual lowering may be absent |
| Supervisor and actor pipelines | [actor supervision contract](https://github.com/atto-lang/atto-spec) | Draft factories, hierarchical control and graph builder |
| Ownership and permission admission | [ownership and admission contract](https://github.com/atto-lang/atto-spec) | Needs static negative tests and runtime enforcement |
| Historical Java/LLVM comparison | [executable conformance contract](https://github.com/atto-lang/atto-platform-conformance) | Explicitly *not* conformance certification |
| Full backend parity | [conformance suite](https://github.com/atto-lang/atto-platform-conformance) | Requires same-source executed guest, exact-tree receipts |

## Never silently reconcile conflicting syntax

Examples of differences that should trigger verification rather than guesswork:
- `define actor` and the newer separate `declare actor` / namespace `mtd` shape;
- older direct `self.field` actor access vs sealed-state `handle_state(critical-read/critical-write ...)` target;
- `trap` returning an `Option` from ordinary thrown failures vs a separate `Result`-shaped request for sends;
- functional `|>` vs actor `pipe`;
- Java/Graal, native LLVM, browser Wasm and strict AOT subsets.

When uncertain, show a labeled **TARGET / illustrative** snippet and ask the target compiler to prove or reject it. Never label it `works` without executable evidence.
