#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod document;
mod row;


pub use document::Document;
use editor::Editor;
pub use terminal::Terminal;
pub use row::Row;
pub use editor::Position;

fn main() {

    Editor::defualt().run();
    
}
