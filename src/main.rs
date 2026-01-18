#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

fn main() {
    print!("\x1b[2J");
    Editor::default().run();
}
