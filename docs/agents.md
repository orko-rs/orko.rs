# Agents

An `Agent` is a provider plus a system prompt, an optional model override and a set of tools. You make one with `create_agent`.

## The builder

```rust
use orko::{create_agent, providers::openai};

let agent = create_agent(openai("gpt-4o-mini"))
    .with_system_prompt("You are a terse assistant.") // prepended to every conversation
    .with_model("gpt-4o")                             // overrides the provider's default model
    .with_tool(get_weather())                         // one tool
    .with_tools([search(), calculator()])             // several tools
    .with_max_tool_turns(4)                           // cap on provider round-trips (default 8)
    .build()?;
```

`create_agent` takes any type that implements `Provider`. The builder is `Clone`, so you can set up a base agent once and branch it into variants.

`build()` fails with `Error::Config` if two tools have the same name.

## Invoking

`invoke` runs the full tool-calling loop and returns the final text:

```rust
let reply: String = agent.invoke("Summarise the Rust 2024 edition.").await?;
```

Each turn it:

1. sends the conversation and the tool specs to the provider,
2. streams the response and puts the tool-call fragments back together,
3. if the model asked for tools, runs each one, adds the results to the conversation and goes back to step 1,
4. otherwise returns the text. If the model only sent a refusal, the refusal text is returned.

If the model is still asking for tools after `max_tool_turns` round-trips, `invoke` returns `Error::ToolCallDepth`.

Problems with a tool are **not** errors from `invoke`. An unknown tool name, arguments that don't parse, or a tool that returns `Err` all go back to the model as the tool result (`error: ...`), so the model can fix the call on its next turn.

## Conversations

`invoke` takes anything that converts into a `Prompt`: a `&str`, a `String`, a single `Message` or a `Vec<Message>`.

```rust
use orko::Message;

let history = vec![
    Message::user("My name is Ada."),
    Message::assistant("Nice to meet you, Ada."),
    Message::user("What's my name?"),
];
let reply = agent.invoke(history).await?;
```

The agent doesn't keep memory between calls. To hold a conversation, keep the `Vec<Message>` yourself and push each reply onto it:

```rust
let mut history = vec![Message::user("Pick a number between 1 and 10.")];
let reply = agent.invoke(history.clone()).await?;
history.push(Message::assistant(reply));
history.push(Message::user("Double it."));
let reply = agent.invoke(history.clone()).await?;
```

The system prompt from the builder is always put in front. You don't need to add it to the history.

## Images and files

A message's content is either plain text or a list of parts. Pass a `Vec<ContentPart>` to send images or files:

```rust
use orko::{ContentPart, MediaSource, Message};

let msg = Message::user(vec![
    ContentPart::text("What is in this picture?"),
    ContentPart::image_url("https://example.com/orca.jpg"),
]);
let reply = agent.invoke(msg).await?;

// Inline file, base64-encoded. Chat Completions only takes files inline.
let pdf = Message::user(vec![
    ContentPart::text("Summarise this report."),
    ContentPart::File {
        source: MediaSource::base64("application/pdf", base64_data),
        name: Some("report.pdf".into()),
    },
]);
```

Whether the model can read images or files depends on the model you choose.

## Streaming

`invoke_stream` returns the raw stream of `CompletionChunk`s for a single turn. It does **not** run tools. Use it when you want tokens as they arrive. You need `futures` for `StreamExt`:

```toml
futures = "0.3"
```

```rust
use futures::StreamExt;
use std::io::Write;

let mut stream = agent.invoke_stream("Write a haiku about orcas.").await?;
while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    if let Some(thinking) = &chunk.reasoning {
        eprint!("{thinking}"); // e.g. deepseek-reasoner's reasoning
    }
    print!("{}", chunk.content);
    std::io::stdout().flush().ok();

    if let Some(usage) = chunk.usage {
        eprintln!("\n[{} prompt / {} completion tokens]",
            usage.prompt_tokens, usage.completion_tokens);
    }
}
```

The fields of a `CompletionChunk`:

| Field | Meaning |
|---|---|
| `content` | text delta (may be empty) |
| `reasoning` | reasoning/thinking delta, for models that stream it |
| `refusal` | refusal text delta |
| `tool_calls` | `ToolCallDelta` fragments, grouped by `index` |
| `finish_reason` | on the last chunk: `Stop`, `Length`, `ToolCalls` or `ContentFilter` |
| `usage` | on the last chunk, if the provider reports token counts |

## Errors

Every public API returns `orko::Result<T>`, which uses a single `orko::Error` enum:

| Variant | When |
|---|---|
| `Provider` | HTTP failure, non-2xx status (e.g. `401` from a missing key), bad SSE |
| `Serialization` | JSON encode/decode failure |
| `ToolExecution` | a tool failed (inside `invoke` it is sent to the model instead) |
| `Config` | duplicate tool name, malformed `"provider:model"` spec |
| `ToolCallDepth` | the tool loop hit `max_tool_turns` |
| `Other` | anything else |

The enum is `#[non_exhaustive]`, so keep a `_` arm when you match on it.
