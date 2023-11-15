use std::collections::VecDeque;

use crate::{editor_action::EditorAction, cursor::Cursor};

/// An undo redo context is an element which can be undo / redo.
/// It consists of the editor action, and some additional data.
#[derive(Debug)]
struct UndoRedoContext {
    action: EditorAction,
    cursor: Cursor
}


/// The undo redo manager is the class in charge of the 'u' and 'C-r' commands.
pub struct UndoRedoManager {
    /// The buffer keeps track of the actions
    buffer: VecDeque<UndoRedoContext>
}


impl UndoRedoManager {
    pub fn new() -> Self {
        Self { buffer: VecDeque::new() }
    }

    /// Add an action in the manager, so that it can be undone later on.
    pub fn add_action(&mut self, action: EditorAction, mut cursor: Cursor) {
        cursor.x += 1;
        self.buffer.push_back(UndoRedoContext { action, cursor });
        eprintln!("buffer = {:?}", self.buffer);
    }

    /// Returns the action to undo at the provided position
    pub fn undo(&mut self) -> Option<(EditorAction, Cursor)> {
        if let Some(to_redo) = self.buffer.pop_back() {
            eprintln!("Action to cancel: {:#?}", to_redo.action);
            Some(
                (to_redo.action.undo_action(), to_redo.cursor)
            )
        } else {
            None
        }
    }
}

