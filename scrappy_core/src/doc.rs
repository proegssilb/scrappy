use std::sync::mpsc::Sender;

// TODO: Figure out how to better handle the return value of lines()
pub trait Document {
    type LinesIterator: Iterator<Item=String>;

    fn cursor_loc(self) -> Loc;
    fn lines(self) -> Self::LinesIterator;
    fn selection(self) -> Option<Selection>;
    fn is_modified(self) -> bool;

    fn edits_channel(self) -> Sender<DocumentEdit>;
}

#[derive(Clone, Debug)]
pub enum DocumentEdit {
    Close,
    InsertChar(Loc, char),
    InsertStr(Loc, String),
    Delete(Selection),
    MoveCursor(Direction),
    MoveCursorTo(Loc),
    SetSelectionMode(SelectionMode),
    AddListener(Sender<DocumentEvent>),
    Save,
    SaveAs(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Selection {
    pub start: Loc,
    pub end: Loc,
    pub text: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SelectionMode {
    None,
    Selecting,
    Selected
}

#[derive(Clone, Debug)]
pub enum DocumentEvent {
    LineChanged(String, String),
    CursorMoved(Loc, Loc, Option<Direction>)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Loc {
    pub line: usize,
    pub col: usize,
}
