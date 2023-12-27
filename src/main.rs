use dioxus_router::prelude::*;
use dioxus::{prelude::*, html::section};

use crate::component::{Header, Award, Member};
mod component;

fn main() {
    dioxus_web::launch(app);
}

#[derive(Routable, PartialEq, Debug, Clone)]
enum Route {
    #[route("/")]
    IndexPage {},
    #[route("/member")]
    MemberPage {},
    // #[route("/:..route")]
    // NotFound {
    //     route: Vec<String>,
    // },
}


// create a component that renders a div with the text "Hello, world!"
fn app(cx: Scope) -> Element {
    render!{
        link{
            rel: "stylesheet",
            href: "style.css"
        }
        Header{}
        Router::<Route> { }}
}



fn IndexPage(cx: Scope) -> Element {
    cx.render(rsx! { 
        main {
            article {
                id: "introduce", 
                section {
                    h1 {
                        "Application n Architecture"
                    }
                    h2 {
                        "Node.js와 Linux 서버 구축,
                        MySQL 등을 이용해서 웹 서버를 구축하는 동아리 입니다. "
                    }
                }
                svg{
                    width: "215",
                    height:"77",
                    view_box: "0 0 215 77",
                    fill: "none",
                    xmlns: "http://www.w3.org/2000/svg",
                    path {
                        d: "M12.9518 0H215L202.048 77H0L12.9518 0Z"
                    },
                    
                    path {
                        d: "M65.565 26.835V29.705H58.6V32.82C58.6 34.5233 58.9383 36.215 59.615 37.895C60.2917 39.5517 61.2367 41.045 62.45 42.375C63.6867 43.6817 65.0867 44.6733 66.65 45.35L64.725 48.08C62.9517 47.2633 61.3883 46.0617 60.035 44.475C58.6817 42.8883 57.6433 41.0683 56.92 39.015C56.1967 41.2317 55.1467 43.215 53.77 44.965C52.4167 46.6917 50.8533 47.975 49.08 48.815L47.12 46.12C48.66 45.3967 50.0367 44.3233 51.25 42.9C52.4867 41.4767 53.4433 39.8783 54.12 38.105C54.7967 36.3317 55.135 34.57 55.135 32.82V29.705H48.205V26.835H65.565ZM69.24 23.755H72.67V54.73H69.24V23.755ZM103.436 23.755V47.275H100.006V44.37H93.8808V41.955H100.006V23.755H103.436ZM77.6758 37.825C81.4325 37.825 84.9558 37.7667 88.2458 37.65C91.5358 37.5333 94.6392 37.3 97.5558 36.95L97.7658 39.4C95.3392 39.7967 92.6792 40.0883 89.7858 40.275V46.295H86.3558V40.45C83.9058 40.5667 81.1642 40.625 78.1308 40.625L77.6758 37.825ZM80.0208 30.37C80.0208 29.2967 80.3475 28.3517 81.0008 27.535C81.6542 26.7183 82.5525 26.0883 83.6958 25.645C84.8625 25.1783 86.1925 24.945 87.6858 24.945C89.1792 24.945 90.5092 25.1783 91.6758 25.645C92.8425 26.0883 93.7408 26.7183 94.3708 27.535C95.0242 28.3517 95.3508 29.2967 95.3508 30.37C95.3508 32.0267 94.6392 33.3567 93.2158 34.36C91.8158 35.34 89.9725 35.83 87.6858 35.83C86.1925 35.83 84.8625 35.6083 83.6958 35.165C82.5292 34.7217 81.6192 34.0917 80.9658 33.275C80.3358 32.435 80.0208 31.4667 80.0208 30.37ZM81.8758 45H85.3058V51.3H104.171V54.1H81.8758V45ZM83.3108 30.37C83.3108 31.2567 83.7075 31.9683 84.5008 32.505C85.3175 33.0183 86.3792 33.275 87.6858 33.275C89.0158 33.275 90.0775 33.0183 90.8708 32.505C91.6642 31.9917 92.0608 31.28 92.0608 30.37C92.0608 29.46 91.6642 28.7483 90.8708 28.235C90.0775 27.7217 89.0158 27.465 87.6858 27.465C86.3792 27.465 85.3175 27.7333 84.5008 28.27C83.7075 28.7833 83.3108 29.4833 83.3108 30.37ZM132.452 23.755V36.25H137.282V39.12H132.452V54.73H129.022V23.755H132.452ZM108.267 28.55H115.722V24.175H119.187V28.55H126.502V31.35H108.267V28.55ZM109.807 40.835C109.807 39.435 110.133 38.1867 110.787 37.09C111.463 35.9933 112.385 35.1417 113.552 34.535C114.718 33.9283 116.025 33.625 117.472 33.625C118.942 33.625 120.26 33.9283 121.427 34.535C122.593 35.1417 123.503 35.9933 124.157 37.09C124.81 38.1867 125.137 39.435 125.137 40.835C125.137 42.235 124.81 43.4833 124.157 44.58C123.503 45.6533 122.593 46.505 121.427 47.135C120.26 47.7417 118.942 48.045 117.472 48.045C116.025 48.045 114.718 47.7417 113.552 47.135C112.385 46.505 111.463 45.6417 110.787 44.545C110.133 43.4483 109.807 42.2117 109.807 40.835ZM113.132 40.835C113.132 42.1417 113.528 43.2033 114.322 44.02C115.138 44.8133 116.188 45.21 117.472 45.21C118.755 45.21 119.805 44.8133 120.622 44.02C121.438 43.2033 121.847 42.1417 121.847 40.835C121.847 39.5283 121.438 38.4667 120.622 37.65C119.805 36.8333 118.755 36.425 117.472 36.425C116.188 36.425 115.138 36.8333 114.322 37.65C113.528 38.4433 113.132 39.505 113.132 40.835ZM164.547 23.755V54.73H161.117V23.755H164.547ZM139.277 46.435C143.291 44.4983 146.312 42.165 148.342 39.435C150.372 36.6817 151.551 33.4733 151.877 29.81H140.747V27.01H155.412C155.412 31.8633 154.292 36.1567 152.052 39.89C149.812 43.6 146.149 46.6917 141.062 49.165L139.277 46.435Z",
                        fill: "#fff"
                    }
                }
            }
            img {
                src: "slide/light-orange.svg"
            }
            article {
                id: "awards",
                section{
                    h1 {
                        "총 120개의 대회에서 수상"
                    }
                    h2 {
                        "2023년에 7개수상 "
                    }
                }
                section {
                    // Award {
                    //     t: "대상".to_string(),
                    //     d: "suspiciously 대회".to_string()
                    // }
                    
                    Award {
                        t: "대상".to_string(),
                        d: "suspiciously 대회".to_string()
                    }
                    
                    Award {
                        t: "대상".to_string(),
                        d: "suspiciously 대회".to_string()
                    }
                }
            }
        }
    })
}




