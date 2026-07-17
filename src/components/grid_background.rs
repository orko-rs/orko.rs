use dioxus::prelude::*;

const TILE: Asset = asset!("/assets/orko_tile.svg");

#[component]
pub fn GridBackground() -> Element {
    rsx! {
        div {
            id: "grid-background",
            style: "background-image: url({TILE}), repeating-conic-gradient(var(--royal-blue-800) 0% 25%, var(--royal-blue-900) 0% 50%);",
        }
    }
}
