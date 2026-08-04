use dioxus::prelude::*;

const REPO_API: &str = "https://api.github.com/repos/orko-rs/orko";

/// Live star count chip for the GitHub community card.
#[component]
pub fn StarCount() -> Element {
    let repo = use_resource(|| async {
        reqwest::get(REPO_API)
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await
    });

    let stars = match &*repo.read() {
        Some(Ok(r)) => match r["stargazers_count"].as_u64().unwrap_or(0) {
            1 => "1 star".to_string(),
            n => format!("{n} stars"),
        },
        _ => "- stars".to_string(),
    };

    rsx! {
        span { class: "cc-stat", "{stars}" }
    }
}
