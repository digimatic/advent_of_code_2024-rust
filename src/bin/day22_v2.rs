use std::collections::HashMap;
use std::{collections::HashSet, env, fs};

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn solve_p1(input: &str, count: usize) -> usize {
    let xs = parse(input);

    let mut sum = 0;
    for x in xs {
        sum += generate(x, count)
    }

    sum
}

fn generate(mut x: usize, count: usize) -> usize {
    for _ in 0..count {
        x = generate_next(x);
    }
    x
}

fn generate_next(secret: usize) -> usize {
    let secret = prune(mix(secret, secret * 64));
    let secret = prune(mix(secret, secret / 32));
    let secret = prune(mix(secret, secret * 2048));
    secret
}

fn mix(x_1: usize, x_2: usize) -> usize {
    x_1 ^ x_2
}

fn prune(x: usize) -> usize {
    let x2 = x % 16777216;
    x2
}

fn generate_next_seq(x: usize) -> (usize, isize, isize) {
    let digit1 = (x % 10) as isize;

    let x2 = generate_next(x);
    let digit2 = (x2 % 10) as isize;
    let diff = digit2 - digit1;

    (x2, digit2, diff)
}

fn generate2(mut x: usize, count: usize) -> Vec<(usize, isize, Option<isize>)> {
    let mut price_change_list = vec![(x, (x % 10) as isize, None)];
    for _ in 0..count {
        let (x2, digit2, diff) = generate_next_seq(x);
        price_change_list.push((x2, digit2, Some(diff)));
        x = x2;
    }
    price_change_list
}

fn solve_p2(input: &str, count: usize) -> usize {
    let xs = parse(input);

    let mut buyer_seqs: Vec<(usize, Vec<(usize, isize, isize)>)> = Vec::new();

    for x in xs {
        let seq = generate2(x, count);
        let seq: Vec<_> = seq[1..]
            .into_iter()
            .map(|(x, y, z)| (*x, *y, z.unwrap()))
            .collect();
        buyer_seqs.push((x, seq));
    }

    let mut seq_to_total: HashMap<Vec<isize>, isize> = HashMap::new();
    for buyer_seq in &buyer_seqs {
        let mut seq_seen = HashSet::new();
        for w in buyer_seq.1.windows(4) {
            let seq: Vec<isize> = w.iter().map(|(_, _, d)| d.clone()).collect();
            if seq_seen.contains(&seq) {
                continue;
            }
            seq_seen.insert(seq.clone());

            seq_to_total
                .entry(seq)
                .and_modify(|total_price| *total_price += w[3].1)
                .or_insert(w[3].1);
        }
    }

    seq_to_total.into_values().max().unwrap() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r#"1
10
100
2024"#;

    const INPUT_EX2: &str = r#"1
2
3
2024"#;

    #[test]
    fn p1_test1() {
        assert_eq!(mix(42, 15), 37);
        assert_eq!(prune(100000000), 16113920);
        assert_eq!(generate(123, 1), 15887950);
        assert_eq!(generate(123, 5), 1553684);
        assert_eq!(generate(123, 10), 5908254);
        assert_eq!(solve_p1(INPUT_EX1, 2000), 37327623);
    }

    #[test]
    fn p2_test1() {
        assert_eq!(generate2(123, 0).last().unwrap(), &(123, 3, None));
        assert_eq!(generate2(123, 1).last().unwrap(), &(15887950, 0, Some(-3)));
        assert_eq!(generate2(123, 2).last().unwrap(), &(16495136, 6, Some(6)));
        assert_eq!(generate2(123, 9).last().unwrap(), &(7753432, 2, Some(-2)));
        assert_eq!(solve_p2(INPUT_EX2, 2000), 23);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input22.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input, 2000));
    println!("Part2: {}", solve_p2(&input, 2000));
}
