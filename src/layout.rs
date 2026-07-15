use crate::Route;
use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/orko_logo.svg");

/// Shared app shell: navbar on top, routed content below, footer at the bottom.
#[component]
pub fn Layout() -> Element {
    rsx! {
        nav { id: "navbar",
            img { src: LOGO, alt: "orko logo" }
            span { "orko" }
        }
        Outlet::<Route> {}
        // footer { id: "footer", "© 2026 orko" }
    }
}
