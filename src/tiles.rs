pub mod border {
    pub const HORIZONTAL: char = '━';
    pub const VERTICAL: char = '┃';
    pub const TOP_LEFT: char = '┏';
    pub const TOP_CENTER: char = '┳';
    pub const TOP_RIGHT: char = '┓';
    pub const CENTER_LEFT: char = '┣';
    pub const CENTER_CENTER: char = '╋';
    pub const CENTER_RIGHT: char = '┫';
    pub const BOTTOM_LEFT: char = '┗';
    pub const BOTTOM_CENTER: char = '┻';
    pub const BOTTOM_RIGHT: char = '┛';
}

pub mod door {
    pub const LOCKED: char = 'D';
    pub const UNLOCKED: char = 'd';
}

pub mod window {
    pub const LOCKED: char = 'W';
    pub const UNLOCKED: char = 'w';
}

pub const EMPTY: char = ' ';
pub const GRASS: char = '.';
pub const PLAYER: char = '☻';
