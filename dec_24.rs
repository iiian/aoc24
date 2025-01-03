mod circuit_sim;

use circuit_sim::*;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec24.txt")?;

    let now = std::time::Instant::now();
    let result = handle_puzzle1(input.as_str());
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

type Units = Option<usize>;
fn handle_puzzle1(input: &str) -> Units {
    let CircuitSpec { inputs, circuitry } = parse(input);
    let mut sim = Sim::from(circuitry);

    sim.run(&inputs)
}

fn get_gates<'a>(
    origin: &'a str,
    dest: &HashSet<&&'a str>,
    circuitry: &HashMap<&'a str, (&'a str, &'a str, &'a str)>,
    out: &mut HashSet<&'a str>,
) -> bool {
    if dest.contains(&origin) {
        return true;
    }
    if let Some((x, y, _)) = circuitry.get(origin) {
        let has_a = get_gates(x, dest, circuitry, out);
        let has_b = get_gates(y, dest, circuitry, out);

        if has_a || has_b {
            out.insert(origin);
            return true;
        }
    }

    false
}

fn handle_puzzle2(input: &str) -> Units {
    let CircuitSpec {
        inputs,
        mut circuitry,
    } = parse(input);
    let revcirc = circuitry.iter().fold(
        HashMap::<&str, HashSet<(&str, &str)>>::new(),
        |mut acc, (z, (x, y, g))| {
            acc.entry(x).or_default().insert((g, z));
            acc.entry(y).or_default().insert((g, z));
            acc
        },
    );

    let is_adder_circuit = |x: &str, y: &str, z: &str, carry: &mut Option<&str>| -> bool {
        let Some(x_conn) = revcirc.get(&x) else {
            return false;
        };
        let Some(y_conn) = revcirc.get(&y) else {
            return false;
        };
        let a_conn = x_conn.intersection(y_conn);

        let z = a_conn
            .into_iter()
            .filter(|(cg, cz)| *cg == "XOR")
            .map(|(_, z)| z)
            .collect::<Vec<_>>();

        let Some((zx, zy, zg)) = circuitry.get(z) else {
            return false;
        };

        if *zx != x || *zy != y {
            return false;
        }

        true
    };

    let ikeys = inputs.keys().cloned().collect::<HashSet<_>>();
    let ckeys = circuitry.keys().cloned().collect::<HashSet<_>>();
    let mut carry = None;
    for i in 0..44 {
        let (x, y, z) = (
            ikeys.get(format!("x{i}").as_str()).unwrap(),
            ikeys.get(format!("y{i}").as_str()).unwrap(),
            ckeys.get(format!("z{i}").as_str()).unwrap(),
        );
        if !is_adder_circuit(x, y, z, &mut carry) {
            println!("Problem with {i}th adder circuit");
        }
    }
    assert_eq!(carry, Some("z45"));

    None
}

fn handle_puzzle2_paused(input: &str) -> Units {
    // I did some sleuthing with function `puzzle2_hunthotspots` (see circuit_sim.rs) and found that these indices tend
    // to diverge from usize + usize functionality. I could be wrong.
    let expected_failure_spots: [usize; 4] = [15, 21, 30, 34];

    let CircuitSpec {
        inputs,
        mut circuitry,
    } = parse(input);
    let ikeys = inputs.keys().cloned().collect::<HashSet<_>>();
    let ckeys = circuitry.keys().cloned().collect::<HashSet<_>>();
    let test_cases = generate_test_cases(&inputs);

    for index in expected_failure_spots {
        let origin = format!("z{index}");
        let origin = ckeys.get(origin.as_str()).unwrap();
        // let destinations = ikeys
        //     .iter()
        //     .filter(|k| {
        //         (k.starts_with('x') || k.starts_with('y'))
        //             && k.ends_with(index.to_string().as_str())
        //     })
        //     .collect();
        let destinations = (index - 2..=index + 1)
            .flat_map(|i| {
                [
                    ikeys.get(format!("x{i}").as_str()),
                    ikeys.get(format!("y{i}").as_str()),
                ]
            })
            .filter_map(|o| o)
            .collect::<HashSet<_>>();

        // each index that causes divergence has a thin trace of gates that we could swap around.
        // specifically, if we're talking about z15, then basically any gate that exists on the
        // path from x14/x15 y14/y15 -> z15 could be the problem. So we want to try taking
        // combinations of them, and swapping them around. This reduces the problem space
        // drastically, if we can assume this system represents some full/half adder circuit.
        let mut gates = HashSet::new();
        get_gates(origin, &destinations, &circuitry, &mut gates);
        gates.remove(origin);

        // make sure that our change actually fixed a problem, else crash, Runtime error.
        let mut fix = false;

        'outer: for pair in gates.into_iter().combinations(2) {
            // a candidate rewiring for fixing our issue
            let mut cand = circuitry.clone();

            let a = cand.get_mut(pair[0]).unwrap() as *mut (&str, &str, &str);
            let b = cand.get_mut(pair[1]).unwrap() as *mut (&str, &str, &str);
            unsafe {
                std::ptr::swap(a, b);
            }

            // now, for our new sim based on our rewired circuitry, check if it passes for all test
            // cases.
            for TestCase { inputs, x, y } in &test_cases {
                let z_expected = x + y;
                let mut sim = Sim::from(cand.clone());
                let z_actual = sim.run(inputs);
                if let Some(z_actual) = z_actual {
                    // the bit is verified corrected if zexp ^ zact == 0 @ index
                    if ((z_expected >> index) & 1) ^ ((z_actual >> index) & 1) == 1 {
                        // this combination didn't resolve the problem.
                        break 'outer;
                    }
                } else {
                    break 'outer;
                }
            }
            circuitry = cand;
            fix = true;
            println!("gate fixed")
        }

        if !fix {
            panic!("your search did not fix all the problems");
        }
    }

    println!("I fixed all the bugs");

    Some(0)
}

