use dioxus::prelude::*;

use crate::{components::{Emoji, SkinTrait}, game::{Card, ColorMode, Skin, Suit, SuitColor}};

pub const KATEX_MAIN: &str = "KaTeX_Main";

impl Skin {
    fn render_rank_internal(&self, card: &Card, _text_mode: bool) -> Element {
        if card.is_hunter() {
            rsx! { 
                span {
                    font_family: KATEX_MAIN,
                    "{card.rank}"
                }
            }
        } else {
            let text = match card.suit {
                Suit::Clubs => "🐰",
                Suit::Diamonds => "🦌",
                Suit::Hearts => "🦊",
                Suit::Spades => "🦆",
            };
            rsx! {
                Emoji { text }
            }
        }
    }
    fn render_suit_internal(&self, card: &Card, _text_mode: bool) -> Element {
        if card.is_hunter() {
            let text = match card.suit.color() {
                SuitColor::Black => "♠",
                SuitColor::Red => "♥",
            };
            rsx! {
                span {
                    font_family: KATEX_MAIN,
                    {text}
                }
            }
        } else {
            rsx! { span {} }
        }
    }
}

const COLOR_RED: [&str; 2] = ["#f00", "#ff8888"];
const COLOR_BLACK: [&str; 2] = ["#000", "#fff"];

impl SkinTrait<Card> for Skin {
    fn get_color(&self, card: &Card, mode: ColorMode) -> String {
        let res = match card.suit.color() {
            SuitColor::Black => COLOR_BLACK,
            SuitColor::Red => COLOR_RED,
        };
        res[mode as usize].to_string()
    }

    fn render_rank(&self, card: &Card) -> Element {
        self.render_rank_internal(card, false)
    }

    // fn render_rank_text(&self, card: &Card) -> Element {
    //     self.render_rank_internal(card, true)
    // }

    fn render_suit(&self, card: &Card) -> Element {
        self.render_suit_internal(card, false)
    }

    // fn render_suit_text(&self, card: &Card) -> Element {
    //     self.render_suit_internal(card, true)
    // }
    
    
}