use core::panic;
use std::ops::{BitXor, BitXorAssign};

use crate::dec_17_part1::{parse, Vm};
use itertools::Itertools;

// R: From<u8>
// + Clone
// + std::ops::BitXor<R, Output = R>
// + std::ops::Rem<usize, Output = R>
// + std::ops::Shr<Output = R>
// + std::ops::BitXorAssign<R>
// + std::cmp::PartialEq<usize>,

#[derive(Clone)]
struct VBit {
    state: Vec<Bit>,
}

impl From<u8> for VBit {
    fn from(mut value: u8) -> Self {
        let mut state = vec![];
        for _ in 0..8 {
            state.push(Bit::Const(value & 1));
            value >>= 1;
        }

        VBit { state }
    }
}

impl std::ops::BitXor<VBit> for VBit {
    type Output = VBit;

    fn bitxor(mut self, rhs: VBit) -> Self::Output {
        self.bitxor_assign(rhs);
        self
    }
}

impl std::ops::Rem<usize> for VBit {
    type Output = VBit;

    fn rem(self, rhs: usize) -> Self::Output {
        assert_eq!(rhs, 8); // only circumstance where it's necessary

        Self::Output {
            state: self.state.into_iter().take(3).collect::<Vec<_>>(),
        }
    }
}

impl Into<usize> for VBit {
    fn into(self) -> usize {
        let mut out = 0;

        for bit in self.state {
            match bit {
                Bit::Named(_) => panic!("haha fuck it happened"),
                Bit::Const(x) => out |= x as usize,
                Bit::Ident => panic!("and again it happened loool"),
                Bit::Invert => panic!("why am I calling inverts!?"),
                Bit::Xor(_, _) => panic!("shit fuck, xors?!"),
            }

            out <<= 1;
        }

        out
    }
}

impl std::ops::Shr for VBit {
    type Output = VBit;

    fn shr(self, rhs: Self) -> Self::Output {
        Self::Output {
            state: self.state.into_iter().skip(rhs.into()).collect(),
        }
    }
}

impl std::ops::BitXorAssign<VBit> for VBit {
    fn bitxor_assign(&mut self, rhs: VBit) {
        for i in 0..(self.state.len().min(rhs.state.len())) {
            let bit_self = &self.state[i];
            let bit_rhs = &rhs.state[i];
            self.state[i] = bit_self.xor(&bit_rhs);
        }
    }
}

impl std::cmp::PartialEq<usize> for VBit {
    fn eq(&self, other: &usize) -> bool {
        if *other == 0 {
            for b in &self.state {
                match b {
                    Bit::Const(x) => {
                        if *x == 1 {
                            return false;
                        }
                    }
                    _ => {}
                }
            }
        } else {
            panic!();
        }

        return true;
    }
}

#[derive(Clone)]
enum Bit {
    Named(String),
    Const(u8),
    Ident,
    Invert,
    Xor(Box<Bit>, Box<Bit>),
}

impl Bit {
    pub fn header(name: String) -> Self {
        Self::Named(name)
    }
    pub fn unknown() -> Self {
        Self::Ident
    }
    pub fn constant(value: u8) -> Self {
        Self::Const(value)
    }
    pub fn xor(&self, other: &Self) -> Self {
        Self::Xor(Box::new(self.clone()), Box::new(other.clone()))
    }

    pub fn set(&mut self, value: u8) {
        *self = Bit::Const(value);
    }

    pub fn try_name(&self) -> Result<&String, &Self> {
        if let Bit::Named(name) = self {
            Ok(name)
        } else {
            Err(self)
        }
    }
}

