pub mod tag;
use tag::Tag;

fn main() {
    let mut tag_game = Tag::new();

    println!("Running game of tag");

    loop {
        tag_game.update();
    }
}
