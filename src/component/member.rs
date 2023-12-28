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
                src: "slide/info.svg"
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
                src: "slide/info.svg"
            }
        }
    }))
}
// svg {
//     width: "10",
//     height: "72",
//     view_box: "0 0 10 72",
//     fill: "none",
//     xmlns: "http://www.w3.org/2000/svg",
//     path {
//         d: "M10 0V72H0L10 0Z"
//     }
// }