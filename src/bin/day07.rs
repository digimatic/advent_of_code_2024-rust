use regex::Regex;
use std::{env, fs};

fn solve_p1(input: &str) -> i64 {
    let lines = input.lines();
    let mut total = 0i64;
    for line in lines {
        let re = Regex::new(r##"\d+"##).unwrap();
        let ints: Vec<_> = re
            .find_iter(line)
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();

        let test_value = ints[0];
        let mut variants = Vec::new();
        variants.push(ints[1]);
        for i in 2..ints.len() {
            let mut variants_new = Vec::new();
            for variant in &variants {
                variants_new.push(variant + &ints[i]);
                variants_new.push(variant * &ints[i]);
            }
            variants = variants_new;
        }

        if variants.contains(&test_value) {
            total += test_value;
        }
    }

    total
}

fn solve_p2(input: &str) -> i64 {
    let lines = input.lines();
    let mut total = 0i64;
    for line in lines {
        let re = Regex::new(r##"\d+"##).unwrap();
        let ints: Vec<_> = re
            .find_iter(line)
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();

        let test_value = ints[0];
        let mut variants = Vec::new();
        variants.push(ints[1]);
        for i in 2..ints.len() {
            let mut variants_new = Vec::new();
            for variant in &variants {
                variants_new.push(variant + &ints[i]);

                variants_new.push(variant * &ints[i]);

                let v3 = variant.to_string() + &(&ints[i]).to_string();
                let v3 = v3.parse::<i64>().unwrap();
                variants_new.push(v3);
            }
            variants = variants_new;
        }

        if variants.contains(&test_value) {
            total += test_value;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX1), 3749);
    }
    #[test]
    fn p2_test1() {
        assert_eq!(solve_p2(INPUT_EX1), 11387);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input07.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input));
    println!("Part2: {}", solve_p2(&input));
}
