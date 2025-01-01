use regex::Regex;
use std::{
    collections::HashMap,
    thread::{self, JoinHandle},
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

struct Sim {
    handles: Vec<JoinHandle<()>>,
    signals: Vec<Signal>,
    out: crossbeam_channel::Receiver<usize>,
}

#[derive(Clone)]
struct Signal {
    inp: Option<crossbeam_channel::Sender<bool>>,
    out: crossbeam_channel::Receiver<bool>,
}

impl Sim {
    pub fn output(self) -> usize {
        for handle in self.handles {
            handle.join().unwrap();
        }
        self.out.recv().unwrap()
    }
}

fn parse(input: &str) -> Sim {
    let (init_raw, circuitry_raw) = input.split_once("\n\n").unwrap();
    let mut circuitry = HashMap::<&str, Signal>::new();
    let mut handles = vec![];

    for line in init_raw.lines() {
        let (bit, value) = line.split_once(": ").unwrap();

        let value = value.parse::<usize>().unwrap() == 1;
        let (inp, out) = crossbeam_channel::unbounded();
        handles.push(thread::spawn(move || {
            let _ = inp.send(value);
        }));
        circuitry.insert(bit, Signal { inp: None, out });
    }

    let and = |a, b| a && b;
    let or = |a, b| a || b;
    let xor = |a: bool, b: bool| a ^ b;

    let re = Regex::new(r"(?<x>\w{3}) (?<g>(XOR|OR|AND)) (?<y>\w{3}) -> (?<z>\w{3})").unwrap();
    for gate_spec in circuitry_raw.lines() {
        let captures = re.captures(gate_spec).unwrap();
        let x = captures.name("x").unwrap().as_str();
        let y = captures.name("y").unwrap().as_str();
        let g = captures.name("g").unwrap().as_str();
        let z = captures.name("z").unwrap().as_str();

        let create_signal = || {
            let (inp, out) = crossbeam_channel::unbounded();
            Signal {
                inp: Some(inp),
                out,
            }
        };
        let sig_x = circuitry.entry(&x).or_insert_with(create_signal).clone();
        let sig_y = circuitry.entry(&y).or_insert_with(create_signal).clone();
        let sig_z = circuitry.entry(&z).or_insert_with(create_signal).clone();
        let op = match g {
            "OR" => or,
            "AND" => and,
            "XOR" => xor,
            _ => unreachable!(),
        };
        let handle = thread::spawn(move || {
            // Ensure the channels are read safely
            match sig_x.out.recv() {
                Ok(x) => match sig_y.out.recv() {
                    Ok(y) => {
                        let z = sig_z.inp.unwrap();
                        let result = op(x, y);
                        let _ = z.send(result);
                    }
                    Err(e) => println!("Error receiving y: {}", e),
                },
                Err(e) => println!("Error receiving x: {}", e),
            }
        });
        handles.push(handle);
    }

    let (sendz, final_z) = crossbeam_channel::unbounded();
    let threadsend = sendz.clone();
    let final_layer = circuitry
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect::<HashMap<_, _>>();

    let final_handle = thread::spawn(move || {
        let mut fz = 0_usize;
        for key in final_layer.keys() {
            let shift = key[1..].parse::<usize>().unwrap();
            match final_layer.get(key).unwrap().out.recv() {
                Ok(x) => {
                    if x == true {
                        fz |= 1 << shift;
                    }
                }
                Err(e) => println!("Error receiving final z value: {}", e),
            }
        }

        let _ = threadsend.send(fz);
    });
    handles.push(final_handle);

    drop(sendz);

    Sim {
        handles,
        out: final_z,
        signals: circuitry.into_values().collect(),
    }
}

type Units = usize;
fn handle_puzzle1(input: &str) -> Units {
    parse(input).output()
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

x00 AND y00 -> aba
x01 AND x02 -> gbs
aba AND gbs -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#;

    assert_eq!(handle_puzzle1(input), 0b100_usize);

    Ok(())
}

// #[test]
// fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
//     let input = r#""#;

//     assert_eq!(handle_puzzle2(input), todo!());

//     Ok(())
// }
