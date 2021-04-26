use crate::{serialize::Serialize, tiles, vector::Vec2};
use console_engine::{pixel, screen::Screen};
use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Write},
};

const DETAILS_SECTION: &str = "[details]";
const MULTI_SHAPE_TILES: [MapTile; 2] = [MapTile::Wall, MapTile::Road];

enum MultiShapeTile {
    Single,
    HorizontalLeft,
    HorizontalRight,
    Horizontal,
    VerticalTop,
    VerticalBottom,
    Vertical,
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    CenterLeft,
    CenterRight,
    Cross,
    Invalid(Vec2),
}

#[derive(Debug, PartialEq)]
pub enum MapTile {
    Empty,
    Player,
    Grass,
    Wall,
    Road,
    Door { locked: bool },
    Window { locked: bool },
}

impl MapTile {
    pub fn is_multishape(&self) -> bool {
        for shape in MULTI_SHAPE_TILES.iter() {
            if self == shape {
                return true;
            }
        }

        false
    }
}

pub struct Map {
    screen: Screen,
    width: usize,
    height: usize,
    tiles: Vec<MapTile>,
    details: String,
    starting_position: Option<Vec2>,
}

impl Map {
    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&MapTile> {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            Some(&self.tiles[index])
        } else {
            None
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: MapTile) {
        assert!(x < self.width && y < self.height);

        let index = y * self.width + x;
        self.tiles[index] = tile;
    }

    pub fn details(&self) -> &str {
        &self.details
    }

    pub fn player_position(&self) -> Option<Vec2> {
        self.starting_position
    }

    pub fn can_move(&self, pos: Vec2) -> bool {
        match self.tiles[pos.to_index(self.width)] {
            MapTile::Wall | MapTile::Window { locked: true } | MapTile::Door { locked: true } => {
                false
            }
            _ => true,
        }
    }

