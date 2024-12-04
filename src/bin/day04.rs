use std::{env, fs};

//---------
// Part 1

const WORD: &'static str = "XMAS";

fn is_word_at_dir(x: i32, y: i32, dx: i32, dy: i32, m: &[Vec<char>], w: i32, h: i32) -> i32 {
    for i in 0..4 {
        let x2 = x + dx * i;
        let y2 = y + dy * i;
        if x2 < 0 || x2 >= w || y2 < 0 || y2 >= h {
            return 0;
        }
        if m[(y + dy * i) as usize][(x + dx * i) as usize]
            != WORD.chars().collect::<Vec<_>>()[i as usize]
        {
            return 0;
        }
    }
    1
}

fn is_word_at(x: i32, y: i32, m: &[Vec<char>], w: i32, h: i32) -> i32 {
    is_word_at_dir(x, y, 1, 0, &m, w, h)
        + is_word_at_dir(x, y, -1, 0, &m, w, h)
        + is_word_at_dir(x, y, 0, 1, &m, w, h)
        + is_word_at_dir(x, y, 0, -1, &m, w, h)
        + is_word_at_dir(x, y, 1, 1, &m, w, h)
        + is_word_at_dir(x, y, 1, -1, &m, w, h)
        + is_word_at_dir(x, y, -1, 1, &m, w, h)
        + is_word_at_dir(x, y, -1, -1, &m, w, h)
}

fn solve_p1(input: &str) -> i64 {
    let m: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut total = 0;

    let h = m.len() as i32;
    let w = m[0].len() as i32;

    for y in 0..h {
        for x in 0..w {
            total += is_word_at(x as i32, y as i32, &m, w, h);
        }
    }

    total as i64
}

//---------
// Part 2

const XWORD: &'static str = "MAS";

fn is_xword_at_dir(x: i32, y: i32, dx: i32, dy: i32, m: &[Vec<char>], w: i32, h: i32) -> bool {
    let word = XWORD.chars().collect::<Vec<_>>();

    for i in -1..=1 {
        let x2 = x + dx * i;
        let y2 = y + dy * i;
        if x2 < 0 || x2 >= w || y2 < 0 || y2 >= h {
            return false;
        }
        let c = word[(i + 1) as usize];
        let row = &m[y2 as usize];
        let cc = row[x2 as usize];
        if cc != c {
            return false;
        }
    }

    true
}

fn is_xword_dir_at(
    x: i32,
    y: i32,
    dx1: i32,
    dy1: i32,
    dx2: i32,
    dy2: i32,
    m: &[Vec<char>],
    w: i32,
    h: i32,
) -> i32 {
    let mut total = 0;

    total += (is_xword_at_dir(x, y, dx1, dy1, m, w, h) && is_xword_at_dir(x, y, dx2, dy2, m, w, h))
        as i32;

    total += (is_xword_at_dir(x, y, -dx1, -dy1, m, w, h)
        && is_xword_at_dir(x, y, dx2, dy2, m, w, h)) as i32;

    total += (is_xword_at_dir(x, y, dx1, dy1, m, w, h)
        && is_xword_at_dir(x, y, -dx2, -dy2, m, w, h)) as i32;

    total += (is_xword_at_dir(x, y, -dx1, -dy1, m, w, h)
        && is_xword_at_dir(x, y, -dx2, -dy2, m, w, h)) as i32;

    total
}

fn solve_p2(input: &str) -> i64 {
    let m: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let h = m.len() as i32;
    let w = m[0].len() as i32;

    let mut total = 0;

    for y in 0..h {
        for x in 0..w {
            total += is_xword_dir_at(x, y, 1, 1, 1, -1, &m, w, h);
        }
    }

    total as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE1: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EXAMPLE1), 18);
    }

    #[test]
    fn p2_test1() {
        assert_eq!(solve_p2(INPUT_EXAMPLE1), 9);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input04.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input));
    println!("Part2: {}", solve_p2(&input));
}
