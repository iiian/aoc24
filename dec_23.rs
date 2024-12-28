use std::collections::{HashMap, HashSet};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec23.txt")?;

    let now = std::time::Instant::now();
    let result = find_triangles(input.lines().collect());
    println!(
        "Puzzle 1: ans {:?}, ({} us)",
        result,
        now.elapsed().as_micros()
    );

    let now = std::time::Instant::now();
    let result = handle_puzzle2(input.as_str());
    println!(
        "Puzzle 2: ans {:?}, ({} us)",
        result,
        now.elapsed().as_micros()
    );

    Ok(())
}

fn parse(input: &str) -> HashSet<&str> {
    input.lines().collect()
}

type Units = usize;
fn handle_puzzle1(input: &str) -> Units {
    let mut t_trips = HashSet::new();
    let edges = parse(input);
    for src in &edges {
        let (a, b) = src.split_once('-').unwrap();

        for e in edges.iter().filter(|e| e.contains(a) || e.contains(b)) {
            if e == src {
                continue;
            }

            let (c, d) = e.split_once('-').unwrap();
            let e = if c == a || c == b { d } else { c };

            let some_ae = edges.contains(format!("{a}-{e}").as_str())
                || edges.contains(format!("{e}-{a}").as_str());
            let some_be = edges.contains(format!("{b}-{e}").as_str())
                || edges.contains(format!("{e}-{b}").as_str());

            if some_ae
                && some_be
                && (e.starts_with('t') || a.starts_with('t') || b.starts_with('t'))
            {
                let mut tripartite = vec![a, b, e];
                tripartite.sort_unstable();
                t_trips.insert(tripartite);
            }
        }
    }

    t_trips.len()
}

fn find_triangles(connections: Vec<&str>) -> usize {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    // Step 1: Build the graph
    for conn in connections {
        let parts: Vec<&str> = conn.split('-').collect();
        let a = parts[0];
        let b = parts[1];

        graph.entry(a).or_insert_with(HashSet::new).insert(b);
        graph.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    let mut triangles: HashSet<Vec<&str>> = HashSet::new();

    // Step 2: Find all triangles
    for a in graph.keys() {
        for b in &graph[a] {
            if b > a {
                for c in &graph[b] {
                    if c > b && graph[a].contains(c) {
                        let mut triangle = vec![*a, *b, *c];
                        triangle.sort();
                        triangles.insert(triangle);
                    }
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

fn handle_puzzle2(input: &str) -> Units {
    todo!()
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
    let input = r#""#;

    assert_eq!(handle_puzzle2(input), todo!());

    Ok(())
}
