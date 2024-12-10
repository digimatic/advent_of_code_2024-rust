use std::fmt;

use crate::vec2i::Vec2i;

#[derive(PartialEq, Eq, Clone)]
pub struct Map {
    pub w: i32,
    pub h: i32,
    pub m: Vec<Vec<char>>,
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Map {{")?;
        writeln!(f, "    w: {},", self.w)?;
        writeln!(f, "    h: {},", self.h)?;
        writeln!(f, "    m: [")?;
        for row in &self.m {
            write!(f, "        ")?;
            for &cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "    ]")?;
        write!(f, "}}")
    }
}

impl Map {
    pub fn inside(&self, p: &Point) -> bool {
        p.x >= 0 && p.x < self.w && p.y >= 0 && p.y < self.h
    }

    pub fn read<P: Into<Vec2i>>(&self, p: P) -> Option<char> {
        let p2: Vec2i = p.into();
        read_at(self, p2)
    }

    pub fn write<P: Into<Vec2i>>(&mut self, p: P, c: char) {
        let p2: Vec2i = p.into();
        if self.inside(&p2) {
            self.m[p2.y as usize][p2.x as usize] = c;
        }
    }
}

pub fn read_map(input: &str) -> Map {
    let m: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let h = m.len() as i32;
    let w = m[0].len() as i32;
    Map { w, h, m }
}
pub type Point = Vec2i;

pub fn read_at<P: Into<Point>>(map: &Map, p: P) -> Option<char> {
    let point: Point = p.into();
    if point.x < 0 || point.x >= map.w || point.y < 0 || point.y >= map.h {
        None
    } else {
        Some(map.m[point.y as usize][point.x as usize])
    }
}
