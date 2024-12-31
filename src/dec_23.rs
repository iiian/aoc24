use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;

pub fn is_maximal_clique(graph: HashMap<&str, HashSet<&str>>, clique: HashSet<&str>) -> bool {
    for e in &clique {
        let e = &graph[e];
        for n in e {
            if !clique.contains(n) && graph[n].intersection(&clique).count() == graph[n].len() {
                return false;
            }
        }
    }

    true
}

pub fn extend_cliques<'a>(
    graph: &HashMap<&'a str, BTreeSet<&'a str>>,
    cliques: &HashSet<BTreeSet<&'a str>>,
) -> HashSet<BTreeSet<&'a str>> {
    let mut bigger_cliques = HashSet::new();

    for clique in cliques {
        for e in clique {
            let e = &graph[e];
            for n in e {
                if !clique.contains(n) && graph[n].intersection(clique).count() == clique.len() {
                    let mut new_clique = clique.clone();
                    new_clique.insert(n);
                    bigger_cliques.insert(new_clique);
                }
            }
        }
    }
    bigger_cliques
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec23.txt")?;

    let now = std::time::Instant::now();
    let input = input.as_str();

    let result = handle_puzzle1(input);
    println!(
        "Puzzle 1: ans {:?}, ({} us)",
        result,
        now.elapsed().as_micros()
    );

    let now = std::time::Instant::now();
    let result = handle_puzzle2(input);
    println!(
        "Puzzle 2: ans {:?}, ({} us)",
        result,
        now.elapsed().as_micros()
    );

    Ok(())
}

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    println!("parsing graph");
    let connections = input.lines().collect::<Vec<_>>();
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    // Step 1: Build the graph
    for conn in connections {
        let parts: Vec<&str> = conn.split('-').collect();
        let a = parts[0];
        let b = parts[1];

        graph.entry(a).or_insert_with(HashSet::new).insert(b);
        graph.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    graph
}

fn handle_puzzle1(input: &str) -> usize {
    let graph: HashMap<&str, HashSet<&str>> = parse(input);

    let mut triangles: HashSet<BTreeSet<&str>> = HashSet::new();

    // Step 2: Find all triangles
    for a in graph.keys() {
        for b in &graph[a] {
            for c in &graph[b] {
                if graph[a].contains(c) {
                    let triangle = BTreeSet::from([*a, *b, *c]);
                    triangles.insert(triangle);
                }
            }
        }
    }

    // Step 3: Filter triangles with at least one 't' and count them
    let mut count = 0;
    for triangle in &triangles {
        if triangle.iter().any(|node| node.starts_with('t')) {
            count += 1;
        }
    }

    count
}

fn handle_puzzle2(input: &str) -> String {
    let graph: HashMap<&str, BTreeSet<&str>> = parse(input)
        .into_iter()
        .map(|(k, v)| {
            let v = {
                let mut out = BTreeSet::new();
                for e in v {
                    out.insert(e);
                }
                out
            };
            (k, v)
        })
        .collect();

    let mut triangles: HashSet<BTreeSet<&str>> = HashSet::new();

    // Step 2: Find all triangles
    for a in graph.keys() {
        for b in &graph[a] {
            for c in &graph[b] {
                if graph[a].contains(c) {
                    let mut triangle = BTreeSet::from([*a, *b, *c]);
                    triangles.insert(triangle);
                }
            }
        }
    }
    let mut clique_size = 3;
    let mut prev = triangles;
    loop {
        println!("clique size = {}", clique_size);
        println!("prev size = {}", prev.len());
        let next = extend_cliques(&graph, &prev);

        if next.is_empty() {
            break;
        }
        prev = next;
        clique_size += 1;
    }

    println!("number of maximum cliques: {}", prev.len());
    prev.into_iter()
        .next()
        .unwrap()
        .into_iter()
        .sorted()
        .join(",")
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"kh-tc
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

    assert_eq!(handle_puzzle1(input), 7);

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"ka-co
ta-co
de-co
ta-ka
de-ta
ka-de"#;

    assert_eq!(handle_puzzle2(input), String::from("codekata"));

    Ok(())
}
