use advent_of_code_2024_rust::{map::Map, vec2i::Vec2i};
use regex::Regex;
use std::{collections::HashMap, collections::HashSet, env, fs};

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let lines: Vec<_> = input.lines().collect();
    let re = Regex::new(r##"[wubrg]+"##).unwrap();
    let patterns: Vec<&str> = re.find_iter(lines[0]).map(|m| m.as_str()).collect();
    let designs = &lines[2..];
    let designs = designs.to_vec();
    (patterns, designs)
}

fn solve_p1(input: &str) -> usize {
    let (patterns, designs) = parse(input);

    let mut cache = HashMap::new();
    designs
        .iter()
        .filter(|design| match_pattern(design, &patterns, &mut cache))
        .count()
}

fn match_pattern<'a>(
    design: &'a str,
    patterns: &[&'a str],
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    if design.is_empty() {
        return true;
    }

    if cache.contains_key(design) {
        return cache[design];
    }

    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            if match_pattern(rest, patterns, cache) {
                cache.insert(design, true);
                return true;
            }
        }
    }

    cache.insert(design, false);
    false
}

fn solve_p2(input: &str) -> usize {
    let (patterns, designs) = parse(input);

    let mut cache = HashMap::new();
    designs
        .iter()
        .map(|design| match_pattern2(design, &patterns, &mut cache))
        .sum()
}

fn match_pattern2<'a>(
    design: &'a str,
    patterns: &[&'a str],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if cache.contains_key(design) {
        return cache[design];
    }

    let mut count = 0;
    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            count += match_pattern2(rest, patterns, cache);
        }
    }

    cache.insert(design, count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX1), 6);
    }

    #[test]
    fn p2_test1() {
        assert_eq!(solve_p2(INPUT_EX1), 16);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input19.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input));
    println!("Part2: {}", solve_p2(&input));
}
