use dioxus::prelude::*;

const LATEST_RELEASE_API: &str = "https://api.github.com/repos/orko-rs/orko/releases/latest";

const FALLBACK: &str = "v0.1.0 (coming soon)";

#[component]
pub fn Version() -> Element {
    let latest = use_resource(|| async {
        reqwest::get(LATEST_RELEASE_API)
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await
    });

    let version = match &*latest.read() {
        Some(Ok(r)) => match r["tag_name"].as_str() {
            Some(t) if t.starts_with('v') => t.to_string(),
            Some(t) => format!("v{t}"),
            None => FALLBACK.to_string(),
        },
        _ => FALLBACK.to_string(),
    };

    rsx! {
        span { class: "version", "{version}" }
    }
}
