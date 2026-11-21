# Providers

A provider turns a list of messages into a stream of completion chunks. That is the whole `Provider` trait. Agents don't know about HTTP, so an HTTP client for a hosted API and an in-process model can implement the same trait.

The `providers` feature (on by default) gives you `orko::providers`, an engine for any API that speaks the **OpenAI Chat Completions** format, plus presets built on it.

## Presets

Each preset is a function that takes a model id and returns an `OpenAiCompatible`. There are 17 of them.

### Hosted

| Function | Base URL | Key read from |
|---|---|---|
| `openai` | `https://api.openai.com/v1` | `OPENAI_API_KEY` |
| `deepseek` | `https://api.deepseek.com/v1` | `DEEPSEEK_API_KEY` |
| `gemini` | `https://generativelanguage.googleapis.com/v1beta/openai` | `GEMINI_API_KEY` |
| `groq` | `https://api.groq.com/openai/v1` | `GROQ_API_KEY` |
| `together` | `https://api.together.xyz/v1` | `TOGETHER_API_KEY` |
| `openrouter` | `https://openrouter.ai/api/v1` | `OPENROUTER_API_KEY` |
| `moonshot` | `https://api.moonshot.ai/v1` | `MOONSHOT_API_KEY` |
| `qwen` | `https://dashscope-intl.aliyuncs.com/compatible-mode/v1` | `DASHSCOPE_API_KEY` |
| `mistral` | `https://api.mistral.ai/v1` | `MISTRAL_API_KEY` |
| `xai` | `https://api.x.ai/v1` | `XAI_API_KEY` |
| `perplexity` | `https://api.perplexity.ai` | `PERPLEXITY_API_KEY` |
| `nvidia` | `https://integrate.api.nvidia.com/v1` | `NVIDIA_API_KEY` |
| `zai` | `https://api.z.ai/api/paas/v4` | `ZAI_API_KEY` |
| `huggingface` | `https://router.huggingface.co/v1` | `HF_TOKEN` |

### Local

| Function | Base URL | Key read from |
|---|---|---|
| `ollama` | `http://localhost:11434/v1` | none |
| `vllm` | `http://localhost:8000/v1` | `VLLM_API_KEY` (optional) |
| `llamacpp` | `http://localhost:8080/v1` | `LLAMACPP_API_KEY` (optional) |

The vLLM and llama.cpp keys are only needed if the server was started with `--api-key`.

```rust
use orko::providers::{deepseek, gemini, ollama, openai, openrouter};

let fast = openai("gpt-4o-mini");
let thinker = deepseek("deepseek-reasoner");
let google = gemini("gemini-2.5-flash");
let routed = openrouter("openai/gpt-4o-mini"); // OpenRouter ids include the vendor
let local = ollama("llama3.2");
```

Each base URL is also exported as a constant (`OPENAI_BASE_URL`, `GROQ_BASE_URL`, `OLLAMA_BASE_URL` and so on).

