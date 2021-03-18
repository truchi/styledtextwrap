use crate::*;

#[derive(Clone, Default, Debug)]
pub struct StyledWord {
    pub style:  Style,
    pub string: String,
    pub white:  usize,
}

impl Fragment for StyledWord {
    fn width(&self) -> usize {
        // Lets say we live in a perfect world for now
        self.string.chars().count()
    }

    fn whitespace_width(&self) -> usize {
        self.white
    }

    fn penalty_width(&self) -> usize {
        // Not sure I understand this yet
        0
    }
}
