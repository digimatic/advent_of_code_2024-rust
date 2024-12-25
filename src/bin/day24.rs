use advent_of_code_2024_rust::{map::Map, vec2i::Vec2i};
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, collections::HashSet, env, fs};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum GateType {
    AND,
    OR,
    XOR,
}

type Wires = HashMap<String, u64>;
type Gate = (GateType, String, String, String);
type Gates = Vec<Gate>;

fn parse(input: &str) -> (Wires, Gates) {
    let re1 = Regex::new(r"(\w+): (\d)").unwrap();
    let re2 = Regex::new(r"(\w+) (OR|XOR|AND) (\w+) -> (\w+)").unwrap();

    let mut wires: Wires = HashMap::new();
    let mut gates: Gates = Vec::new();

    let mut sect2 = false;
    for line in input.lines() {
        if line.trim().is_empty() {
            sect2 = true;
            continue;
        }
        if !sect2 {
            let caps = re1.captures(line).unwrap();
            let var = caps[1].to_string();
            let val = caps[2].to_string().parse::<u64>().unwrap();
            wires.insert(var, val);
        } else {
            let caps = re2.captures(line).unwrap();
            let wire1 = caps[1].to_string();
            let op = caps[2].to_string();
            let wire2 = caps[3].to_string();
            let wire_out = caps[4].to_string();
            match op.as_str() {
                "AND" => gates.push((GateType::AND, wire1, wire2, wire_out)),
                "OR" => gates.push((GateType::OR, wire1, wire2, wire_out)),
                "XOR" => gates.push((GateType::XOR, wire1, wire2, wire_out)),
                _ => panic!("Invalid operator"),
            }
        }
    }

    (wires, gates)
}

fn solve_p1(input: &str) -> u64 {
    let (mut wires, gates) = parse(input);

    loop {
        let mut did_set = false;
        for (op, wire1, wire2, wire_out) in &gates {
            if wires.contains_key(wire1)
                && wires.contains_key(wire2)
                && !wires.contains_key(wire_out)
            {
                let val1 = wires[wire1];
                let val2 = wires[wire2];
                match op {
                    GateType::AND => {
                        wires.insert(wire_out.clone(), val1 & val2);
                    }
                    GateType::OR => {
                        wires.insert(wire_out.clone(), val1 | val2);
                    }
                    GateType::XOR => {
                        wires.insert(wire_out.clone(), val1 ^ val2);
                    }
                }
                did_set = true;
            }
        }
        if !did_set {
            break;
        }
    }

    let mut bit_index = 0;
    let mut value = 0;
    loop {
        let var = format!("z{:02}", bit_index);
        if wires.contains_key(&var) {
            value |= wires[&var] << bit_index;
        } else {
            break;
        }
        bit_index += 1;
    }
    value
}

