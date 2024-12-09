use std::{env, fs};

fn solve_p1(input: &str) -> usize {
    let blocks = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    let mut file = true;
    let mut id = 0;
    let mut disk: Vec<isize> = Vec::new();
    for i in 0..blocks.len() {
        let len = blocks[i];
        if file {
            for _ in 0..len {
                disk.push(id);
            }
            id += 1;
        } else {
            for _ in 0..len {
                disk.push(-1);
            }
        }
        file = !file;
    }

    let mut first_free = disk.iter().position(|&x| x == -1).unwrap();
    let mut last_used = disk.iter().rposition(|&x| x != -1).unwrap();

    while first_free < last_used {
        let last = disk[last_used];
        disk[first_free] = last;
        disk[last_used] = -1;

        while disk[last_used] == -1 {
            last_used -= 1;
        }

        while disk[first_free] != -1 {
            first_free += 1;
        }
    }

    let mut checksum: usize = 0;
    for i in 0..disk.len() {
        if disk[i] != -1 {
            checksum += i * (disk[i] as usize);
        }
    }

    checksum
}

fn solve_p2(input: &str) -> usize {
    let blocks = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    let mut file = true;
    let mut id = 0;
    let mut files: Vec<(isize, usize, usize)> = Vec::new(); // (id, pos, len), (-1, pos, len) is free
    let mut pos = 0;
    for i in 0..blocks.len() {
        let len = blocks[i];
        if file {
            if len > 0 {
                files.push((id, pos, len));
                pos += len;
            }
            id += 1;
        } else {
            if len > 0 {
                files.push((-1, pos, len));
                pos += len;
            }
        }
        file = !file;
    }

    id -= 1;

    while id >= 0 {
        let file_pos = files.iter().position(|(fid, _, _)| *fid == id).unwrap();
        let file = files[file_pos];

        let maybe_empty_pos = files
            .iter()
            .position(|&(fid, _pos, len)| fid == -1 && len >= file.2);
        if let Some(empty_pos) = maybe_empty_pos {
            let (_, epos, elen) = files[empty_pos];
            if epos < file.1 {
                files[file_pos].0 = -1;

                if file.2 < elen {
                    files.insert(empty_pos + 1, (-1, epos + file.2, elen - file.2));
                }

                files[empty_pos] = (id, epos, file.2);
            }
        }

        id -= 1;
    }

    let mut checksum: usize = 0;
    for file in files {
        if file.0 != -1 {
            for i in 0..file.2 {
                let block_id = file.1 + i;

                checksum += (file.0 as usize) * (block_id as usize);
            }
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r#"2333133121414131402"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX1), 1928);
    }

    #[test]
    fn p2_test1() {
        assert_eq!(solve_p2(INPUT_EX1), 2858);
    }
}

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input09.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input));
    println!("Part2: {}", solve_p2(&input));
}
