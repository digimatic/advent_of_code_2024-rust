use std::{
    collections::{HashMap, HashSet, VecDeque},
    env, fs,
};

use advent_of_code_2024_rust::{
    map::{read_map, Map},
    vec2i::Vec2i,
};

// part 1

fn solve_p1(input: &str) -> i64 {
    let map = read_map(input);
    let mut total = 0i64;

    let mut visited = HashSet::new();

    for y in 0..map.h {
        for x in 0..map.w {
            let start_pos = Vec2i::from((x, y));
            if !visited.contains(&start_pos) {
                let plant = map.read(&start_pos).unwrap();
                total += walk_from(&map, start_pos, plant, &mut visited);
            }
        }
    }
    total
}

fn walk_from(map: &Map, start_pos: Vec2i, plant: char, visited: &mut HashSet<Vec2i>) -> i64 {
    let mut q = VecDeque::new();
    q.push_back(start_pos);

    let mut count = 0;
    let mut edges = 0;
    while !q.is_empty() {
        let p = q.pop_front().unwrap();
        let c = map.read(&p);
        if !visited.contains(&p) {
            if let Some(c) = c {
                if c == plant {
                    count += 1;
                    visited.insert(p.clone());

                    let p1 = &p + &Vec2i::from((1, 0));
                    q.push_back(p1);
                    let p1 = &p + &Vec2i::from((-1, 0));
                    q.push_back(p1);
                    let p1 = &p + &Vec2i::from((0, 1));
                    q.push_back(p1);
                    let p1 = &p + &Vec2i::from((0, -1));
                    q.push_back(p1);
                } else {
                    edges += 1;
                }
            } else {
                edges += 1;
            }
        } else {
            let c = c.unwrap_or('.');
            if c != plant {
                edges += 1;
            }
        }
    }

    let total = edges * count;
    total
}

// part 2

fn solve_p2(input: &str) -> usize {
    let map = read_map(input);
    let mut total = 0;

    let mut visited = HashSet::new();

    for y in 0..map.h {
        for x in 0..map.w {
            let start_pos = Vec2i::from((x, y));
            if !visited.contains(&start_pos) {
                let plant = map.read(&start_pos).unwrap();
                total += walk_from2(&map, start_pos, plant, &mut visited);
            }
        }
    }
    total
}

fn walk_from2(map: &Map, start_pos: Vec2i, plant: char, visited: &mut HashSet<Vec2i>) -> usize {
    let mut q = VecDeque::new();

    if map.read(&start_pos).unwrap_or('.') != plant {
        return 0;
    }

    q.push_back(start_pos);

    let mut count = 0;
    let mut hedges: HashMap<Vec2i, (Vec2i, Vec2i)> = HashMap::new(); // p start -> p end, normal pointing in
    let mut vedges: HashMap<Vec2i, (Vec2i, Vec2i)> = HashMap::new();

    while !q.is_empty() {
        let p = q.pop_front().unwrap();
        let c = map.read(&p).unwrap_or('.');
        assert_eq!(c, plant);
        if !visited.contains(&p) {
            count += 1;
            visited.insert(p.clone());

            let pp = &p + &Vec2i::from((1, 0));
            if map.read(&pp).unwrap_or('.') != c {
                // right edge
                let p1 = pp.clone();
                let p2 = &p1 + &Vec2i::from((0, 1));
                let n = Vec2i::from((-1, 0));
                vedges.insert(p1, (p2, n));
            } else {
                q.push_back(pp);
            }

            let pp = &p + &Vec2i::from((-1, 0));
            if map.read(&pp).unwrap_or('.') != c {
                // left edge
                let p1 = p.clone();
                let p2 = &p + &Vec2i::from((0, 1));
                let n = Vec2i::from((1, 1));
                vedges.insert(p1, (p2, n));
            } else {
                q.push_back(pp);
            }

            let pp = &p + &Vec2i::from((0, -1));
            if map.read(&pp).unwrap_or('.') != c {
                // top edge
                let p1 = p.clone();
                let p2 = &p + &Vec2i::from((1, 0));
                let n = Vec2i::from((0, 1));
                hedges.insert(p1, (p2, n));
            } else {
                q.push_back(pp);
            }

            let pp = &p + &Vec2i::from((0, 1));
            if map.read(&pp).unwrap_or('.') != c {
                // bottom edge
                let p1 = pp.clone();
                let p2 = &pp + &Vec2i::from((1, 0));
                let n = Vec2i::from((0, -1));
                hedges.insert(p1, (p2, n));
            } else {
                q.push_back(pp);
            }
        }
    }

    merge_edges(&mut hedges);
    merge_edges(&mut vedges);
    let sides = hedges.len() + vedges.len();

    let total = sides * count;

    total
}

fn merge_edges(edges: &mut HashMap<Vec2i, (Vec2i, Vec2i)>) {
    loop {
        let mut found_merge = false;
        let mut to_remove = Vec::new();
        let mut to_insert = None;

        for (p1s, (p1e, n1)) in edges.iter() {
            if edges.contains_key(p1e) {
                let (p2e, n2) = edges[p1e].clone();
                if *n1 == n2 {
                    to_remove.push(p1s.clone());
                    to_remove.push(p1e.clone());
                    to_insert = Some((p1s.clone(), (p2e, n1.clone())));
                    found_merge = true;
                    break;
                }
            }
        }

        for key in to_remove {
            edges.remove(&key);
        }

        if let Some((key, value)) = to_insert {
            edges.insert(key, value);
        }

        if !found_merge {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r#"AAAA
BBCD
BBCC
EEEC"#;

    const INPUT_EX2: &str = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;

    const INPUT_EX3: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    const INPUT_EX4: &str = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;

    const INPUT_EX5: &str = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX1), 140);
        assert_eq!(solve_p1(INPUT_EX2), 772);
        assert_eq!(solve_p1(INPUT_EX3), 1930);
    }

    #[test]
    fn p2_test1() {
        assert_eq!(solve_p2(INPUT_EX1), 80);
        assert_eq!(solve_p2(INPUT_EX2), 436);
        assert_eq!(solve_p2(INPUT_EX3), 1206);
        assert_eq!(solve_p2(INPUT_EX4), 236);
        assert_eq!(solve_p2(INPUT_EX5), 368);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input12.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input));
    println!("Part2: {}", solve_p2(&input));
}
