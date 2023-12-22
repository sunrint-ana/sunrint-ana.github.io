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
            img {
                src: "slide/award.svg"
            }
            div {
                b { "{cx.props.t}" }
                div {}
                "{cx.props.d}"
            }
            img {
                src: "slide/award.svg"
            }
        }
    }))
}
