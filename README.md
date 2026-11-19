> **AI agent instructions (organization-wide):** Before working on code, issues, or pull requests, read and follow the [organization `AGENTS.md`](https://github.com/atto-lang/.github/blob/main/AGENTS.md), including its **merge-first stealth-development policy**. Honor any repository-local `AGENTS.md`, explicit holds, and GitHub-enforced rules.

# AttoLang AI Skills

Task-focused Markdown skills for AI agents writing, reviewing, testing and migrating **AttoLang** (`.atto`), an actor-first systems language. The project follows the [Agent Skills specification](https://agentskills.io/specification): each reusable skill lives at `skills/<name>/SKILL.md` with a discoverable YAML `name` and `description`.

## Canonical naming and specification

[atto-skills.md](https://github.com/atto-lang/atto-skills.md) is the maintained AttoLang repository. Use `.atto` source, `atto-*` repository names, and [atto-spec](https://github.com/atto-lang/atto-spec) for current language rules (ownership, borrowing, effects, actors, and syntax). Distinguish proposed and parser-only features from independently executed compiler support in [atto-platform-conformance](https://github.com/atto-lang/atto-platform-conformance). Naming migration does not by itself rename implementation artifacts, guarantee backend compatibility or certify guest execution.

## Skills catalog

| Skill | Use it for |
| --- | --- |
| [atto-getting-started](skills/atto-getting-started/SKILL.md) | Start with a runnable-style .atto example and choose a compiler |
| [atto-syntax-and-types](skills/atto-syntax-and-types/SKILL.md) | Bindings, classes, conditions, types and matching |
| [atto-functions-and-modules](skills/atto-functions-and-modules/SKILL.md) | Functions, methods, name lookup, generics and imports |
| [atto-actors-and-state](skills/atto-actors-and-state/SKILL.md) | Actors, spawn/send, sealed state and mailbox safety |
| [atto-supervision](skills/atto-supervision/SKILL.md) | Supervisors, failures, lifecycle and CONTROL |
| [atto-tasks-and-channels](skills/atto-tasks-and-channels/SKILL.md) | Tasks, await, select, channels, backpressure |
| [atto-ownership-and-memory](skills/atto-ownership-and-memory/SKILL.md) | Moves, borrows, sharing, arenas and transfer |
| [atto-errors-and-results](skills/atto-errors-and-results/SKILL.md) | Option, Result, throw, raise, panic and trap |
| [atto-pipelines](skills/atto-pipelines/SKILL.md) | Functional \|> and actor pipe |
| [atto-patterns-and-generators](skills/atto-patterns-and-generators/SKILL.md) | Match/switch, type patterns, yield and iterators |
| [atto-telemetry-and-io](skills/atto-telemetry-and-io/SKILL.md) | Stdio, nonblocking I/O, tracing and instrumentation |
| [atto-backends-and-testing](skills/atto-backends-and-testing/SKILL.md) | Compiler profiles, test evidence and backend parity |

## How to use

1. In an agent compatible with Agent Skills, install or copy the relevant `skills/<name>` directory into the client-supported skills location. Different clients use different discovery paths: follow that client's documentation. You can also give an agent the URL of a specific `SKILL.md`.
2. Start with [atto-getting-started](skills/atto-getting-started/SKILL.md), then load only the skills related to the task.
3. Consult the [AI authoring guidance](AGENTS.md) and [status/grounding policy](docs/FEATURE_STATUS.md). This repository **does not** supply a compiler, runtime, backend certification, or cross-backend conformance evidence.
4. Validate skill metadata and local links with a reviewed Rust-based validator once available. The checked-in validation/CI path still needs its Python-to-Rust migration; do not treat the existing script as a supported new development dependency or report Rust validation before it has executed.

## Provenance and maturity

**DRAFT / TARGET is not IMPLEMENTED.** Some examples are from checked-in playground fixtures; others are explicitly marked as target/proposed syntax. A parser accepting tokens does not certify type checking, runtime lowering, safety, or portable execution.

Authoritative places to consult:
- [Canonical AttoLang specification](https://github.com/atto-lang/atto-spec) (versioned normative contracts; check each proposal's implementation status).
- [AttoLang playground](https://github.com/atto-lang/atto-playground) (actual narrow WebAssembly compiler and example fixtures).
- [Java/Graal compiler](https://github.com/atto-lang/atto-source.java) and [LLVM compiler](https://github.com/atto-lang/atto-source.llvm) (check the particular repository and current implementation).
- [Agent Skills format](https://agentskills.io/specification).

## Contributing

Keep skills short and task-specific. Put actionable agent workflows ahead of design essays. Mark every proposed or backend-dependent code fragment, link to an authoritative spec/fixture, add negative tests or review cases where appropriate, and **do not claim feature support without exact-head guest execution**.

See [AGENTS.md](AGENTS.md) and [docs/FEATURE_STATUS.md](docs/FEATURE_STATUS.md).
