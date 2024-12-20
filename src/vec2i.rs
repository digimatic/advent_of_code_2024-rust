use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Debug, PartialEq, Clone, Hash, Eq, Ord, PartialOrd)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Self {
        Vec2i { x, y }
    }

    pub fn add<V: Into<Vec2i>>(self, other: V) -> Vec2i {
        let other = other.into();
        Vec2i {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn neighbours_8(&self) -> Vec<Vec2i> {
        (-1..=1)
            .flat_map(|y| {
                (-1..=1).filter_map(move |x| {
                    if x == 0 && y == 0 {
                        None
                    } else {
                        Some(
                            self + &Vec2i {
                                x: x as i32,
                                y: y as i32,
                            },
                        )
                    }
                })
            })
            .collect::<Vec<_>>()
    }

    pub fn neighbours_4(&self) -> Vec<Vec2i> {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(dx, dy)| self + &Vec2i { x: dx, y: dy })
            .collect()
    }
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

impl<T: Into<Vec2i>> Add<T> for Vec2i {
    type Output = Vec2i;

    fn add(self, other: T) -> Vec2i {
        let rhs = other.into();
        Vec2i {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Into<Vec2i>> Add<T> for &Vec2i {
    type Output = Vec2i;

    fn add(self, other: T) -> Vec2i {
        let rhs = other.into();
        Vec2i {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Into<Vec2i>> Sub<T> for Vec2i {
    type Output = Vec2i;

    fn sub(self, other: T) -> Vec2i {
        let other = other.into();
        Vec2i {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Into<Vec2i>> Sub<T> for &Vec2i {
    type Output = Vec2i;

    fn sub(self, other: T) -> Vec2i {
        let other = other.into();
        Vec2i {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl AddAssign for Vec2i {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Vec2i {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<Vec2i> for i32 {
    type Output = Vec2i;

    fn mul(self, rhs: Vec2i) -> Vec2i {
        Vec2i {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec2i_test() {
        assert_eq!(Vec2i::new(1, 2) + Vec2i::new(3, 4), Vec2i::new(4, 6));
        assert_eq!(&Vec2i::new(1, 2) + &Vec2i::new(3, 4), Vec2i::new(4, 6));
        assert_eq!(Vec2i::new(1, 2) + &Vec2i::new(3, 4), Vec2i::new(4, 6));
        assert_eq!(&Vec2i::new(1, 2) + Vec2i::new(3, 4), Vec2i::new(4, 6));

        assert_eq!(Vec2i::new(1, 2) - Vec2i::new(3, 5), Vec2i::new(-2, -3));
        assert_eq!(&Vec2i::new(1, 2) - &Vec2i::new(3, 5), Vec2i::new(-2, -3));
        assert_eq!(Vec2i::new(1, 2) - &Vec2i::new(3, 5), Vec2i::new(-2, -3));
        assert_eq!(&Vec2i::new(1, 2) - Vec2i::new(3, 5), Vec2i::new(-2, -3));

        assert_eq!(5 * Vec2i::new(2, 3), Vec2i::new(10, 15));
    }
}
