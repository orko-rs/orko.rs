use crate::components::{Footer, GithubLink, NavMenu, Version, XLink};
use crate::Route;
use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/orko_logo.svg");

#[component]
pub fn Layout() -> Element {
    rsx! {
        nav { id: "navbar",
            img { src: LOGO, alt: "orko logo", draggable: false }
            span { "Orko" }
            Version {}
            div { class: "nav-links",
                a { class: "nav-link", href: "#", "Docs" }
                GithubLink {}
                XLink {}
            }
            NavMenu {}
        }
        Outlet::<Route> {}
        Footer {}
    }
}
