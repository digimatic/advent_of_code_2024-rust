use itertools::Itertools;
use std::{collections::HashMap, collections::HashSet, env, fs};

fn parse(input: &str) -> Vec<(String, String)> {
    let nss = input
        .lines()
        .map(|line| {
            let ns = line.split("-").map(|s| s.to_string()).collect::<Vec<_>>();
            (ns[0].clone(), ns[1].clone())
        })
        .collect::<Vec<_>>();

    nss
}

fn solve_p1(input: &str, name_start: char) -> usize {
    let nss = parse(input);

    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for (ns1, ns2) in nss {
        connections
            .entry(ns1.clone())
            .or_default()
            .insert(ns2.clone());
        connections.entry(ns2).or_default().insert(ns1.clone());
    }

    let mut sets = HashSet::new();
    for (p1, p1ts) in &connections {
        for p in p1ts.iter().permutations(2) {
            let p2 = p[0];
            let p3 = p[1];
            let p2ts = &connections[p2];
            let p3ts = &connections[p3];
            if p2ts.contains(p1) && p2ts.contains(p3) && p3ts.contains(p1) && p3ts.contains(p2) {
                let mut set = vec![p1, p2, p3];
                set.sort();
                sets.insert(set);
            }
        }
    }

    let name_start = name_start.to_string();

    let sets = sets
        .into_iter()
        .filter(|set| set.iter().find(|p| p.starts_with(&name_start)).is_some())
        .collect::<HashSet<_>>();

    sets.len()
}

fn find_maximum_clique(graph: &HashMap<String, HashSet<String>>) -> Option<HashSet<String>> {
    let mut max_clique = None;
    let r = HashSet::new();
    let p: HashSet<_> = graph.keys().cloned().collect();
    let x = HashSet::new();

    bron_kerbosch(graph, r, p, x, &mut max_clique);
    max_clique
}

// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
// Bron-Kerbosch with pivoting modified to find only the largest maximum clique
fn bron_kerbosch(
    graph: &HashMap<String, HashSet<String>>,
    r: HashSet<String>,
    p: HashSet<String>,
    mut x: HashSet<String>,
    max_clique: &mut Option<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        *max_clique = Some(max_clique.as_ref().map_or(r.clone(), |curr| {
            if r.len() > curr.len() {
                r.clone()
            } else {
                curr.clone()
            }
        }));
        return;
    }

    let pivot = p
        .union(&x)
        .max_by_key(|v| p.intersection(&graph[*v]).count())
        .cloned();

    let vertices: Vec<_> = match pivot {
        Some(u) => p.difference(&graph[&u]).cloned().collect(),
        None => p.iter().cloned().collect(),
    };

    for v in vertices {
        let neighbors = &graph[&v];

        let mut new_r = r.clone();
        new_r.insert(v.clone());

        let new_p: HashSet<_> = p.intersection(neighbors).cloned().collect();
        let new_x: HashSet<_> = x.intersection(neighbors).cloned().collect();

        bron_kerbosch(graph, new_r, new_p, new_x, max_clique);

        x.insert(v);
    }
}

fn solve_p2(input: &str) -> String {
    let nss = parse(input);

    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for (ns1, ns2) in nss {
        connections
            .entry(ns1.clone())
            .or_default()
            .insert(ns2.clone());
        connections
            .entry(ns2.clone())
            .or_default()
            .insert(ns1.clone());
    }

    let largest_common = find_maximum_clique(&connections).expect("No maximum clique found");
    let mut largest_common = largest_common.into_iter().collect::<Vec<_>>();
    largest_common.sort();
    largest_common.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EX1: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;

    #[test]
    fn p1_test1() {
        assert_eq!(solve_p1(INPUT_EX1, 't'), 7);
    }

    #[test]
    fn p2_test1() {
        assert_eq!(solve_p2(INPUT_EX1), "co,de,ka,ta");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = if args.len() > 1 {
        &args[1]
    } else {
        "input23.txt"
    };

    let input = fs::read_to_string(input_file).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read file '{}'", input_file);
        std::process::exit(1);
    });

    println!("Part1: {}", solve_p1(&input, 't'));
    println!("Part2: {}", solve_p2(&input));
}
