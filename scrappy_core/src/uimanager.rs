use crate::commands::{Command, Keystroke};
use std::sync::mpsc::Sender;

pub trait UiManager {
    fn register_command<T: 'static + Copy + Send + Sync>(description: str, default_shortcut: Keystroke, menu_path: str, channel: Sender<T>, msg: T) -> Command;
}