use bus::{Bus, BusReader};
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    sync::mpsc,
    thread,
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

    // let now = std::time::Instant::now();
    // let result = handle_puzzle2(input.as_str());
    // println!(
    //     "Puzzle 2: ans {:?}, ({} us)",
    //     result,
    //     now.elapsed().as_micros()
    // );

    Ok(())
}

struct CircuitSpec<'a> {
    inputs: HashMap<&'a str, bool>,
    circuitry: HashMap<&'a str, (&'a str, &'a str, &'a str)>,
}

enum InputPair {
    Partial(bool),
}

fn parse<'a>(input: &'a str) -> CircuitSpec<'a> {
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

type Units = usize;
fn handle_puzzle1(input: &str) -> Units {
    let CircuitSpec { inputs, circuitry } = parse(input);

    let and = |a, b| a && b;
    let or = |a, b| a || b;
    let xor = |a: bool, b: bool| a ^ b;

    let mut channels = HashMap::new();
    let mut gates = vec![];

    // fill out `channels` and `gates`, separating Buses from BusReaders, while
    // persisting both of them for later
    for z in circuitry.keys().filter(|k| k.starts_with('z')) {
        resolve_circuitry(z, &circuitry, &mut channels, &mut gates);
    }

    // construct the final layer that accretes all z## outputs into a usize
    let (zftx, zfrx) = mpsc::channel();
    let mut handles = vec![];
    let zrecv = channels
        .iter_mut()
        .filter(|(c, _)| c.starts_with('z'))
        .map(|(zname, zbus)| (zname[1..].parse::<usize>().unwrap(), zbus.add_rx()))
        .collect::<HashMap<_, _>>();
    handles.push(thread::spawn(move || {
        let mut z_final = 0_usize;
        for (off, mut zrecv) in zrecv {
            let b = zrecv.recv().unwrap();
            if b {
                z_final |= 1 << off;
            }
        }

        zftx.send(z_final).unwrap();
    }));

    // construct a gang of threads to run the gates async
    handles.extend(
        gates
            .into_iter()
            .map(|Gate { mut x, mut y, g, z }| {
                let mut z = channels.remove(z).unwrap();
                let gate = match g {
                    "OR" => or,
                    "AND" => and,
                    "XOR" => xor,
                    _ => unreachable!(),
                };
                thread::spawn(move || {
                    let x = x.recv().unwrap();
                    let y = y.recv().unwrap();
                    z.broadcast(gate(x, y));
                })
            })
            .collect::<Vec<_>>(),
    );

    // kick off the system
    for (name, value) in inputs {
        channels.remove(name).unwrap().broadcast(value);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    zfrx.recv().unwrap()
}

struct Gate<'a> {
    x: BusReader<bool>,
    y: BusReader<bool>,
    g: &'a str,
    z: &'a str,
}

fn resolve_circuitry<'a>(
    z: &'a str,
    circuitry: &HashMap<&'a str, (&'a str, &'a str, &'a str)>,
    channels: &mut HashMap<&'a str, Bus<bool>>,
    gates: &mut Vec<Gate<'a>>,
) {
    if channels.contains_key(z) {
        return;
    }

    channels.insert(z, Bus::new(64));

    if let Some((x, y, g)) = circuitry.get(z) {
        resolve_circuitry(&x, circuitry, channels, gates);
        resolve_circuitry(&y, circuitry, channels, gates);
        let x = channels.get_mut(x).unwrap().add_rx();
        let y = channels.get_mut(y).unwrap().add_rx();
        gates.push(Gate { x, y, g, z })
    }
}

// fn handle_puzzle2(input: &str) -> Units {
//     todo!()
// }

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

    assert_eq!(handle_puzzle1(input), 0b100_usize);

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
