use std::{collections::HashSet, env, fs};

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
    let mut map = read_map(input);

    let mut p: Point = (0, 0) as Point;
    let mut d: (i32, i32) = (0, -1);
    for y in 0..map.h {
        for x in 0..map.w {
            if let Some('^') = read_at(&map, (x, y)) {
                map.m[y as usize][x as usize] = '.';
                p = (x, y);
                break;
            }
        }
    }

    let mut visited: HashSet<Point> = HashSet::new();

    while p.0 >= 0 && p.0 < map.w && p.1 >= 0 && p.1 < map.h {
        visited.insert(p);

        let p2 = (p.0 + d.0, p.1 + d.1);
        if read_at(&map, p2) == Some('#') {
            d = match d {
                (0, -1) => (1, 0),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                (1, 0) => (0, 1),
                _ => panic!(),
            };
        } else {
            p = p2;
        }
    }

    visited.len() as i64
}

fn solve_p2(input: &str) -> i64 {
    let mut map = read_map(input);

    let mut start_pos: Point = (0, 0) as Point;
    let start_dir: (i32, i32) = (0, -1);
    for y in 0..map.h {
        for x in 0..map.w {
            if let Some('^') = read_at(&map, (x, y)) {
                map.m[y as usize][x as usize] = '.';
                start_pos = (x, y);
                break;
            }
        }
    }

    let mut loop_count = 0;

    for oy in 0..map.h {
        for ox in 0..map.w {
            if (ox, oy) != start_pos && read_at(&map, (ox, oy)) == Some('.') {
                let mut visited: HashSet<(Point, (i32, i32))> = HashSet::new();
                let mut p = start_pos;
                let mut d = start_dir;

                while p.0 >= 0 && p.0 < map.w && p.1 >= 0 && p.1 < map.h {
                    visited.insert((p, d));

                    let p2 = (p.0 + d.0, p.1 + d.1);
                    let c = read_at(&map, p2);
                    if c == Some('#') || p2 == (ox, oy) {
                        d = match d {
                            (0, -1) => (1, 0),
                            (0, 1) => (-1, 0),
                            (-1, 0) => (0, -1),
                            (1, 0) => (0, 1),
                            _ => panic!(),
                        };
                    } else {
                        p = p2;
                    }

                    if visited.contains(&(p, d)) {
                        loop_count += 1;
                        break;
                    }
                }
            }
        }
    }

    loop_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX1), 41);
    }

    #[test]
    fn p2_test1() {
        assert_eq!(solve_p2(INPUT_EX1), 6);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input06.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input));
    println!("Part2: {}", solve_p2(&input));
}
