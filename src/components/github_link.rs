use dioxus::prelude::*;

const GITHUB_LOGO: Asset = asset!("/assets/github_logo.svg");

/// Icon link to the Orko GitHub repo. Reuses the .x-link styling.
#[component]
pub fn GithubLink() -> Element {
    rsx! {
        a {
            class: "x-link",
            href: "https://github.com/orko-rs/orko",
            target: "_blank",
            rel: "noopener",
            aria_label: "Orko on GitHub",
            img {
                src: GITHUB_LOGO,
                alt: "GitHub logo",
                width: "18",
                height: "18",
                draggable: false,
            }
            span { class: "star-label", "Star on GitHub" }
        }
    }
}
