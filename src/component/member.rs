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
            class: "member",
            img {
                src: "slide/award.svg"
            }
            div {
                img {
                    id: "img",
                    src: "member/{cx.props.profile}.png"
                }
                section {
                    b { "{cx.props.name}" }
                    "{cx.props.stack}"
                }
            }
            img {
                src: "slide/award.svg"
            }
        }
    }))
}