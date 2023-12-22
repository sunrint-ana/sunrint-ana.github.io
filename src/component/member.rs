use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct MemberProps{
    profile: String,
    name: String,
    stack: String
}

pub fn Member(cx: Scope<MemberProps>) -> Element {
    cx.render(rsx!(p {
        div {
            div {
                img {
                    src: "{cx.props.profile}"
                }
            }
            div {
                h1 { "cx.props.name" }
                h2 { "cx.props.stack"  }
            }
        }
    }))
}
