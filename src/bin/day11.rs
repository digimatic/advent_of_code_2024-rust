use regex::Regex;
use std::{collections::HashMap, env, fs};

fn solve_p1(input: &str, count: usize) -> u64 {
    let re = Regex::new(r##"\d+"##).unwrap();
    let mut ints: Vec<u64> = re
        .find_iter(input.trim())
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect();

    for _ in 0..count {
        let n = ints.len();
        for i in 0..n {
            let j = n - 1 - i;

            let x = ints[j];

            if x == 0 {
                ints[j] = 1;
            } else if x.to_string().len() % 2 == 0 {
                let xs = x.to_string();
                let (x1s, x2s) = xs.split_at(xs.len() / 2);
                ints[j] = x2s.parse::<u64>().unwrap();
                ints.insert(j, x1s.parse::<u64>().unwrap());
            } else {
                ints[j] = x * 2024;
            }
        }
    }

    ints.len() as u64
}

fn run(x: u64, count: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    if count == 0 {
        return 1;
    }

    if cache.contains_key(&(x, count)) {
        return cache[&(x, count)];
    }

    let mut sum = 0;

    if x == 0 {
        sum += run(1, count - 1, cache);
    } else if x.to_string().len() % 2 == 0 {
        let xs = x.to_string();
        let (x1s, x2s) = xs.split_at(xs.len() / 2);
        sum += run(x1s.parse::<u64>().unwrap(), count - 1, cache);
        sum += run(x2s.parse::<u64>().unwrap(), count - 1, cache);
    } else {
        sum += run(x * 2024, count - 1, cache);
    }

    cache.insert((x, count), sum);

    sum
}

fn solve_p2(input: &str, count: usize) -> usize {
    let re = Regex::new(r##"\d+"##).unwrap();
    let ints: Vec<u64> = re
        .find_iter(input.trim())
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect();

    let mut sum = 0;
    let mut cache = HashMap::new();
    for x in ints {
        sum += run(x, count, &mut cache);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r#"125 17"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX1, 25), 55312);
    }

    #[test]
    fn p2_test1() {
        assert_eq!(solve_p2(INPUT_EX1, 25), 55312);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input11.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input, 25));
    println!("Part2: {}", solve_p2(&input, 75));
}
