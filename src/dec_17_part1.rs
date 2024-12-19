use std::{error::Error, str::FromStr};

#[derive(Clone, Copy)]
pub(crate) enum Instr {
    Adv(u8),
    Bxl(u8),
    Bst(u8),
    Jnz(u8),
    Bxc,
    Out(u8),
    Bdv(u8),
    Cdv(u8),
}

pub(crate) static REG_A: usize = 0;

pub(crate) static REG_B: usize = 1;

pub(crate) static REG_C: usize = 2;

impl Instr {
    pub(crate) fn combo<R>(x: u8, reg: &[R]) -> R
    where
        R: From<u8> + Clone,
    {
        match x {
            x if x < 4 => R::from(x),
            x if x < 7 => reg[(x - 4) as usize].clone(),
            _ => unreachable!(),
        }
    }

    pub(crate) fn exec<R>(&self, vm: &mut Vm<R>)
    where
        R: From<u8>
            + Clone
            + std::ops::BitXor<R, Output = R>
            + std::ops::Rem<usize, Output = R>
            + std::ops::Shr<Output = R>
            + std::ops::BitXorAssign<R>
            + std::cmp::PartialEq<usize>,
    {
        match self {
            Instr::Adv(x) => Instr::div(*x, vm, REG_A),
            Instr::Bxl(x) => {
                let reg_b = &mut vm.registers[REG_B];
                *reg_b ^= From::from(*x);
            }
            Instr::Bst(x) => {
                let bot_three_bits = Instr::combo(*x, &vm.registers) % 8;
                vm.registers[REG_B] = bot_three_bits;
            }
            Instr::Jnz(x) => {
                if vm.registers[REG_A] != 0_usize {
                    vm.ip = *x as usize;
                    return;
                }
            }
            Instr::Bxc => {
                let reg_c = vm.registers[REG_C].clone();
                let reg_b = &mut vm.registers[REG_B];
                *reg_b = reg_b.clone() ^ reg_c;
            }
            Instr::Out(x) => {
                let output = Instr::combo(*x, &vm.registers) % 8;
                vm.output.push(output);
            }
            Instr::Bdv(x) => Instr::div(*x, vm, REG_B),
            Instr::Cdv(x) => Instr::div(*x, vm, REG_C),
        }

        vm.ip += 1;
    }

    pub(crate) fn div<R>(x: u8, vm: &mut Vm<R>, reg: usize)
    where
        R: From<u8> + Clone + std::ops::Shr<Output = R>,
    {
        let shr = Instr::combo(x, &vm.registers);
        let num = vm.registers[REG_A].clone();
        let out = &mut vm.registers[reg];
        *out = num >> shr;
    }
}

pub(crate) struct InstrIterator<I: Iterator<Item = u8>> {
    pub(crate) source: I,
}

impl<I: Iterator<Item = u8>> Iterator for InstrIterator<I> {
    type Item = Instr;

    fn next(&mut self) -> Option<Self::Item> {
        let opcode = self.source.next()?;
        match opcode {
            0x00 => Some(Instr::Adv(self.source.next()?)),
            0x01 => Some(Instr::Bxl(self.source.next()?)),
            0x02 => Some(Instr::Bst(self.source.next()?)),
            0x03 => Some(Instr::Jnz(self.source.next()?)),
            0x04 => {
                self.source.next()?;
                Some(Instr::Bxc)
            }
            0x05 => Some(Instr::Out(self.source.next()?)),
            0x06 => Some(Instr::Bdv(self.source.next()?)),
            0x07 => Some(Instr::Cdv(self.source.next()?)),
            _ => None, // Invalid opcode
        }
    }
}

// Convenience method to create the iterator
pub(crate) trait InstrIteratorExt: Iterator<Item = u8> {
    fn to_instr_iter(self) -> InstrIterator<Self>
    where
        Self: Sized,
    {
        InstrIterator { source: self }
    }
}

impl<I: Iterator<Item = u8>> InstrIteratorExt for I {}

pub(crate) type TReg = usize;

#[derive(Clone)]
pub(crate) struct Vm<R> {
    pub(crate) registers: Vec<R>,
    pub(crate) ip: usize,
    pub(crate) instructions: Vec<Instr>,
    pub(crate) output: Vec<R>,
}

impl<R> Vm<R>
where
    R: From<u8>
        + Clone
        + std::ops::BitXor<R, Output = R>
        + std::ops::Rem<usize, Output = R>
        + std::ops::Shr<Output = R>
        + std::ops::BitXorAssign<R>
        + std::cmp::PartialEq<usize>,
{
    pub fn exec(&mut self) -> bool {
        if let Some(instr) = self.instructions.get(self.ip).cloned() {
            instr.exec(self);
        }

        self.ip < self.instructions.len()
    }

    pub fn reset(&mut self) {
        self.registers[REG_A] = From::from(0_u8);
        self.registers[REG_B] = From::from(0_u8);
        self.registers[REG_C] = From::from(0_u8);

        self.ip = 0;
        self.output = vec![];
    }

    pub(crate) fn new(instructions: Vec<Instr>, registers: Vec<R>) -> Self {
        Self {
            registers,
            ip: 0,
            instructions,
            output: vec![],
        }
    }
}

pub(crate) type ParseOutput<R> = (Vm<R>, String);

pub(crate) fn parse<R>(input: &str) -> ParseOutput<R>
where
    R: From<u8>
        + From<usize>
        + Clone
        + std::ops::BitXor<R, Output = R>
        + std::ops::Rem<usize, Output = R>
        + std::ops::Shr<Output = R>
        + std::ops::BitXorAssign<R>
        + std::cmp::PartialEq<usize>,
{
    let mut lines = input.lines();

    let reg_a = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse::<usize>()
        .unwrap();

    let reg_b = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse::<usize>()
        .unwrap();

    let reg_c = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse::<usize>()
        .unwrap();

    let registers = vec![From::from(reg_a), From::from(reg_b), From::from(reg_c)];

    lines.next();

    let instructions = lines.next().unwrap().split_once(": ").unwrap().1;
    let quine_tgt = instructions;

    let instructions = instructions
        .split(",")
        .map(|num| num.parse::<u8>().unwrap())
        .to_instr_iter()
        .collect::<Vec<_>>();

    (Vm::new(instructions, registers), quine_tgt.to_string())
}

pub(crate) type Units = String;

pub(crate) fn handle_puzzle1(input: &str) -> Units {
    let (mut vm, _) = parse::<usize>(input);

    while vm.exec() {}

    vm.output
        .into_iter()
        .map(|k| k.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    assert_eq!(handle_puzzle1(input), "4,6,3,5,6,3,5,2,1,0");

    Ok(())
}
