---
name: atto-telemetry-and-io
description: Add AttoLang I/O, tracing and instrumentation safely. Use for stdio channels, network requests, using telemetry.span scopes, OpenTelemetry, hot instrumentation and nonblocking services.
---

# I/O and observability in AttoLang

## Stdio and asynchronous I/O

For simple programs, the [playground](https://github.com/atto-lang/atto-playground/blob/main/examples/hello.atto) demonstrates `stdio.println(...)`. The language **target** models `process.stdout`, `process.stderr`, `process.stdin` and `stdio.out`, `stdio.err`, `stdio.in` as channel-style streams. An adapter such as `stdio.in.stream()` or `Observable.from_ch(stdio.in)` needs implementation proof.

Hosted I/O should normally be asynchronous/nonparking. An explicit synchronous-style wrapper must use `_blocking` in its function name, yet suspend the logical task rather than parking an actor worker. `_async` is optional for genuine async methods.

## Tracing scopes (proposed construct)

~~~atto
using telemetry.span("http.request") as span {
  // Handler body; close scope exactly once on return/failure/cancel.
}
~~~

The `using telemetry.span(...) as span { ... }` construct is a **desired language/runtime feature**, not universal compiler syntax. Require a verified compiler-owned affine scope, nonescaping borrow, cleanup and observer behavior before generating it as production AttoLang.

## What instrumentation may safely do

- Compiler-inserted AOT probes can be disabled by default and bounded for low-overhead tracing.
- Runtime re-instrumentation of a running process depends on an explicitly supported JIT/tiering profile; **full closed-world AOT cannot silently accept arbitrary new guest code**.
- Preserve actor identity, trace context, deadlines, cancellation and supervisor CONTROL isolation.
- Keep traces separate from logs: no explicit log statement is required for compiler-inserted spans, but trace export only exists if instrumentation and exporter are enabled.
- Apply explicit permission checks and redact secrets/PII from telemetry attributes.

## Evidence gates

Demand an end-to-end guest HTTP handler trace, span nesting, cancellation unwinding, disabled-mode overhead measurements, and exact compiler/runtime revisions before claiming instrumentation is implemented.

References: [functions/effects](https://github.com/atto-lang/atto-spec), [async/I/O](https://github.com/atto-lang/atto-spec), and [AOT policy](https://github.com/atto-lang/atto-spec).
