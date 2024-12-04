use std::{env, fs};

use regex::Regex;

fn solve_p1(input: &str) -> i64 {
    let lines = input.lines();

    let mut total = 0;
    for s in lines {
        let re = Regex::new(r##"\d+"##).unwrap();
        let ints: Vec<_> = re
            .find_iter(s)
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();

        if is_safe_sequence(&ints) {
            total += 1;
        }
    }

    total
}

fn is_safe_sequence(ints: &[i64]) -> bool {
    let safe = {
        let mut safe = true;
        for i in 1..ints.len() {
            let diff = ints[i - 1] - ints[i];
            if diff >= 1 && diff <= 3 {
            } else {
                safe = false;
                break;
            }
        }
        safe
    };

    if safe {
        return true;
    }

    let safe = {
        let mut safe = true;
        for i in 1..ints.len() {
            let diff = ints[i] - ints[i - 1];
            if diff >= 1 && diff <= 3 {
            } else {
                safe = false;
                break;
            }
        }
        safe
    };

    if safe {
        return true;
    }

    false
}

fn solve_p2(input: &str) -> i64 {
    let lines = input.lines();

    let mut total = 0;
    for s in lines {
        let re = Regex::new(r##"\d+"##).unwrap();
        let ints: Vec<_> = re
            .find_iter(s)
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();

        for i in 0..ints.len() {
            let mut seq = ints.clone();
            seq.remove(i);
            if is_safe_sequence(&seq) {
                total += 1;
                break;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r#"7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX1), 2);
    }

    #[test]
    fn p2_test1() {
        assert_eq!(solve_p2(INPUT_EX1), 4);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input02.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input));
    println!("Part2: {}", solve_p2(&input));
}
