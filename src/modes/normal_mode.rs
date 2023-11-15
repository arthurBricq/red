use crate::editor_model::*;
use crate::editor_action::*;

const DIGIT_BASELINE: u32 = 48;
const MAX_DIGIT: u32 = 10;

fn is_digit(ch: u32) -> bool {
    ch >= DIGIT_BASELINE && ch < DIGIT_BASELINE + MAX_DIGIT
}

/// A buffering mode is a way to wait for another key.
/// After pressing some keys, like 'r' or 'f', the editor waits for another key.
/// The buffering mode specifies which action is expected to happen once the
/// buffer is full.
#[derive(Clone, Copy)]
enum BufferingMode {
    Replace,
    Forward,
    Backward,
    /// When typing a number
    Number,
}

impl BufferingMode {
    /// Return the action created by pressing the provided key while in this buffering mode.
    ///
    /// The parameter `is_buffering` can be changed by this function if the buffering is finished.
    ///
    /// For instance, if you are in 'forward' mode (after pressing 'f'), the action is to apply a
    /// forward motion, and the buffering is finished.
    ///
    /// Otherwise, if you typed a number (go to line), as long as `G` is not pressed, you want to 
    /// keep buffering.
    fn get_action(&self, buffer: &Vec<u32>, is_buffering: &mut bool) -> EditorAction {
        // Get the last element of the buffer
        // This is the last key that was pressed
        let ch = buffer.last().unwrap();
        match char::from_u32(*ch) {
            ESCAPE | ENTER | BACKSPACE => EditorAction::None,
            _ => match *self {
                BufferingMode::Replace => {
                    *is_buffering = false;
                    EditorAction::CompositeAction {
                        actions: vec![
                            Box::new(EditorAction::MoveCursor { dx: 1, dy: 0 }),
                            Box::new(EditorAction::DeleteCharAtCursor),
                            Box::new(EditorAction::AddCharAtCursor { ch: *ch }),
                            Box::new(EditorAction::MoveCursor { dx: -1, dy: 0 }),
                        ],
                    }
                }
                BufferingMode::Forward => {
                    *is_buffering = false;
                    EditorAction::ApplyMotion {
                        motion: crate::motion::Motion::Forward { ch: *ch },
                    }
                }
                BufferingMode::Backward => {
                    *is_buffering = false;
                    EditorAction::ApplyMotion {
                        motion: crate::motion::Motion::Backward { ch: *ch },
                    }
                }
                BufferingMode::Number => {
                    if !is_digit(*ch) {
                        // If it is not a digit, the buffering is finished
                        *is_buffering = false;
                        // If 'G' is pressed, triggers a line move
                        if char::from_u32(*ch).unwrap() == 'G' {
                            // Compute the line number 
                            // Don't forget to remove the last element, which will be 'G'
                            let as_string = buffer.iter()
                                .map(|ch| char::from_u32(*ch).unwrap().to_string())
                                .collect::<Vec<String>>()
                                .split_last().unwrap().1.join("");

                            // Try to parse a number from this string
                            if let Ok(line_number) = usize::from_str_radix(&as_string, 10) {
                                return EditorAction::JumpToLine { line: line_number }
                            }
                        }
                    } 
                    EditorAction::None
                },
            },
        }
    }
}

pub struct NormalMode {
    is_buffering: bool,
    buffering_mode: Option<BufferingMode>,
    buffer: Vec<u32>,
}

impl EditorMode for NormalMode {
    fn key_tapped(&mut self, ch: u32) -> EditorAction {
        if self.is_buffering && self.buffering_mode.is_some() {
            // If we are buffering, send the next character to the buffering mode
            self.buffer.push(ch);
            return self.buffering_mode.unwrap().get_action(&self.buffer, &mut self.is_buffering);
        } else {
            // Otherwise, match the character with the expected action
            match char::from_u32(ch) {
                Some('j') | DOWN => EditorAction::MoveCursor { dx: 0, dy: -1 },
                Some('k') | UP => EditorAction::MoveCursor { dx: 0, dy: 1 },
                Some('h') | LEFT => EditorAction::MoveCursor { dx: -1, dy: 0 },
                Some('l') | RIGHT => EditorAction::MoveCursor { dx: 1, dy: 0 },
                BACKSPACE => EditorAction::MoveCursor { dx: -1, dy: 0 },
                ENTER => EditorAction::MoveCursorDown,
                ESCAPE => EditorAction::AbortCurrentAction,
                Some('i') => EditorAction::SwitchToInsertMode,
                Some('o') => EditorAction::CompositeAction {
                    actions: vec![
                        Box::new(EditorAction::MoveCursorDown),
                        Box::new(EditorAction::JumpLineAtCursor),
                        Box::new(EditorAction::MoveCursor { dx: 0, dy: 1 }),
                        Box::new(EditorAction::SwitchToInsertMode),
                    ],
                },
                Some('O') => EditorAction::CompositeAction {
                    actions: vec![
                        Box::new(EditorAction::MoveCursor { dx: 0, dy: 1 }),
                        Box::new(EditorAction::JumpLineAtCursor),
                        Box::new(EditorAction::SwitchToInsertMode),
                    ],
                },
                Some('w') => EditorAction::MoveByWords { n_words: 1 },
                Some('b') => EditorAction::MoveByWords { n_words: -1 },
                Some(':') => EditorAction::SwitchToCommandMode,
                Some('v') => EditorAction::ToggleSelectionState,
                Some('y') => EditorAction::Yank,
                Some('u') => EditorAction::Undo,
                Some('p') => EditorAction::Put,
                // x is like pressing backspace with a previous right arrow move
                Some('x') => EditorAction::CompositeAction {
                    actions: vec![
                        Box::new(EditorAction::MoveCursor { dx: 1, dy: 0 }),
                        Box::new(EditorAction::DeleteCharAtCursor),
                    ],
                },
                Some('r') => {
                    self.start_buffering(BufferingMode::Replace);
                    EditorAction::None
                }
                Some('f') => {
                    self.start_buffering(BufferingMode::Forward);
                    EditorAction::None
                }
                Some('F') => {
                    self.start_buffering(BufferingMode::Backward);
                    EditorAction::None
                }
                Some(';') => {
                    // re-apply the previous motion
                    if self.buffering_mode.is_some() && self.buffer.len() > 0 {
                        self.buffering_mode.unwrap().get_action(&self.buffer, &mut self.is_buffering)
                    } else {
                        EditorAction::None
                    }
                }
                Some('1') | Some('2') | Some('3') | Some('4') | Some('5') | Some('6')
                | Some('7') | Some('8') | Some('9') | Some('0') => {
                    self.start_buffering(BufferingMode::Number);
                    self.buffer.push(ch);
                    EditorAction::None
                },
                _ => EditorAction::None,
            }
        }
    }

    fn get_description(&self) -> String {
        if self.is_buffering {
            "Normal Mode (buffering)".to_string()
        } else {
            "Normal Mode".to_string()
        }
    }
}

impl NormalMode {
    pub fn new() -> Self {
        Self {
            buffering_mode: None,
            is_buffering: false,
            buffer: Vec::new(),
        }
    }

    /// Call this function when we must start a new buffering mode.
    ///
    /// For instance, after the 'f' key is pressed.
    fn start_buffering(&mut self, mode: BufferingMode) {
        self.buffer.clear();
        self.is_buffering = true;
        self.buffering_mode = Some(mode);
    }
}
