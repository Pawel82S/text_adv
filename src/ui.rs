mod event_window;
mod simple_player_scr;
mod text_scr;

use crate::tiles;
use console_engine::{pixel, screen::Screen};
pub use event_window::EventWindow;
pub use simple_player_scr::SimplePlayerScr;
pub use text_scr::TextScr;

pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

pub fn main_layout_screen(width: u32, height: u32) -> Screen {
    let mut s = Screen::new(width, height);

    s.v_line(60, 0, height as i32, pixel::pxl(tiles::Border::VERTICAL));
    s.h_line(61, 10, width as i32, pixel::pxl(tiles::Border::HORIZONTAL));
    s.h_line(
        61,
        height as i32 - 5,
        width as i32,
        pixel::pxl(tiles::Border::HORIZONTAL),
    );

    let pxl = pixel::pxl(tiles::Border::CENTER_LEFT);
    s.set_pxl(60, 10, pxl);
    s.set_pxl(60, height as i32 - 5, pxl);

    s
}
