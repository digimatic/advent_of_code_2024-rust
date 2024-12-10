use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Clone, Hash, Eq, Ord, PartialOrd)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Default for Vec2i {
    fn default() -> Self {
        Vec2i { x: 0, y: 0 }
    }
}

impl From<(i32, i32)> for Vec2i {
    fn from(tuple: (i32, i32)) -> Self {
        Vec2i {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl From<&(i32, i32)> for Vec2i {
    fn from(tuple: &(i32, i32)) -> Self {
        Vec2i {
            x: (*tuple).0,
            y: (*tuple).1,
        }
    }
}

impl From<&Vec2i> for Vec2i {
    fn from(other: &Vec2i) -> Self {
        Vec2i {
            x: other.x,
            y: other.y,
        }
    }
}

impl Add for Vec2i {
    type Output = Vec2i;

    fn add(self, other: Vec2i) -> Vec2i {
        Vec2i {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2i {
    type Output = Vec2i;

    fn sub(self, other: Vec2i) -> Vec2i {
        Vec2i {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add<&Vec2i> for &Vec2i {
    type Output = Vec2i;

    fn add(self, other: &Vec2i) -> Vec2i {
        Vec2i {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<&Vec2i> for &Vec2i {
    type Output = Vec2i;

    fn sub(self, other: &Vec2i) -> Vec2i {
        Vec2i {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
