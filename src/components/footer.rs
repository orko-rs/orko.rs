use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { id: "footer",
            div { class: "footer-cols container",
                div { class: "footer-col",
                    div { class: "footer-heading", "Site" }
                    a { href: "#toolkit", "Toolkit" }
                    a { href: "#releases", "Releases" }
                    a { href: "#community", "Community" }
                    a { href: "#documentation", "Docs" }
                }
                div { class: "footer-col",
                    div { class: "footer-heading", "Socials" }
                    a {
                        href: "https://x.com/orko_agents",
                        target: "_blank",
                        rel: "noopener",
                        "X"
                    }
                    a {
                        href: "https://github.com/orko-rs/orko",
                        target: "_blank",
                        rel: "noopener",
                        "GitHub"
                    }
                }
                div { class: "footer-col",
                    div { class: "footer-heading", "Resources" }
                    a {
                        href: "https://github.com/orko-rs/orko/issues",
                        target: "_blank",
                        rel: "noopener",
                        "Issues"
                    }
                    a {
                        href: "https://github.com/orko-rs/orko/blob/master/LICENSE",
                        target: "_blank",
                        rel: "noopener",
                        "License"
                    }
                }
            }
            div { class: "footer-mark",
                span { class: "footer-word", aria_hidden: "true", "Orko" }
                span { class: "footer-copy", "© 2026 Orko · Apache-2.0" }
            }
        }
    }
}
