use bus::Bus;
use bus::BusReader;
use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

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

static TIMEOUT: Duration = Duration::from_millis(1_000);

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
                    let Ok(x) = x.recv_timeout(TIMEOUT) else {
                        break;
                    };
                    let Ok(y) = y.recv_timeout(TIMEOUT) else {
                        break;
                    };
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

    pub fn run(&mut self, inputs: &HashMap<&'a str, bool>) -> Option<usize> {
        for (name, value) in inputs {
            self.channels.get_mut(name).unwrap().broadcast(*value);
        }

        let mut z_final = 0_usize;
        for (off, zrecv) in &mut self.zrecv {
            if let Ok(b) = zrecv.recv_timeout(TIMEOUT) {
                if b {
                    z_final |= 1 << off;
                }
            } else {
                return None;
            }
        }

        Some(z_final)
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
