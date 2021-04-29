use super::Size;
use crate::tiles;
use console_engine::{pixel, screen::Screen, Color, ConsoleEngine, KeyCode};

const OPTIONS_SPACING: usize = 3;

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

        screen.h_line(0, 0, width, pixel::pxl(tiles::border::HORIZONTAL));
        screen.h_line(0, height, width, pixel::pxl(tiles::border::HORIZONTAL));
        screen.v_line(0, 0, height, pixel::pxl(tiles::border::VERTICAL));
        screen.v_line(width, 0, height, pixel::pxl(tiles::border::VERTICAL));
        screen.set_pxl(0, 0, pixel::pxl(tiles::border::TOP_LEFT));
        screen.set_pxl(width, 0, pixel::pxl(tiles::border::TOP_RIGHT));
        screen.set_pxl(0, height, pixel::pxl(tiles::border::BOTTOM_LEFT));
        screen.set_pxl(width, height, pixel::pxl(tiles::border::BOTTOM_RIGHT));

        let center_x = |length| ((self.size.width - length as u32) / 2) as i32;

        if self.title.len() > 0 {
            // Put title at top frame in the center.
            screen.print(center_x(self.title.len()), 0, &self.title);
        }

        if self.contents.len() > 0 {
            // Put constents at the window center
            screen.print(center_x(self.contents.len()), 1, &self.contents);
        }

        let mut spacings = self.options.len() - 1;
        let options_len: usize = self.options.iter().map(|opt| opt.len()).sum();
        let total_len = spacings * OPTIONS_SPACING + options_len;
        let mut x = center_x(total_len);

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
