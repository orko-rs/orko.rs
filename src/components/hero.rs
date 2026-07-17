use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/orko_logo.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { id: "hero",
            div { class: "hero-text",
                h1 { "Orko" }
                p { class: "tagline", "The Agent orchestration toolkit for Rust." }
                p { "<coming soon>" }
                // div { class: "cta",
                //     // a { class: "btn primary", href: "/docs", "Get Started" }
                //     a { class: "btn", href: "https://github.com/orko-rs/orko", "GitHub" }
                // }
            }
            img {
                class: "hero-logo",
                src: LOGO,
                alt: "orko logo",
                draggable: false,
            }
            ul { class: "hero-features",
                li { "Composable agents" }
                li { "Typed tool calls" }
                li { "Async-first, pure Rust" }
            }
        }
    }
}
