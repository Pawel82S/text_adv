use console_engine::screen::Screen;

pub struct TextScr {
    contents: Vec<String>,
    screen: Screen,
}

impl TextScr {
    pub fn new(width: u32, height: u32, contents: Vec<String>) -> Self {
        let screen = Screen::new(width, height);
        let mut result = Self { contents, screen };
        result.render();
        result
    }

    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    pub fn set_contents(&mut self, contents: Vec<String>) {
        self.contents = contents;
        self.render();
    }

    fn render(&mut self) {
        let lines = 0..self.screen.get_height() as i32;
        for (idx, txt) in lines.zip(self.contents.iter()) {
            self.screen.print(0, idx, txt);
        }
    }
}
