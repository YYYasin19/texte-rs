mod editor;
mod terminal;
mod file;
mod row;

use editor::Editor;
pub use editor::Position;
pub use terminal::Terminal;
pub use file::File;
pub use row::Row;


fn main() {
    let mut editor = Editor::default();
    editor.run();
}
