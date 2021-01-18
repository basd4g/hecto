mod row;
mod document;
mod editor;
mod terminal;
pub use row::Row;
use editor::Editor;
pub use document::Document;
pub use editor::Position;
pub use terminal::Terminal;

fn main() {
  Editor::default().run();
}
