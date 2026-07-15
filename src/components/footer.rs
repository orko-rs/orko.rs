use dioxus::prelude::*;

/// Page footer.
#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { id: "footer",
            a {
                class: "x-link",
                href: "https://x.com/orko_agents",
                target: "_blank",
                rel: "noopener",
                "[X]"
            }
            // "© 2026 orko"
        }
    }
}
