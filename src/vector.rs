use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Hash, Eq, Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0, y: 0 };
    pub const UP: Vec2 = Vec2 { x: 0, y: -1 };
    pub const DOWN: Vec2 = Vec2 { x: 0, y: 1 };
    pub const LEFT: Vec2 = Vec2 { x: -1, y: 0 };
    pub const RIGHT: Vec2 = Vec2 { x: 1, y: 0 };

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_index(idx: usize, width: usize) -> Self {
        let x = (idx % width) as i32;
        let y = (idx / width) as i32;
        Self { x, y }
    }

    pub fn to_index(&self, width: usize) -> usize {
        self.y as usize * width + self.x as usize
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_math() {
        let v1 = Vec2 { x: -3, y: 4 };
        let v2 = Vec2 { x: 1, y: -2 };
        let v3 = Vec2 { x: -5, y: 5 };
        let v4 = Vec2 { x: -5, y: 5 };

        assert_eq!(v1 + v2, Vec2 { x: -2, y: 2 });
        assert_eq!(v3 - v4, Vec2 { x: 0, y: 0 });
    }
}
