use crate::editor_model::*;
use crate::editor_action::*;

pub struct InsertMode {}

impl EditorMode for InsertMode {
    fn key_tapped(&mut self, ch: u32) -> EditorAction {
        match char::from_u32(ch) {
            DOWN => EditorAction::MoveCursor { dx: 0, dy: -1 },
            UP => EditorAction::MoveCursor { dx: 0, dy: 1 },
            LEFT => EditorAction::MoveCursor { dx: -1, dy: 0 },
            RIGHT => EditorAction::MoveCursor { dx: 1, dy: 0 },
            ENTER => EditorAction::JumpLineAtCursor,
            BACKSPACE => EditorAction::DeleteCharAtCursor,
            ESCAPE => EditorAction::SwitchToNormalMode,
            _ => EditorAction::AddCharAtCursor { ch }
        }
    }

    fn get_description(&self) -> String {
        "Insert Mode".to_string()
    }

}
