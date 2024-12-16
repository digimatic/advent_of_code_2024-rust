use advent_of_code_2024_rust::{
    map::{read_map, Map},
    vec2i::Vec2i,
};
use std::{collections::HashMap, collections::HashSet, env, fs};

fn parse(input: &str) -> (Vec2i, Vec2i, Map) {
    let mut map = read_map(input);
    let mut start_pos = Vec2i::new(0, 0);
    let mut end_pos = Vec2i::new(0, 0);
    for y in 0..map.h {
        for x in 0..map.w {
            if map.read(Vec2i::new(x, y)) == Some('S') {
                map.write(Vec2i::new(x, y), '.');
                start_pos = Vec2i::new(x, y);
            } else if map.read(Vec2i::new(x, y)) == Some('E') {
                map.write(Vec2i::new(x, y), '.');
                end_pos = Vec2i::new(x, y);
            }
        }
    }
    (start_pos, end_pos, map)
}

fn turn_left(dir: &Vec2i) -> Vec2i {
    match dir {
        Vec2i { x: 1, y: 0 } => Vec2i::new(0, -1),
        Vec2i { x: -1, y: 0 } => Vec2i::new(0, 1),
        Vec2i { x: 0, y: 1 } => Vec2i::new(1, 0),
        Vec2i { x: 0, y: -1 } => Vec2i::new(-1, 0),
        _ => unreachable!(),
    }
}

fn turn_right(dir: &Vec2i) -> Vec2i {
    match dir {
        Vec2i { x: 1, y: 0 } => Vec2i::new(0, 1),
        Vec2i { x: -1, y: 0 } => Vec2i::new(0, -1),
        Vec2i { x: 0, y: 1 } => Vec2i::new(-1, 0),
        Vec2i { x: 0, y: -1 } => Vec2i::new(1, 0),
        _ => unreachable!(),
    }
}

fn solve_p1(input: &str) -> usize {
    let (start_pos, end_pos, map) = parse(input);

    let mut visited: HashMap<(Vec2i, Vec2i), usize> = HashMap::new();
    let mut queue = Vec::new();
    let start_dir = Vec2i::new(1, 0);
    queue.push((start_pos, start_dir, 0));
    let mut best_cost = std::usize::MAX;
    while !queue.is_empty() {
        let (pos, dir, cost) = queue.pop().unwrap();

        if pos == end_pos {
            if cost < best_cost {
                best_cost = cost;
            }
            continue;
        }

        if visited.contains_key(&(pos.clone(), dir.clone())) {
            let exist_cost = visited[&(pos.clone(), dir.clone())];
            if cost < exist_cost {
                visited.insert((pos.clone(), dir.clone()), cost);
            } else {
                continue;
            }
        } else {
            visited.insert((pos.clone(), dir.clone()), cost);
        }

        if cost > best_cost {
            continue;
        }

        {
            let new_dir = turn_left(&dir);
            queue.push((pos.clone(), new_dir, cost + 1000));
        }
        {
            let new_dir = turn_right(&dir);
            queue.push((pos.clone(), new_dir, cost + 1000));
        }
        {
            let new_pos = &pos + &dir;
            let c = map.read(&new_pos);
            if c == Some('.') {
                queue.push((new_pos, dir.clone(), cost + 1));
            }
        }
    }
    best_cost
}

fn solve_p2(input: &str) -> usize {
    let (start_pos, end_pos, map) = parse(input);

    let mut visited: HashMap<(Vec2i, Vec2i), usize> = HashMap::new();
    let mut queue = Vec::new();
    let start_dir = Vec2i::new(1, 0);
    queue.push((start_pos.clone(), start_dir, 0, vec![start_pos.clone()]));
    let mut best_cost = std::usize::MAX;
    let mut best_paths = Vec::new();
    while !queue.is_empty() {
        let (pos, dir, cost, path) = queue.pop().unwrap();

        if pos == end_pos {
            if cost < best_cost {
                best_cost = cost;
                best_paths = vec![path.clone()];
            } else if cost == best_cost {
                best_paths.push(path.clone());
            }
            continue;
        }

        if visited.contains_key(&(pos.clone(), dir.clone())) {
            let exist_cost = visited[&(pos.clone(), dir.clone())];
            if cost <= exist_cost {
                visited.insert((pos.clone(), dir.clone()), cost);
            } else {
                continue;
            }
        } else {
            visited.insert((pos.clone(), dir.clone()), cost);
        }

        if cost > best_cost {
            continue;
        }

        {
            let new_dir = turn_left(&dir);
            queue.push((pos.clone(), new_dir, cost + 1000, path.clone()));
        }
        {
            let new_dir = turn_right(&dir);
            queue.push((pos.clone(), new_dir, cost + 1000, path.clone()));
        }
        {
            let new_pos = &pos + &dir;
            let c = map.read(&new_pos);
            if c == Some('.') {
                let mut path2 = path.clone();
                path2.push(new_pos.clone());
                queue.push((new_pos, dir.clone(), cost + 1, path2));
            }
        }
    }

    let mut best_paths_positions: HashSet<Vec2i> = HashSet::new();
    for best_path in &best_paths {
        for pos in best_path {
            best_paths_positions.insert(pos.clone());
        }
    }

    best_paths_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX1), 7036);
    }

    #[test]
    fn p2_test1() {
        assert_eq!(solve_p2(INPUT_EX1), 45);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input16.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input));
    println!("Part2: {}", solve_p2(&input));
}
