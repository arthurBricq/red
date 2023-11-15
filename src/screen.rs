use std::ops::Range;

/// Helper class to hold information about the usable screen
#[derive(Debug)]
pub struct Screen {
    /// Index of the top of the screen
    pub top: i32,
    /// Maximum size (in displayable characters)
    pub h: i32,
    pub w: i32
}

impl Screen {
    pub fn max_line(&self) -> i32 {
        self.h + self.top
    }

    /// Returns the indices where to split the provided line so that it fits on &self
    pub fn split_line(&self, line: &String) -> Vec<Range<usize>> {
        let n = line.len();
        let mut idx = Vec::new();

        let mut i: usize = 0;
        while i < n {
            let j = i + self.w as usize;
            if j > n {
                idx.push(i..n);
            } else {
                idx.push(i..j);
            }
            i += self.w as usize;
        }
        idx
    }

    pub fn is_line_visible(&self, line_number: i32) -> bool {
        line_number >= self.top && line_number < self.top + self.h 
    }
}

#[cfg(test)]
mod tests {
    use crate::screen::*;

    #[test]
    fn screen_split_lines() {
        println!("Hello world");
        let screen = Screen {top:0, h:100, w: 10};
        let line = "123456789-123456789-123456789".to_string();
        let ranges = screen.split_line(&line);
        assert_eq!(ranges.len(), 3);
        assert_eq!("123456789-", &line[ranges.get(0).unwrap().to_owned()]);
        assert_eq!("123456789-", &line[ranges.get(1).unwrap().to_owned()]);
        assert_eq!("123456789", &line[ranges.get(2).unwrap().to_owned()]);
    }

    #[test]
    fn screen_split_lines_1line() {
        println!("Hello world");
        let screen = Screen {top:0, h:100, w: 100};
        let line = "123456789-123456789-123456789".to_string();
        let ranges = screen.split_line(&line);
        assert_eq!(ranges.len(), 1);
    }
}
