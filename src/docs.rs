use crate::Route;
use dioxus::prelude::*;
use pulldown_cmark::{html, Options, Parser};

// ponytail: pages are compiled in from /docs; add a page by adding a line here
const PAGES: [(&str, &str, &str); 4] = [
    ("getting-started", "Getting started", include_str!("../docs/getting-started.md")),
    ("agents", "Agents", include_str!("../docs/agents.md")),
    ("tools", "Tools", include_str!("../docs/tools.md")),
    ("providers", "Providers", include_str!("../docs/providers.md")),
];

pub const FIRST_PAGE: &str = PAGES[0].0;

fn render(md: &str) -> String {
    let mut out = String::new();
    html::push_html(&mut out, Parser::new_ext(md, Options::ENABLE_TABLES));
    out
}

#[component]
pub fn DocPage(slug: String) -> Element {
    let page = PAGES.iter().find(|p| p.0 == slug);
    let title = page.map_or("Not found", |p| p.1);
    let body = page.map_or_else(
        || render("# Page not found\n\n[Back to the docs](/docs)"),
        |p| render(p.2),
    );

    // ponytail: highlight.js from CDN, re-run per page; vendor it if offline builds matter
    use_effect(use_reactive!(|slug| {
        let _ = slug;
        document::eval(r#"
            document.querySelectorAll('.doc-body pre:not(:has(.copy-btn))').forEach(pre => {
                const b = document.createElement('button');
                b.className = 'copy-btn';
                b.textContent = 'Copy';
                b.onclick = () => navigator.clipboard.writeText(pre.querySelector('code').innerText).then(() => {
                    b.textContent = 'Copied';
                    setTimeout(() => b.textContent = 'Copy', 1500);
                });
                pre.append(b);
            });
            (function h(){ window.hljs ? hljs.highlightAll() : setTimeout(h, 50) })();
        "#);
    }));

    rsx! {
        document::Stylesheet { href: "https://cdn.jsdelivr.net/npm/@highlightjs/cdn-assets@11.11.1/styles/github-dark.min.css" }
        document::Script { src: "https://cdn.jsdelivr.net/npm/@highlightjs/cdn-assets@11.11.1/highlight.min.js" }
        document::Title { "{title} · Orko docs" }
        div { class: "docs container",
            aside { class: "docs-nav",
                p { class: "section-label", "Docs" }
                for (s , t , _) in PAGES {
                    Link {
                        to: Route::DocPage { slug: s.to_string() },
                        active_class: "active",
                        onclick: |_| {
                            document::eval("window.scrollTo(0, 0)");
                        },
                        "{t}"
                    }
                }
            }
            article { class: "doc-body", dangerous_inner_html: body }
        }
    }
}
