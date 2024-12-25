use std::{env, fs};

type Heights = Vec<i64>;

fn parse(input: &str) -> (Vec<Heights>, Vec<Heights>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let mut first = true;
    let mut is_lock = true;
    let mut heights: Heights = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            first = true;
            if is_lock {
                locks.push(heights.clone());
            } else {
                heights = heights.into_iter().map(|h| h - 1).collect();
                keys.push(heights.clone());
            }
            continue;
        }
        if first {
            is_lock = line.starts_with('#');
            first = false;
            heights.clear();
            heights.resize(line.len(), 0);
            continue;
        }

        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                heights[i] += 1;
            }
        }
    }

    if is_lock {
        locks.push(heights.clone());
    } else {
        heights = heights.into_iter().map(|h| h - 1).collect();
        keys.push(heights.clone());
    }

    (locks, keys)
}

fn check_key(key: &[i64], lock: &[i64]) -> bool {
    for i in 0..key.len() {
        let key_height = key[i];
        let lock_free_height = 5 - lock[i];
        if key_height > lock_free_height {
            return false;
        }
    }
    true
}

fn solve_p1(input: &str) -> usize {
    let (locks, keys) = parse(input);

    let mut fit_count = 0;
    for lock in &locks {
        for key in &keys {
            let b = check_key(key, lock);
            if b {
                fit_count += 1;
            }
        }
    }

    fit_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX1), 3);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input25.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input));
}
