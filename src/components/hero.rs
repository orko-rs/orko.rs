use super::InstallCmd;
use dioxus::prelude::*;

const LOGO: Asset = asset!(
    "/assets/orko_logo.svg",
    AssetOptions::builder().with_hash_suffix(false)
);

#[component]
pub fn Hero() -> Element {
    let version = use_resource(|| async {
        let v = reqwest::get("https://crates.io/api/v1/crates/orko")
            .await
            .ok()?
            .json::<serde_json::Value>()
            .await
            .ok()?;
        v["crate"]["max_stable_version"]
            .as_str()
            .map(|s| format!(" · v{s}"))
    });
    let version = version.read().clone().flatten().unwrap_or_default();

    rsx! {
        section { id: "hero", class: "container",
            h1 {
                "Orko"
                img {
                    class: "h1-mark",
                    src: LOGO,
                    alt: "",
                    draggable: false,
                }
            }
            p { class: "tagline",
                "The agent orchestration toolkit for Rust. Compose efficient agents."
            }
            p { class: "hero-meta", "Apache-2.0{version}" }
            div { class: "cta",
                Link { class: "btn primary", to: "/docs/getting-started", "Get Started" }
                a {
                    class: "btn ghost",
                    href: "https://github.com/orko-rs/orko",
                    target: "_blank",
                    rel: "noopener",
                    "View on GitHub"
                }
                InstallCmd {}
            }
        }
    }
}
