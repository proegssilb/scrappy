use std::io::Result as IoResult;
use std::rc::Rc;
use std::sync::mpsc::Sender;

pub trait Document {
    fn save(self) -> IoResult<bool>;
    fn save_as(self, path: &str) -> IoResult<bool>;

    fn cursor_loc(self) -> Loc;
    fn text(self) -> Rc<str>;

    fn edits_channel(self) -> Sender<DocumentEdits>;
    fn register_listener(self, channel: Sender<DocumentEvents>);
}

pub enum DocumentEdits {
    InsertChar(Loc, char),
    InsertStr(Loc, String),
    MoveCursor(Direction),
    MoveCursotTo(Loc),
}

pub enum DocumentEvents {
    LineChanged(String, String),
    CursorMoved(Loc, Loc, Option<Direction>)
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Loc {
    pub line: u32,
    pub col: u32,
}