struct TestCase<'a> {
    inputs: HashMap<&'a str, bool>,
    x: usize,
    y: usize,
}

fn generate_test_cases<'a>(inputs: &HashMap<&'a str, bool>) -> Vec<TestCase<'a>> {
    let (mut x, mut y) = (true, true);
    let mut all_inputs = vec![
        inputs.clone(),
        inputs.keys().map(|k| (*k, true)).collect::<HashMap<_, _>>(),
        inputs
            .keys()
            .sorted_by_key(|k| *k)
            .map(|k| {
                let b = if k.starts_with('x') { &mut x } else { &mut y };
                *b = !*b;
                (*k, *b)
            })
            .collect::<HashMap<_, _>>(),
    ];
    let mut x = !x;
    all_inputs.push(
        inputs
            .keys()
            .sorted_by_key(|k| *k)
            .map(|k| {
                let b = if k.starts_with('x') { &mut x } else { &mut y };
                *b = !*b;
                (*k, *b)
            })
            .collect::<HashMap<_, _>>(),
    );
    let (mut x, mut y) = (!x, !y);
    all_inputs.push(
        inputs
            .keys()
            .sorted_by_key(|k| *k)
            .map(|k| {
                let b = if k.starts_with('x') { &mut x } else { &mut y };
                *b = !*b;
                (*k, *b)
            })
            .collect::<HashMap<_, _>>(),
    );

    all_inputs
        .into_iter()
        .map(|inputs| {
            let mut x_expected = 0_usize;
            let mut y_expected = 0_usize;

            for (k, b) in &inputs {
                let (tgt, pos) = k.split_at(1);
                let tgt = match tgt {
                    "x" => &mut x_expected,
                    "y" => &mut y_expected,
                    _ => unreachable!(),
                };
                let pos = pos.parse::<usize>().unwrap();
                if *b {
                    *tgt |= 1 << pos;
                }
            }
            TestCase {
                inputs,
                x: x_expected,
                y: y_expected,
            }
        })
        .collect()
}

fn parse(input: &str) -> CircuitSpec {
    let (inputs_raw, circuitry_raw) = input.split_once("\n\n").unwrap();

    let mut inputs = HashMap::new();
    let mut circuitry = HashMap::new();

    for input in inputs_raw.lines() {
        let (bit, value) = input.split_once(": ").unwrap();
        inputs.insert(bit, value.parse::<u8>().unwrap() == 1_u8);
    }

    let re = Regex::new(r"(?<x>\w{3}) (?<g>(XOR|OR|AND)) (?<y>\w{3}) -> (?<z>\w{3})").unwrap();
    for gate_spec in circuitry_raw.lines() {
        let captures = re.captures(gate_spec).unwrap();
        let x = captures.name("x").unwrap().as_str();
        let y = captures.name("y").unwrap().as_str();
        let g = captures.name("g").unwrap().as_str();
        let z = captures.name("z").unwrap().as_str();

        circuitry.insert(z, (x, y, g));
    }

    CircuitSpec { inputs, circuitry }
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> abq
x01 XOR y01 -> lbc
x02 OR y02 -> dba
dba AND lbc -> z00
lbc AND abq -> z01
x01 OR lbc -> z02
"#;

    assert_eq!(handle_puzzle1(input), Some(0b100_usize));

    //     println!("Test #2");

    //     let input = r#"x00: 1
    // x01: 0
    // x02: 1
    // x03: 1
    // x04: 0
    // y00: 1
    // y01: 1
    // y02: 1
    // y03: 1
    // y04: 1

    // ntg XOR fgs -> mjb
    // y02 OR x01 -> tnw
    // kwq OR kpj -> z05
    // x00 OR x03 -> fst
    // tgd XOR rvg -> z01
    // vdt OR tnw -> bfw
    // bfw AND frj -> z10
    // ffh OR nrd -> bqk
    // y00 AND y03 -> djm
    // y03 OR y00 -> psh
    // bqk OR frj -> z08
    // tnw OR fst -> frj
    // gnj AND tgd -> z11
    // bfw XOR mjb -> z00
    // x03 OR x00 -> vdt
    // gnj AND wpb -> z02
    // x04 AND y00 -> kjc
    // djm OR pbm -> qhw
    // nrd AND vdt -> hwm
    // kjc AND fst -> rvg
    // y04 OR y02 -> fgs
    // y01 AND x02 -> pbm
    // ntg OR kjc -> kwq
    // psh XOR fgs -> tgd
    // qhw XOR tgd -> z09
    // pbm OR djm -> kpj
    // x03 XOR y03 -> ffh
    // x00 XOR y04 -> ntg
    // bfw OR bqk -> z06
    // nrd XOR fgs -> wpb
    // frj XOR qhw -> z04
    // bqk OR frj -> z07
    // y03 OR x01 -> nrd
    // hwm AND bqk -> z03
    // tgd XOR rvg -> z12
    // tnw OR pbm -> gnj"#;

    //     assert_eq!(handle_puzzle1(input), 0b0011111101000_usize);

    Ok(())
}

// #[test]
// fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
//     let input = r#""#;

//     assert_eq!(handle_puzzle2(input), todo!());

//     Ok(())
// }
