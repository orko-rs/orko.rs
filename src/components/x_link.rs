use dioxus::prelude::*;

const X_LOGO: Asset = asset!("/assets/x_logo.svg");

#[component]
pub fn XLink() -> Element {
    rsx! {
        a {
            class: "icon-chip",
            href: "https://x.com/orko_agents",
            target: "_blank",
            rel: "noopener",
            aria_label: "Orko on X",
            img { src: X_LOGO, alt: "", width: "15", height: "15", draggable: false }
        }
    }
}
