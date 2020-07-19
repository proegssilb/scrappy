#![windows_subsystem = "windows"]

use fltk::app::*;

mod editorview;

use editorview::EditorView;

fn main() {
    let app = App::default();
    let mut view = EditorView::new();
    view.initialize();
    while app.wait().expect("Couldn't run editor!") {
        view.loop_step(app);
    }
}
