use dioxus::prelude::*;

const X_LOGO: Asset = asset!("/assets/x_logo.svg");

/// Icon link to the Orko X account.
#[component]
pub fn XLink() -> Element {
    rsx! {
        a {
            class: "x-link",
            href: "https://x.com/orko_agents",
            target: "_blank",
            rel: "noopener",
            aria_label: "Orko on X",
            img { src: X_LOGO, alt: "X logo", width: "18", height: "18", draggable: false }
        }
    }
}
