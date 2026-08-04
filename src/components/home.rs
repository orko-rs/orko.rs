use super::{Hero, Releases, StarCount};
use dioxus::prelude::*;

const GITHUB_LOGO: Asset = asset!("/assets/github_logo.svg");
const X_LOGO: Asset = asset!("/assets/x_logo.svg");

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Crates {}
        section { id: "releases", class: "section container",
            p { class: "section-label", "Releases" }
            Releases {}
        }
        Community {}
    }
}

const CRATES: [(&str, &str); 6] = [
    ("orko-core", "the foundation"),
    ("orko-runtime", "the engine"),
    ("orko-graph", "the choreography"),
    ("orko-providers", "the connections"),
    ("orko-macros", "the ergonomics"),
    ("orko-mcp", "the bridge"),
];

#[component]
fn Crates() -> Element {
    rsx! {
        section { id: "toolkit", class: "section container",
            p { class: "section-label", "6 Crate Toolkit" }
            h2 { "Inside the toolkit" }
            p { class: "section-sub",
                "Six focused crates with strict boundaries, so you pull in exactly what you use."
            }
            div { class: "crate-grid",
                for (name , role) in CRATES {
                    div { class: "crate-card",
                        div { class: "crate-name", {name} }
                        div { class: "crate-role", {role} }
                    }
                }
            }
        }
    }
}

#[component]
fn Community() -> Element {
    rsx! {
        section { id: "community", class: "section container",
            p { class: "section-label", "Community" }
            h2 { "Follow the build" }
            p { class: "section-sub",
                "Like pods of orcas, we collaborate to build. Get involved."
            }
            div { class: "community-grid",
                a {
                    class: "community-card",
                    href: "https://github.com/orko-rs/orko",
                    target: "_blank",
                    rel: "noopener",
                    div { class: "cc-head",
                        span { class: "cc-icon",
                            img { src: GITHUB_LOGO, alt: "", draggable: false }
                        }
                        span {
                            div { class: "cc-name", "orko-rs / orko" }
                            div { class: "cc-sub", "GitHub" }
                        }
                        StarCount {}
                    }
                }
                a {
                    class: "community-card",
                    href: "https://x.com/orko_agents",
                    target: "_blank",
                    rel: "noopener",
                    div { class: "cc-head",
                        span { class: "cc-icon",
                            img { src: X_LOGO, alt: "", draggable: false }
                        }
                        span {
                            div { class: "cc-name", "@orko_agents" }
                            div { class: "cc-sub", "X" }
                        }
                    }
                }
            }
        }
    }
}