fn MemberPage(cx: Scope) -> Element {
    cx.render(rsx! { 
        main {
            article {
                id: "introduce", 
                section {
                    h1 {
                        "AnA Members"
                    }
                    h2 {
                        "AnA와 함깨한 사람들입니다(아님 말고)"
                    }
                }
            }
            img {
                src: "slide/light-orange.svg"
            }
            article {
                class: "members",
                h1 {
                    "special thx"
                }
                section {
                    Member {
                        profile: "5-23".to_string(),
                        name: "5-23".to_string(),
                        stack: "fulltack / design".to_string()
                    },
                    Member {
                        profile: "5-23".to_string(),
                        name: "5-23".to_string(),
                        stack: "fulltack / design".to_string()
                    },
                    Member {
                        profile: "5-23".to_string(),
                        name: "5-23".to_string(),
                        stack: "fulltack / design".to_string()
                    },
                }
            }
            
            img {
                class: "reverse",
                src: "slide/dark-orange.svg",
            }
            article {
                class: "members reverse",
                h1 {
                    "2023"
                }
                section {
                    Member {
                        profile: "5-23".to_string(),
                        name: "5-23".to_string(),
                        stack: "fulltack / design".to_string()
                    },
                    Member {
                        profile: "5-23".to_string(),
                        name: "5-23".to_string(),
                        stack: "fulltack / design".to_string()
                    },
                    Member {
                        profile: "5-23".to_string(),
                        name: "5-23".to_string(),
                        stack: "fulltack / design".to_string()
                    },
                }
            }

            img {
                class: "no-reverse",
                src: "slide/light-orange.svg",
            }
            article {
                class: "members",
                h1 {
                    "2022"
                }
                section {
                    Member {
                        profile: "5-23".to_string(),
                        name: "5-23".to_string(),
                        stack: "fulltack / design".to_string()
                    },
                    Member {
                        profile: "5-23".to_string(),
                        name: "5-23".to_string(),
                        stack: "fulltack / design".to_string()
                    },
                    Member {
                        profile: "5-23".to_string(),
                        name: "5-23".to_string(),
                        stack: "fulltack / design".to_string()
                    },
                }
            }
        }
    })
}


fn NotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}