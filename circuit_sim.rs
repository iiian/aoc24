use bus::Bus;
use bus::BusReader;
use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;

pub(crate) struct CircuitSpec<'a> {
    pub(crate) inputs: HashMap<&'a str, bool>,
    pub(crate) circuitry: HashMap<&'a str, (&'a str, &'a str, &'a str)>,
}

pub(crate) struct Sim<'a> {
    pub(crate) channels: HashMap<&'a str, Bus<bool>>,
    pub(crate) handles: Vec<JoinHandle<()>>,
    pub(crate) zrecv: HashMap<usize, BusReader<bool>>,
}

pub(crate) struct Gate<'a> {
    pub(crate) x: BusReader<bool>,
    pub(crate) y: BusReader<bool>,
    pub(crate) g: &'a str,
    pub(crate) z: &'a str,
}

impl<'a> Sim<'a> {
    pub fn resolve_circuitry(
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
            Sim::resolve_circuitry(x, circuitry, channels, gates);
            Sim::resolve_circuitry(y, circuitry, channels, gates);
            let x = channels.get_mut(x).unwrap().add_rx();
            let y = channels.get_mut(y).unwrap().add_rx();
            gates.push(Gate { x, y, g, z })
        }
    }

    pub fn from(circuitry: HashMap<&'a str, (&'a str, &'a str, &'a str)>) -> Self {
        let and = |a, b| a && b;
        let or = |a, b| a || b;
        let xor = |a: bool, b: bool| a ^ b;

        let mut channels = HashMap::new();
        let mut gates = vec![];

        // fill out `channels` and `gates`, separating Buses from BusReaders, while
        // persisting both of them for later
        for z in circuitry.keys().filter(|k| k.starts_with('z')) {
            Sim::resolve_circuitry(z, &circuitry, &mut channels, &mut gates);
        }

        // construct the final layer that accretes all z## outputs into a usize
        let zrecv = channels
            .iter_mut()
            .filter(|(c, _)| c.starts_with('z'))
            .map(|(zname, zbus)| (zname[1..].parse::<usize>().unwrap(), zbus.add_rx()))
            .collect::<HashMap<_, _>>();

        // construct a gang of threads to run the gates async
        let handles = gates
            .into_iter()
            .map(|Gate { mut x, mut y, g, z }| {
                let mut z = channels.remove(z).unwrap();
                let gate = match g {
                    "OR" => or,
                    "AND" => and,
                    "XOR" => xor,
                    _ => unreachable!(),
                };
                thread::spawn(move || loop {
                    let Ok(x) = x.recv() else { break };
                    let Ok(y) = y.recv() else { break };
                    let Ok(()) = z.try_broadcast(gate(x, y)) else {
                        break;
                    };
                })
            })
            .collect::<Vec<_>>();

        Self {
            channels,
            handles,
            zrecv,
        }
    }

    pub fn run(&mut self, inputs: &HashMap<&'a str, bool>) -> usize {
        for (name, value) in inputs {
            self.channels.get_mut(name).unwrap().broadcast(*value);
        }

        let mut z_final = 0_usize;
        for (off, zrecv) in &mut self.zrecv {
            let b = zrecv.recv().unwrap();
            if b {
                z_final |= 1 << off;
            }
        }

        z_final
    }
}

impl<'a> Drop for Sim<'a> {
    fn drop(&mut self) {
        self.zrecv.drain();
        self.channels.drain();
        for handle in self.handles.drain(..) {
            handle.join().unwrap();
        }
    }
}

// fn puzzle2_hunthotspots(input: &str) {
//     let CircuitSpec { inputs, circuitry } = todo!();
//     let mut sim = Sim::from(circuitry);
//     let (mut x, mut y) = (true, true);
//     let mut all_inputs = vec![
//         inputs.clone(),
//         inputs.keys().map(|k| (*k, true)).collect::<HashMap<_, _>>(),
//         inputs
//             .keys()
//             .sorted_by_key(|k| *k)
//             .map(|k| {
//                 let mut b = if k.starts_with('x') { &mut x } else { &mut y };
//                 *b = !*b;
//                 (*k, (*b).clone())
//             })
//             .collect::<HashMap<_, _>>(),
//     ];
//     let mut x = !x;
//     all_inputs.push(
//         inputs
//             .keys()
//             .sorted_by_key(|k| *k)
//             .map(|k| {
//                 let mut b = if k.starts_with('x') { &mut x } else { &mut y };
//                 *b = !*b;
//                 (*k, (*b).clone())
//             })
//             .collect::<HashMap<_, _>>(),
//     );
//     let (mut x, mut y) = (!x, !y);
//     all_inputs.push(
//         inputs
//             .keys()
//             .sorted_by_key(|k| *k)
//             .map(|k| {
//                 let mut b = if k.starts_with('x') { &mut x } else { &mut y };
//                 *b = !*b;
//                 (*k, (*b).clone())
//             })
//             .collect::<HashMap<_, _>>(),
//     );
//     for inputs in all_inputs {
//         let mut x_expected = 0_usize;
//         let mut y_expected = 0_usize;
//
//         for (k, b) in &inputs {
//             let (tgt, pos) = k.split_at(1);
//             let tgt = match tgt {
//                 "x" => &mut x_expected,
//                 "y" => &mut y_expected,
//                 _ => unreachable!(),
//             };
//             let pos = pos.parse::<usize>().unwrap();
//             if *b {
//                 *tgt |= 1 << pos;
//             }
//         }
//
//         let z_expected = x_expected + y_expected;
//         let z_actual = sim.run(&inputs);
//         println!("{x_expected:064b} (x)");
//         println!("{y_expected:064b} (+y)");
//         println!("{z_expected:064b} (=expected)");
//         println!("{z_actual:064b} (=actual)");
//         println!("{:064b}", !(z_expected ^ z_actual),);
//         println!();
//     }
// }
