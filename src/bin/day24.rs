use std::fmt::Display;
use std::fs::File;
use std::io::{Result, Write};
use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum GateType {
    AND,
    OR,
    XOR,
}

impl Display for GateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GateType::AND => write!(f, "AND"),
            GateType::OR => write!(f, "OR"),
            GateType::XOR => write!(f, "XOR"),
        }
    }
}

type Wires = HashMap<String, u64>;
type Gate = (GateType, String, String, String);
type Gates = Vec<Gate>;

fn gate_to_string(gate: &Gate) -> String {
    format!("{} {} {} -> {}", gate.1, gate.0, gate.2, gate.3)
}

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

// Open as pdf:
//  dot -Tpdf -Gnodesep=0.5 -Granksep=0.5 diagram.dot -o circuit.pdf
//  open circuit.pdf
fn write_dot(gates: &Gates, filename: &str) -> Result<()> {
    let mut file = File::create(filename)?;

    writeln!(file, "digraph Circuit {{")?;
    writeln!(file, "    rankdir=LR;")?;
    writeln!(file, "    node [fontsize=10];")?;
    writeln!(file, "    edge [fontsize=8];")?;

    // First pass: group gates by their input pattern
    let mut pattern_gates: HashMap<(String, String), Vec<(usize, &Gate)>> = HashMap::new();
    for (i, gate @ (_, in1, in2, _)) in gates.iter().enumerate() {
        // Extract the number part from input wires (e.g., "x05" -> "05")
        if (in1.starts_with('x') || in1.starts_with('y'))
            && (in2.starts_with('x') || in2.starts_with('y'))
        {
            let num1 = &in1[1..];
            let num2 = &in2[1..];
            if num1 == num2 {
                pattern_gates
                    .entry((num1.to_string(), num2.to_string()))
                    .or_default()
                    .push((i, gate));
            }
        }
    }

    // Create input node ranks
    writeln!(file, "    // Input ranks")?;
    for i in 0..45 {
        writeln!(file, "    {{ rank=same; x{:02}; y{:02}; }}", i, i)?;
        writeln!(file, "    x{:02} [shape=circle];", i)?;
        writeln!(file, "    y{:02} [shape=circle];", i)?;
    }

    // Create output node ranks
    writeln!(file, "\n    // Output ranks")?;
    for i in 0..46 {
        writeln!(file, "    z{:02} [shape=circle];", i)?;
    }
    writeln!(
        file,
        "    {{ rank=same; {} }}",
        (0..46)
            .map(|i| format!("z{:02}", i))
            .collect::<Vec<_>>()
            .join("; ")
    )?;

    // Write gates and connections, with pattern-based ranking
    writeln!(file, "\n    // Gates and connections")?;

    // First write pattern-matched gates with rank constraints
    for ((_, _), gates_group) in &pattern_gates {
        // Sort gates by type (XOR first, then AND, then OR)
        let mut sorted_gates = gates_group.clone();
        sorted_gates.sort_by_key(|(_, (gate_type, _, _, _))| match gate_type {
            GateType::XOR => 0,
            GateType::AND => 1,
            GateType::OR => 2,
        });

        // Create a rank for this group
        if sorted_gates.len() > 1 {
            write!(file, "    {{ rank=same; ")?;
            for (i, _) in &sorted_gates {
                write!(file, "gate_{}; ", i)?;
            }
            writeln!(file, "}}")?;
        }

        // Write the gates
        for (i, (gate_type, in1, in2, out)) in sorted_gates {
            let gate_name = format!("gate_{}", i);
            let shape = match gate_type {
                GateType::AND => "box",
                GateType::OR => "diamond",
                GateType::XOR => "hexagon",
            };
            writeln!(
                file,
                "    {} [shape={}, label=\"{:?}\\n#{}\"];",
                gate_name, shape, gate_type, i
            )?;
            writeln!(file, "    {} -> {};", in1, gate_name)?;
            writeln!(file, "    {} -> {};", in2, gate_name)?;
            writeln!(file, "    {} -> {};", gate_name, out)?;
        }
    }

    // Write remaining gates
    for (i, (gate_type, in1, in2, out)) in gates.iter().enumerate() {
        if !pattern_gates
            .values()
            .any(|group| group.iter().any(|(idx, _)| *idx == i))
        {
            let gate_name = format!("gate_{}", i);
            let shape = match gate_type {
                GateType::AND => "box",
                GateType::OR => "diamond",
                GateType::XOR => "hexagon",
            };
            writeln!(
                file,
                "    {} [shape={}, label=\"{:?}\\n#{}\"];",
                gate_name, shape, gate_type, i
            )?;
            writeln!(file, "    {} -> {};", in1, gate_name)?;
            writeln!(file, "    {} -> {};", in2, gate_name)?;
            writeln!(file, "    {} -> {};", gate_name, out)?;
        }
    }

    // Add invisible edges to help maintain vertical alignment
    writeln!(file, "\n    // Invisible edges for alignment")?;
    for i in 0..44 {
        writeln!(file, "    x{:02} -> x{:02} [style=invis];", i, i + 1)?;
        writeln!(file, "    y{:02} -> y{:02} [style=invis];", i, i + 1)?;
        writeln!(file, "    z{:02} -> z{:02} [style=invis];", i, i + 1)?;
    }
    writeln!(file, "    z44 -> z45 [style=invis];")?;

    writeln!(file, "}}")?;

    Ok(())
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

fn solve_p2(input: &str) -> String {
    let (wires, gates) = parse(input);
    write_dot(&&gates, "diagram.dot").ok();

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

    let invalid_gate_indices =
        find_invalidate_gates(&gates, highest_x_bits, highest_y_bits, highest_z_bits);
    assert_eq!(invalid_gate_indices.len(), 8);
    let mut invalid_gate_names: Vec<String> = invalid_gate_indices
        .into_iter()
        .map(|j| &gates[j].3)
        .cloned()
        .collect();
    invalid_gate_names.sort();
    let r = invalid_gate_names.join(",");
    return r;
}

fn find_gate_with_output(gates: &[Gate], wire: &str) -> Option<usize> {
    for (i, wire_pair) in gates.iter().enumerate() {
        if wire_pair.3 == wire {
            return Some(i);
        }
    }
    None
}

fn find_gates_with_input(gates: &[Gate], wire: &str) -> Vec<usize> {
    let mut matching_gates: Vec<usize> = Vec::new();
    for (i, wire_pair) in gates.iter().enumerate() {
        if wire_pair.1 == wire || wire_pair.2 == wire {
            matching_gates.push(i);
        }
    }
    matching_gates
}

fn find_invalidate_gates(
    gates: &[Gate],
    _highest_x_bits: u64,
    _highest_y_bits: u64,
    highest_z_bits: u64,
) -> Vec<usize> {
    let mut bad_gates: Vec<usize> = Vec::new();

    for (i, gate) in gates.iter().enumerate() {
        let (op, inp1, inp2, out) = gate;
        let connected_out_gates = find_gates_with_input(gates, out);
        let output_types: HashSet<_> = connected_out_gates
            .iter()
            .map(|&j| &gates[j].0)
            .cloned()
            .collect();
        let mut input_types: HashSet<GateType> = HashSet::new();
        let input_gate1_type = find_gate_with_output(gates, inp1)
            .map(|j| &gates[j].0)
            .cloned();
        if let Some(ref gate_type) = input_gate1_type {
            input_types.insert(gate_type.clone());
        }
        let input_gate2_type = find_gate_with_output(gates, inp2)
            .map(|j| &gates[j].0)
            .cloned();
        if let Some(ref gate_type) = input_gate2_type {
            input_types.insert(gate_type.clone());
        }

        // Rule 1: Z-output gates shall be XOR, but not for last bit Z-output gates
        if out.starts_with("z") && *op != GateType::XOR {
            let out_index = out.strip_prefix("z").unwrap().parse::<u64>().unwrap();
            if out_index < highest_z_bits {
                println!("Bad Gate (Rule 1) #{}: {}", i, gate_to_string(gate));
                bad_gates.push(i);
                continue;
            }
        }

        if *op == GateType::XOR {
            if out == "z01" {
                continue;
            }
            if ((inp1.starts_with("x") && inp2.starts_with("y"))
                || (inp1.starts_with("y") && inp2.starts_with("x")))
                && output_types == HashSet::from([GateType::XOR, GateType::AND])
            {
                continue; // ok
            }
            if out.starts_with("z") {
                continue;
            }
            println!("Bad Gate (Rule 2) #{}: {}", i, gate_to_string(gate));
            bad_gates.push(i);
        } else if *op == GateType::OR {
            if output_types == HashSet::from([GateType::XOR, GateType::AND]) {
                continue;
            }
            if *out == format!("z{:02}", highest_z_bits) {
                continue;
            }
            println!("Bad Gate (Rule 3) #{}: {}", i, gate_to_string(gate));
            bad_gates.push(i);
        } else if *op == GateType::AND {
            if output_types == HashSet::from([GateType::OR]) {
                continue;
            }
            if (inp1 == "x00" && inp2 == "y00") || (inp1 == "y00" && inp2 == "x00") {
                continue;
            }
            println!("Bad Gate (Rule 4) #{}: {}", i, gate_to_string(gate));
            bad_gates.push(i);
        }
    }
    bad_gates
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX1), 4);
        assert_eq!(solve_p1(INPUT_EX2), 2024);
    }
}

fn main() {
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

    println!("Part1: {}", solve_p1(&input));
    println!("Part2: {}", solve_p2(&input));
}
