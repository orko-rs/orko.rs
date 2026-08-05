use crate::components::{Footer, GithubLink, NavMenu, XLink};
use crate::Route;
use dioxus::prelude::*;

const LOGO: Asset = asset!(
    "/assets/orko_logo.svg",
    AssetOptions::builder().with_hash_suffix(false)
);

#[component]
pub fn Layout() -> Element {
    rsx! {
        nav { id: "navbar",
            img { src: LOGO, alt: "orko logo", draggable: false }
            span { "Orko" }
            div { class: "nav-links",
                a { class: "nav-link", href: "#toolkit", "Toolkit" }
                a { class: "nav-link", href: "#releases", "Releases" }
                a { class: "nav-link", href: "#community", "Community" }
                a { class: "nav-link", href: "#documentation", "Docs" }
                XLink {}
                GithubLink {}
            }
            NavMenu {}
        }
        Outlet::<Route> {}
        Footer {}
    }
}
