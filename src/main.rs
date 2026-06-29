use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::CardComponent, game::{Card, Skin, Suit}};

mod game;
mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");

// altered version of KaTeX_Main to include filled "red" suits
const KATEX_SUITS: Asset = asset!("/assets/KaTeX_Suits.woff2");

const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
         document::Style {
            r#"
            @font-face {{
                font-family: KaTeX_Main;
                font-style: normal;
                font-weight: 700;
                src: url({KATEX_SUITS}) format("woff2");
            }}
            "#,
        }

        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",

            CardComponent { 
                position: Vec2::new(10., 10.),
                width: 14.,
                card: Card { rank: 6, suit: Suit::Spades, },
                skin: Skin
            }
        }
    }
}
