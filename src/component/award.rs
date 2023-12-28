use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct AwardProps{
    t: String,
    d: String
}



pub fn Award(cx: Scope<AwardProps>) -> Element {
    cx.render(rsx!(p {
        div {
            class: "award",
            svg {
                width: "10",
                height: "72",
                view_box: "0 0 10 72",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    d: "M10 0V72H0L10 0Z"
                }
            }
            div {
                b { "{cx.props.t}" }
                div {}
                "{cx.props.d}"
            }
            svg {
                width: "10",
                height: "72",
                view_box: "0 0 10 72",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    d: "M10 0V72H0L10 0Z"
                }
            }
        }
    }))
}
