# Getting started

Orko is an agent orchestration toolkit for Rust. You give it a provider (OpenAI, DeepSeek, or your own) and some tools. Orko runs the agent loop: it streams the completion, calls the tools the model asks for, sends the results back, and returns the final answer.

> Orko is pre-1.0 (`0.1.x`). Expect breaking changes between minor versions.

## Requirements

- Rust **1.82** or newer
- An async runtime. The built-in HTTP transport runs on [Tokio](https://tokio.rs).
- An API key for a provider: `OPENAI_API_KEY` or `DEEPSEEK_API_KEY`

## Install

Orko isn't on crates.io yet, so add it from git. Put this in your `Cargo.toml`:

```toml
[dependencies]
orko = { git = "https://github.com/orko-rs/orko", features = ["macros"] }
orko-core = { git = "https://github.com/orko-rs/orko" } # needed by #[tool]
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

`orko` is the facade crate. It re-exports everything from `orko-core` at its root, and the other crates sit behind cargo features:

| Feature | What it adds | Default |
|---|---|---|
| `providers` | `orko::providers`: 17 presets (`openai`, `deepseek`, `gemini`, `ollama` and more), `from_str`, `OpenAiCompatible` | yes |
| `macros` | the `#[tool]` attribute macro | no |
| `graph`, `mcp`, `runtime` | placeholders, still empty | no |
| `full` | all of the above | no |

The `#[tool]` macro expands to paths under `::orko_core`, so `orko-core` has to be a direct dependency when you use it. If you don't use `#[tool]`, `orko` is enough.

## Your first agent

```rust
use orko::{create_agent, providers::openai, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Reads the key from OPENAI_API_KEY.
    let agent = create_agent(openai("gpt-4o-mini"))
        .with_system_prompt("You are a helpful assistant. Be brief.")
        .build()?;

    let reply = agent.invoke("What is Rust, in one sentence?").await?;
    println!("{reply}");
    Ok(())
}
```

Run it:

```sh
export OPENAI_API_KEY=sk-...
cargo run
```

To use DeepSeek instead, swap the provider. Nothing else changes:

```rust
use orko::providers::deepseek;

// Reads the key from DEEPSEEK_API_KEY.
let agent = create_agent(deepseek("deepseek-chat")).build()?;
```

## Give it a tool

```rust
use orko::{create_agent, providers::openai, tool, Result};

/// Get the current weather for a city.
#[tool]
fn get_weather(city: String) -> String {
    format!("It is sunny and 22°C in {city}.")
}

#[tokio::main]
async fn main() -> Result<()> {
    let agent = create_agent(openai("gpt-4o-mini"))
        .with_tool(get_weather())
        .build()?;

    println!("{}", agent.invoke("Should I bring an umbrella in Lisbon?").await?);
    Ok(())
}
```

The model sees `get_weather`, calls it with `{"city": "Lisbon"}`, and answers using the result. See [Tools](/docs/tools) for everything `#[tool]` can do.

## Try the examples

The repository has two examples. They work without an API key: if the key isn't set, they swap in a fake transport that returns a fixed reply.

```sh
git clone https://github.com/orko-rs/orko
cd orko
cargo run --example hello_openai
cargo run --example hello_deepseek
```

## How it's put together

| Crate | Role | Status |
|---|---|---|
| `orko-core` | `Provider`, `Agent`, `Tool`, messages, streaming chunks | alpha |
| `orko-macros` | `#[tool]` | review |
| `orko-providers` | OpenAI-compatible engine and 17 presets | review |
| `orko-graph` | graph-based multi-agent workflows | scaffolded |
| `orko-runtime` | Tokio runtime integration | scaffolded |
| `orko-mcp` | Model Context Protocol | scaffolded |

Every crate depends only on `orko-core`. `orko-core` has four dependencies (`serde`, `serde_json`, `thiserror`, `futures`), no HTTP and no runtime. Public traits use `std::future::Future` and `futures::Stream`, so nothing in the public API ties you to Tokio.

## Next

- [Agents](/docs/agents): the builder, conversations, images, streaming and errors
- [Tools](/docs/tools): `#[tool]` and the `Tool` trait
- [Providers](/docs/providers): presets, custom endpoints, transports and your own `Provider`
