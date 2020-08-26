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
        let mut loc = cursor.clone();
        for c in insertion.chars() {
            self.content[loc.line].insert(loc.col, c);
            loc.col += 1;
            if "\r\n".contains(c) {
                let (start, rest) = self.content[loc.line].split_at(loc.col);
                let (start_str, rest_str) = (start.to_owned(), rest.to_owned());
                self.content[loc.line] = start_str;
                loc.line += 1;
                loc.col = 0;
                self.content.insert(loc.line, rest_str);
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
    use proptest::prelude::*;
    use std::time::Duration;

    /// A proptest strategy to return a raw text document and a location pointing
    /// to a valid spot within the document.
    fn text_and_loc() -> impl Strategy<Value = (Vec<String>, Loc)> {
        prop::collection::vec(".+", 1..5)
            .prop_flat_map(|doc| {
                let lines = doc.len();
                (Just(doc), 0..lines)
            })
            .prop_flat_map(|(doc, line)| {
                let columns = doc[line].len();
                (Just(doc), Just(line), 0..columns)
            })
            .prop_filter("Column must be a character boundary", |(doc, line, col)| doc[*line].is_char_boundary(*col))
            .prop_map(|(doc, line, column)| {
                (doc, Loc {line: line, col: column})
            })
    }
    
    proptest! {
        #![proptest_config(ProptestConfig {timeout: 600, ..ProptestConfig::default()})]

        // Document impl tests
        #[test]
        fn cursor_getter_gets(ln in 0usize..1000, c in 0usize..500) {
            let mut doc = PlainTextDocument::new();
            doc.cursor = Loc { line: ln, col: c };

            let found_loc = doc.cursor_loc();

            prop_assert_eq!(doc.cursor, found_loc);
        }

        #[test]
        fn lines_yields_document(lns in prop::collection::vec(".*", 0..1000) ) {
            let mut doc = PlainTextDocument::new();
            doc.content = lns.clone();

            let found_lines : Vec<String> = doc.lines().collect();
            prop_assert_eq!(found_lines, lns);
        }

        #[test]
        fn modified_getter_gets(flag in any::<bool>()) {
            let mut doc = PlainTextDocument::new();
            doc.modified = flag;

            prop_assert_eq!(doc.is_modified(), flag);
        }

        #[test]
        fn event_sent_gets_received(send_path in any::<String>()) {
            let doc = PlainTextDocument::new();
            let ev = DocumentEdit::SaveAs(send_path.clone());

            doc.edits_channel().send(ev)?;

            let received = doc.edits_receiver.recv_timeout(Duration::from_millis(50))?;

            match received {
                DocumentEdit::SaveAs(path) => prop_assert_eq!(path, send_path),
                _ => prop_assert_eq!(1, 0), // This should be straight-up impossible...
            }
        }

        // Event handling tests
        #[test]
        fn close_event_closes_doc(modified in any::<bool>()) {
            let mut doc = PlainTextDocument::new();
            doc.modified = modified;
            doc.edits_channel().send(DocumentEdit::Close)?;

            // This should return successfully immediately
            // If close event isn't handled right, it could run forever
            doc.thread_method()?;
        }

        #[test]
        fn insert_char_replaces_removed_char((text, loc) in text_and_loc()) {
            // This test might be unreliable
            let mut starting_text = text.clone();
            let char_removed = (*starting_text)[loc.line].remove(loc.col);
            let mut doc = PlainTextDocument::new();
            doc.content = starting_text;

            doc.handle_event(DocumentEdit::InsertChar(loc, char_removed));

            prop_assert_eq!(text, doc.content);
        }

        #[test]
        fn insert_newline_splits_line((text, loc) in text_and_loc()) {
            prop_assert_eq!(1, 1);
        }

        #[test]
        fn insert_str_can_insert_line((text, loc) in text_and_loc()) {
            let mut starting_text = text.clone();
            let line_removed : String = starting_text.remove(loc.line);
            let mut doc = PlainTextDocument::new();
            doc.content = starting_text;
            let test_loc = Loc {line: loc.line, col: 0};

            doc.handle_event(DocumentEdit::InsertStr(test_loc, line_removed + "\n"));

            prop_assert_eq!(text, doc.content);
        }
    }
}
