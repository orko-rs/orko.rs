use super::{GithubLink, XLink};
use dioxus::prelude::*;

/// Mobile-only navbar dropdown; native <details>, no state needed.
#[component]
pub fn NavMenu() -> Element {
    rsx! {
        details { class: "nav-menu",
            summary { aria_label: "Menu", "\u{2630}" }
            div { class: "nav-menu-panel",
                a { class: "nav-link", href: "#", "Docs" }
                GithubLink {}
                XLink {}
            }
        }
    }
}
