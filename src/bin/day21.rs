use advent_of_code_2024_rust::vec2i::Vec2i;
use itertools::Itertools;
use maplit::hashmap;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    env, fs,
    rc::Rc,
};

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn get_numpad() -> HashMap<char, Vec2i> {
    let num_pad: HashMap<char, Vec2i> = hashmap! {
        '_' => Vec2i::new(0, 3),
        '0' => Vec2i::new(1, 3),
        'A' => Vec2i::new(2, 3),
        '1' => Vec2i::new(0, 2),
        '2' => Vec2i::new(1, 2),
        '3' => Vec2i::new(2, 2),
        '4' => Vec2i::new(0, 1),
        '5' => Vec2i::new(1, 1),
        '6' => Vec2i::new(2, 1),
        '7' => Vec2i::new(0, 0),
        '8' => Vec2i::new(1, 0),
        '9' => Vec2i::new(2, 0),
    };
    num_pad
}

fn get_dirpad() -> HashMap<char, Vec2i> {
    let pad: HashMap<char, Vec2i> = hashmap! {
        '_' => Vec2i::new(0, 0),
        '^' => Vec2i::new(1, 0),
        'A' => Vec2i::new(2, 0),
        '<' => Vec2i::new(0, 1),
        'v' => Vec2i::new(1, 1),
        '>' => Vec2i::new(2, 1),
    };
    pad
}

type Pad = HashMap<char, Vec2i>;
type Moves = VecDeque<char>;

fn get_directions_to_target(target_start: &Vec2i, target_end: &Vec2i, pad: &Pad) -> Vec<Moves> {
    if target_start == target_end {
        let mut moves = Moves::new();
        moves.push_back('A');
        return vec![moves];
    }

    let invalid_pos = &pad[&'_'];

    let dir = target_end.clone() - target_start.clone();

    let mut dirs = VecDeque::new();
    if dir.x > 0 {
        for _ in 0..dir.x.abs() {
            dirs.push_back('>')
        }
    }
    if dir.x < 0 {
        for _ in 0..dir.x.abs() {
            dirs.push_back('<')
        }
    }
    if dir.y > 0 {
        for _ in 0..dir.y.abs() {
            dirs.push_back('v')
        }
    }
    if dir.y < 0 {
        for _ in 0..dir.y.abs() {
            dirs.push_back('^')
        }
    }

    let n = dirs.len();
    let valid_moves: Vec<VecDeque<_>> = dirs
        .into_iter()
        .permutations(n)
        .unique()
        .filter_map(|moves| {
            let mut next = target_start.clone();
            for move_char in &moves {
                match move_char {
                    '>' => next.x += 1,
                    '<' => next.x -= 1,
                    'v' => next.y += 1,
                    '^' => next.y -= 1,
                    _ => unreachable!(),
                }
                if next == *invalid_pos || !pad.values().contains(&next) {
                    return None;
                }
            }
            let mut current: Moves = moves.into_iter().collect();
            current.push_back('A');
            Some(current)
        })
        .collect();
    valid_moves
}

fn solve(input: &str, num_robots: usize) -> usize {
    let xs = parse(input);
    let num_pad = get_numpad();
    let dir_pad = get_dirpad();

    let pads = vec![num_pad, dir_pad];

    let shortest_move_len_cache = Rc::new(RefCell::new(HashMap::new()));

    let mut sum = 0;
    for code in xs {
        let code_chars = code.chars().collect::<VecDeque<_>>();

        let min_len = get_shortest_move_len(
            &code_chars,
            num_robots,
            &pads,
            1,
            0,
            shortest_move_len_cache.clone(),
        );

        let part_code = code[0..=2].parse::<usize>().unwrap();
        let prod = min_len * part_code;
        sum += prod;
    }
    sum
}

fn get_shortest_move_len(
    code_chars: &VecDeque<char>,
    num_robots: usize,
    pads: &[Pad],
    source_pad: usize,
    target_pad: usize,
    shortest_move_len_cache: Rc<RefCell<HashMap<(VecDeque<char>, usize, usize, usize), usize>>>,
) -> usize {
    let key = (code_chars.clone(), num_robots, source_pad, target_pad);
    if let Some(&len) = shortest_move_len_cache.borrow().get(&key) {
        return len;
    }

    let mut code_chars2 = vec!['A'];
    code_chars2.extend(code_chars);

    let mut total_length = 0;
    for w in code_chars2.windows(2) {
        let target_start_char = w[0];
        let target_end_char = w[1];
        let target_start = &pads[target_pad][&target_start_char];
        let target_end = &pads[target_pad][&target_end_char];

        let all_paths = get_directions_to_target(target_start, target_end, &pads[target_pad]);
        if num_robots == 0 {
            total_length += all_paths.iter().map(|path| path.len()).min().unwrap();
        } else {
            total_length += all_paths
                .iter()
                .map(|path| {
                    get_shortest_move_len(
                        path,
                        num_robots - 1,
                        pads,
                        source_pad,
                        source_pad,
                        shortest_move_len_cache.clone(),
                    )
                })
                .min()
                .unwrap();
        }
    }

    shortest_move_len_cache
        .borrow_mut()
        .insert(key, total_length);

    total_length
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r#"029A
980A
179A
456A
379A"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve(INPUT_EX1, 2), 126384);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input21.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve(&input, 2));
    println!("Part2: {}", solve(&input, 25));
}
