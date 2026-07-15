use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/orko_logo.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { id: "hero",
            div {
                id: "logo-strip",
                style: "background-image: url({LOGO});",
                role: "img",
                aria_label: "orko logo",
            }
            h1 { "Orko" }
            p { "<coming soon>" }
            // p { class: "tagline", "The Agent orchestration toolkit for Rust." }
            // div { class: "cta",
            //     // a { class: "btn primary", href: "/docs", "Get Started" }
            //     a { class: "btn", href: "https://github.com/orko-rs/orko", "GitHub" }
            // }
        }
    }
}
