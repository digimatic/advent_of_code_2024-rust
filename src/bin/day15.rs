use advent_of_code_2024_rust::vec2i::Vec2i;
use std::{collections::HashMap, env, fs};

// Part 1

fn parse(input: &str) -> (Vec2i, HashMap<Vec2i, char>, Vec<char>) {
    let lines = input.lines().collect::<Vec<&str>>();
    let h = lines.iter().position(|s| s.is_empty()).unwrap();
    let mut map: HashMap<Vec2i, char> = HashMap::new();
    let mut start_pos = Vec2i::new(0, 0);
    for y in 0..h {
        let line = lines[y].chars().collect::<Vec<char>>();
        let w = line.len();
        for x in 0..w {
            let c = line[x];
            if c == '@' {
                start_pos = Vec2i::new(x as i32, y as i32);
            } else if c == '.' {
            } else {
                map.insert(Vec2i::new(x as i32, y as i32), c);
            }
        }
    }

    let movements = lines[h..].join("").chars().collect::<Vec<_>>();
    (start_pos, map, movements)
}

fn solve_p1(input: &str) -> i32 {
    let (mut pos, mut map, movements) = parse(input);

    for movement in movements {
        let dir = match movement {
            '^' => Vec2i::new(0, -1),
            'v' => Vec2i::new(0, 1),
            '<' => Vec2i::new(-1, 0),
            '>' => Vec2i::new(1, 0),
            _ => continue,
        };

        if try_move(pos.clone(), '@', dir.clone(), &mut map) {
            pos = &pos + &dir;
        }
    }

    let sum = map
        .into_iter()
        .filter_map(|(p, c)| if c == 'O' { Some(p) } else { None })
        .map(|Vec2i { x, y }| y * 100 + x)
        .sum();
    sum
}

fn try_move(pos: Vec2i, old_c: char, dir: Vec2i, map: &mut HashMap<Vec2i, char>) -> bool {
    let new_pos = &pos + &dir;
    if map.contains_key(&new_pos) {
        let c = map[&new_pos];
        if c == 'O' {
            if try_move(new_pos.clone(), c, dir.clone(), map) {
                if c != '@' {
                    map.insert(new_pos.clone(), old_c);
                }
                map.remove(&pos);
                return true;
            } else {
                return false;
            }
        } else if c == '#' {
            return false;
        } else {
            panic!()
        }
    }

    map.insert(new_pos.clone(), old_c);
    map.remove(&pos);
    return true;
}

// Part 2

fn parse2(input: &str) -> (Vec2i, HashMap<Vec2i, char>, Vec<char>) {
    let lines = input.lines().collect::<Vec<&str>>();
    let h = lines.iter().position(|s| s.is_empty()).unwrap();
    let mut map: HashMap<Vec2i, char> = HashMap::new();
    let mut start_pos = Vec2i::new(0, 0);
    for y in 0..h {
        let line = lines[y].chars().collect::<Vec<char>>();
        let w = line.len();
        for x in 0..w {
            let c = line[x];
            if c == '@' {
                start_pos = Vec2i::new((2 * x) as i32, y as i32);
            } else if c == '.' {
            } else if c == 'O' {
                map.insert(Vec2i::new((2 * x) as i32, y as i32), '[');
                map.insert(Vec2i::new((2 * x + 1) as i32, y as i32), ']');
            } else if c == '#' {
                map.insert(Vec2i::new((2 * x) as i32, y as i32), '#');
                map.insert(Vec2i::new((2 * x + 1) as i32, y as i32), '#');
            } else {
                panic!()
            }
        }
    }

    let movements = lines[h..].join("").chars().collect::<Vec<_>>();
    (start_pos, map, movements)
}

fn solve_p2(input: &str) -> i32 {
    let (mut pos, mut map, movements) = parse2(input);

    for movement in movements {
        let dir = match movement {
            '^' => Vec2i::new(0, -1),
            'v' => Vec2i::new(0, 1),
            '<' => Vec2i::new(-1, 0),
            '>' => Vec2i::new(1, 0),
            _ => continue,
        };

        if let Some(new_map) = try_move2(pos.clone(), '@', dir.clone(), map.clone()) {
            pos = &pos + &dir;
            map = new_map;
        }
    }

    let sum = map
        .into_iter()
        .filter_map(|(p, c)| if c == '[' { Some(p) } else { None })
        .map(|Vec2i { x, y }| y * 100 + x)
        .sum();
    sum
}

fn try_move2(
    pos: Vec2i,
    old_c: char,
    dir: Vec2i,
    mut map: HashMap<Vec2i, char>,
) -> Option<HashMap<Vec2i, char>> {
    let new_pos = &pos + &dir;
    if map.contains_key(&new_pos) {
        let c = map[&new_pos];
        if c == '[' && dir.x == 0 {
            let new_pos2 = &new_pos + &Vec2i::new(1, 0);
            let c2 = map[&new_pos2];

            if let Some(map) = try_move2(new_pos.clone(), c, dir.clone(), map) {
                if let Some(mut map) = try_move2(new_pos2.clone(), c2, dir.clone(), map) {
                    if c != '@' {
                        map.insert(new_pos.clone(), old_c);
                    }
                    map.remove(&pos);
                    return Some(map);
                }
            }
            return None;
        } else if c == ']' && dir.x == 0 {
            let new_pos2 = &new_pos + &Vec2i::new(-1, 0);
            let c2 = map[&new_pos2];

            if let Some(map) = try_move2(new_pos.clone(), c, dir.clone(), map) {
                if let Some(mut map) = try_move2(new_pos2.clone(), c2, dir.clone(), map) {
                    if c != '@' {
                        map.insert(new_pos.clone(), old_c);
                    }
                    map.remove(&pos);
                    return Some(map);
                }
            }
            return None;
        } else if (c == '[') && dir.x == 1 {
            let new_pos2 = &new_pos + &Vec2i::new(1, 0);
            let c2 = map[&new_pos2];

            if let Some(map) = try_move2(new_pos2.clone(), c2, dir.clone(), map) {
                if let Some(mut map) = try_move2(new_pos.clone(), c, dir.clone(), map) {
                    if c != '@' {
                        map.insert(new_pos.clone(), old_c);
                    }
                    map.remove(&pos);
                    return Some(map);
                } else {
                    panic!();
                }
            }
            return None;
        } else if c == ']' && dir.x == -1 {
            let new_pos2 = &new_pos + &Vec2i::new(-1, 0);
            let c2 = map[&new_pos2];

            if let Some(map) = try_move2(new_pos2.clone(), c2, dir.clone(), map) {
                if let Some(mut map) = try_move2(new_pos.clone(), c, dir.clone(), map) {
                    if c != '@' {
                        map.insert(new_pos.clone(), old_c);
                    }
                    map.remove(&pos);
                    return Some(map);
                } else {
                    panic!();
                }
            }
            return None;
        } else if c == '#' {
            return None;
        } else {
            panic!()
        }
    }

    map.insert(new_pos.clone(), old_c);
    map.remove(&pos);
    Some(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX0: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

    const INPUT_EX1: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX0), 2028);
    }
    #[test]
    fn p1_test2() {
        assert_eq!(solve_p1(INPUT_EX1), 10092);
    }

    #[test]
    fn p2_test1() {
        assert_eq!(solve_p2(INPUT_EX1), 9021);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input15.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input));
    println!("Part2: {}", solve_p2(&input));
}
