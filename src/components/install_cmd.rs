use dioxus::prelude::*;

/// Terminal-style `cargo add orko` chip; click copies to clipboard.
#[component]
pub fn InstallCmd() -> Element {
    let mut copied = use_signal(|| false);

    rsx! {
        button {
            class: "install-cmd",
            onclick: move |_| {
                document::eval("navigator.clipboard.writeText('cargo add orko')");
                copied.set(true);
            },
            onmouseleave: move |_| copied.set(false),
            span { class: "prompt", "$" }
            code { "cargo add orko" }
            span { class: "copy-hint",
                if copied() {
                    "copied!"
                } else {
                    "copy"
                }
            }
        }
    }
}
