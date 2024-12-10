use std::{collections::HashSet, env, fs};

use advent_of_code_2024_rust::{
    map::{read_map, Map},
    vec2i::Vec2i,
};

fn solve_p1(input: &str) -> usize {
    let map = read_map(input);
    let start_positions = start_positions(&map);

    let mut trails = 0;
    for start_pos in start_positions {
        trails += walk_from(&map, start_pos);
    }

    trails
}

fn walk_from(map: &Map, start_pos: Vec2i) -> usize {
    let mut visited = HashSet::new();

    let mut stack = Vec::new();
    let mut count = 0;

    stack.push((start_pos, 0));

    while !stack.is_empty() {
        if let Some((p, h)) = stack.pop() {
            if map.inside(&p) {
                let hh = map.read(&p).unwrap_or_default().to_digit(10).unwrap() as u8;
                if h == hh && !visited.contains(&p) {
                    visited.insert(p.clone());
                    if hh == 9 {
                        count += 1;
                    } else {
                        let hn = h + 1;
                        let pr = &p + &Vec2i::from((1, 0));
                        let pl = &p + &Vec2i::from((-1, 0));
                        let pu = &p + &Vec2i::from((0, -1));
                        let pd = &p + &Vec2i::from((0, 1));
                        stack.push((pr, hn));
                        stack.push((pl, hn));
                        stack.push((pu, hn));
                        stack.push((pd, hn));
                    }
                }
            }
        }
    }

    count
}

fn start_positions(map: &Map) -> Vec<Vec2i> {
    let mut start_positions = Vec::new();
    for y in 0..map.h {
        for x in 0..map.w {
            let h = map.read((x, y)).unwrap();
            if h == '0' {
                start_positions.push(Vec2i::from((x, y)));
            }
        }
    }
    start_positions
}

fn solve_p2(input: &str) -> usize {
    let map = read_map(input);
    let start_positions = start_positions(&map);

    let mut trails = 0;
    for start_pos in start_positions {
        trails += walk_from2(&map, start_pos);
    }

    trails
}

fn walk_from2(map: &Map, start_pos: Vec2i) -> usize {
    let mut stack = Vec::new();
    let mut count = 0;

    stack.push((start_pos, 0));

    while !stack.is_empty() {
        if let Some((p, h)) = stack.pop() {
            if map.inside(&p) {
                let hh = map.read(&p).unwrap_or_default().to_digit(10).unwrap() as u8;
                if h == hh {
                    if hh == 9 {
                        count += 1;
                    } else {
                        let hn = h + 1;
                        let pr = &p + &Vec2i::from((1, 0));
                        let pl = &p + &Vec2i::from((-1, 0));
                        let pu = &p + &Vec2i::from((0, -1));
                        let pd = &p + &Vec2i::from((0, 1));
                        stack.push((pr, hn));
                        stack.push((pl, hn));
                        stack.push((pu, hn));
                        stack.push((pd, hn));
                    }
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_P1EX1: &str = r#"0123
1234
8765
9876"#;

    const INPUT_P1EX5: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    const INPUT_P2EX1: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_P1EX1), 1);
        assert_eq!(solve_p1(INPUT_P1EX5), 36);
    }

    #[test]
    fn p2_test1() {
        assert_eq!(solve_p2(INPUT_P2EX1), 81);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input10.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input));
    println!("Part2: {}", solve_p2(&input));
}