A few presets set [quirks](#quirks) for you. `openai` sends `max_completion_tokens` instead of `max_tokens`. `mistral` and `perplexity` don't send `stream_options`, and usage still arrives in the last chunk.

`qwen` uses DashScope's international endpoint. For the mainland China one, use `OpenAiCompatible::new("https://dashscope.aliyuncs.com/compatible-mode/v1", model)`. The same goes for a self-hosted NVIDIA NIM or a local server on a non-default host or port.

### API keys

Hosted presets read their key from the environment when the preset is created. If it isn't set, requests go out without auth and the server's `401` comes back as `Error::Provider`. To pass the key yourself:

```rust
let p = openai("gpt-4o-mini").with_api_key(my_secret);
```

`deepseek-reasoner` streams its thinking in `CompletionChunk::reasoning`. `invoke` returns only the final answer. Use `invoke_stream` if you want to show the reasoning (see the streaming section in [Agents](/docs/agents)).

Anthropic, Azure OpenAI, AWS Bedrock and in-process Candle models are planned. Their status is tracked on [GitHub](https://github.com/orko-rs/orko/labels/crate%3A%20providers).

## Choosing a provider at run time

`from_str` takes a `"provider:model"` string and returns a type-erased `BoxProvider`:

```rust
use orko::{create_agent, providers};

let spec = std::env::var("ORKO_MODEL").unwrap_or("openai:gpt-4o-mini".into());
let agent = create_agent(providers::from_str(&spec)?).build()?;
```

The provider name is the preset function's name, so all 17 presets work. Everything after the first `:` is the model, so `"ollama:llama3.2:3b"` is fine. A malformed spec or an unknown provider returns `Error::Config`.

## Any OpenAI-compatible endpoint

The presets are just `OpenAiCompatible` with a base URL and a key. You can point it at any server that speaks Chat Completions and has no preset: a model server on another machine, a gateway, or a proxy.

```rust
use orko::providers::OpenAiCompatible;

// Ollama on another machine, no auth.
let remote = OpenAiCompatible::new("http://gpu-box.local:11434/v1", "llama3.2");

// A gateway with its own key variable.
let gateway = OpenAiCompatible::new("https://llm.internal.example.com/v1", "my-model")
    .with_api_key_env("GATEWAY_API_KEY");
```

| Method | What it does |
|---|---|
| `new(base_url, model)` | base URL without `/chat/completions`, plus the default model |
| `with_api_key(key)` | send `Authorization: Bearer <key>` |
| `with_api_key_env(var)` | read the key from an environment variable, if set |
| `with_quirks(quirks)` | adjust for servers that differ from OpenAI |
| `with_transport(transport)` | send requests through your own HTTP layer |

### Quirks

Some compatible servers reject fields OpenAI accepts. Start from the defaults and change what you need:

```rust
use orko::providers::{OpenAiCompatible, Quirks};

let mut quirks = Quirks::default();
quirks.stream_usage = false;                       // don't send stream_options.include_usage
quirks.max_tokens_field = "max_completion_tokens"; // or "max_tokens" (default)

let p = OpenAiCompatible::new(base_url, model).with_quirks(quirks);
```

Response fields that only differ by name are handled for you. Both `reasoning_content` (DeepSeek) and `reasoning` (OpenRouter) end up in `CompletionChunk::reasoning`, and unknown fields are ignored.

## Transports

`OpenAiCompatible` builds the request JSON and parses the SSE stream (the `codec` module), and hands the actual HTTP to a `Transport`.

The default, `TokioTransport` (feature `transport-tokio`, on by default), uses hyper 1.x with rustls and the platform's root certificates. It must run inside a Tokio runtime. Requests time out after 60 seconds:

```rust
use orko::providers::TokioTransport;
use std::{sync::Arc, time::Duration};

let transport = TokioTransport::new()?.with_timeout(Duration::from_secs(120));
let p = openai("gpt-4o").with_transport(Arc::new(transport));
```

### Your own transport

Implement `Transport` to use another runtime or HTTP client, or to add a proxy, custom TLS or connection pooling. It is one method: POST these bytes with these headers and return the status and a stream of the body.

```rust
use orko::providers::{HttpResponse, Transport};
use orko::Result;
use std::{future::Future, pin::Pin};

struct MyTransport { /* your client */ }

impl Transport for MyTransport {
    fn post<'a>(
        &'a self,
        url: &'a str,
        headers: &'a [(String, String)],
        body: Vec<u8>,
    ) -> Pin<Box<dyn Future<Output = Result<HttpResponse>> + Send + 'a>> {
        Box::pin(async move {
            // send the request, then:
            // Ok(HttpResponse { status, body: Box::pin(byte_stream) })
            todo!()
        })
    }
}
```

Return non-2xx responses as normal responses. The engine turns them into `Error::Provider` with the body text. To drop the Tokio dependency completely, turn off default features:

```toml
orko = { git = "https://github.com/orko-rs/orko", default-features = false, features = ["providers"] }
```

### Testing without a network

A transport that returns a canned SSE body lets you test agents offline. The repository's examples do exactly this when no API key is set:

```rust
struct Canned(&'static str);

impl Transport for Canned {
    fn post<'a>(&'a self, _: &'a str, _: &'a [(String, String)], _: Vec<u8>)
        -> Pin<Box<dyn Future<Output = Result<HttpResponse>> + Send + 'a>>
    {
        let sse = format!(
            "data: {{\"choices\":[{{\"delta\":{{\"content\":\"{}\"}},\"finish_reason\":\"stop\"}}]}}\n\ndata: [DONE]\n\n",
            self.0
        );
        Box::pin(async move {
            Ok(HttpResponse {
                status: 200,
                body: Box::pin(futures::stream::iter([Ok(sse.into_bytes())])),
            })
        })
    }
}

let p = openai("gpt-4o-mini").with_transport(Arc::new(Canned("hi from the stub")));
```

## Writing your own `Provider`

If your backend doesn't speak Chat Completions, implement `Provider` directly. The rest of Orko doesn't depend on `orko-providers`.

```rust
use orko::{
    CompletionChunk, CompletionRequest, CompletionStream, ContentView, FinishReason, Provider, Result,
};
use std::future::Future;

/// Replies with the last user message, reversed.
struct Reverse;

impl Provider for Reverse {
    fn complete(&self, request: CompletionRequest) -> impl Future<Output = Result<CompletionStream>> + Send {
        async move {
            let text = match request.messages.last().map(|m| m.content.pull()) {
                Some(ContentView::Text(t)) => t.chars().rev().collect(),
                _ => String::new(),
            };
            let chunk = CompletionChunk {
                content: text,
                finish_reason: Some(FinishReason::Stop),
                ..Default::default()
            };
            let stream: CompletionStream = Box::pin(futures::stream::iter([Ok(chunk)]));
            Ok(stream)
        }
    }
}

let agent = create_agent(Reverse).build()?;
assert_eq!(agent.invoke("orko").await?, "okro");
```

`request.options` carries the model override, `temperature`, `max_tokens` and the tool specs for the turn. To ask for tools, emit `ToolCallDelta`s in `CompletionChunk::tool_calls`. The agent puts them back together by `index` and runs them.

### Generic or boxed

`Agent<P>` is generic over its provider, so the usual path has no allocation or dynamic dispatch. When you need to pick a provider at run time or keep different ones in a collection, erase the type:

```rust
use orko::{boxed, BoxProvider};

let providers: Vec<BoxProvider> = vec![boxed(openai("gpt-4o-mini")), boxed(Reverse)];
```

`BoxProvider` implements `Provider` too, so `create_agent(boxed(p))` works. `Arc<P>` also implements `Provider`, which makes it cheap to share one provider between several agents.
