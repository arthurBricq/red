use crate::cursor::Cursor;

/// A selection is a tuple from an end to a start
#[derive(Clone, Copy, Debug)]
pub struct Selection {
    start: Cursor,
    end: Cursor
}

impl Selection {
    pub fn new(start: Cursor, end: Cursor) -> Self {
        Self {start, end}
    }

    /// Returns true if the selection contains the given line
    pub fn contains_line(&self, line_number: usize) -> bool {
        self.start.y <= line_number && self.end.y >= line_number
    }

    /// Sets the new end for this selection
    /// If the provided end is before the start, the two values are swap
    pub fn set_new_end(&mut self, pos: Cursor) {
        self.end = pos;
    }

    pub fn start(&self) -> &Cursor {
        if self.start < self.end {
            &self.start
        } else {
            &self.end
        }
    }

    pub fn end(&self) -> &Cursor {
        if self.start < self.end {
            &self.end
        } else {
            &self.start
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::selection::*;
    #[test]
    fn test_selection() {

        let s = Selection {
            start: Cursor { x: 0, y: 0 },
            end: Cursor { x: 0, y: 0 }
        };
        //s.set_new_end(Cursor { x: 1, y: 0 });
        //assert_eq!(s.end.x, 1);

        let mut option = Some(s);
        option.as_mut().unwrap().set_new_end(Cursor { x: 1, y: 0 });
        assert_eq!(option.unwrap().end.x, 1);
    }

    #[test]
    fn test_backward_selection() {
        let mut s = Selection {
            start: Cursor { x: 10, y: 0 },
            end: Cursor { x: 11, y: 0 }
        };

        s.set_new_end(Cursor { x: 9, y: 0 });
        s.set_new_end(Cursor { x: 8, y: 0 });
        s.set_new_end(Cursor { x: 7, y: 0 });

        assert_eq!(s.start().x, 7);
        assert_eq!(s.end().x, 10);
    }

}

