#![allow(non_snake_case)]

use dioxus::prelude::*;

use utils::{get_random_word, LetterStateVec};
use components::{
    game::Game,
    instructions::Instructions,
    keyboard::Keyboard
};

#[macro_use]
pub mod utils;

pub mod components;



fn App(cx: Scope) -> Element {
    let (hidden_word, _set_hidden_word) = use_state(&cx, || get_random_word(5));
    let (guessed_words, set_guessed_words) = use_state(&cx, Vec::<String>::new);
    let (guessed_letters, set_guessed_letters) = use_state(&cx, LetterStateVec::new);
    let (current_guessed_word, set_current_guessed_word) = use_state(&cx, Vec::<char>::new);

    cx.render(rsx! (
        div {
            style: "max-width: 600px; background-color: #13141c; width: 100%; height: 100%",
            style {
                ["body {
                    margin: 0px
                } p {color: white} * {
                    font-family: 'Montserrat','Open Sans',sans-serif;
                    text-rendering: optimizeLegibility;"]
            }
            p { "{hidden_word}" },
            Game {
                word: hidden_word,
                guessed_words: guessed_words,
                guessed_letters: guessed_letters
            },
            Keyboard {
                set_guessed_words: set_guessed_words,
                guessed_letters: guessed_letters,
                set_guessed_letters: set_guessed_letters,
                current_guessed_word: current_guessed_word,
                set_current_guessed_word: set_current_guessed_word,
                hidden_word: hidden_word
            },
            p {
                [format_args!("{:?}", current_guessed_word.iter().collect::<String>())]
            },
            Instructions {}
        }
    ))
}

fn main() {
    dioxus::web::launch(App)
}
