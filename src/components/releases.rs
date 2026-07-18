use dioxus::prelude::*;

const RELEASES_API: &str = "https://api.github.com/repos/orko-rs/orko/releases?per_page=3";

#[component]
pub fn Releases() -> Element {
    let releases = use_resource(|| async {
        reqwest::get(RELEASES_API)
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await
    });

    rsx! {
        section { class: "releases",
            h2 { "Releases" }
            match &*releases.read() {
                Some(Ok(list)) if list.as_array().is_some_and(|a| !a.is_empty()) => rsx! {
                    for r in list.as_array().unwrap().iter().take(3) {
                        a {
                            class: "release",
                            href: r["html_url"].as_str().unwrap_or("https://github.com/orko-rs/orko/releases"),
                            target: "_blank",
                            rel: "noopener",
                            span { class: "release-tag", {r["tag_name"].as_str().unwrap_or("?")} }
                            span { class: "release-date",
                                {r["published_at"].as_str().and_then(|d| d.get(..10)).unwrap_or("")}
                            }
                        }
                    }
                },
                Some(_) => rsx! {
                    p { class: "release-empty", "no releases yet" }
                },
                None => rsx! {
                    p { class: "release-empty", "loading..." }
                },
            }
        }
    }
}
