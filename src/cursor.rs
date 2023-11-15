/// Holds a the model of the cursor
/// A cursor position is within the model, i.e. it is not the cursor position on the screen.
/// To get the cursor position on the screen, you must use the screen class as well.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl PartialOrd for Cursor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.y < other.y {
            Some(std::cmp::Ordering::Less)
        } else if self.y == other.y {
            Some(self.x.cmp(&other.x))
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }
}
