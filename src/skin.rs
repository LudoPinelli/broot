use std::io::{self, Write};

use crate::screens::{Screen, ScreenArea};

use std::collections::HashMap;
use termion::color::{self, *};

pub struct SkinEntry {
    pub fg: String,
    pub bg: String,
    //pub fb String,
}

impl SkinEntry {
    pub fn apply(&self, screen: &mut Screen) -> io::Result<()> {
        write!(screen.stdout, "{}{}", self.fg, self.bg)
    }
    pub fn fgbg(&self) -> String {
        format!("{}{}", self.fg, self.bg)
    }
}

macro_rules! Skin {
    (
        $($name:ident: $fg:expr, $bg:expr)*
    ) => {
        pub struct Skin {
            $(pub $name: SkinEntry,)*
            pub reset: SkinEntry,
        }
        impl Skin {
            pub fn create(mut skin_conf: HashMap<String, String>) -> Skin {
                Skin {
                    $($name: SkinEntry {
                        fg: skin_conf.remove(&format!("{}_fg", stringify!($name))).unwrap_or(
                            format!("{}", color::Fg($fg)).to_string()
                        ),
                        bg: skin_conf.remove(&format!("{}_bg", stringify!($name))).unwrap_or(
                            format!("{}", color::Bg($bg)).to_string()
                        ),
                    },)*
                    reset: SkinEntry {
                        fg: format!("{}", color::Fg(color::Reset)).to_string(),
                        bg: format!("{}", color::Bg(color::Reset)).to_string(),
                    },
                }
            }
        }
    }
}

Skin! {
    status_normal: White, AnsiValue::grayscale(2)
    status_error: Red, AnsiValue::grayscale(2)
    tree: AnsiValue::grayscale(5), Reset
}

