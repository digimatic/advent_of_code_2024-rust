use advent_of_code_2024_rust::{
    map::{read_map, Map},
    vec2i::Vec2i,
};
use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

fn parse(input: &str) -> (Map, Vec2i) {
    let map = read_map(input);
    let mut start_pos: Vec2i = Vec2i::new(0, 0);
    for y in 0..map.h {
        for x in 0..map.w {
            if map.read(Vec2i::new(x, y)) == Some('S') {
                start_pos = Vec2i::new(x, y);
            }
        }
    }
    (map, start_pos)
}

fn walk_from(map: &Map, start_pos: &Vec2i) -> Vec<Vec2i> {
    let mut path = Vec::new();
    let mut p = start_pos.clone();
    path.push(start_pos.clone());

    while map.read(&p) != Some('E') {
        let next_pos = p
            .neighbours_4()
            .into_iter()
            .find(|pp| {
                let c = map.read(pp);
                !path.contains(&pp) && (c == Some('.') || c == Some('E'))
            })
            .unwrap();
        path.push(next_pos.clone());
        p = next_pos;
    }

    path
}

fn possible_cheats_from(
    map: &Map,
    n: usize,
    start_pos: &Vec2i,
    best_path: &HashMap<Vec2i, usize>,
    max_cheat_length: usize,
    savings: &mut HashMap<usize, usize>,
) {
    let mut visited: HashSet<Vec2i> = HashSet::new();
    visited.insert(start_pos.clone());

    let min_pos = start_pos - Vec2i::new((max_cheat_length) as i32, (max_cheat_length) as i32);
    let max_pos = start_pos + Vec2i::new((max_cheat_length) as i32, (max_cheat_length) as i32);

    for y in min_pos.y..=max_pos.y {
        for x in min_pos.x..=max_pos.x {
            let end_pos = Vec2i::new(x, y);
            let d = &end_pos - start_pos;
            let d = (d.x.abs() + d.y.abs()) as usize;
            if d < 2 || d > max_cheat_length {
                continue;
            }
            if !map.inside(&end_pos) {
                continue;
            }
            if visited.contains(&end_pos) {
                continue;
            }
            if map.read(&end_pos) == Some('#') {
                continue;
            }

            visited.insert(end_pos.clone());

            if let Some(&n2) = best_path.get(&end_pos) {
                if n2 > n {
                    let saving = n2 - n - d;
                    savings.entry(saving).and_modify(|v| *v += 1).or_insert(1);
                }
            }
        }
    }
}

fn solve(input: &str, max_cheat_length: usize, min_saving: usize, print_limit: usize) -> usize {
    println!();

    let (map, start_pos) = parse(input);

    let best_path = walk_from(&map, &start_pos);
    println!("Best length: {}", best_path.len() - 1);

    let best_path: Vec<_> = best_path.into_iter().enumerate().collect();
    let best_path_map: HashMap<Vec2i, usize> =
        best_path.iter().map(|(n, p)| (p.clone(), *n)).collect();

    let mut savings: HashMap<usize, usize> = HashMap::new();

    for (n, p) in &best_path {
        possible_cheats_from(&map, *n, &p, &best_path_map, max_cheat_length, &mut savings);
    }

    let mut savings: Vec<_> = savings.into_iter().collect();

    savings.sort_unstable_by_key(|&(k, _)| k);

    let mut total_count = 0;
    for (saving, count) in savings {
        if saving >= print_limit {
            println!(
                "There is {} cheats that saves {} picoseconds",
                count, saving
            );
        }
        if saving >= min_saving {
            total_count += count;
        }
    }
    total_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test1() {
        assert_eq!(solve(INPUT_EX1, 2, 38, 0), 3);
    }

    #[test]
    fn p2_test1() {
        assert_eq!(solve(INPUT_EX1, 20, 72, 50), 29);
    }
}

const INPUT_EX1: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

fn main() {
    assert_eq!(solve(INPUT_EX1, 2, 38, 0), 3);
    assert_eq!(solve(INPUT_EX1, 20, 72, 50), 29);

    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input20.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve(&input, 2, 100, 10000));
    println!("Part2: {}", solve(&input, 20, 100, 10000));
}
