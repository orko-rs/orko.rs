# Tools

A tool is something the model can call by name while it works out an answer. Orko sends the tool's name, description and JSON Schema to the provider. When the model calls it, the agent runs it and sends the result back.

You can write tools in two ways: put the `#[tool]` macro on a function, or implement the `Tool` trait yourself.

## `#[tool]`

Turn on the `macros` feature and add `orko-core` as a direct dependency (the macro's expansion refers to `::orko_core`):

```toml
orko = { git = "https://github.com/orko-rs/orko", features = ["macros"] }
orko-core = { git = "https://github.com/orko-rs/orko" }
```

Then mark a function:

```rust
use orko::tool;

/// Add two numbers.
#[tool]
fn add(a: f64, b: f64) -> f64 {
    a + b
}
```

`#[tool]` replaces the function with a constructor of the same name that returns `Arc<dyn Tool>`. Call it to get the tool:

```rust
let agent = create_agent(openai("gpt-4o-mini"))
    .with_tool(add())
    .build()?;
```

### Name and description

- The **name** is the function name (`add`).
- The **description** comes from `#[tool(desc = "...")]` if you give one. Otherwise it comes from the doc comment.

```rust
#[tool(desc = "Look up a user's email address by their id.")]
fn find_email(user_id: u64) -> String {
    format!("user{user_id}@example.com")
}
```

Write the description for the model. It decides when to call the tool based on this text.

### Parameter types

The JSON Schema is built from the parameter types at compile time:

| Rust type | JSON Schema |
|---|---|
| `String` | `"string"` |
| `i8` … `i128`, `u8` … `u128`, `isize`, `usize` | `"integer"` |
| `f32`, `f64` | `"number"` |
| `bool` | `"boolean"` |
| `Vec<T>` | `"array"` with `items` from `T` |
| `Option<T>` | schema of `T`, and the parameter is **not** in `required` |
| anything else | `{}` (open schema) |

Any type that implements `serde::Deserialize` works as a parameter. Types the macro doesn't know get the open schema `{}`, and serde still checks the value when the tool runs. If the model sends a wrong argument, the tool returns `Error::ToolExecution` and the agent passes that message back to the model.

```rust
/// Search the docs. `limit` is optional.
#[tool]
fn search_docs(query: String, tags: Vec<String>, limit: Option<u32>) -> String {
    let limit = limit.unwrap_or(5);
    format!("top {limit} results for {query:?} in {tags:?}")
}
```

### Return values

The return type has to implement `Display`. The tool's output is `value.to_string()`. To return structured data, return a JSON string:

```rust
/// Get stock info for a product.
#[tool]
fn stock(sku: String) -> String {
    serde_json::json!({ "sku": sku, "in_stock": 12 }).to_string()
}
```

### Async tools

`async fn` works the same way. The agent awaits it inside the tool-calling loop:

```rust
/// Fetch the title of a web page.
#[tool]
async fn page_title(url: String) -> String {
    match fetch(&url).await {
        Ok(html) => extract_title(&html),
        Err(e) => format!("could not fetch {url}: {e}"),
    }
}
```

Tool failures are best returned as text the model can read, like the example above. Panics are not caught.

### Limits

- No generic functions, and no `self` (free functions only).
- Parameters have to be plain identifiers (no patterns like `(a, b): (i32, i32)`).
- Only `desc = "..."` is accepted inside `#[tool(...)]`.

## Calling a tool directly

Tools are just values, so you can test them without a model:

```rust
use serde_json::json;

let t = add();
assert_eq!(t.name(), "add");

// In async code:
let out = t.call(json!({ "a": 2, "b": 3 })).await?;

// In sync code (tests, scripts). This blocks the current thread:
let out = t.call_blocking(json!({ "a": 2, "b": 3 }))?;
assert_eq!(out, "5");
```

Don't use `call_blocking` inside an async runtime. It blocks the thread until the tool finishes.

## Implementing `Tool` by hand

Write the trait yourself when the tool has state (a database pool, an HTTP client, config) or a schema the macro can't describe. You need `serde_json` as a dependency.

```rust
use orko::{Error, Result, Tool};
use serde_json::{json, Value};
use std::future::Future;
use std::pin::Pin;

struct Kv {
    store: std::collections::HashMap<String, String>,
}

impl Tool for Kv {
    fn name(&self) -> &str {
        "kv_get"
    }

    fn description(&self) -> &str {
        "Read a value from the key-value store."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": { "key": { "type": "string", "enum": ["region", "plan"] } },
            "required": ["key"]
        })
    }

    fn call<'a>(&'a self, args: Value) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let key = args["key"]
                .as_str()
                .ok_or_else(|| Error::ToolExecution("`key` must be a string".into()))?;
            Ok(self.store.get(key).cloned().unwrap_or_else(|| "not found".into()))
        })
    }
}
```

Register it with `Arc`:

```rust
use std::sync::Arc;

let agent = create_agent(openai("gpt-4o-mini"))
    .with_tool(Arc::new(Kv { store }))
    .build()?;
```

`call` returns a boxed future instead of using `async fn`, so the trait stays object-safe. That is what lets an agent hold tools of different types as `Arc<dyn Tool>`.

## Tool names must be unique

`build()` returns `Error::Config("duplicate tool name ...")` if two tools have the same name.
