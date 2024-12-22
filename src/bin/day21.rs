use advent_of_code_2024_rust::vec2i::Vec2i;
use maplit::hashmap;
use std::{collections::{HashMap, HashSet}, env, fs};

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

fn all_variants(moves: Vec<Vec<Moves>>) -> Vec<Moves> {
    fn combine(current: &mut Moves, parts: &[Vec<Moves>], result: &mut Vec<Moves>) {
        if parts.is_empty() {
            result.push(current.clone());
            return;
        }

        for variant in &parts[0] {
            current.extend(variant);
            combine(current, &parts[1..], result);
            current.truncate(current.len() - variant.len());
        }
    }

    let mut result = Vec::new();
    combine(&mut Vec::new(), &moves, &mut result);
    result
}


fn solve_p1(input: &str, num_robots: usize) -> usize {
    let xs = parse(input);
    let num_pad = get_numpad();
    let dir_pad = get_dirpad();

    let mut sum = 0;
    for code in xs {
        print!("Code: {}", code);
        let code_chars = code.chars().collect::<Vec<_>>();
        let mut dirs1 = get_all_variants(&code_chars, &num_pad);

        for i in 0..num_robots {
            println!(" Iteration {}", i);
            let dirs2 = all_possible_moves(dirs1, &dir_pad);
            //println!("Shortest step 2: {}", dirs2_min);

            dirs1 = dirs2;
        }

        let dirs1_min = dirs1.iter().map(|xs| xs.len()).min().unwrap();
        let part_code = code[0..=2].parse::<usize>().unwrap();
        let prod = dirs1_min * part_code;
        sum += prod;

        // let dirs2 = get_directions(&dirs1, &dir_pad);
        // let dirs3 = get_directions(&dirs2, &dir_pad);
        // let dirs3 = String::from_iter(dirs3);
        // println!("{}", dirs3);
    }

    sum
}

fn all_possible_moves(dirs1: Vec<Moves>, dir_pad: &Pad) -> Vec<Moves> {
    let mut dirs2 = Vec::new();
    for dir1 in &dirs1 {
        let dirs2s = get_all_variants(dir1, dir_pad);
        // let dirs2s1 = dirs2s[0].clone();
        // dirs2.push(dirs2s1);
        dirs2.extend(dirs2s);
 
        // let shortest_len = dirs2s.iter().map(|variant| variant.len()).min().unwrap();
        // let dirs2s: Vec<_> = dirs2s.into_iter().filter(|variant| variant.len() == shortest_len).collect();
        // // println!("Shortest variant: {}", shortest_len);
        // // println!("{:?}", dirs2);
        // dirs2.push(dirs2s[0].clone());
    }

    let dirs2_min = dirs2.iter().map(|xs| xs.len()).min().unwrap_or(0);
    let dirs2: Vec<_> = dirs2.into_iter().filter(|variant| variant.len() <= dirs2_min).collect();
    println!("Num variants: {}", dirs2.len());
    let dirs2: HashSet<Moves> = dirs2.into_iter().collect();
    let dirs2: Vec<Moves> = dirs2.into_iter().collect();
    dirs2
}

fn get_all_variants(code_chars: &Vec<char>, num_pad: &HashMap<char, Vec2i>) -> Vec<Moves> {
    let dirs1 = get_directions(&code_chars, num_pad);
    // robot 1 contolling num pad
    let dirs1 = all_variants(dirs1);
    let dirs1: Vec<Moves> = dirs1.into_iter().map(|variant| variant.into_iter().rev().collect()).collect();
    dirs1
}

type Pad = HashMap<char, Vec2i>;
type Moves = Vec<char>;

fn get_directions(code: &[char], pad: &Pad) -> Vec<Vec<Moves>> {
    let current_key = 'A';
    let mut directions = Vec::new();
    let mut current_pos: Vec2i = pad[&current_key].clone();
    for key in code {
        let target_pos = &pad[key];
        let all_moves = get_directions_to_target(&current_pos, target_pos, pad);
        //println!("{:?}", all_moves);
        directions.push(all_moves);
        current_pos = target_pos.clone();
    }
    //println!("{:?}", &directions);

    directions
}

fn get_directions_to_target(current_pos: &Vec2i, target_pos: &Vec2i, pad: &Pad) -> Vec<Moves> {
    if current_pos == target_pos {
        return vec![ vec!['A'] ];
    }

    let invalid_pos = &pad[&'_'];

    let dir = target_pos.clone() - current_pos.clone();

    let mut dirs = vec![];

    if dir.x > 0 {
        let next = current_pos + Vec2i::new(1, 0);
        if next != *invalid_pos {
            let mut all_dirs = get_directions_to_target(&next, target_pos, pad);
            for dirs in &mut all_dirs {
                dirs.push('>');
            }
            dirs.extend(all_dirs);
        }
    }
    if dir.x < 0 {
        let next = current_pos + Vec2i::new(-1, 0);
        if next != *invalid_pos {
            let mut all_dirs = get_directions_to_target(&next, target_pos, pad);
            for dirs in &mut all_dirs {
                dirs.push('<');
            }
            dirs.extend(all_dirs);
        }
    }

    if dir.y > 0 {
        let next = current_pos + Vec2i::new(0, 1);
        if next != *invalid_pos {
            let mut all_dirs = get_directions_to_target(&next, target_pos, pad);
            for dirs in &mut all_dirs {
                dirs.push('v');
            }
            dirs.extend(all_dirs);
        }
    }
    if dir.y < 0 {
        let next = current_pos + Vec2i::new(0, -1);
        if next != *invalid_pos {
            let mut all_dirs = get_directions_to_target(&next, target_pos, pad);
            for dirs in &mut all_dirs {
                dirs.push('^');
            }
            dirs.extend(all_dirs);
        }
    }

    dirs
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn p1_test1() {
//     }

//     #[test]
//     fn p2_test1() {
//     }
// }

const INPUT_EX1: &str = r#"029A
980A
179A
456A
379A"#;

fn main() {
    assert_eq!(solve_p1(INPUT_EX1,2 ), 126384);
    // assert_eq!(solve_p2(INPUT_EX1,), );
    println!("All tests passed!");
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

    println!("Part1: {}", solve_p1(&input,2));
    println!("Part2: {}", solve_p1(&input,25));
}
