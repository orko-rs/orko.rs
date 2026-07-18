use super::{InstallCmd, Releases, Stats};
use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/orko_logo.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { id: "hero",
            div { class: "hero-text",
                h2 { class: "boot-line", "initializing agents..." }
                h1 { "Orko" }
                p { class: "tagline", "The Agent orchestration toolkit for Rust." }
                p { "<coming soon>" }
                InstallCmd {}
                div { class: "cta",
                    // ponytail: dead link until docs exist, like the navbar Docs
                    a { class: "btn primary", href: "#", "Get Started" }
                }
                Stats {}
            }
            img {
                class: "hero-logo",
                src: LOGO,
                alt: "orko logo",
                draggable: false,
            }
            Releases {}
        }
    }
}