impl ToString for Bit {
    fn to_string(&self) -> String {
        match self {
            Bit::Named(h) => h.to_string(),
            Bit::Const(x) => x.to_string(),
            Bit::Ident => String::from("="),
            Bit::Invert => String::from("~"),
            Bit::Xor(a, b) => {
                match (a.try_name(), b.try_name()) {
                    (Ok(name_a), Ok(name_b)) => return format!("{name_a}⊕{name_b}"),
                    (Err(unnamed), Ok(name_a)) | (Ok(name_a), Err(unnamed)) => {
                        return match unnamed.to_string().as_str() {
                            "=" | "0" => name_a.to_string(),
                            "~" | "1" => format!("~{name_a}"),
                            _ => unreachable!(),
                        };
                    }
                    _ => {
                        let a = a.to_string();
                        let b = b.to_string();

                        // 0, 1, =, ~
                        match (a.as_str(), b.as_str()) {
                            ("~", "=")
                            | ("=", "~")
                            | ("~", "0")
                            | ("0", "~")
                            | ("1", "=")
                            | ("=", "1") => "~".to_string(),
                            ("=", "=")
                            | ("~", "~")
                            | ("~", "1")
                            | ("1", "~")
                            | ("=", "0")
                            | ("0", "=") => "=".to_string(),
                            ("1", "0") | ("0", "1") => "1".to_string(),
                            ("1", "1") | ("0", "0") => "0".to_string(),
                            ("~", x) | (x, "~") | ("1", x) | (x, "1") => {
                                if x.starts_with("~") {
                                    x.chars().skip(1).collect::<String>()
                                } else {
                                    "~".to_string() + x
                                }
                            }
                            ("=", x) | (x, "=") | ("0", x) | (x, "0") => x.to_string(),
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
    }
}

/// A virtual superposition with 64 bits
#[derive(Clone)]
struct Space {
    state: [Option<bool>; 64],
}

impl Space {
    pub fn new() -> Self {
        Self { state: [None; 64] }
    }

    pub fn set(&mut self, constraint: &str, value: bool) -> bool {
        if let Some((negate, index)) = constraint.split_once("a") {
            let negate = negate == "~";
            let index = index.parse::<usize>().unwrap();
            if let Some(inbounds_ref) = self.state.get_mut(index) {
                let value = value ^ negate;
                if let Some(current_constraint) = inbounds_ref {
                    // if already in use, it must match our additional constraint
                    *current_constraint == value
                } else {
                    // set the value
                    *inbounds_ref = Some(value);
                    true
                }
            } else {
                panic!();
            }
        } else {
            true
        }
    }

    pub fn unset(&mut self, constraint: &str) {
        if let Some((_, index)) = constraint.split_once("a") {
            let index = index.parse::<usize>().unwrap();
            if let Some(inbounds_ref) = self.state.get_mut(index) {
                *inbounds_ref = None;
            } else {
                panic!();
            }
        }
    }

    pub fn finalized(&self) -> usize {
        assert!(self.state.len() <= 64);

        let mut out = 0_usize;

        for bit in &self.state {
            if let Some(true) = bit {
                out |= 1;
            }
            out <<= 1;
        }

        out
    }
}

pub(crate) fn handle_puzzle2(input: &str) -> Option<usize> {
    let targets = Vec::<usize>::from([2, 4, 1, 1, 7, 5, 4, 6, 0, 3, 1, 4, 5, 5, 3, 0]);
    // UNCOMMENT BELOW to visualize what the fuck I'm even doing in `get_constraints`
    // for (k, target) in targets.iter().enumerate() {
    //     print_table_console(k, target.clone());
    // }

    // let (mut vm, _) = parse::<VBit>(input);
    // vm.registers[0] = VBit {
    //     state: (0..64).map(|i| Bit::header(format!("a{}", i))).collect(),
    // };

    // while vm.exec() {}

    // let mut space = Space::new();

    // let constraints: Constraints = todo!();

    // let first_discovered = solve(&mut space, &targets, &constraints, 0);

    // first_discovered.unwrap()

    let (mut vm, _) = parse::<usize>(input);

    let mut a_start = 0b000_000_000_000____000_000_000_000____000_000_000_000____000_000_000_000;
    let mut incr = 15;
    let mut max_score = 0;

    loop {
        vm.reset();
        vm.registers[0] = a_start;

        while vm.exec() {}

        let score = vm
            .output
            .iter()
            .rev()
            .zip(targets.iter().rev())
            .take_while(|(a, b)| a == b)
            .count();

        if score == 16 {
            return Some(a_start);
        } else if score > max_score {
            incr = 15 - score;
            max_score = score;
        }

        a_start += 1 << (3 * incr);
    }

    // Idk, I quit
}

type Constraints = Vec<Vec<Vec<Bit>>>;

/// a recursive backtracking solution to chunk through the constraint sets
/// over the bits of register A, s.t. it produces a quine with the VM.
fn solve(
    space: &mut Space,
    targets: &[usize],
    constraints: &Constraints,
    depth: usize,
) -> Option<usize> {
    if depth >= targets.len() {
        return Some(space.finalized());
    }

    let mut min_solution: Option<usize> = None;

    let target = targets[depth];

    for constraint in &constraints[depth] {
        for (i, cstr) in constraint.iter().enumerate() {
            let tgt_bit = if (target >> i) & 1 == 1 { true } else { false };
            if !space.set(cstr.to_string().as_str(), tgt_bit) {
                return min_solution;
            }
        }
        if let Some(x) = solve(space, targets, constraints, depth + 1) {
            min_solution = min_solution.map_or(Some(x), |m| Some(m.min(x)));
        }
        for cstr in constraint {
            space.unset(cstr.to_string().as_str());
        }
    }

    min_solution
}

fn get_constraints(targets: &Vec<usize>) -> Vec<Vec<Vec<Bit>>> {
    targets
        .iter()
        .enumerate()
        .map(|(k, _tgt)| {
            let a = (0 + (3 * k)..8 + (3 * k))
                .map(|x| Bit::header(format!("a{}", x)))
                .collect::<Vec<_>>();

            (0..8)
                .map(|a_mod_8| {
                    let mut a2 = a.clone();

                    a2[0].set((a_mod_8 ^ 1) & 1);
                    a2[1].set(((a_mod_8 ^ 1) >> 1) & 1);
                    a2[2].set(((a_mod_8 ^ 1) >> 2) & 1);

                    let a2 = a2.iter().take(3).map(|b| b.clone()).collect::<Vec<_>>();

                    let mut a3 = a.clone();

                    a3[0].set(a_mod_8 & 1);
                    a3[1].set((a_mod_8 >> 1) & 1);
                    a3[2].set((a_mod_8 >> 2) & 1);

                    let a3 = a3
                        .iter()
                        .skip((a_mod_8 ^ 1) as usize)
                        .map(|bit| bit.clone())
                        .take(3)
                        .collect::<Vec<_>>();

                    a2.iter()
                        .zip(a3.iter())
                        .enumerate()
                        .map(|(i, (a2, a3))| {
                            let mut x = a2.xor(a3);
                            if i == 2 {
                                x = x.xor(&Bit::constant(1));
                            }

                            x
                        })
                        .collect()
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>()
}

fn print_table_console(k: usize, target: usize) {
    let ao = (0 + (3 * k)..8 + (3 * k))
        .rev()
        .map(|x| Bit::header(format!("a{}", x)))
        .collect::<Vec<_>>();
    let bo = (0 + (3 * k)..8 + (3 * k))
        .rev()
        .map(|x| Bit::header(format!("b{}", x)))
        .collect::<Vec<_>>();

    let mut a = Vec::<Vec<Bit>>::new();
    for i in 0..8 {
        let mut aa = ao.clone();
        for i in 0..5 {
            aa[i].set(0);
        }

        aa[5].set((i >> 2) & 1);
        aa[6].set((i >> 1) & 1);
        aa[7].set(i & 1);

        a.push(aa);
    }

    let mut a2 = Vec::<Vec<Bit>>::new();
    for i in 0..8 {
        let mut aa = ao.clone();
        for i in 0..5 {
            aa[i].set(0);
        }

        aa[5].set(((i ^ 1) >> 2) & 1);
        aa[6].set(((i ^ 1) >> 1) & 1);
        aa[7].set((i ^ 1) & 1);
        a2.push(aa);
    }

    let mut a3 = Vec::<Vec<Bit>>::new();
    for i in 0..8_usize {
        let mut aa = ao.clone();

        aa[5].set(((i >> 2) & 1) as u8);
        aa[6].set(((i >> 1) & 1) as u8);
        aa[7].set((i & 1) as u8);

        let rlo = 0..(i ^ 1);
        let rhi = 0..8 - (i ^ 1);
        let aa = rlo
            .rev()
            .into_iter()
            .map(|e| Bit::Named(format!("a{}", 8 + e)))
            .chain(aa[rhi].iter().map(|e| e.clone()))
            .collect();
        a3.push(aa);
    }

    println!(
        "Input #{:>2}, target = {}{}{}",
        k,
        (target >> 2) & 1,
        (target >> 1) & 1,
        target & 1
    );
    println!(
        "+{:-^37}+{:-^37}+{:-^37}+{:-^133}+",
        " A ", " A2 := A mod 8 ⊕ 1 ", " A3 := A >> (A mod 8 ⊕ 1) ", " A2 ⊕ A3 "
    );
    let headers = ao.iter().map(|b| format!("{:>3}", b.to_string())).join(" ");
    let headers2 = bo
        .iter()
        .map(|b| format!("{:>15}", b.to_string()))
        .join(" ");
    println!(
        "| ... {} | ... {} | ... {} | ... {}",
        headers, headers, headers, headers2
    );
    println!("+{:-^37}+{:-^37}+{:-^37}+{:-^133}+", "", "", "", "");
    for ((a, b), c) in a.into_iter().zip(a2.into_iter()).zip(a3) {
        let aout = a.iter().map(|b| format!("{:>3}", b.to_string())).join(" ");
        let aout = format!("... {}", aout);
        let bout = b.iter().map(|b| format!("{:>3}", b.to_string())).join(" ");
        let bout = format!("... {}", bout);
        let cout = c.iter().map(|b| format!("{:>3}", b.to_string())).join(" ");
        let cout = format!("... {}", cout);
        let a2_xor_a3 = b
            .iter()
            .zip(c.iter())
            .enumerate()
            .map(|(i, (b, c))| {
                let mut x = b.xor(c);
                if i == 5 {
                    x = x.xor(&Bit::constant(1));
                }

                format!("{:>15}", x.to_string())
            })
            .join(" ");
        let bxorc = format!("... {}", a2_xor_a3);
        println!("| {} | {} | {} | {} |", aout, bout, cout, bxorc);
    }
    println!();
}
