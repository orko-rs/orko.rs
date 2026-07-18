use dioxus::prelude::*;

/// Page footer.
#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { id: "footer",
            a {
                class: "footer-link",
                href: "https://x.com/orko_agents",
                target: "_blank",
                rel: "noopener",
                "X"
            }
            a {
                class: "footer-link",
                href: "https://github.com/orko-ai/orko",
                target: "_blank",
                rel: "noopener",
                "GitHub"
            }
            // "© 2026 orko"
        }
    }
}