    fn render_map(&mut self) {
        for (idx, tile) in self.tiles.iter().enumerate() {
            let current_position = Vec2::from_index(idx, self.width);
            let ch = if tile.is_multishape() {
                // Handle multishape tiles. This code is very non optimal, but it works.
                // I will optimize it if it will be too slow or I will have enough time.
                // TODO: Optimize this code to use just indexes in vector.

                let beginning_row = idx < self.width;
                let ending_row = current_position.y as usize == self.height - 1;
                let beginning_column = idx % self.width == 0;
                let ending_column = current_position.x as usize == self.width - 1;

                let mut check_directions = vec![Vec2::UP, Vec2::DOWN, Vec2::LEFT, Vec2::RIGHT];
                if beginning_row {
                    check_directions.retain(|elem| *elem != Vec2::UP);
                } else if ending_row {
                    check_directions.retain(|elem| *elem != Vec2::DOWN);
                }
                if beginning_column {
                    check_directions.retain(|elem| *elem != Vec2::LEFT);
                } else if ending_column {
                    check_directions.retain(|elem| *elem != Vec2::RIGHT);
                }

                let mut shape = MultiShapeTile::Single;
                for direction in check_directions.iter() {
                    let checked_index = (current_position + *direction).to_index(self.width);
                    if *tile != self.tiles[checked_index] {
                        continue;
                    }

                    shape = match shape {
                        MultiShapeTile::Single => match *direction {
                            Vec2::UP => MultiShapeTile::VerticalTop,
                            Vec2::DOWN => MultiShapeTile::VerticalBottom,
                            Vec2::LEFT => MultiShapeTile::HorizontalLeft,
                            Vec2::RIGHT => MultiShapeTile::HorizontalRight,
                            _ => MultiShapeTile::Invalid(*direction),
                        },
                        MultiShapeTile::HorizontalLeft => match *direction {
                            Vec2::UP => MultiShapeTile::BottomRight,
                            Vec2::RIGHT => MultiShapeTile::Horizontal,
                            Vec2::DOWN => MultiShapeTile::TopRight,
                            _ => MultiShapeTile::Invalid(*direction),
                        },
                        MultiShapeTile::HorizontalRight => match *direction {
                            Vec2::LEFT => MultiShapeTile::Horizontal,
                            Vec2::UP => MultiShapeTile::BottomLeft,
                            Vec2::DOWN => MultiShapeTile::TopLeft,
                            _ => MultiShapeTile::Invalid(*direction),
                        },
                        MultiShapeTile::Horizontal => match *direction {
                            Vec2::UP => MultiShapeTile::BottomCenter,
                            Vec2::DOWN => MultiShapeTile::TopCenter,
                            _ => MultiShapeTile::Invalid(*direction),
                        },
                        MultiShapeTile::VerticalTop => match *direction {
                            Vec2::DOWN => MultiShapeTile::Vertical,
                            Vec2::LEFT => MultiShapeTile::BottomRight,
                            Vec2::RIGHT => MultiShapeTile::BottomLeft,
                            _ => MultiShapeTile::Invalid(*direction),
                        },
                        MultiShapeTile::VerticalBottom => match *direction {
                            Vec2::UP => MultiShapeTile::Vertical,
                            Vec2::LEFT => MultiShapeTile::TopRight,
                            Vec2::RIGHT => MultiShapeTile::TopLeft,
                            _ => MultiShapeTile::Invalid(*direction),
                        },
                        MultiShapeTile::Vertical => match *direction {
                            Vec2::LEFT => MultiShapeTile::CenterRight,
                            Vec2::RIGHT => MultiShapeTile::CenterLeft,
                            _ => MultiShapeTile::Invalid(*direction),
                        },
                        MultiShapeTile::TopLeft => match *direction {
                            Vec2::UP => MultiShapeTile::CenterLeft,
                            Vec2::LEFT => MultiShapeTile::TopCenter,
                            _ => MultiShapeTile::Invalid(*direction),
                        },
                        MultiShapeTile::TopCenter => match *direction {
                            Vec2::UP => MultiShapeTile::Cross,
                            _ => MultiShapeTile::Invalid(*direction),
                        },
                        MultiShapeTile::TopRight => match *direction {
                            Vec2::UP => MultiShapeTile::CenterRight,
                            Vec2::RIGHT => MultiShapeTile::TopCenter,
                            _ => MultiShapeTile::Invalid(*direction),
                        },
                        MultiShapeTile::BottomLeft => match *direction {
                            Vec2::DOWN => MultiShapeTile::CenterLeft,
                            Vec2::LEFT => MultiShapeTile::BottomCenter,
                            _ => MultiShapeTile::Invalid(*direction),
                        },
                        MultiShapeTile::BottomCenter => match *direction {
                            Vec2::DOWN => MultiShapeTile::Cross,
                            _ => MultiShapeTile::Invalid(*direction),
                        },
                        MultiShapeTile::BottomRight => match *direction {
                            Vec2::DOWN => MultiShapeTile::CenterRight,
                            Vec2::RIGHT => MultiShapeTile::BottomCenter,
                            _ => MultiShapeTile::Invalid(*direction),
                        },
                        MultiShapeTile::CenterLeft => match *direction {
                            Vec2::LEFT => MultiShapeTile::Cross,
                            _ => MultiShapeTile::Invalid(*direction),
                        },
                        MultiShapeTile::CenterRight => match *direction {
                            Vec2::RIGHT => MultiShapeTile::Cross,
                            _ => MultiShapeTile::Invalid(*direction),
                        },
                        _ => MultiShapeTile::Invalid(*direction),
                    };
                }

                match shape {
                    MultiShapeTile::Invalid(direction) => {
                        panic!("Invalid direction for neighbour tile: {:?}", direction)
                    }
                    MultiShapeTile::Cross => tiles::border::CENTER_CENTER,
                    MultiShapeTile::Single => tiles::border::CENTER_CENTER,
                    MultiShapeTile::TopLeft => tiles::border::TOP_LEFT,
                    MultiShapeTile::TopRight => tiles::border::TOP_RIGHT,
                    MultiShapeTile::TopCenter => tiles::border::TOP_CENTER,
                    MultiShapeTile::BottomLeft => tiles::border::BOTTOM_LEFT,
                    MultiShapeTile::BottomRight => tiles::border::BOTTOM_RIGHT,
                    MultiShapeTile::BottomCenter => tiles::border::BOTTOM_CENTER,
                    MultiShapeTile::CenterLeft => tiles::border::CENTER_LEFT,
                    MultiShapeTile::CenterRight => tiles::border::CENTER_RIGHT,
                    MultiShapeTile::HorizontalLeft
                    | MultiShapeTile::HorizontalRight
                    | MultiShapeTile::Horizontal => tiles::border::HORIZONTAL,
                    MultiShapeTile::VerticalTop
                    | MultiShapeTile::VerticalBottom
                    | MultiShapeTile::Vertical => tiles::border::VERTICAL,
                }
            } else {
                match *tile {
                    MapTile::Empty => tiles::EMPTY,
                    MapTile::Door { locked } => {
                        if locked {
                            tiles::door::LOCKED
                        } else {
                            tiles::door::UNLOCKED
                        }
                    }
                    MapTile::Grass => tiles::GRASS,
                    MapTile::Window { locked } => {
                        if locked {
                            tiles::window::LOCKED
                        } else {
                            tiles::window::UNLOCKED
                        }
                    }
                    MapTile::Player => tiles::PLAYER,
                    _ => panic!("Tile {:?} is not implemented or multishape", tile),
                }
            };

            // Render ch at proper position
            let pixel = pixel::pxl(ch);
            self.screen
                .set_pxl(current_position.x, current_position.y, pixel);
        }
    }

