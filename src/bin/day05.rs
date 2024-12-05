use regex::Regex;
use std::{cmp::Ordering, collections::HashSet, env, fs};

fn check_rule(u: i64, v: i64, rules: &[(i64, i64)]) -> bool {
    for (l, r) in rules {
        if *l == v && *r == u {
            return false;
        }
    }
    true
}

fn is_valid_update(update: &[i64], rules: &[(i64, i64)]) -> bool {
    for i in 0..update.len() {
        let u = update[i];
        for j in i + 1..update.len() {
            let v = update[j];
            if !check_rule(u, v, rules) {
                return false;
            }
        }
    }
    true
}

fn solve_p1(input: &str) -> i64 {
    let lines = input.lines();
    let mut total = 0i64;
    let mut rules = Vec::new();
    let mut in_updates = false;
    for line in lines {
        if line.is_empty() {
            in_updates = true;
        } else if !in_updates {
            let re = Regex::new(r"(\d+)\|(\d+)").unwrap();
            let caps = re.captures(line).unwrap();
            let n1 = caps[1].to_string().parse::<i64>().unwrap();
            let n2 = caps[2].to_string().parse::<i64>().unwrap();
            rules.push((n1, n2));
        } else {
            let mut update = Vec::new();
            line.split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .for_each(|n| update.push(n));

            if is_valid_update(&update, &rules) {
                total += update[update.len() / 2];
            }
        }
    }

    total
}

fn reorder_update(update: &mut [i64], rules: &HashSet<(i64, i64)>) {
    update.sort_by(|&a, &b| {
        if a == b {
            Ordering::Equal
        } else if rules.contains(&(a, b)) {
            Ordering::Less
        } else if rules.contains(&(b, a)) {
            Ordering::Greater
        } else {
            panic!("Invalid rules found while reordering update: {:?}", rules)
        }
    });
}

fn solve_p2(input: &str) -> i64 {
    let lines = input.lines();
    let mut total = 0i64;
    let mut rules = Vec::new();
    let mut in_updates = false;
    let mut rules2 = HashSet::new();
    for line in lines {
        if line.is_empty() {
            in_updates = true;

            for &r in &rules {
                rules2.insert(r);
            }
        } else if !in_updates {
            let re = Regex::new(r"(\d+)\|(\d+)").unwrap();
            let caps = re.captures(line).unwrap();
            let n1 = caps[1].to_string().parse::<i64>().unwrap();
            let n2 = caps[2].to_string().parse::<i64>().unwrap();
            rules.push((n1, n2));
        } else {
            let mut update = Vec::new();
            line.split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .for_each(|n| update.push(n));

            if !is_valid_update(&update, &rules) {
                reorder_update(&mut update, &rules2);
                total += update[update.len() / 2];
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX1), 143);
    }

    #[test]
    fn p2_test1() {
        assert_eq!(solve_p2(INPUT_EX1), 123);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input05.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input));
    println!("Part2: {}", solve_p2(&input));
}
