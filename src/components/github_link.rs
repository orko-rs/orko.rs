use dioxus::prelude::*;

const GITHUB_LOGO: Asset = asset!("/assets/github_logo.svg");

#[component]
pub fn GithubLink() -> Element {
    rsx! {
        a {
            class: "icon-chip",
            href: "https://github.com/orko-rs/orko",
            target: "_blank",
            rel: "noopener",
            aria_label: "Orko on GitHub",
            img {
                src: GITHUB_LOGO,
                alt: "",
                width: "15",
                height: "15",
                draggable: false,
            }
        }
    }
}
