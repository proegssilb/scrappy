use fltk::{
    app::*,
    button::*,
    dialog::*,
    input::*,
    menu::*,
    text::{TextBuffer, TextEditor},
    window::Window,
};
use std::{fs, path};

#[derive(Copy, Clone)]
pub enum Message {
    Changed,
    New,
    Open,
    Save,
    SaveAs,
    Quit,
    Cut,
    Copy,
    Paste,
    About,
}

pub struct SearchReplaceDialog {
    dialog: Window,
    find_text: Input,
    replace_text: Input,
    find_next: Button,
    replace: Button,
    replace_all: Button,
    cancel: Button,
}

pub struct EditorView {
    menu_bar: MenuBar,
    event_sender: Sender<Message>,
    event_receiver: Receiver<Message>,
    window: Window,
    editor: TextEditor,
    saved: bool,
    filename: String,
}

impl EditorView {
    pub fn new() -> EditorView {
        let (s, r) = channel::<Message>();
        let wind = Window::default()
            .with_size(800, 600)
            .center_screen()
            .with_label("Scrappy");
        let mut editor = TextEditor::new(5, 40, wind.width() - 10, wind.height() - 45, "");
        editor.set_buffer(Some(TextBuffer::default()));
        EditorView {
            menu_bar: MenuBar::new(0, 0, wind.width(), 40, ""),
            event_sender: s,
            event_receiver: r,
            window: wind,
            editor: editor,
            saved: false,
            filename: String::from(""),
        }
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn set_filename(&mut self, name: &str) {
        self.filename = String::from(name);
    }

    pub fn initialize(&mut self) {
        self.initialize_menu();
        self.initialize_editor();
        self.window.make_resizable(true);
        self.window.end();
        self.window.show();

        let s = self.event_sender.clone();

        self.window.set_callback(Box::new(move || {
            if event() == Event::Close {
                s.send(Message::Quit);
            }
        }));
    }

    pub fn loop_step(&mut self, app: App) {
        use Message::*;
        match self.event_receiver.recv() {
            Some(msg) => match msg {
                Changed => self.saved = false,
                New => {
                    if self.editor.buffer().unwrap().text() != "" {
                        let x = choice(200, 200, "File unsaved, Do you wish to continue?", "Yes", "No!", "");
                        if x == 0 {
                            self.editor.buffer().unwrap().set_text("");
                        }
                    }
                },
                Open => {
                    let mut dlg = FileDialog::new(FileDialogType::BrowseFile);
                    dlg.set_option(FileDialogOptions::NoOptions);
                    dlg.set_filter("*.txt");
                    dlg.show();
                    self.set_filename(&dlg.filename().to_string_lossy().to_string());
                    let filename = self.filename.clone();
                    if filename.is_empty() {
                        return;
                    }
                    match path::Path::new(&self.filename()).exists() {
                        true => self.editor.buffer().unwrap().set_text(
                            fs::read_to_string(&self.filename())
                                .unwrap()
                                .as_str(),
                        ),
                        false => alert(200, 200, "File does not exist!"),
                    }
                },
                Save => self.save_file(),
                SaveAs => {
                    self.saved = false;
                    self.save_file()
                },
                Quit => {
                    if !self.saved {
                        let x = choice(200, 200, "Would you like to save your work?", "Yes", "No", "");
                        if x == 0 {
                            self.save_file();
                            app.quit();
                        } else {
                            app.quit();
                        }
                    } else {
                        app.quit();
                    }
                },
                Cut => self.editor.cut(),
                Copy => self.editor.copy(),
                Paste => self.editor.paste(),
                About => message(200, 200, "Scrappy is a small, crappy text editor. It includes large swathes of code from https://github.com/MoAlyousef/fltk-rs/blob/master/examples/editor.rs.",),
            },
            _ => ()
        }
    }

    pub fn save_file(&mut self) {
        let mut filename = self.filename.clone();
        if self.saved {
            if filename.is_empty() {
                let mut dlg = FileDialog::new(FileDialogType::BrowseSaveFile);
                dlg.set_option(FileDialogOptions::SaveAsConfirm);
                dlg.show();
                filename = dlg.filename().to_string_lossy().to_string();
                if filename.is_empty() {
                    return;
                }
                match path::Path::new(&filename).exists() {
                    true => {
                        fs::write(&filename, self.editor.buffer().unwrap().text()).unwrap();
                        self.saved = true;
                    }
                    false => alert(200, 200, "Please specify a file!"),
                }
            } else {
                match path::Path::new(&filename).exists() {
                    true => {
                        fs::write(&filename, self.editor.buffer().unwrap().text()).unwrap();
                        self.saved = true;
                    }
                    false => alert(200, 200, "Please specify a file!"),
                }
            }
        } else {
            let mut dlg = FileDialog::new(FileDialogType::BrowseSaveFile);
            dlg.set_option(FileDialogOptions::SaveAsConfirm);
            dlg.show();
            filename = dlg.filename().to_string_lossy().to_string();
            if filename.is_empty() {
                return;
            }
            fs::write(&filename, self.editor.buffer().unwrap().text()).unwrap();
            self.saved = true;
        }
    }

    fn initialize_menu(&mut self) {
        self.menu_bar.add_emit(
            "File/New...",
            Shortcut::Ctrl | 'n',
            MenuFlag::Normal,
            self.event_sender,
            Message::New,
        );

        self.menu_bar.add_emit(
            "File/Open...",
            Shortcut::Ctrl | 'o',
            MenuFlag::Normal,
            self.event_sender,
            Message::Open,
        );

        self.menu_bar.add_emit(
            "File/Save",
            Shortcut::Ctrl | 's',
            MenuFlag::Normal,
            self.event_sender,
            Message::Save,
        );

        self.menu_bar.add_emit(
            "File/Save as...",
            Shortcut::None,
            MenuFlag::MenuDivider,
            self.event_sender,
            Message::SaveAs,
        );

        self.menu_bar.add_emit(
            "File/Quit",
            Shortcut::None,
            MenuFlag::Normal,
            self.event_sender,
            Message::Quit,
        );

        self.menu_bar.add_emit(
            "Edit/Cut",
            Shortcut::Ctrl | 'x',
            MenuFlag::Normal,
            self.event_sender,
            Message::Cut,
        );

        self.menu_bar.add_emit(
            "Edit/Copy",
            Shortcut::Ctrl | 'c',
            MenuFlag::Normal,
            self.event_sender,
            Message::Copy,
        );

        self.menu_bar.add_emit(
            "Edit/Paste",
            Shortcut::Ctrl | 'v',
            MenuFlag::Normal,
            self.event_sender,
            Message::Paste,
        );

        self.menu_bar.add_emit(
            "Help/About",
            Shortcut::None,
            MenuFlag::Normal,
            self.event_sender,
            Message::About,
        );
    }

    fn initialize_editor(&mut self) {
        self.editor.set_text_font(Font::Courier);
        self.editor.set_linenumber_width(18);
        self.editor
            .set_linenumber_fgcolor(Color::from_u32(0x8b8386));
        self.editor.set_trigger(CallbackTrigger::Changed);
    }
}
