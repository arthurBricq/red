use crate::motion::Motion;

/// Enum that holds a change to apply to the model
///
/// This allows the model to use different editor modes. Each mode is responsible for returning an
/// EditorAction.
///
/// Editor actions can be clone (using the .clone method) to be copied.
#[derive(Clone, Debug)]
pub enum EditorAction {
    /// Add a character at the current cursor position
    AddCharAtCursor {
        ch: u32,
    },
    /// Delete a character at the current cursor position
    DeleteCharAtCursor,
    /// Add a breakline at the current cursor position
    JumpLineAtCursor,
    /// Move the cursor to the beginning of the next line
    MoveCursorDown,
    /// Move the cursor with the desired impact
    MoveCursor {
        dx: i32,
        dy: i32,
    },
    /// Move by a desired amount of words. Can be negative
    MoveByWords {
        n_words: i32,
    },
    ApplyMotion {
        motion: Motion
    },
    // Switch between modes
    SwitchToInsertMode,
    SwitchToNormalMode,
    SwitchToCommandMode,
    /// Toggle the selection mode: ON or OFF
    ToggleSelectionState,
    /// Yank the current selection (if available)
    Yank,
    /// Put the current yanking
    Put,
    /// Called when the escape key is pressed
    AbortCurrentAction,
    /// Save the file
    Save,
    /// Exit the program
    Exit,
    /// A composite action contains a list of actions to execute
    CompositeAction {
        actions: Vec<Box<EditorAction>>
    },
    /// Jump the cursor to the provided line
    JumpToLine {
        line: usize
    },
    /// Undo action
    Undo,
    None,
}

impl EditorAction {
    /// Returns true if this action can be undone using the undo redo manager
    pub fn can_be_undo(&self) -> bool {
        match self {
            EditorAction::AddCharAtCursor { ch } => true,
            EditorAction::DeleteCharAtCursor => true,
            EditorAction::JumpLineAtCursor => true,
            EditorAction::MoveCursorDown => false,
            EditorAction::MoveCursor { dx, dy } => false,
            EditorAction::MoveByWords { n_words } => false,
            EditorAction::ApplyMotion { motion } => false,
            EditorAction::SwitchToInsertMode => false,
            EditorAction::SwitchToNormalMode => false,
            EditorAction::SwitchToCommandMode => false,
            EditorAction::ToggleSelectionState => false,
            EditorAction::Yank => false,
            EditorAction::Put => false,
            EditorAction::AbortCurrentAction => false,
            EditorAction::Save => false,
            EditorAction::Exit => false,
            EditorAction::CompositeAction { actions } => true,
            EditorAction::JumpToLine { line } => false,
            EditorAction::Undo => false,
            EditorAction::None => false,
        }        
    }

    pub fn undo_action(&self) -> EditorAction {
        match self {
            EditorAction::AddCharAtCursor { ch } => EditorAction::DeleteCharAtCursor,
           _ => EditorAction::None 
        }
    }
}
