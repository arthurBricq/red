pub struct Yanker {
    line: Option<String> 
}

impl Yanker {
    pub fn new() -> Self {
        Self { line: None }
    }

    pub fn yank(&mut self, content: String) {
        self.line = Some(content);
    }

    pub fn get_content(&self) -> &Option<String> {
        &self.line
    }
}
