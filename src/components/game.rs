use dioxus::prelude::*;

use crate::{utils::{LetterStateVec, LetterState}, styled};

styled!(GameInner, div, "");
styled!(GameWordRow, div, "display: flex; justify-content: center; flex-direction: row");

#[derive(Props)]
pub struct GameLetterProps<'a> {
    pub guessed_letters: &'a LetterStateVec,
    pub letter: char
}

pub fn GameLetter<'a>(cx: Scope<'a, GameLetterProps<'a>>) -> Element<'a> {
    let letter = cx.props.letter;

    let bg_colour = cx.props.guessed_letters
        .iter()
        .find_map(|(guessed_letter, state)| (guessed_letter == &letter).then(|| state))
        .unwrap_or(&LetterState::Guessed)
        .to_colour();

    cx.render(rsx! (
        div {
            class: "GameLetter",
            style: format_args!("text-transform: capitalize; border: 2px solid transparent; border-radius: 5px; font-size: 28px; height: 64px; width: 64px; align-items: center; margin: 3px; justify-content: center; background-color: {}", bg_colour),
            "{letter}"
        }
    ))
}

#[derive(Props, PartialEq)]
pub struct GameProps<'a> {
    pub word: &'a str,
    pub guessed_words: &'a[String],
    pub guessed_letters: &'a LetterStateVec
}

pub fn Game<'a>(cx: Scope<'a, GameProps<'a>>) -> Element<'a> {
    cx.render(rsx! (
        GameInner {
            cx.props.guessed_words.iter().map(|word| {
                rsx! (
                    GameWordRow {
                        {
                            word.chars().map(|letter| {
                                rsx! (
                                    GameLetter {
                                        guessed_letters: cx.props.guessed_letters,
                                        letter: letter
                                    }
                                )
                            })
                        }
                    }
                )
            })
        }
    ))
}
