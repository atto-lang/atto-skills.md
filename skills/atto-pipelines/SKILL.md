---
name: atto-pipelines
description: Implement AttoLang functional pipelines and actor-to-actor pipelines. Use for the |> operator, pipe keyword, Actor.pipe/Actor.connect, typed stages, bounded buffers and graph lifecycle.
---

# Function pipelines versus actor pipelines

They are different operations and must not be conflated.

## Functional `|>`

~~~atto
fnc increment(int x): int { return x + 1; }
val result = 41 |> increment
~~~

This is a value/function pipeline in the design, and has been exercised on a specific Java snapshot. Confirm the compiler profile before claiming it works in native LLVM/Wasm/other backends.

## Actor `pipe` (TARGET, not verified cross-backend)

~~~atto
spawn Reader() pipe spawn Parser() pipe spawn Writer()
~~~

This connects actors through typed, bounded, backpressured DATA channels. It **does not** mean apply a function to a value, share actor heaps, create new supervisors, or bypass mailbox authority. Do not present this example as currently runnable without same-source guest proof.

The more explicit factory APIs `Actor.pipe(A(), B(), ...)`, deferred graph builder `.start()`, and `Actor.connect(existing_a, existing_b)` remain TARGET / NOT_VERIFIED. They differ in whether new actors are created and who owns them. Existing-actor connections must not steal or reparent a supervisor.

## Agent design checklist

1. Identify stage input/output message types, ownership transport (take/copy/frozen share), capacity, and backpressure.
2. Authenticate actor identity/incarnation and graph admission before routing DATA.
3. Keep privileged supervisor CONTROL independent of DATA piping; a supervisor is not a DATA stage.
4. Specify startup, partial rollback, cancellation, stale generations, and exactly-once resource retirement.
5. If a nested pipeline has a supervisor scope, bind its children to that scope explicitly; don't infer reparenting from syntax.
6. Test full/closed pipes, mismatched types, stale-generation replays, shared-memory races, restart and cancellation on the selected backend.

See [pipeline factories](https://github.com/ores-truffle-oreslang/attolang-language-spec.md/blob/main/spec/14-actor-pipeline-factories.md) and [nested builders](https://github.com/ores-truffle-oreslang/attolang-language-spec.md/blob/main/spec/15-nested-actor-pipelines.md).
