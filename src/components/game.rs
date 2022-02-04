use dioxus::prelude::*;

use crate::{utils::{LetterStateVec, LetterState}, styled};

styled!(GameInner, div, "");
styled!(GameWordRow, div, "display: flex; justify-content: center; flex-direction: row");
styled!(EmptyLetter, div, "justify-content: center; display: flex; font-family: 'Montserrat','Open Sans',sans-serif; text-rendering: optimizeLegibility; color: white; font-size: 28px; min-width: 64px; min-height: 64px; border: 2px solid rgba(154,166,180,.4); border-radius: 5px; margin: 3px; background-color: transparent; text-transform: capitalize; align-items: center;");

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
            style: format_args!("display: flex; font-family: 'Montserrat','Open Sans',sans-serif; text-rendering: optimizeLegibility; color: white; text-transform: capitalize; border: 2px solid transparent; border-radius: 5px; font-size: 28px; height: 64px; width: 64px; align-items: center; margin: 3px; justify-content: center; background-color: {}", bg_colour),
            "{letter}"
        }
    ))
}

#[derive(Clone, Copy, Debug)]
enum Letter {
    Filled(char),
    Current(char),
    None
}

#[derive(Props, PartialEq)]
pub struct GameProps<'a> {
    pub word: &'a str,
    pub guessed_words: &'a[String],
    pub guessed_letters: &'a LetterStateVec,
    pub current_guessed_word: &'a[char]
}

pub fn Game<'a>(cx: Scope<'a, GameProps<'a>>) -> Element<'a> {
    let mut board = Vec::<Vec<Letter>>::with_capacity(6);

    for word in cx.props.guessed_words {
        board.push(word.chars().map(Letter::Filled).collect())
    }

    if board.len() != 6 && !cx.props.current_guessed_word.is_empty() {
        board.push(cx.props.current_guessed_word.iter().map(|c| Letter::Current(*c)).collect());
        board.last_mut().unwrap().extend(vec![Letter::None; 5 - cx.props.current_guessed_word.len()]);
    }

    board.extend(vec![vec![Letter::None; 5]; 6 - board.len()]);

    cx.render(rsx! (
        GameInner {
            board.into_iter().map(|word| {
                rsx! (
                    GameWordRow {
                        word.into_iter().map(|letter| {
                            match letter {
                                Letter::Filled(letter) => {
                                    rsx! {
                                        GameLetter {
                                            guessed_letters: cx.props.guessed_letters,
                                            letter: letter
                                        }
                                    }
                                },
                                Letter::Current(letter) => {
                                    rsx! {
                                        EmptyLetter {
                                            "{letter}"
                                        }
                                    }
                                }
                                Letter::None => {
                                    rsx! {
                                        EmptyLetter {}
                                    }
                                }
                            }
                        })
                    }
                )
            })
        }
    ))
}
