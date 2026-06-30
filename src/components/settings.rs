use dioxus::prelude::*;

use crate::game::{GameState, ScreenState};

#[component]
pub fn Settings(game_state: Signal<GameState>) -> Element {
    let mut state = use_signal(|| {
        game_state.read().new_settings_state()
    });
    let mut ok = move || {
        game_state.write().apply_settings(&state.read());
        game_state.write().screen_state = ScreenState::Game;
    };
    let mut cancel = move || {
        game_state.write().screen_state = ScreenState::Game;
    };

    let onmounted = async move |e: Event<MountedData>| {
        let _ = e.set_focus(true).await;
    };
    let onkeydown = move |e: Event<KeyboardData>| {
        let key = e.key();
        match key {
            Key::Enter => {
                ok();
            }
            Key::Escape => {
                cancel();
            }
            _ => {}
        }
    };

    let allow_undo_changed = move |evt: Event<FormData>| {
        state.write().allow_undo = evt.checked();
    };

    rsx! {
        div {
            id: "settingsDialog",
            tabindex: -1,
            onmounted: onmounted,
            onkeydown: onkeydown,

            p {
                "Allow undo/reset: "
                input {
                    r#type: "checkbox",
                    checked: state.read().allow_undo,
                    onchange: allow_undo_changed,
                }
            }

            p {
                button {
                    r#type: "button",
                    onclick: move |_| ok(),
                    "OK"
                }
                " ",
                button {
                    r#type: "button",
                    onclick: move |_| cancel(),
                    "Cancel"
                }
            }

            p {
                class: "copyright",
                "Game rules: “Proletariat’s Patience” by Zachtronics", br{},
                "Webapp © OnlineMathLearning.com"
            }
        }
    }
}