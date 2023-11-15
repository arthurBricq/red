use crate::editor_model::EditorModel;

/// A 2D point
/// Represents the position
pub struct Position(usize, usize);

impl Position {
    pub fn x(&self) -> usize {
        self.0
    }
    pub fn y(&self) -> usize {
        self.1
    }
}

#[derive(Clone, Debug)]
/// Defines a motion
pub enum Motion {
    Words {
        n_words: i32,
    },
    /// move forward until the next occurrence of the provided character is found on the same line
    Forward { ch: u32 },
    /// move backward until the next occurrence of the provided character is found on the same line
    Backward { ch: u32 },
}

impl Motion {
    /// Returns the start and end position of the given selection
    ///
    /// Implementation problem
    /// - this function uses an assumption on the model : it only works with the current
    ///   representation of a file
    pub fn apply(&self, model: &EditorModel) -> (Position, Position) {
        use Motion::*;
        let lines = model.get_lines();
        let cursor = model.get_cursor();

        match *self {
            Forward { ch } => {
                let ch = char::from_u32(ch).unwrap();
                let mut x_pos = cursor.x + 1;
                let line = &lines[cursor.y];
                let chars: Vec<char> = line.chars().collect();
                while x_pos < line.len() {
                    if ch == chars[x_pos] {
                        return (Position(cursor.x, cursor.y), Position(x_pos, cursor.y));
                    }
                    x_pos += 1;
                }

                // If we reach this, it means there was no match 
                return (Position(cursor.x, cursor.y), Position(cursor.x, cursor.y));
            }
            Backward { ch } => {
                let ch = char::from_u32(ch).unwrap();
                let mut x_pos = cursor.x - 1;
                let line = &lines[cursor.y];
                let chars: Vec<char> = line.chars().collect();
                while x_pos > 0 {
                    if ch == chars[x_pos] {
                        return (Position(cursor.x, cursor.y), Position(x_pos, cursor.y));
                    }
                    x_pos -= 1;
                }
                // If we reach this, it means there was no match 
                return (Position(cursor.x, cursor.y), Position(cursor.x, cursor.y));
            }
            Words { n_words } => {
                // Get the lines
                let n_lines = lines.len();
                let mut x_pos = cursor.x;
                let mut y_pos = cursor.y;

                let line = &lines[cursor.y];
                let chars: Vec<char> = line.chars().collect();

                let mut to_process = n_words.abs();

                // If you're going backward at the first character before you
                // is a separator, then you will have to skip it
                if n_words < 0 && x_pos > 0 && chars[x_pos - 1] == ' ' {
                    to_process += 1;
                }

                // Until there are words to process, keep progressing through the file
                while to_process > 0 {

                    // Necessary to update the array chars
                    let line = &lines[y_pos];
                    let chars: Vec<char> = line.chars().collect();

                    if n_words > 0 { // going forward
                        if chars.len() == 0 {
                            y_pos += 1;
                            to_process -= 1;
                        } else if x_pos < chars.len() - 1 {
                            x_pos += 1;
                            if chars[x_pos] == ' ' {
                                to_process -= 1;
                                x_pos += 1;
                            }
                        } else {
                            if y_pos < n_lines - 1 {
                                x_pos = 0;
                                y_pos += 1;
                            }
                            to_process -= 1;
                        }
                    } else { // going backward 
                        eprintln!("{x_pos}, {y_pos}, {chars:?}");
                        if x_pos > 0 {
                            x_pos -= 1;
                            if chars[x_pos] == ' ' || x_pos == 0 {
                                to_process -= 1;
                            }
                        } else if x_pos == 0 {
                            if y_pos > 0 {
                                // move one line up
                                y_pos -= 1;
                                if lines[y_pos].len() > 0 {
                                    x_pos = lines[y_pos].len() - 1;
                                } else {
                                    x_pos = 0;
                                }
                            } else {
                                to_process -= 1;
                            }
                        }
                    }
                }

                // If you are going backward, you have to add a character at the end
                if n_words < 0 && x_pos != 0 {
                    x_pos += 1;
                }

                return (Position(cursor.x, cursor.y), Position(x_pos, y_pos));
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn basic_char_iterator() {
        let line = "Hello world, this is my sentence".to_string();
        let x_pos = 13;
        let split = line.split_at(x_pos).0.split(' ');
        for c in split {
            println!("words = {c}");
        }

        //assert!(false);
    }
}
