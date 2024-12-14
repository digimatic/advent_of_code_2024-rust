use advent_of_code_2024_rust::{map::Map, vec2i::Vec2i};
use regex::Regex;
use std::{collections::HashSet, env, fs};

fn solve_p1(input: &str, width: i32, height: i32, steps: i32) -> i64 {
    let mut xs = parse(input);

    for _ in 0..steps {
        step(&mut xs, width, height);
    }

    let (quadrant1_count, quadrant2_count, quadrant3_count, quadrant4_count) =
        count_quadrants(&xs, width, height);
    let r = quadrant1_count * quadrant2_count * quadrant3_count * quadrant4_count;
    r
}

fn parse(input: &str) -> Vec<(Vec2i, Vec2i)> {
    let lines = input.lines();
    let re = Regex::new(r##"\-?\d+"##).unwrap();
    let mut xs: Vec<(Vec2i, Vec2i)> = Vec::new();
    for line in lines {
        let ints: Vec<_> = re
            .find_iter(line)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect();
        xs.push((
            Vec2i::from((ints[0], ints[1])),
            Vec2i::from((ints[2], ints[3])),
        ));
    }
    xs
}

fn count_quadrants(xs: &Vec<(Vec2i, Vec2i)>, width: i32, height: i32) -> (i64, i64, i64, i64) {
    let mut quadrant1_count = 0;
    let mut quadrant2_count = 0;
    let mut quadrant3_count = 0;
    let mut quadrant4_count = 0;
    for (p, _) in xs {
        if p.x < width / 2 && p.y < height / 2 {
            quadrant1_count += 1;
        } else if p.x >= (1 + (width / 2)) && p.y < height / 2 {
            quadrant2_count += 1;
        } else if p.x < width / 2 && p.y >= (1 + (height / 2)) {
            quadrant3_count += 1;
        } else if p.x >= (1 + (width / 2)) && p.y >= (1 + (height / 2)) {
            quadrant4_count += 1;
        } else {
        }
    }
    (
        quadrant1_count,
        quadrant2_count,
        quadrant3_count,
        quadrant4_count,
    )
}

fn has_cluster(xs: &Vec<(Vec2i, Vec2i)>, num_in_line: usize) -> bool {
    let pixels: HashSet<_> = xs.iter().map(|(p, _)| p).collect();
    pixels
        .iter()
        .any(|&p| (0..num_in_line).all(|i| pixels.contains(&(p + &Vec2i::from((i as i32, 0))))))
}

fn step(xs: &mut Vec<(Vec2i, Vec2i)>, width: i32, height: i32) {
    for (p, v) in xs {
        let mut p2 = &*p + &*v;
        while p2.x < 0 {
            p2.x = p2.x + width;
        }
        while p2.y < 0 {
            p2.y = p2.y + height;
        }
        p2.x %= width;
        p2.y %= height;
        *p = p2;
    }
}

fn solve_p2(input: &str, width: i32, height: i32) -> usize {
    let mut xs = parse(input);

    let mut steps = 0;
    loop {
        steps += 1;
        {
            step(&mut xs, width, height);
        }

        if has_cluster(&xs, 10) {
            println!("After {} steps:", steps);
            print_map(width, height, &xs);
            return steps;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX1, 11, 7, 100), 12);
    }

    #[test]
    fn p2_test1() {}
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input14.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input, 101, 103, 100));
    println!("Part2: {}", solve_p2(&input, 101, 103));
}

fn print_map(width: i32, height: i32, xs: &Vec<(Vec2i, Vec2i)>) {
    let line = vec!['.'; width as usize];
    let mut map: Map = Map {
        w: width,
        h: height,
        m: vec![line; height as usize],
    };
    for (p, _) in xs {
        let c = map.read(p).unwrap_or('.');
        if c != '.' {
            let c = 1 + (c as u8);
            let c = c as char;
            map.write(p, c);
        } else {
            map.write(p, '1');
        }
    }
    for lines in map.m {
        println!("{}", lines.into_iter().collect::<String>());
    }
}
