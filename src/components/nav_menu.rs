use dioxus::prelude::*;

#[component]
pub fn NavMenu() -> Element {
    rsx! {
        details { class: "nav-menu",
            summary { aria_label: "Menu", "\u{2630}" }
            div { class: "nav-menu-panel",
                a { class: "nav-link", href: "#toolkit", "Toolkit" }
                a { class: "nav-link", href: "#releases", "Releases" }
                a { class: "nav-link", href: "#community", "Community" }
                a { class: "nav-link", href: "#documentation", "Docs" }
                a { class: "nav-link", href: "https://x.com/orko_agents", "X" }
                a { class: "nav-link", href: "https://github.com/orko-rs/orko", "GitHub" }
            }
        }
    }
}
