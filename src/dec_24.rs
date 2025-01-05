mod circuit_sim;

use circuit_sim::*;
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec24.txt")?;

    let now = std::time::Instant::now();
    let result = handle_puzzle1(input.as_str());
    println!(
        "Puzzle 1: ans {:?}, ({} us)",
        result,
        now.elapsed().as_micros()
    );

    // discovered hot spots:
    // ccp OR hhw -> fph should go to z15
    // z21 thinks that x21 is it's carry line
    // z29/30: 29 carry out is dwm, but 30 carry in is wrk
    // z34: fcv AND ksm -> z34?
    //
    // one pair is confirmed to be z15/fph
    // second pair is gds/z21
    // third pair must be jrs/wrk ?
    // fourth is z34/cqk
    let now = std::time::Instant::now();
    let mut result = ["z15", "fph", "gds", "z21", "jrs", "wrk", "cqk", "z34"];
    result.sort();
    println!(
        "Puzzle 2: ans {:?}, ({} us)",
        result.join(","),
        now.elapsed().as_micros()
    );

    assert_valid_adder_circuit(input.as_str());

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

struct Adder<'a> {
    x: &'a str,
    y: &'a str,
    /// equivalently, the second xor gate
    z: &'a str,
    cin: Option<&'a str>,
    xori: Option<&'a str>,
    andi: Option<&'a str>,
    andc: Option<&'a str>,
    cout: Option<&'a str>,
}

impl<'a> Adder<'a> {
    pub fn new(x: &'a str, y: &'a str, z: &'a str) -> Self {
        Self {
            x,
            y,
            z,
            cin: None,
            xori: None,
            andi: None,
            andc: None,
            cout: None,
        }
    }
}

impl<'a> Display for Adder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &format!(
                "{}",
                vec![
                    format!(""),
                    format!(""),
                    format!("{} --*--+===+", self.x),
                    format!(
                        "      |  |XOR|-* {} ----------------+===+ ",
                        self.xori.unwrap_or("???")
                    ),
                    format!(
                        "{} -*---+===+ |                     |XOR| --- {}",
                        self.y, self.z
                    ),
                    format!("     ||        |      /--------------+===+"),
                    format!("{} ---------*-------/", self.cin.unwrap_or("???")),
                    format!("     ||      | |"),
                    format!("     |.-------------+===+"),
                    format!(
                        "     |       | |    |AND|---{}----+",
                        self.andi.unwrap_or("???")
                    ),
                    format!("     .--------------+===+          +-+===+"),
                    format!(
                        "             | |                     | OR| --- {}",
                        self.cout.unwrap_or("???")
                    ),
                    format!("             | .----+===+          +-+===+"),
                    format!(
                        "             |      |AND|---{}----+ ",
                        self.andc.unwrap_or("???")
                    ),
                    format!("             .------+===+"),
                ]
                .join("\n")
            )
            .as_str(),
        )
    }
}

fn investigate_puzzle2(input: &str) -> Units {
    let CircuitSpec {
        inputs,
        circuitry: circ,
    } = parse(input);

    let inv = circ
        .iter()
        .map(|(k, v)| (*v, *k))
        .collect::<HashMap<_, _>>();
    let mut adders = vec![];
    let ikeys = inputs.keys().cloned().into_iter().collect::<HashSet<_>>();
    let zkeys = circ.keys().cloned().into_iter().collect::<HashSet<_>>();
    for i in 0..=44_usize {
        let (x, y, z) = (
            ikeys.get(format!("x{i:02}").as_str()).unwrap(),
            ikeys.get(format!("y{i:02}").as_str()).unwrap(),
            zkeys.get(format!("z{i:02}").as_str()).unwrap(),
        );

        let mut adder = Adder::new(x, y, z);
        let (zx, zy, _) = circ.get(z).unwrap();
        if i > 0 {
            let xori = gate(&inv, x, y, "XOR").unwrap();
            let c = if xori == *zx { zy } else { zx };
            adder.cin = Some(c);
            adder.xori = Some(xori);
            adder.andi = gate(&inv, x, y, "AND");
            adder.andc = gate(&inv, c, &xori, "AND");
            if let Some(ref andi) = adder.andi {
                if let Some(ref andc) = adder.andc {
                    adder.cout = gate(&inv, andi, andc, "OR");
                }
            }
        } else {
            let c = gate(&inv, x, y, "AND");
            adder.cout = c;
        }
        adders.push(adder);
    }

    for adder in adders {
        println!("{}", adder);
    }

    None
}

fn gate<'a>(
    circinv: &HashMap<(&'a str, &'a str, &'a str), &'a str>,
    x: &&'a str,
    y: &&'a str,
    gate: &str,
) -> Option<&'a str> {
    circinv
        .get(&(x, y, gate))
        .or_else(|| circinv.get(&(y, x, gate)))
        .map(|e| *e)
}

fn assert_valid_adder_circuit(input: &str) {
    // I did some sleuthing with function `puzzle2_hunthotspots` (see circuit_sim.rs) and found that these indices tend
    // to diverge from usize + usize functionality. I could be wrong.

    let CircuitSpec { inputs, circuitry } = parse(input);

    let test_cases = generate_test_cases(&inputs);

    // a candidate rewiring for fixing our issue
    let mut cand = circuitry.clone();

    // now, for our new sim based on our rewired circuitry, check if it passes for all test
    // cases.
    for TestCase { inputs, x, y } in &test_cases {
        let z_expected = x + y;
        let mut sim = Sim::from(cand.clone());
        if let Some(z_actual) = sim.run(inputs) {
            println!("{z_expected:064b}");
            println!("{z_actual:064b}");
            println!("{:064b}", z_expected ^ z_actual);
            println!();
        }
    }
    println!("gate fixed")
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
