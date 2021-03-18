use std::{thread::sleep, time::Duration};
use styledtextwrap::*;

fn main() {
    let mut width = 20;

    while width > 0 {
        let mut screen = Screen::new(width, 10, Background(255, 255, 255));
        screen.text("Hello, cruel <red><bold>world</bold>!</red>");

        println!("{}", width);
        println!("{}", screen);
        width -= 1;

        sleep(Duration::from_millis(50));
    }
}
