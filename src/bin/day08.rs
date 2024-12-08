use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

struct Map {
    w: i32,
    h: i32,
    m: Vec<Vec<char>>,
}

fn read_map(input: &str) -> Map {
    let m: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let h = m.len() as i32;
    let w = m[0].len() as i32;
    Map { w, h, m }
}

type Point = (i32, i32);

fn read_at(map: &Map, p: Point) -> Option<char> {
    if p.0 < 0 || p.0 >= map.w || p.1 < 0 || p.1 >= map.h {
        None
    } else {
        Some(map.m[p.1 as usize][p.0 as usize])
    }
}

fn solve_p1(input: &str) -> i64 {
    let map = read_map(input);

    let mut antennas = HashMap::new();
    for y in 0..map.h {
        for x in 0..map.w {
            if let Some(f) = read_at(&map, (x, y)) {
                if f != '.' {
                    let c: &mut _ = antennas.entry(f).or_insert(Vec::new());
                    c.push((x, y));
                }
            }
        }
    }
    let mut antinodes = HashSet::new();
    for (_, ps) in antennas {
        for i in 0..ps.len() {
            for j in (i + 1)..ps.len() {
                let p1 = &ps[i];
                let p2 = &ps[j];
                let dist = (p2.0 - p1.0, p2.1 - p1.1);
                let a1 = (p1.0 - dist.0, p1.1 - dist.1);
                let a2 = (p2.0 + dist.0, p2.1 + dist.1);
                if a1.0 >= 0 && a1.0 < map.w && a1.1 >= 0 && a1.1 < map.h {
                    antinodes.insert(a1);
                }
                if a2.0 >= 0 && a2.0 < map.w && a2.1 >= 0 && a2.1 < map.h {
                    antinodes.insert(a2);
                }
            }
        }
    }

    antinodes.len() as i64
}

fn solve_p2(input: &str) -> i64 {
    let map = read_map(input);

    let mut antennas = HashMap::new();
    for y in 0..map.h {
        for x in 0..map.w {
            if let Some(f) = read_at(&map, (x, y)) {
                if f != '.' {
                    let c: &mut _ = antennas.entry(f).or_insert(Vec::new());
                    c.push((x, y));
                }
            }
        }
    }

    let mut antinodes = HashSet::new();
    for (_, ps) in antennas {
        for i in 0..ps.len() {
            for j in (i + 1)..ps.len() {
                let mut p1 = ps[i];
                let mut p2 = ps[j];
                let dist = (p2.0 - p1.0, p2.1 - p1.1);
                while p1.0 >= 0 && p1.0 < map.w && p1.1 >= 0 && p1.1 < map.h {
                    antinodes.insert(p1);
                    p1 = (p1.0 - dist.0, p1.1 - dist.1);
                }
                while p2.0 >= 0 && p2.0 < map.w && p2.1 >= 0 && p2.1 < map.h {
                    antinodes.insert(p2);
                    p2 = (p2.0 + dist.0, p2.1 + dist.1);
                }
            }
        }
    }

    antinodes.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX1), 14);
    }

    #[test]
    fn p2_test1() {
        assert_eq!(solve_p2(INPUT_EX1), 34);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input08.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input));
    println!("Part2: {}", solve_p2(&input));
}
