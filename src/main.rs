extern crate matrix_display;
use matrix_display::*;
pub mod tag;
use std::{thread, time};
use tag::Tag;

fn main() {
    let mut tag_game = Tag::new();

    println!("Running game of tag");

    loop {
        let format = Format::new(4, 4);
        let mut matrix = tag_game.get_display_matrix();
        let display = MatrixDisplay::new(&format, &mut matrix);

        display.print(&mut std::io::stdout(), &style::BordersStyle::Plain);

        tag_game.update();

        thread::sleep(time::Duration::from_millis(1000));
    }
}
