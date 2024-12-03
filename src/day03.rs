use regex::Regex;
use std::{env, fs};

fn solve_p1(input: &str) -> i64 {
    let mut total = 0i64;

    let re = Regex::new(r##"mul\(\d+,\d+\)"##).unwrap();
    let es: Vec<_> = re.find_iter(input).map(|m| m.as_str()).collect();
    for e in es {
        let re = Regex::new(r"(\d+),(\d+)").unwrap();
        let caps = re.captures(e).unwrap();
        let n1 = caps[1].to_string().parse::<i64>().unwrap();
        let n2 = caps[2].to_string().parse::<i64>().unwrap();
        total += n1 * n2;
    }

    total
}

fn solve_p2(input: &str) -> i64 {
    let mut total = 0i64;

    let re = Regex::new(r##"mul\(\d+,\d+\)|don\'t\(\)|do\(\)"##).unwrap();
    let re2 = Regex::new(r"(\d+),(\d+)").unwrap();
    let es: Vec<_> = re.find_iter(input).map(|m| m.as_str()).collect();
    let mut enabled = true;
    for e in es {
        if e == "don't()" {
            enabled = false;
        } else if e == "do()" {
            enabled = true;
        } else {
            if enabled {
                let caps = re2.captures(e).unwrap();
                let n1 = caps[1].to_string().parse::<i64>().unwrap();
                let n2 = caps[2].to_string().parse::<i64>().unwrap();
                total += n1 * n2;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

    const INPUT_EX2: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX1), 161);
    }

    #[test]
    fn p2_test1() {
        assert_eq!(solve_p2(INPUT_EX2), 48);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input03.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input));
    println!("Part2: {}", solve_p2(&input));
}
