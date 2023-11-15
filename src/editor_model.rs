use crate::{
    modes::insert_mode::InsertMode,
    modes::{command_mode::CommandMode, normal_mode::NormalMode},
    motion::Motion,
    editor_action::EditorAction,
    screen::Screen,
    yanker::Yanker,
    cursor::Cursor,
    selection::Selection, undo_redo::UndoRedoManager,
};

use std::fs;
use std::process::exit;

use ncurses::{KEY_BACKSPACE, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP, endwin};

// It is required to recreate constants for the specials keys
// This makes it easier to parse them when received
pub const LEFT: Option<char> = char::from_u32(KEY_LEFT as u32);
pub const RIGHT: Option<char> = char::from_u32(KEY_RIGHT as u32);
pub const UP: Option<char> = char::from_u32(KEY_UP as u32);
pub const DOWN: Option<char> = char::from_u32(KEY_DOWN as u32);
pub const ENTER: Option<char> = char::from_u32(10 as u32);
pub const ESCAPE: Option<char> = char::from_u32(27 as u32);
pub const BACKSPACE: Option<char> = char::from_u32(KEY_BACKSPACE as u32);

/// A trait that defines an edition mode.
/// For instance, the insert or the normal mode.
pub trait EditorMode {
    fn key_tapped(&mut self, ch: u32) -> EditorAction;
    fn get_description(&self) -> String;
}

/// The editor model is the class which holds the text data and is in charge of all the editions.
pub struct EditorModel {
    filename: String,
    cursor: Cursor,
    screen: Screen,
    /// Contains the lines of the text
    lines: Vec<String>,
    /// The editor mode is in charge of parsing key tapped and returning editor actions
    editor_mode: Box<dyn EditorMode>,
    /// The yanker keeps track of what is yanked and what is put
    yanker: Yanker,
    /// The selection is a property of the model. By default it is none.
    selection: Option<Selection>,
    /// The undo redo manager is in charge of keeping track of the last actions
    undo_redo_mgr: UndoRedoManager,
}

impl EditorModel {
    #[cfg(test)]
    pub fn new() -> Self {

        let mut tmp: Vec<String> = Vec::new();
        tmp.push("Hello World".to_owned());
        tmp.push("This is another sentence".to_owned());
        Self {
            cursor: Cursor { x: 0, y: 0 },
            lines: tmp,
            editor_mode: Box::new(NormalMode::new()),
            screen: Screen { top: 0, h: 0, w: 0 },
            yanker: Yanker::new(),
            selection: None,
            filename: "new_file.txt".to_string(),
            undo_redo_mgr: UndoRedoManager::new()
        }
    }

    pub fn from_file(filename: String) -> Self {
        let lines = if let Ok(content) = fs::read_to_string(filename.clone()) {
            content.lines().map(|s| s.to_string()).collect()
        } else {
            let mut tmp = Vec::new();
            tmp.push("".to_string());
            tmp
        };
        Self {
            cursor: Cursor { x: 0, y: 0 },
            lines,
            editor_mode: Box::new(NormalMode::new()),
            screen: Screen { top: 0, h: 0, w: 0 },
            yanker: Yanker::new(),
            selection: None,
            filename,
            undo_redo_mgr: UndoRedoManager::new()
        }
    }

    pub fn save_file(&self) {
        let contents = self.lines.join("\n");
        fs::write(&self.filename, contents).map_err(|err| println!("{err:?}")).ok();
    }

    #[cfg(test)]
    pub fn set_text(&mut self, text: String) {
        self.lines = text.lines().map(|s| s.to_string()).collect();
    }

    #[cfg(test)]
    /// Sends the cursor at the top of the document
    pub fn reset_cursor(&mut self) {
        self.cursor.x = 0;
        self.cursor.y = 0;
    }

    #[cfg(test)]
    pub fn force_insert_mode(&mut self) {
        self.handle_editor_action(EditorAction::SwitchToInsertMode, false);
    }

    #[cfg(test)]
    pub fn force_normal_mode(&mut self) {
        self.handle_editor_action(EditorAction::SwitchToNormalMode, false);
    }

    pub fn set_screen_h(&mut self, h: i32) {
        self.screen.h = h;
    }

    pub fn set_screen_w(&mut self, w: i32) {
        self.screen.w = w;
    }

    pub fn get_screen_info(&self) -> &Screen {
        &self.screen
    }

    /// Returns the position of the cursor on the screen
    pub fn get_cursor(&self) -> &Cursor {
        &self.cursor
    }

