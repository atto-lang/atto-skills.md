---
name: atto-backends-and-testing
description: Validate AttoLang code across compiler backends and CI. Use for Java/GraalVM, LLVM native AOT, Wasm, Forge, .NET AOT, Rust compiler subsets, language conformance and release claims.
---

# Backend selection, tests, and evidence

## Distinguish intended grammar from execution

The [canonical language spec](https://github.com/atto-lang/atto-spec) includes normative contracts and TARGET proposals. AttoLang spans Java/Graal, LLVM, Wasm, Forge, .NET Native AOT and additional backend efforts. An implementation repository README is not enough to prove that source compiles or runs. The browser [atto-playground](https://github.com/atto-lang/atto-playground) uses a separate Rust-to-Wasm compiler with a deliberately restricted subset.

## Agent verification workflow

1. Identify **exact repository, branch, source SHA, compiler SHA, target profile, and fixture** before claiming correctness.
2. Inspect a **real executable example** or grammar test in that backend; adapt syntax only if compiler actually accepts it.
3. Run parser + formatter round-trip, static positive and negative tests, and **guest compile-and-execute**.
4. For actor tasks/channels, test mailbox capacity, cancellation, supervision priority, ownership transfers, cross-actor state confinement and resource retirement under concurrency.
5. For native code, run memory/sanitizer and AOT tests; a compiler build is not a guest execution. For full AOT, prove guest application code has no hidden interpreter/JIT fallback.
6. Repeat the **same source** on each claimed backend. Record stdout/stderr, typed output, exit status, failure class, compiler version and actual executed steps.
7. Mark each backend `passed`, `failed`, `unsupported` or `not executed`. **Never** interpret `pending`, `skipped`, empty CI jobs, draft PRs or model-only tests as passing evidence.
8. Never mirror private compiler trees to another org merely because that org has Actions minutes. Source export needs independent authorization and verified exact tree/blob identities.

## Minimal report template

| Item | Value |
| --- | --- |
| Source file / blob | exact `path.atto` + blob SHA |
| Backend / compiler | repository + immutable SHA |
| Invocation | actual command and target/profile |
| Parser and type checks | pass/fail with diagnostics |
| Executed guest | yes/no; observable output and exit |
| Negative tests | exact rejecting fixtures |
| CI evidence | workflow, job/run IDs, stepful logs |
| Status | passed/failed/unsupported/not executed |

## Useful baselines

- [atto-playground README and `npm run verify`](https://github.com/atto-lang/atto-playground)
- [conformance registry](https://github.com/atto-lang/atto-platform-conformance)
- [AttoLang platform conformance](https://github.com/atto-lang/atto-platform-conformance) — check exact backend/source receipts, *not* blanket certification
