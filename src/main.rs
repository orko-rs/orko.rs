use dioxus::prelude::*;

mod components;
mod layout;

use components::{GridBackground, Home};
use layout::Layout;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const PRESS_START: Asset = asset!("/assets/fonts/press-start-2p.woff2");
const VT323: Asset = asset!("/assets/fonts/vt323.woff2");
const SPACE_GROTESK: Asset = asset!("/assets/fonts/space-grotesk.woff2");
const PIXEL_BORDER: Asset = asset!("/assets/pixel_border.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Orko" }
        document::Meta {
            name: "description",
            content: "Orko - coming soon for Rust",
        }
        document::Meta { property: "og:title", content: "Orko" }
        document::Meta {
            property: "og:description",
            content: "Orko - coming soon for Rust",
        }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { name: "twitter:card", content: "summary" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Style {
            "@font-face {{ font-family: 'Press Start 2P'; src: url({PRESS_START}) format('woff2'); font-weight: 400; font-display: swap; }}
            @font-face {{ font-family: 'VT323'; src: url({VT323}) format('woff2'); font-weight: 400; font-display: swap; }}
            @font-face {{ font-family: 'Space Grotesk'; src: url({SPACE_GROTESK}) format('woff2'); font-weight: 300 700; font-display: swap; }}
            #navbar, #hero, #footer, .btn, #navbar .x-link, .nav-menu-panel, .nav-menu summary, .releases {{ border-image-source: url({PIXEL_BORDER}); }}"
        }
        GridBackground {}
        Router::<Route> {}
    }
}
