use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! { 
        header {
            img {
                src: "logo.svg"
            }
            nav {
                a {
                    href: "member",
                    "member"
                }
            }
        }
        div {
            margin_bottom: "70px"
        }
    })
}
