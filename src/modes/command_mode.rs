use crate::editor_model::*;
use crate::editor_action::*;
use std::ops::Add;

pub struct CommandMode {
    /// In typing command, all the characters are collected in a buffer
    command: String,
}

impl CommandMode {
    fn enter_key_pressed(&mut self) -> EditorAction {
        // Match the final command
        match self.command.as_str() {
            "w" => {
                return EditorAction::CompositeAction {
                    actions: vec![
                        Box::new(EditorAction::Save),
                        Box::new(EditorAction::SwitchToNormalMode),
                    ],
                }
            }
            "q" => {
                return EditorAction::Exit
            }
            "wq" | "x" => {
                return EditorAction::CompositeAction {
                    actions: vec![
                        Box::new(EditorAction::Save),
                        Box::new(EditorAction::Exit),
                    ],
                }
            }
            _ => {
                eprintln!("Unknown command: {}", self.command)
            }
        }

        // Reset the command
        self.command = String::new();
        EditorAction::SwitchToNormalMode
    }

    fn backspace_key_pressed(&mut self) -> EditorAction {
        self.command.pop();
        EditorAction::None
    }

    fn escape_key_pressed(&mut self) -> EditorAction {
        EditorAction::None
    }
}

impl EditorMode for CommandMode {
    fn key_tapped(&mut self, ch: u32) -> EditorAction {
        match char::from_u32(ch) {
            ENTER => self.enter_key_pressed(),
            BACKSPACE => self.backspace_key_pressed(),
            ESCAPE => self.escape_key_pressed(),
            _ => {
                if let Some(c) = char::from_u32(ch) {
                    self.command.push(c);
                }
                EditorAction::None
            }
        }
    }
    fn get_description(&self) -> String {
        "Command: ".to_string().add(&self.command)
    }
}

impl CommandMode {
    pub fn new() -> Self {
        Self {
            command: String::new(),
        }
    }
}
