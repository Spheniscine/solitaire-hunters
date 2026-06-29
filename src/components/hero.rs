use dioxus::prelude::*;
use async_std::stream::StreamExt;
use glam::Vec2;

#[component]
pub fn Hero() -> Element {
    let mut state = use_signal(|| {
        // if let Some(mut state) = LocalStorage.load_game_state() {
        //     state.board.selected = None;
        //     state.screen_state = ScreenState::Game;
        //     return state;
        // }
        GameState::init()
    });
}