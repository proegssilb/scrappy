use std::io::Result as IoResult;
use std::rc::Rc;
use crate::doc::Document;

pub trait DocumentManager<'a> {
    fn supports_multidoc() -> bool;
    fn get_open_documents(self) -> Vec<&'a str>;
    fn get_document(self, doc_path: &str) -> Rc<dyn Document<LinesIterator = dyn Iterator<Item=String>>>;
    fn add_document(self, doc: Rc<dyn Document<LinesIterator = dyn Iterator<Item=String>>>);
    fn close_document(self, doc: Rc<dyn Document<LinesIterator = dyn Iterator<Item=String>>>, do_save: bool) -> IoResult<bool>;
    fn open_document(self, path: &str) -> Rc<dyn Document<LinesIterator = dyn Iterator<Item=String>>>;
    fn new_document(self) -> Rc<dyn Document<LinesIterator = dyn Iterator<Item=String>>>;
}