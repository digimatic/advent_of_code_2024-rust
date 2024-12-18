use advent_of_code_2024_rust::vec2i::Vec2i;
use regex::Regex;
use std::{
    collections::{HashSet, VecDeque},
    env, fs,
};

fn parse(input: &str) -> Vec<Vec2i> {
    let re = Regex::new(r##"\d+"##).unwrap();
    input
        .lines()
        .map(|line| {
            let xs: Vec<_> = re
                .find_iter(line.trim())
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .collect();
            Vec2i::new(xs[0], xs[1])
        })
        .collect()
}

fn solve_p1(input: &str, w: i32, n: usize) -> usize {
    let xs = parse(input);

    let bytes = &xs[0..n];
    let bytes = bytes
        .iter()
        .map(|p| Vec2i::new(p.x, p.y))
        .collect::<HashSet<_>>();

    shortest_path(&bytes, w, w, Vec2i::new(0, 0), Vec2i::new(w - 1, w - 1)).unwrap()
}

fn shortest_path(
    obstacles: &HashSet<Vec2i>,
    w: i32,
    h: i32,
    start: Vec2i,
    target: Vec2i,
) -> Option<usize> {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut visited = HashSet::new();

    let dirs = vec![
        Vec2i::new(0, 1),
        Vec2i::new(0, -1),
        Vec2i::new(1, 0),
        Vec2i::new(-1, 0),
    ];

    while !queue.is_empty() {
        let (pos, path_len) = queue.pop_front().unwrap();

        if pos == target {
            return Some(path_len);
        }

        if visited.contains(&pos) {
            continue;
        }

        visited.insert(pos.clone());

        for dir in &dirs {
            let next = &pos + dir;
            if next.x < 0 || next.x >= w || next.y < 0 || next.y >= h || obstacles.contains(&next) {
                continue;
            }

            queue.push_back((next.clone(), path_len + 1));
        }
    }

    None
}

fn solve_p2(input: &str, w: i32, i0: usize) -> String {
    let xs = parse(input);

    for i in i0..xs.len() {
        let bytes = &xs[0..i];
        let bytes = bytes
            .iter()
            .map(|p| Vec2i::new(p.x, p.y))
            .collect::<HashSet<_>>();

        let path_len = shortest_path(&bytes, w, w, Vec2i::new(0, 0), Vec2i::new(w - 1, w - 1));
        if path_len.is_none() {
            let p = &xs[i - 1];
            return format!("{},{}", p.x, p.y);
        }
    }
    panic!("No solution for the second part found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX1, 7, 12), 22);
    }

    #[test]
    fn p2_test1() {
        assert_eq!(solve_p2(INPUT_EX1, 7, 12), "6,1");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input18.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input, 71, 1024));
    println!("Part2: {}", solve_p2(&input, 71, 1024));
}
