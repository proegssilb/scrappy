use core::ops::Add;

// TODO: Split this up into [Command, UiCommand], or keep it together?
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    command_id: String,
    command_description: String,
    menu_index: i32,
}

// Borrowed from the fltk crate.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyModifier {
    None = 0,
    Shift = 0x00010000,
    CapsLock = 0x00020000,
    Ctrl = 0x00040000,
    Alt = 0x00080000,
}

pub struct Keystroke(i32);

impl Keystroke {
    pub fn from_char(c: char) -> Keystroke {
        KeyModifier::None + c
    }
}

impl Add<KeyModifier> for KeyModifier {
    type Output = Keystroke;

    fn add(self, other: KeyModifier) -> Self::Output {
        Keystroke(self as i32 | other as i32)
    }
}

impl Add<KeyModifier> for Keystroke {
    type Output = Keystroke;

    fn add(self, other: KeyModifier) -> Self::Output {
        Keystroke(self.0 | other as i32)
    }
}

impl Add<char> for KeyModifier {
    type Output = Keystroke;

    fn add(self, other: char) -> Self::Output {
        if other.is_ascii() && (other.is_ascii_graphic() || other.is_ascii_whitespace()) {
            let mut buff = [0];
            other.encode_utf8(&mut buff);
            Keystroke(KeyModifier::None as i32 + i32::from(buff[0]))
        } else {
            Keystroke(self as i32)
        }
    }
}

impl Add<char> for Keystroke {
    type Output = Keystroke;

    fn add(self, other: char) -> Self::Output {
        if other.is_ascii() && (other.is_ascii_graphic() || other.is_ascii_whitespace()) {
            let mut buff = [0];
            other.encode_utf8(&mut buff);
            Keystroke(KeyModifier::None as i32 + i32::from(buff[0]))
        } else {
            self
        }
    }
}