use advent_of_code_2024_rust::vec2i::Vec2i;
use regex::Regex;
use std::{env, fs};

// Part 1

fn solve_p1(input: &str) -> i64 {
    let lines = input.lines();

    let re = Regex::new(r##"\d+"##).unwrap();
    let mut xs: Vec<(i32, i32)> = Vec::new();
    for line in lines {
        let ints: Vec<_> = re
            .find_iter(line)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect();
        if ints.len() == 2 {
            xs.push((ints[0], ints[1]));
        }
    }

    let mut total_cost = 0;
    for i in (0..xs.len()).step_by(3) {
        let button_a: Vec2i = xs[i].into();
        let button_b: Vec2i = xs[i + 1].into();
        let prize: Vec2i = xs[i + 2].into();
        let best = solve(button_a, button_b, prize);
        if let Some((_, _, cost)) = best {
            total_cost += cost as i64;
        }
    }

    total_cost
}

fn solve(button_a: Vec2i, button_b: Vec2i, prize: Vec2i) -> Option<(i32, i32, i32)> {
    let mut best = None;
    for a in 0..=100 {
        for b in 0..=100 {
            let x = a * button_a.x + b * button_b.x;
            let y = a * button_a.y + b * button_b.y;
            if Vec2i::from((x, y)) == prize {
                if let Some((_, _, best_cost)) = best {
                    if best_cost < (a * 3 + b) {
                        best = Some((a, b, a * 3 + b));
                    }
                } else {
                    best = Some((a, b, a * 3 + b));
                }
            }
        }
    }

    best
}

// Part 2

fn solve_p2(input: &str, adds: i64) -> i64 {
    let lines = input.lines();

    let re = Regex::new(r##"\d+"##).unwrap();
    let mut xs: Vec<(i64, i64)> = Vec::new();
    for line in lines {
        let ints: Vec<_> = re
            .find_iter(line)
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();
        if ints.len() == 2 {
            xs.push((ints[0], ints[1]));
        }
    }

    let mut total_cost = 0;
    for i in (0..xs.len()).step_by(3) {
        let button_a = xs[i];
        let button_b = xs[i + 1];
        let prize = xs[i + 2];
        let cost = solve2(
            (button_a.0, button_a.1),
            (button_b.0, button_b.1),
            (prize.0 + adds, prize.1 + adds),
        );
        total_cost += cost;
    }

    total_cost
}

fn linear_solve(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64) -> Option<(i64, i64)> {
    let k = a * d - b * c;
    if k == 0 {
        return None;
    }

    let ai = d;
    let bi = -b;
    let ci = -c;
    let di = a;

    let ra = (e * ai + f * bi) % k;
    let rb = (e * ci + f * di) % k;
    if ra == 0 && rb == 0 {
        let ra = (e * ai + f * bi) / k;
        let rb = (e * ci + f * di) / k;
        return Some((ra, rb));
    }
    return None;
}

fn solve2(button_a: (i64, i64), button_b: (i64, i64), prize: (i64, i64)) -> i64 {
    let presses = linear_solve(
        button_a.0, button_b.0, button_a.1, button_b.1, prize.0, prize.1,
    );

    presses.map(|(a, b)| a * 3 + b).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX1), 480);
    }

    #[test]
    fn p2_test1() {
        assert_eq!(solve_p2(INPUT_EX1, 0), 480);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input13.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input));
    println!("Part2: {}", solve_p2(&input, 10000000000000i64));
}
