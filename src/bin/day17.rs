use regex::Regex;
use std::{env, fs};

fn parse(input: &str) -> (u64, u64, u64, Vec<u64>) {
    let re = Regex::new(r##"\d+"##).unwrap();
    let xs: Vec<_> = input
        .lines()
        .map(|line| {
            re.find_iter(line.trim())
                .map(|m| m.as_str().parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();
    (xs[0][0], xs[1][0], xs[2][0], xs[4].clone())
}

fn solve_p1(input: &str) -> String {
    let (a, b, c, isl) = parse(input);

    let output = run_program(a, b, c, &isl, false);

    output
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn run_program(
    mut a: u64,
    mut b: u64,
    mut c: u64,
    isl: &[u64],
    one_output: bool, // for part 2
) -> Vec<u64> {
    let mut i: u64 = 0;
    let mut output = Vec::new();
    loop {
        let opcode = isl[i as usize];
        let operand = isl[(i + 1) as usize];

        i += 2;

        let val = match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => a,
            5 => b,
            6 => c,
            _ => unreachable!(),
        };

        match opcode {
            0 => a = a / (1 << val),
            1 => b = b ^ operand,
            2 => {
                b = val % 8;
            }
            3 => {
                if a != 0 {
                    i = operand
                }
            }
            4 => {
                b = b ^ c;
            }
            5 => {
                output.push(val % 8);
                if one_output {
                    return output;
                }
            }
            6 => b = a / (1 << val),
            7 => c = a / (1 << val),
            _ => unreachable!(),
        }

        if i >= (isl.len() as u64) {
            break;
        }
    }
    output
}

/*
Program: 2,4,1,2,7,5,4,1,1,3,5,5,0,3,3,0

 2,4    b = a % 8
 1,2    b = b ^ 2
 7,5    c = a / (1<<b)
 4,1    b = b ^ c
 1,3    b = b ^ 3
 5,5    output b % 8
 0,3    a = a / (1 << 3)
 3,0    if a != 0 goto start
 */

fn solve_p2(input: &str) -> u64 {
    let (_, _, _, isl) = parse(input);

    let mut possible_start_as = vec![0];

    for &expected_output in isl.iter().rev() {
        let mut next_possible_start_a = Vec::new();
        for a in possible_start_as {
            for i in 0..=7 {
                let next_a = (a<<3) | i;
                let output = run_program(next_a, 0, 0, &isl, true)[0];
                if output == expected_output {
                    next_possible_start_a.push(next_a);
                }
            }
        }
        possible_start_as = next_possible_start_a;
    }

    let r = possible_start_as.into_iter().min().unwrap();
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    /*
    0 3  a = a / 8
    5 4  output a % 8
    3 0  if a!= 0 goto start
    */
    const INPUT_EX2: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX1), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn p2_test1() {
        assert_eq!(solve_p2(INPUT_EX2), 117440);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input17.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input));
    println!("Part2: {}", solve_p2(&input));
}