    fn split_for_map_and_details(raw_data: &str) -> (String, String) {
        let map: String = raw_data
            .lines()
            .filter(|line| !line.is_empty() && !line.trim_start().starts_with(";"))
            .take_while(|line| line.trim() != DETAILS_SECTION)
            .map(|line| format!("{}\n", line))
            .collect();

        let map_details: String = raw_data
            .lines()
            .skip_while(|line| line.trim() != DETAILS_SECTION)
            .skip(1)
            .filter(|line| !line.is_empty() && !line.trim_start().starts_with(";"))
            .map(|line| format!("{}\n", line))
            .collect();

        (map, map_details)
    }

    fn parse_data(map_data: String, details: String) -> Self {
        let mut tiles = Vec::new();
        let mut player_position = None;
        let mut width = 0;
        let mut height = 0;

        for (y, line) in map_data.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let tile = Self::tile_from_char(ch);

                if tile == MapTile::Player {
                    player_position = Some(Vec2::new(x as i32, y as i32));
                    tiles.push(MapTile::Empty);
                } else {
                    tiles.push(tile);
                }
                width = width.max(x + 1);
            }
            height = height.max(y + 1);
        }

        let screen = Screen::new(width as u32, height as u32);

        Self {
            screen,
            width,
            height,
            tiles,
            details,
            starting_position: player_position,
        }
    }

    fn tile_from_char(ch: char) -> MapTile {
        match ch {
            '@' => MapTile::Player,
            '.' => MapTile::Grass,
            '+' => MapTile::Road,
            '#' => MapTile::Wall,
            'W' => MapTile::Window { locked: true },
            'w' => MapTile::Window { locked: false },
            'D' => MapTile::Door { locked: true },
            'd' => MapTile::Door { locked: false },
            _ => MapTile::Empty,
        }
    }
}

impl Serialize for Map {
    fn load_from_file(file_name: &str) -> io::Result<Map> {
        let raw_data = fs::read_to_string(file_name)?;

        let (map_data, details) = Self::split_for_map_and_details(&raw_data);

        let mut map = Self::parse_data(map_data, details);
        map.render_map();
        Ok(map)
    }

    fn save_to_file(&self, file_name: &str) -> io::Result<()> {
        unimplemented!()
    }

    fn load_from_reader(reader: &mut BufReader<File>) -> Result<Map, &str> {
        unimplemented!()
    }

    fn save_to_writer(&self, writer: &mut BufWriter<File>) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn row_and_colums() {
        let width = 8;
        let tiles_len = 31;

        let beginning_row = |idx| idx < width;
        let ending_row = |idx| idx > tiles_len - width;
        let beginning_column = |idx| idx % width == 0;
        let ending_column = |idx| (idx / width + 1) * width % width == 0;

        assert!(beginning_row(0));
        assert!(beginning_row(3));
        assert!(beginning_row(7));

        assert!(ending_row(24));
        assert!(ending_row(27));
        assert!(ending_row(31));

        assert!(beginning_column(0));
        assert!(beginning_column(16));
        assert!(beginning_column(24));

        assert!(ending_column(7));
        assert!(ending_column(15));
        assert!(ending_column(31));
    }
}
