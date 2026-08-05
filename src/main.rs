use dioxus::prelude::*;

mod components;
mod layout;

use components::Home;
use layout::Layout;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
// unhashed so index.html can link it before the wasm boots (avoids unstyled flash)
const MAIN_CSS: Asset = asset!(
    "/assets/main.css",
    AssetOptions::builder().with_hash_suffix(false)
);

const DESCRIPTION: &str =
    "Orko: the agent orchestration toolkit for Rust. Compose efficient agents, under one runtime.";

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Orko" }
        document::Meta { name: "description", content: DESCRIPTION }
        document::Meta { property: "og:title", content: "Orko" }
        document::Meta { property: "og:description", content: DESCRIPTION }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { name: "twitter:card", content: "summary" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
