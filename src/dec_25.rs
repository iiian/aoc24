#![feature(impl_trait_in_bindings)]

use std::collections::{HashMap, HashSet, VecDeque};

use itertools::iproduct;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec25.txt")?;

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

type KL = (u8, u8, u8, u8, u8);
type ParseOutput = (HashSet<KL>, HashSet<KL>);
fn parse(input: &str) -> ParseOutput {
    let kls = input.split("\n\n");

    kls.into_iter().fold(
        (HashSet::<KL>::new(), HashSet::<KL>::new()),
        |(mut keys, mut locks), schematic| {
            let is_key = schematic.starts_with('.');
            let iter: Box<dyn Iterator<Item = &str>> = if is_key {
                Box::new(schematic.lines())
            } else {
                Box::new(schematic.lines().rev())
            };
            let mut pins = (0, 0, 0, 0, 0);
            let mut u = vec![0, 1, 2, 3, 4];
            for (i, line) in iter.enumerate().skip(1) {
                let mut next = vec![];
                let chars = line.as_bytes();
                while let Some(j) = u.pop() {
                    if chars[j] == b'#' {
                        match j {
                            0 => pins.0 = 6 - i as u8,
                            1 => pins.1 = 6 - i as u8,
                            2 => pins.2 = 6 - i as u8,
                            3 => pins.3 = 6 - i as u8,
                            4 => pins.4 = 6 - i as u8,
                            _ => unreachable!(),
                        }
                    } else {
                        next.push(j);
                    }
                }
                u = next;
            }
            assert_eq!(u.len(), 0);
            let entity = if is_key { &mut keys } else { &mut locks };
            entity.insert(pins);
            (keys, locks)
        },
    )
}
type Units = usize;
fn handle_puzzle1(input: &str) -> Units {
    let (keys, locks) = parse(input);

    let mut count = 0;
    for key in &keys {
        for lock in &locks {
            if fits(key, lock) {
                count += 1;
            }
        }
    }

    count
}

fn fits(key: &(u8, u8, u8, u8, u8), lock: &(u8, u8, u8, u8, u8)) -> bool {
    key.0 + lock.0 < 6
        && key.1 + lock.1 < 6
        && key.2 + lock.2 < 6
        && key.3 + lock.3 < 6
        && key.4 + lock.4 < 6
}

fn handle_puzzle2(input: &str) -> Units {
    todo!()
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;

    assert_eq!(handle_puzzle1(input), 3);

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#""#;

    assert_eq!(handle_puzzle2(input), todo!());

    Ok(())
}
