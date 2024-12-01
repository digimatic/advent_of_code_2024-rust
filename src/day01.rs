use regex::Regex;
use std::{env, fs};

fn solve_p1(input: &str) -> i64 {
    let lines = input.lines();
    let mut total = 0i64;
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();
    for line in lines {
        let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
        let caps = re.captures(line).unwrap();
        let n1 = caps[1].to_string().parse::<i64>().unwrap();
        let n2 = caps[2].to_string().parse::<i64>().unwrap();
        l1.push(n1);
        l2.push(n2);
    }

    while !l1.is_empty() {
        let n1 = (&l1).into_iter().min().unwrap();
        let n2 = (&l2).into_iter().min().unwrap();
        let d = (n1 - n2).abs();
        total += d;
        let i1 = l1.iter().position(|x| *x == *n1).unwrap();
        l1.remove(i1);
        let i2 = l2.iter().position(|x| *x == *n2).unwrap();
        l2.remove(i2);
    }

    total
}

fn solve_p2(input: &str) -> i64 {
    let lines = input.lines();
    let mut total = 0i64;
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();
    for line in lines {
        let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
        let caps = re.captures(line).unwrap();
        let n1 = caps[1].to_string().parse::<i64>().unwrap();
        let n2 = caps[2].to_string().parse::<i64>().unwrap();
        l1.push(n1);
        l2.push(n2);
    }

    for n1 in l1 {
        let count = (&l2).into_iter().filter(|&&x| x == n1).count() as i64;
        total += count * n1;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test1() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        assert_eq!(solve_p1(input), 11);
    }

    #[test]
    fn p2_test1() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        assert_eq!(solve_p2(input), 31);
    }
}

fn main() {
    let input_file = env::args().nth(1).unwrap_or("input01.txt".into());
    let input = fs::read_to_string(&input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input));
    println!("Part2: {}", solve_p2(&input));
}
