use rand::seq::SliceRandom;

const WORDS: &str = include_str!("words.txt");

pub fn get_random_word(length: usize) -> &'static str {
    WORDS
        .split('\n')
        .filter(|&word| word.len() == length)
        .collect::<Vec<&str>>()
        .choose(&mut rand::thread_rng())
        .unwrap()
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum LetterState {
    None,
    InWord,
    Correct,
    Guessed,
}

impl LetterState {
    pub fn to_colour(&self) -> &str {
        match self {
            LetterState::None => "#656780",
            LetterState::Correct => "#79b851",
            LetterState::InWord => "#f3c237",
            LetterState::Guessed => "#3d4054"
        }
    }
}

impl<'a> Default for &'a LetterState {
    fn default() -> Self {
        &LetterState::None
    }
}

pub type LetterStateVec = Vec<(char, LetterState)>;

#[macro_export]
macro_rules! styled {
    ($name:ident, $element:ident, $style:expr) => {
        #[inline_props]
        pub fn $name<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
            cx.render(rsx! {
                $element {
                    class: format_args!("{}", stringify!($name)),
                    style: $style,
                    children
                }
            })
        }
    }
}

#[macro_export]
macro_rules! println {
    ($string:expr, $( $arg:expr ),+) => {{
        use web_sys::console::log_1;
        log_1(&format!("{}\n", format!($string, $( $arg ),*)).into())
    }};
    ($string:expr) => {{
        use web_sys::console::log_1;
        log_1(&format!("{}\n", $string).into())
    }}
}
