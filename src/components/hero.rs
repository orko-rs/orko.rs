use super::InstallCmd;
use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/orko_logo.svg");

#[component]
pub fn Hero() -> Element {
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
            p { class: "hero-meta", "Apache-2.0 · Coming soon" }
            div { class: "cta",
                a { class: "btn primary", href: "#", "Get Started" }
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
