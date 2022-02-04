use dioxus::prelude::*;
use crate::{utils::{LetterStateVec, LetterState}};

styled!(KeyboardInner, div, "display: flex; flex-direction: column; margin-top: 30px");
styled!(KeyboardRow, div, "display: flex; flex-direction: row; justify-content: stretch");

static KEYBOARDBUTTONSTYLE: &str = "font-family: 'Montserrat','Open Sans',sans-serif; text-rendering: optimizeLegibility; color: white; text-transform: capitalize; padding: 15px; cursor: pointer; font-size: 17px; margin: 3px; justify-content: center; border: 2px solid transparent; border-radius: 4px; display: flex; flex: 1 1; background-color:";

#[derive(Props)]
pub struct KeyboardButtonProps<'a> {
    pub guessed_letters: &'a LetterStateVec,
    pub guessed_word: &'a UseState<Vec<char>>,
    pub letter: char
}

pub fn KeyboardButton<'a>(cx: Scope<'a, KeyboardButtonProps<'a>>) -> Element<'a> {
    let guessed_word = cx.props.guessed_word;
    let letter = cx.props.letter;

    let bg_colour = cx.props.guessed_letters
        .iter()
        .find_map(|(guessed_letter, state)| (guessed_letter == &letter).then(|| state))
        .unwrap_or_default()
        .to_colour();

    cx.render(rsx! (
        div {
            class: "KeyboardButton",
            style: format_args!("{} {}", KEYBOARDBUTTONSTYLE, bg_colour),
            onclick: move |_| {
                if guessed_word.get().len() < 6 {
                    guessed_word.to_owned().make_mut().push(letter);
                }
            },
            "{letter}"
        }
    ))
}

#[derive(Props, PartialEq)]
pub struct KeyboardProps<'a> {
    pub set_guessed_words: &'a UseState<Vec<String>>,
    pub set_guessed_letters: &'a UseState<LetterStateVec>,
    pub set_current_guessed_word: &'a UseState<Vec<char>>,
    pub current_guessed_word: &'a[char],
    pub hidden_word: &'static str,
}

pub fn Keyboard<'a>(cx: Scope<'a, KeyboardProps<'a>>) -> Element<'a> {
    let set_guessed_words = cx.props.set_guessed_words;
    let set_guessed_letters = cx.props.set_guessed_letters;
    let set_current_guessed_word = cx.props.set_current_guessed_word;
    let hidden_word = cx.props.hidden_word;
    let current_guessed_word = cx.props.current_guessed_word;

    cx.render(rsx! (
        KeyboardInner {
            KeyboardRow {
                {
                    "qwertyuiop".chars().map(|letter|
                        rsx! (
                            KeyboardButton {
                                letter: letter,
                                guessed_letters: set_guessed_letters.get().as_ref(),
                                guessed_word: set_current_guessed_word
                            }
                        )
                    )
                },
            },
            KeyboardRow {
                {
                    "asdfghjkl".chars().map(|letter|
                        rsx! (
                            KeyboardButton {
                                letter: letter,
                                guessed_letters: set_guessed_letters.get().as_ref(),
                                guessed_word: set_current_guessed_word
                            }
                        )
                    )
                }
            },
            KeyboardRow {
                div {
                    class: "KeyboardButton",
                    style: format_args!("{} {}", KEYBOARDBUTTONSTYLE, LetterState::None.to_colour()),
                    onclick: |_| {set_current_guessed_word.make_mut().pop();},
                    "Delete"
                },
                {
                    "zxcvbnm".chars().map(|letter|
                        rsx! (
                            KeyboardButton {
                                letter: letter,
                                guessed_letters: set_guessed_letters.get().as_ref(),
                                guessed_word: set_current_guessed_word
                            }
                        )
                    )
                },
                div {
                    class: "KeyboardButton",
                    style: format_args!("{} {}", KEYBOARDBUTTONSTYLE, LetterState::None.to_colour()),
                    onclick: move |_| {
                        set_guessed_words.to_owned().make_mut().push(current_guessed_word.iter().collect());
                        let letters = current_guessed_word.iter().enumerate().map(|(i, letter)| {
                            let letter_index = hidden_word.chars().position(|c| &c == letter);

                            let letter_state = match letter_index {
                                Some(index) => {
                                    match index == i {
                                        true => LetterState::Correct,
                                        false => LetterState::InWord,
                                    }
                                },
                                None => LetterState::Guessed
                            };
                            (*letter, letter_state)
                        }).collect::<LetterStateVec>();

                        set_guessed_letters.make_mut().extend(letters);
                        set_current_guessed_word.make_mut().clear()
                    },
                    "enter"
                }
            },
        }
    ))
}
