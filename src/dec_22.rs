use std::collections::{hash_map::Entry, HashMap, LinkedList};

use itertools::Itertools;
use rayon::prelude::*;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec22.txt")?;

    let now = std::time::Instant::now();
    let result = handle_puzzle1(input.as_str());
    println!(
        "Puzzle 1: ans {:?}, ({} us)",
        result,
        now.elapsed().as_micros()
    );

    let now = std::time::Instant::now();
    let result = handle_puzzle2(input.as_str(), 2000);
    println!(
        "Puzzle 2: ans {:?}, ({} us)",
        result,
        now.elapsed().as_micros()
    );

    Ok(())
}

type ParseOutput = Vec<usize>;
fn parse(input: &str) -> ParseOutput {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

type Units = usize;
fn handle_puzzle1(input: &str) -> Units {
    parse(input)
        .into_iter()
        .map(|seed| Monke { seed }.nth(1_999).unwrap())
        .sum()
}

fn handle_puzzle2(input: &str, take_size: usize) -> (LinkedList<i8>, usize) {
    let k = input
        .lines()
        .map(|line| line.parse().unwrap())
        .par_bridge()
        .map(|seed| {
            Monke { seed }.into_delta().take(take_size).fold(
                HashMap::<LinkedList<i8>, usize>::new(),
                |mut acc, (delta, value)| {
                    if let Entry::Vacant(e) = acc.entry(delta) {
                        e.insert(value as usize);
                    }
                    acc
                },
            )
        })
        .collect::<Vec<_>>();

    k.into_iter()
        .fold(HashMap::<LinkedList<i8>, usize>::new(), |mut acc, next| {
            for (key, value) in next {
                *acc.entry(key).or_default() += value;
            }
            acc
        })
        .into_iter()
        .max_by_key(|(_, v)| *v)
        .unwrap()
}

#[derive(Clone)]
struct Monke {
    seed: usize,
}

impl Iterator for Monke {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.seed ^= self.seed * 64;
        self.seed %= 16777216;

        self.seed ^= self.seed / 32;
        self.seed %= 16777216;

        self.seed ^= self.seed * 2048;
        self.seed %= 16777216;
        Some(self.seed)
    }
}

impl Monke {
    pub fn into_delta(self) -> DeltaMonke {
        let mut prev = LinkedList::new();
        let mut monke = self;

        for _ in 0..=4 {
            prev.push_back((monke.next().unwrap() % 10) as i8);
        }

        DeltaMonke { monke, prev }
    }
}

#[derive(Clone)]
struct DeltaMonke {
    monke: Monke,
    prev: LinkedList<i8>,
}

impl Iterator for DeltaMonke {
    type Item = (LinkedList<i8>, i8);

    fn next(&mut self) -> Option<Self::Item> {
        let next = (self.monke.next().unwrap() % 10) as i8;
        self.prev.pop_front();
        self.prev.push_back(next);
        let out_delt = self
            .prev
            .iter()
            .tuple_windows()
            .map(|(a, b)| *b - *a)
            .collect::<LinkedList<_>>();

        Some((out_delt, next))
    }
}

#[test]
fn test_sequence() {
    let mut iter = Monke { seed: 123 };
    let expectation = vec![
        15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254,
    ];
    for expectation in expectation {
        assert_eq!(iter.next(), Some(expectation));
    }
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"1
10
100
2024"#;

    assert_eq!(handle_puzzle1(input), 37327623);

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"1
2
3
2024"#;

    assert_eq!(
        handle_puzzle2(input, 2000),
        (LinkedList::from([-2, 1, -1, 3]), 23)
    );

    Ok(())
}