    /// Returns the start and end position of the current selection
    pub fn get_selection(&self) -> Option<Selection> {
        self.selection
    }

    /// Returns the lines of the model
    pub fn get_lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn get_status_message(&self) -> String {
        [
            "   Press F1 to quit",
            self.editor_mode.get_description().as_str(),
            if self.selection.is_some() {
                "selecting"
            } else {
                ""
            },
        ]
        .join("  |  ")
    }

    // Part of the model that receives inputs.
    // This could be defined in a trait

    fn get_current_line_length(&self) -> usize {
        self.lines[self.cursor.y].len()
    }

    /// Changes self with the new provided cursor, while applying the logic for the selection
    fn set_cursor(&mut self, new_cursor_pos: Cursor) {
        self.cursor = new_cursor_pos;

        if self.selection.is_some() {
            self.selection.as_mut().unwrap().set_new_end(new_cursor_pos);
        }
    }

    fn left_arrow_tapped(&mut self) {
        if self.cursor.x > 0 {
            self.set_cursor(Cursor {
                x: self.cursor.x - 1,
                y: self.cursor.y,
            });
        }
    }

    fn right_arrow_tapped(&mut self) {
        if self.get_current_line_length() > 0 && self.cursor.x < self.get_current_line_length()
        {
            self.set_cursor(Cursor {
                x: self.cursor.x + 1,
                y: self.cursor.y,
            });
        }
    }

    /// If the x-cursor is beyonds the limits of the line, moves it to the length of the line
    fn fit_xcursor_to_line(&mut self, cursor: &mut Cursor) {
        let n = self.lines[cursor.y].len();
        if n > 0 && cursor.x > n - 1 {
            cursor.x = n - 1;
        } else if n == 0 {
            cursor.x = 0;
        }
    }

    fn top_arrow_tapped(&mut self) {
        if self.cursor.y > 0 {
            // 1. move the screen up
            if self.cursor.y == self.screen.top as usize {
                self.screen.top -= 1;
            }
            // 2. Update the cursor position
            let mut new_cursor = self.cursor;
            new_cursor.y -= 1;
            self.fit_xcursor_to_line(&mut new_cursor);
            self.set_cursor(new_cursor);
        }
    }

    fn bottom_arrow_tapped(&mut self) {
        // handle the screen
        if self.cursor.y < self.lines.len() - 1 {
            // 1. move the screen down
            if self.cursor.y == (self.screen.max_line() - 1) as usize {
                self.screen.top += 1;
            }
            // 2. Update the cursor position
            let mut new_cursor = self.cursor;
            new_cursor.y += 1;
            self.fit_xcursor_to_line(&mut new_cursor);
            self.set_cursor(new_cursor);
        }
    }

