// This is NOT how I render, but I have similar grids

use crate::*;
use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    char:  char,
    style: Style,
}

#[derive(Clone, Default, Debug)]
pub struct Screen {
    width:      usize,
    height:     usize,
    background: Background,
    cells:      Vec<Vec<Cell>>,
}

impl Screen {
    pub fn new(width: usize, height: usize, background: Background) -> Self {
        let mut cells = Vec::new();

        cells.resize_with(height, || {
            let mut line = Vec::new();
            line.resize(width, Cell::default());
            line
        });

        Self {
            width,
            height,
            background,
            cells,
        }
    }

    pub fn text(&mut self, text: &str) {
        let text = to_text(text);
        let wrapped = wrap_optimal_fit(&text, |_| self.width);

        self.render(wrapped);
    }

    fn render(&mut self, wrapped: Vec<&[StyledWord]>) {
        let mut y = 0;
        for line in wrapped {
            if y == self.height {
                break;
            }

            let mut x = 0;
            for word in line {
                let style = word.style;

                for char in word.string.chars() {
                    if x == self.width {
                        break;
                    }

                    self.cells[y][x] = Cell { char, style };
                    x += 1;
                }
            }

            y += 1;
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            char:  ' ',
            style: Style::default(),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}", self.style, self.char)
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for line in &self.cells {
            write!(f, "{}", self.background)?;
            for cell in line {
                write!(f, "{}", cell)?;
            }
            write!(f, "\x1B[39m\x1B[49m{}\n", Weight::Normal)?;
        }

        Ok(())
    }
}

// Lame and buggy HTML "macro"
pub fn to_text(mut s: &str) -> Vec<StyledWord> {
    let mut text = Vec::new();
    let mut word = StyledWord::default();
    let mut new = false;

    s = s.trim();
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        match c {
            '\n' => {} // panic!("No idea what to do with line breaks"),
            ' ' => {
                word.white += 1;
                new = true;
            }
            '<' => {
                text.push(word);
                word = StyledWord::default();
                word.string.push('<');

                while let Some(c) = chars.next() {
                    if c != '>' {
                        word.string.push(c);
                    } else {
                        word.string.push('>');
                        break;
                    }
                }
                new = true;
            }
            c => {
                if new == true {
                    text.push(word);
                    word = StyledWord::default();
                }

                word.string.push(c);
                new = false;
            }
        }
    }

    if word.string.len() != 0 {
        text.push(word);
    }

    let mut styles = vec![Style::default()];

    for word in &mut text {
        let style = *styles.get(styles.len() - 1).unwrap();

        if is_opening_tag(&word.string) {
            styles.push(open(&word.string, style));

            word.string = "__REMOVE__".into();
        } else if is_closing_tag(&word.string) {
            styles.pop();
            word.string = "__REMOVE__".into();
        } else {
            word.style = style;
        }
    }

    text.into_iter()
        .filter(|word| word.string != "__REMOVE__")
        .collect()
}

fn is_opening_tag(s: &str) -> bool {
    s == "<black>"
        || s == "<white>"
        || s == "<red>"
        || s == "<green>"
        || s == "<blue>"
        || s == "<bold>"
        || s == "<normal>"
}

fn is_closing_tag(s: &str) -> bool {
    s == "</black>"
        || s == "</white>"
        || s == "</red>"
        || s == "</green>"
        || s == "</blue>"
        || s == "</bold>"
        || s == "</normal>"
}

fn open(tag: &str, mut style: Style) -> Style {
    if tag == "<white>" {
        style.foreground = Foreground(255, 255, 255);
    } else if tag == "<black>" {
        style.foreground = Foreground(0, 0, 0);
    } else if tag == "<red>" {
        style.foreground = Foreground(255, 0, 0);
    } else if tag == "<green>" {
        style.foreground = Foreground(0, 255, 0);
    } else if tag == "<blue>" {
        style.foreground = Foreground(0, 0, 255);
    } else if tag == "<bold>" {
        style.weight = Weight::Bold;
    } else if tag == "<normal>" {
        style.weight = Weight::Normal;
    }

    style
}
