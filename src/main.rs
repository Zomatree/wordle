#![allow(non_snake_case)]

use dioxus::prelude::*;

use components::{game::Game, instructions::Instructions, keyboard::Keyboard};
use utils::{get_random_word, LetterStateVec};

#[macro_use]
pub mod utils;

pub mod components;

styled!(Top, div, "max-width: 600px; justify-content: center; display: flex; flex-direction: column; margin: 0 auto");

fn App(cx: Scope) -> Element {
    let (hidden_word, _set_hidden_word) = use_state(&cx, || get_random_word(5));
    let (guessed_words, set_guessed_words) = use_state(&cx, Vec::<String>::new);
    let (guessed_letters, set_guessed_letters) = use_state(&cx, LetterStateVec::new);
    let (current_guessed_word, set_current_guessed_word) = use_state(&cx, Vec::<char>::new);

    cx.render(rsx! (
        div {
            style: "background-color: #13141c; min-width: 100%; height: 100%",
            style {
                ["body {
                    margin: 0px
                };", "* {
                    "]
            },
            Top {
                Game {
                    word: hidden_word,
                    guessed_words: guessed_words,
                    guessed_letters: guessed_letters,
                    current_guessed_word: current_guessed_word
                },
                Keyboard {
                    set_guessed_words: set_guessed_words,
                    set_guessed_letters: set_guessed_letters,
                    current_guessed_word: current_guessed_word,
                    set_current_guessed_word: set_current_guessed_word,
                    hidden_word: hidden_word
                },
            }
            Instructions {}
        }
    ))
}

fn main() {
    dioxus::web::launch(App)
}
