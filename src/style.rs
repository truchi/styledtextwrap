// More convoluted in real life, does not matter here

use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Default, Debug)]
pub struct Foreground(pub u8, pub u8, pub u8);

#[derive(Copy, Clone, Default, Debug)]
pub struct Background(pub u8, pub u8, pub u8);

#[derive(Copy, Clone, Debug)]
pub enum Weight {
    Bold,
    Normal,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Style {
    pub foreground: Foreground,
    pub weight:     Weight,
}

impl Default for Weight {
    fn default() -> Self {
        Weight::Normal
    }
}

impl Display for Foreground {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\x1B[38;2;{};{};{}m", self.0, self.1, self.2)
    }
}

impl Display for Background {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\x1B[48;2;{};{};{}m", self.0, self.1, self.2)
    }
}

impl Display for Weight {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Weight::Bold => write!(f, "\x1B[1m"),
            Weight::Normal => write!(f, "\x1B[22m"),
        }
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}", self.foreground, self.weight)
    }
}
