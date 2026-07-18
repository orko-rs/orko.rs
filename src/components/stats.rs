use dioxus::prelude::*;

const REPO_API: &str = "https://api.github.com/repos/orko-rs/orko";

#[component]
pub fn Stats() -> Element {
    let repo = use_resource(|| async {
        reqwest::get(REPO_API)
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await
    });

    let stars = match &*repo.read() {
        Some(Ok(r)) => r["stargazers_count"].as_u64().unwrap_or(0).to_string(),
        _ => "-".to_string(),
    };

    rsx! {
        div { class: "stats",
            span { class: "stat", " {stars} stars" }
            span { class: "stat", "Apache-2.0" }
        }
    }
}