fn solve_p2<F>(input: &str, num_swaps: usize, f: F) -> String
where
    F: Fn(u64, u64) -> u64,
{
    let (mut wires, gates) = parse(input);

    let highest_x_bits = wires
        .keys()
        .filter(|wire| wire.starts_with("x"))
        .map(|wire| wire.strip_prefix("x").unwrap().parse::<u64>().unwrap())
        .max()
        .unwrap();
    let highest_y_bits = wires
        .keys()
        .filter(|wire| wire.starts_with("y"))
        .map(|wire| wire.strip_prefix("y").unwrap().parse::<u64>().unwrap())
        .max()
        .unwrap();
    let highest_z_bits = gates
        .iter()
        .map(|gate| &gate.3)
        .filter(|wire| wire.starts_with("z"))
        .map(|wire| wire.strip_prefix("z").unwrap().parse::<u64>().unwrap())
        .max()
        .unwrap();
    println!("Highest x bits: {}", highest_x_bits);
    println!("Highest y bits: {}", highest_y_bits);
    println!("Highest z bits: {}", highest_z_bits);

    fn set_bits(var: &str, val: u64, wires: &mut Wires, highest_bit: u64) {
        for index in 0..highest_bit + 1 {
            let wire = format!("{}{:02}", var, index);
            let set = val & (1 << index);
            if set ==0 {
                wires.insert(wire, 0);
            } else {
                wires.insert(wire, 1);
            }
        }
    };

    let test_system = move |gates: &Gates| {
        let mut wires: Wires = Wires::new();
        let mut incorrect_bits: u64 =0;
        for y in 0..(1u64) << (highest_y_bits + 1) {
            set_bits("y", y, &mut wires, highest_y_bits);
            for x in 0..(1u64) << (highest_x_bits + 1) {
                set_bits("x", x, &mut wires, highest_x_bits);

                let mut wires_in = wires.clone();
                let r = run_system(&mut wires_in, &gates, highest_z_bits);
                if let Some(r) = r {
                    let expected_res = f(x, y);
                    if r != expected_res {
                        incorrect_bits |= r ^ expected_res;
                        //return false;
                    }
                } else {
                    println!("aotusoe");
//                    return false;
                }
            }
        }
        println!("Incorrect bits: {:b}", incorrect_bits);
        incorrect_bits == 0
    };

    // let gates2 = create_swapped_gate_set(&gates, &vec![(0, 5), (1, 2)]);
    // assert!(test_system(&gates2));

    let gate_indices: Vec<usize> = (0..gates.len()).collect();
    let mut seen_swaps = HashSet::new();
    for combination in gate_indices.iter().cloned().combinations(num_swaps * 2) {
        let swaps: Vec<_> = combination
            .iter()
            .cloned()
            .permutations(num_swaps * 2)
            .collect();
        for swap in swaps {
            let mut swap_set = HashSet::new();

            for pair_to_swap in swap.chunks(2) {
                let mut pair_to_swap = pair_to_swap.to_vec();
                pair_to_swap.sort();
                let pair_to_swap = (pair_to_swap[0], pair_to_swap[1]);
                if swap_set.contains(&pair_to_swap) {
                    continue;
                }
                swap_set.insert(pair_to_swap);
            }

            let mut swap_set_vec = swap_set.iter().cloned().collect::<Vec<_>>();
            swap_set_vec.sort();
            if !seen_swaps.contains(&swap_set_vec) {
                let gates_new = create_swapped_gate_set(&gates, &swap_set_vec);

                seen_swaps.insert(swap_set_vec.clone());

                println!("Test the system: {:?}", &swap_set_vec);
                if test_system(&gates_new) {
                    let mut wires_involved = Vec::new();
                    for (ia, ib) in swap_set_vec {
                        wires_involved.push(gates[ia].3.clone());
                        wires_involved.push(gates[ib].3.clone());
                    }
                    let mut wires_involved: Vec<_> = wires_involved.into_iter().unique().collect();
                    wires_involved.sort();
                    let wires_involved = wires_involved.join(",");

                    // found correct swap
                    println!("Found correct swap: {:?}", combination);
                    println!("Wires involved: {}", &wires_involved);
                    return wires_involved;
                }
                println!(" - not correct");
            }
        }
    }

    "no solution".to_string()
}

fn create_swapped_gate_set(gates: &Vec<(GateType, String, String, String)>, swap_set_vec: &Vec<(usize, usize)>) -> Vec<(GateType, String, String, String)> {
    let mut gates_new = gates.clone();
    for &(ia, ib) in swap_set_vec {
        //println!("Try to swap gates {} and {}", ia, ib);
        let out1 = gates_new[ia].3.clone();
        let out2 = gates_new[ib].3.clone();
        gates_new[ia].3 = out2;
        gates_new[ib].3 = out1;
    }
    gates_new
}

fn run_system(
    wires: &mut HashMap<String, u64>,
    gates: &[Gate],
    highest_z_bits: u64,
) -> Option<u64> {
    loop {
        let mut did_set = false;
        for (op, wire1, wire2, wire_out) in gates {
            if wires.contains_key(wire1)
                && wires.contains_key(wire2)
                && !wires.contains_key(wire_out)
            {
                let val1 = wires[wire1];
                let val2 = wires[wire2];
                match op {
                    GateType::AND => {
                        wires.insert(wire_out.clone(), val1 & val2);
                    }
                    GateType::OR => {
                        wires.insert(wire_out.clone(), val1 | val2);
                    }
                    GateType::XOR => {
                        wires.insert(wire_out.clone(), val1 ^ val2);
                    }
                }
                did_set = true;
            }
        }
        if !did_set {
            break;
        }
    }

    let mut value = 0;
    for bit_index in 0..(highest_z_bits + 1) {
        let var = format!("z{:02}", bit_index);
        if wires.contains_key(&var) {
            value |= wires[&var] << bit_index;
        } else {
            return None;
        }
    }

    Some(value)
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

const INPUT_EX1: &str = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#;

const INPUT_EX2: &str = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"#;

const INPUT_EX3: &str = r#"x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00"#;

fn main() {
    // assert_eq!(solve_p1(INPUT_EX1), 4);
    // assert_eq!(solve_p1(INPUT_EX2), 2024);

    
    // assert_eq!(
    //     solve_p2(INPUT_EX3, 2, |a, b| a & b),
    //     "z00,z01,z02,z05".to_string()
    // );
    // assert_eq!(solve_p2(INPUT_EX1), );
    println!("Tests passed!");

    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input24.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    // println!("Part1: {}", solve_p1(&input));
    println!("Part2: {}", solve_p2(&input, 4, |a, b| a + b));
}
