use dioxus::prelude::*;

use crate::{components::{KATEX_MAIN, VIDEO_GAMEPLAY, rem}, game::{GameState, ScreenState}};

#[component]
fn Emph(children: Element) -> Element {
    rsx! {
        strong {
            color: "#ff0",
            {children}
        }
    }
}

#[component]
pub fn Help(game_state: Signal<GameState>) -> Element {
    // let st = game_state.read();
    // let skin = st.skin;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; font-size: 4.5rem; color: #fff; padding: 4rem;",
            class: "help",

            div {
                text_align: "left",

                p {
                    margin_top: "0",
                    "The deck is a special 36-card deck, whose cards are in one of two categories:"
                    ul {
                        li {
                            "20 ", Emph{"hunter"}, " (number) cards: ranks ",
                            span {
                                font_family: KATEX_MAIN,
                                font_size: "1.2em",
                                "1"
                            }, "~", 
                            span {
                                font_family: KATEX_MAIN,
                                font_size: "1.2em",
                                "5"
                            }, ", 2 suits, 2 copies each."
                        },
                        li { "16 ", Emph{"animal"}, " cards: 4 kinds, 4 copies each."
                        },
                    }
                }

                p {
                    Emph{"Hunter"}," cards stack by ",Emph{"alternating suit"}," and ",Emph{"descending rank"},
                    ", and can be moved together as a stack of any size."
                }

                p {
                    Emph{"Animal"}," cards stack ",Emph{"by kind"},", and can also be moved as a stack. When a complete stack of
                    4 animal cards are alone on the board, they will ",Emph{"hide"}," and become immovable."
                }

                p {
                    Emph {"NOTE:"}, " To move cards, click to select a card or stack, then click the destination. ", Emph{"“Drag and drop” is not required."}
                }

                p {
                    "The ",Emph{"free cell"}," on the top-right may store a single card of any kind."
                }

                p {
                    "To ",Emph{"win the game"},", all animals must hide, and all hunters must be in 4 complete stacks of 5 cards each."
                }

                div {
                    position: "absolute",
                    bottom: rem(2.),
                    width: "92rem",
                    display: "flex",
                    justify_content: "center",

                    a {
                        href: VIDEO_GAMEPLAY,
                        target: "_blank",
                        text_decoration: "none",
                        margin_right: rem(4.),
                        div {
                            width: rem(30.),
                            position: "relative",
                            class: "game-button",
                            "Example video"
                        }
                    }

                    div {
                        width: rem(30.),
                        position: "relative",
                        class: "game-button",
                        onclick: move |_| game_state.write().screen_state = ScreenState::Game,
                        "Back to game"
                    }
                }
                
            }
        }
        
    }
}