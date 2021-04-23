use super::Size;
use crate::tiles;
use console_engine::{pixel, screen::Screen, Color, ConsoleEngine, KeyCode};

const OPTIONS_SPACING: u32 = 3;

pub struct EventWindow {
    title: String,
    contents: String,
    size: Size,
    options: Vec<String>,
    selected: usize,
}

impl EventWindow {
    pub fn new(size: Size, contents: String, title: String, options: Vec<String>) -> Self {
        Self {
            title,
            contents,
            size,
            options,
            selected: 0,
        }
    }

    pub fn handle_input(&mut self, engine: &ConsoleEngine) {
        if engine.is_key_pressed(KeyCode::Left) {
            if self.selected == 0 {
                self.selected = self.options.len() - 1;
            } else {
                self.selected -= 1;
            }
        } else if engine.is_key_pressed(KeyCode::Right) {
            if self.selected == self.options.len() - 1 {
                self.selected = 0;
            } else {
                self.selected += 1;
            }
        }
    }

    pub fn render(&self) -> Screen {
        assert!(self.options.len() > 0);

        let mut screen = Screen::new(self.size.width, self.size.height);
        let width = self.size.width as i32 - 1;
        let height = self.size.height as i32 - 1;

        screen.h_line(0, 0, width, pixel::pxl(tiles::Border::HORIZONTAL));
        screen.h_line(0, height, width, pixel::pxl(tiles::Border::HORIZONTAL));
        screen.v_line(0, 0, height, pixel::pxl(tiles::Border::VERTICAL));
        screen.v_line(width, 0, height, pixel::pxl(tiles::Border::VERTICAL));
        screen.set_pxl(0, 0, pixel::pxl(tiles::Border::TOP_LEFT));
        screen.set_pxl(width, 0, pixel::pxl(tiles::Border::TOP_RIGHT));
        screen.set_pxl(0, height, pixel::pxl(tiles::Border::BOTTOM_LEFT));
        screen.set_pxl(width, height, pixel::pxl(tiles::Border::BOTTOM_RIGHT));

        if self.title.len() > 0 {
            // Put title at top frame in the center.
            let x = ((self.size.width - self.title.len() as u32) / 2) as i32;
            screen.print(x, 0, &self.title);
        }

        if self.contents.len() > 0 {
            // Put constents at the window center
            let x = ((self.size.width - self.contents.len() as u32) / 2) as i32;
            screen.print(x, 1, &self.contents);
        }

        let mut spacings = self.options.len() - 1;
        let options_len: u32 = self.options.iter().map(|opt| opt.len() as u32).sum();
        let total_len = spacings as u32 * OPTIONS_SPACING + options_len;
        let mut x = ((self.size.width - total_len) / 2) as i32;

        for (idx, option) in self.options.iter().enumerate() {
            screen.print_fbg(
                x,
                2,
                option,
                Color::White,
                if idx == self.selected {
                    Color::DarkBlue
                } else {
                    Color::Black
                },
            );

            if spacings > 0 {
                x += option.len() as i32 + OPTIONS_SPACING as i32;
                spacings -= 1;
            }
        }

        screen
    }

    pub fn selected(&self) -> usize {
        self.selected
    }
}
