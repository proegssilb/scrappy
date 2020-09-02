use scrappy_core::*;
use std::io::Result as IoResult;
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};

// TODO: Sort out field visibility.
#[derive(Debug)]
pub struct PlainTextDocument {
    content: Vec<String>,
    cursor: Loc,
    selection_start: Option<Loc>,
    modified: bool,
    file_name: String,
    inbound_edits: Sender<DocumentEdit>,
    edits_receiver: Receiver<DocumentEdit>,
    listeners: Vec<Sender<DocumentEvent>>,
}

impl PartialEq for PlainTextDocument {
    fn eq(&self, other: &Self) -> bool { 
        return self.content == other.content &&
            self.cursor == other.cursor &&
            self.selection_start == other.selection_start &&
            self.modified == other.modified &&
            self.file_name == other.file_name
    }
}

impl PlainTextDocument {
    pub fn new() -> PlainTextDocument {
        let (s, r) = channel::<DocumentEdit>();
        PlainTextDocument {
            content: Vec::new(),
            cursor: Loc { line: 0, col: 0 },
            selection_start: None,
            modified: false,
            file_name: "".to_string(),
            inbound_edits: s,
            edits_receiver: r,
            listeners: Vec::new(),
        }
    }

    pub fn thread_method(&mut self) -> Result<(), RecvError> {
        loop {
            let ev = self.edits_receiver.recv()?;
            if self.handle_event(ev) {
                continue;
            } else {
                break;
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, event: DocumentEdit) -> bool {
        match event {
            DocumentEdit::Close => false,
            DocumentEdit::AddListener(s) => {
                self.listeners.insert(0, s);
                true
            }
            DocumentEdit::Save => {
                // TODO: Handle possible errors.
                self.save();
                true
            }
            DocumentEdit::SaveAs(_new_path) => true,
            DocumentEdit::InsertChar(cursor, ch) => {
                self.insert_char(cursor, ch);
                true
            }
            DocumentEdit::InsertStr(cursor, s) => {
                self.insert_string(cursor, s);
                true
            }
            DocumentEdit::Delete(selection) => {
                self.delete(selection);
                true
            }
            DocumentEdit::MoveCursor(_dir) => true,
            DocumentEdit::MoveCursorTo(_loc) => true,
            DocumentEdit::SetSelectionMode(_mode) => true,
        }
    }

    fn save(&mut self) -> IoResult<()> {
        todo!()
    }

    fn insert_char(&mut self, cursor: Loc, ch: char) {
        self.content[cursor.line].insert(cursor.col, ch);
    }

    fn insert_string(&mut self, cursor: Loc, insertion: String) {
        if cursor.col == 0 {
            self.content.insert(cursor.line, insertion);
        } else {
            let extra = self.content[cursor.line].split_off(cursor.col);
            self.content[cursor.line].push_str(&insertion);
            if insertion.ends_with('\n') {
                self.content.insert(cursor.line + 1, extra);
            } else {
                self.content[cursor.line].push_str(&extra);
            }
        }
    }

    fn delete(&mut self, _sel: Selection) {
        todo!()
    }
}

impl<'a> Document for &'a PlainTextDocument {
    type LinesIterator = std::iter::Cloned<std::slice::Iter<'a, std::string::String>>;

    fn cursor_loc(self) -> Loc {
        self.cursor
    }

    fn lines(self) -> Self::LinesIterator {
        self.content.iter().cloned()
    }

    fn selection(self) -> Option<Selection> {
        self.selection_start.map(|loc| Selection {
            start: loc,
            end: self.cursor,
            text: "".to_string(),
        })
    }

    fn is_modified(self) -> bool {
        self.modified
    }

    fn edits_channel(self) -> Sender<DocumentEdit> {
        self.inbound_edits.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;
    use fake::{Fake, Faker};
    
    #[test]
    fn cursor_getter_gets() {
        let ln : usize = Faker.fake::<usize>();
        let c : usize = Faker.fake::<usize>();
        let mut doc = PlainTextDocument::new();
        doc.cursor = Loc { line: ln, col: c };

        let found_loc = doc.cursor_loc();

        assert_eq!(doc.cursor, found_loc);
    }

    #[test]
    fn lines_yields_document() {
        let lns = Faker.fake::<Vec<String>>();
        let mut doc = PlainTextDocument::new();
        doc.content = lns.clone();

        let found_lines : Vec<String> = doc.lines().collect();
        assert_eq!(found_lines, lns);
    }

    #[test]
    fn modified_getter_gets() {
        let mut doc = PlainTextDocument::new();
        assert_eq!(doc.is_modified(), false);

        doc.modified = true;
        assert_eq!(doc.is_modified(), true);

        doc.modified = false;
        assert_eq!(doc.is_modified(), false);
    }

    #[test]
    fn event_sent_gets_received() {
        let doc = PlainTextDocument::new();
        let send_path = Faker.fake::<String>();
        let ev = DocumentEdit::SaveAs(send_path.clone());

        let res = doc.edits_channel().send(ev);
        assert_eq!(res.is_ok(), true);

        let received_res = doc.edits_receiver.recv_timeout(Duration::from_millis(50));
        assert_eq!(received_res.is_ok(), true);

        let received = received_res.unwrap_or(DocumentEdit::Close);

        match received {
            DocumentEdit::SaveAs(path) => assert_eq!(path, send_path),
            _ => assert_eq!(1, 0), // This should be straight-up impossible...
        }
    }

    // Event handling tests
    fn text_and_loc() -> (Vec<String>, Loc) {
        let mut text = Faker.fake::<Vec<String>>();
        // for ln in text.iter_mut() {
        //     ln.push('\n');
        // }
        let l = (0..text.len()).fake::<usize>();
        let char_locs : Vec<(usize, char)> = text[l].char_indices().collect();
        let c = (0..text[l].chars().count()).fake::<usize>();
        return (text, Loc {line: l, col: char_locs[c].0});
    }

    #[test]
    fn close_event_closes_doc() {
        let mut doc = PlainTextDocument::new();
        let handled = doc.handle_event(DocumentEdit::Close);
        assert_eq!(handled, false);
    }

    #[test]
    fn insert_char_replaces_removed_char() {
        let (text, loc) = text_and_loc();

        let mut starting_text = text.clone();
        let char_removed = (*starting_text)[loc.line].remove(loc.col);
        let mut doc = PlainTextDocument::new();
        doc.content = starting_text;

        let handled = doc.handle_event(DocumentEdit::InsertChar(loc, char_removed));

        assert_eq!(text, doc.content);
        assert_eq!(handled, true);
    }

    #[test]
    fn insert_str_can_insert_line() {
        let (text, loc) = text_and_loc();

        let mut starting_text = text.clone();
        let line_removed : String = starting_text.remove(loc.line);
        let mut doc = PlainTextDocument::new();
        doc.content = starting_text;
        let test_loc = Loc {line: loc.line, col: 0};

        let handled = doc.handle_event(DocumentEdit::InsertStr(test_loc, line_removed));

        assert_eq!(text, doc.content);
        assert_eq!(handled, true);
    }

    #[test]
    fn insert_newline_splits_line() {
        let (text, loc) = text_and_loc();
        let inserted = Faker.fake::<String>() + "\n";
        let mut doc = PlainTextDocument::new();
        doc.content = text.clone();

        let handled = doc.handle_event(DocumentEdit::InsertStr(loc, inserted.clone()));

        let mut expected_text = text.clone();
        let split_line = expected_text[loc.line].split_at(loc.col);
        let first = split_line.0.to_string();
        let second = split_line.1.to_string();
        expected_text[loc.line] = first + &inserted;
        expected_text.insert(loc.line + 1, second);

        assert_eq!(expected_text, doc.content);
        assert_eq!(handled, true);
    }
}
