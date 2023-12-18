use dioxus::{prelude::*, html::{img, div, h1}};

fn main() {
    // launch the web app
    dioxus_web::launch(app);
}

// create a component that renders a div with the text "Hello, world!"
fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div { 
            // background_color: "red",
            h1 {
                "Application n Architecture"
            }
            p {
                "I use nixos btw"
            }
            button {
                "sus"
            }
        }

        
        div { 
            // background_color: "red",
            Award{
                t: "10".to_string(),
                d: "222".to_string()
            }
        } 
    })
}
#[derive(PartialEq, Props, Clone)]
struct AwardProps{
    t: String,
    d: String
}



fn Award(cx: Scope<AwardProps>) -> Element {
    cx.render(rsx!(p {
        div {
            display: "inline-flex",
            font_size: "20px",
            gap: "10px",
            align_items: "center",
            justify_content: "center",
            b { "{cx.props.t}" }
            div {
                width: "3px",
                height: "20px",
                background: "#000",
            }
            "{cx.props.d}"
        }
    }))
}

#[derive(PartialEq, Props, Clone)]
struct MemberProps{
    profile: String,
    name: String,
    stack: String
}

fn Member(cx: Scope<MemberProps>) -> Element {
    cx.render(rsx!(p {
        div {
            div {
                img {
                    src: "{cx.props.profile}"
                }
            }
            div {
                h1 { "cx.props.name" }
                h2 { "cx.props.stack" }
            }
        }
    }))
}