    /// Changes self according to what the given action asks for
    fn handle_editor_action(&mut self, action: EditorAction, is_undo: bool) {

        // Keep track of the action through the undo redo manager
        if !is_undo && action.can_be_undo() {
            let clone = action.clone();
            self.undo_redo_mgr.add_action(clone, self.cursor);
        }

        match action {
            EditorAction::AddCharAtCursor { ch } => self.add_character_at_cursor(ch),
            EditorAction::JumpLineAtCursor => self.add_new_line(),
            EditorAction::DeleteCharAtCursor => self.remove_character_at_cursor(),
            EditorAction::MoveCursorDown => {
                if self.cursor.y < self.lines.len() - 1 {
                    self.set_cursor(Cursor {
                        x: 0,
                        y: self.cursor.y + 1,
                    });
                }
            }
            EditorAction::MoveCursor { dx, dy } => match (dx, dy) {
                (1, 0) => self.right_arrow_tapped(),
                (-1, 0) => self.left_arrow_tapped(),
                (0, 1) => self.top_arrow_tapped(),
                (0, -1) => self.bottom_arrow_tapped(),
                _ => {}
            },
            EditorAction::MoveByWords { n_words } => {
                let selection = Motion::Words { n_words }.apply(self);
                self.set_cursor(Cursor {
                    x: selection.1.x(),
                    y: selection.1.y(),
                });
            }
            EditorAction::ApplyMotion { motion } => {
                let selection = motion.apply(self);
                self.set_cursor(Cursor {
                    x: selection.1.x(),
                    y: selection.1.y(),
                });
            }
            EditorAction::SwitchToInsertMode => {
                self.editor_mode = Box::new(InsertMode {});
            }
            EditorAction::SwitchToNormalMode => {
                self.editor_mode = Box::new(NormalMode::new());
            }
            EditorAction::SwitchToCommandMode => {
                self.editor_mode = Box::new(CommandMode::new());
            }
            EditorAction::Save => {
                self.save_file();
            }
            EditorAction::Exit => {
                endwin();
                exit(0);
            }
            EditorAction::CompositeAction { actions } => {
                for action in actions {
                    self.handle_editor_action(*action, is_undo)
                }
            }
            EditorAction::AbortCurrentAction => {
                if self.selection.is_some() {
                    self.selection = None;
                }
            }
            EditorAction::ToggleSelectionState => {
                if self.selection.is_none() {
                    self.selection = Some(Selection::new(self.cursor, self.cursor));
                } else {
                    self.selection = None;
                }
            }
            EditorAction::Yank => {
                if self.selection.is_some() {
                    let mut content = "".to_string();
                    let s = self.selection.unwrap().start().y;
                    let e = self.selection.unwrap().end().y;
                    for i in s..e+1 {
                        if i == s && i == e {
                            let x1 = self.selection.unwrap().start().x;
                            let x2 = self.selection.unwrap().end().x+1;
                            content.push_str(&self.lines[i][x1..x2]);
                        } else if i == s {
                            // Skip the beginning
                            let x = self.selection.unwrap().start().x;
                            let n = self.lines[i].len();
                            content.push_str(&self.lines[i][x..n]);
                        } else if i == e {
                            // Skip the end
                            let x = self.selection.unwrap().end().x+1;
                            content.push_str(&self.lines[i][0..x]);
                        } else {
                            // it means we have yanked the full lines
                            content.push_str(&self.lines[i]);
                        }
                    }
                    self.yanker.yank(content);
                    // Clear the current selection
                    self.selection = None;
                }
            }
            EditorAction::Put => {
                if let Some(content) = self.yanker.get_content() {
                    self.lines[self.cursor.y].insert_str(self.cursor.x, &content);
                }
            }
            EditorAction::JumpToLine { line } => {
                if line <= self.lines.len() {
                    self.cursor.x = 0;
                    self.cursor.y = line;
                }
            }
            EditorAction::Undo => {
                if let Some(to_undo) = self.undo_redo_mgr.undo() {
                    // Move to the cursor position
                    self.cursor = to_undo.1;
                    // Apply the action
                    self.handle_editor_action(to_undo.0, true);
                }
            }
            EditorAction::None => {}
        }

    }

    fn add_character_at_cursor(&mut self, ch: u32) {
        if let Some(ch) = std::char::from_u32(ch) {
            // Edit the model
            if let Some(line) = self.lines.get_mut(self.cursor.y) {
                // Note: O(n) operation to insert inside a string ...
                if self.cursor.x <= line.len() {
                    // safe guards
                    line.insert(self.cursor.x, ch);
                }
            }
            // Move the cursor one character to the right
            self.cursor.x += 1;
        }
    }

    fn remove_character_at_cursor(&mut self) {
        // If x is bigger than 0, it means we remove exactly 1 character
        if self.cursor.x > 0 {
            let line = &mut self.lines[self.cursor.y];
            if (self.cursor.x) <= line.len() {
                // Delete the current character
                line.remove(self.cursor.x - 1);
                self.cursor.x -= 1;
            }
        } else if self.cursor.x == 0 && self.cursor.y > 0 {
            // Delete the current line, append its content to the previous line
            let line = self.lines.remove(self.cursor.y);
            let previous_line = &mut self.lines[self.cursor.y - 1];
            let dx = previous_line.len();
            previous_line.push_str(line.as_str());
            self.cursor.y -= 1;
            self.cursor.x = dx;
        }
    }

    /// Called upon enter key pressed. Asks for adding a break line at current position
    fn add_new_line(&mut self) {
        // Get the part of the current line that is after the cursor
        let line = &self.lines[self.cursor.y];
        if self.cursor.x <= line.len() {
            let tmp = line.split_at(self.cursor.x);
            let old_line = tmp.0.to_string();
            let new_line = tmp.1.to_string();
            // Rewrite the old lines
            self.lines[self.cursor.y] = old_line;
            // Add the new line
            self.lines.insert(self.cursor.y + 1, new_line);
        } else {
            // Simply add a blank line
            self.lines.insert(self.cursor.y + 1, "".to_string());
        }
        // Move the cursor below
        self.bottom_arrow_tapped();
        // Enforce the cursor to zero
        self.cursor.x = 0;
    }

    pub fn key_tapped(&mut self, ch: u32) {
        let action = self.editor_mode.key_tapped(ch);
        self.handle_editor_action(action, false);
    }
}
