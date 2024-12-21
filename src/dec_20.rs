use std::{
    collections::HashMap,
    fmt::Write,
    sync::{Arc, Mutex},
};

use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec20.txt")?;

    let (track, start, end, h, w) = parse(&input);

    println!("Track length (ps): {}", track.len());

    let now = std::time::Instant::now();
    let result = handle_puzzle1(input.as_str())
        .into_iter()
        .filter_map(|(savings, count)| if savings >= 100 { Some(count) } else { None })
        .sum::<usize>();
    println!(
        "Puzzle 1: ans {:?}, ({} us)",
        result,
        now.elapsed().as_micros()
    );

    let now = std::time::Instant::now();
    let result = handle_puzzle2(input.as_str())
        .into_iter()
        .filter_map(|(savings, count)| if savings >= 100 { Some(count) } else { None })
        .sum::<usize>();
    println!(
        "Puzzle 2: ans {:?}, ({} us)",
        result,
        now.elapsed().as_micros()
    );

    Ok(())
}

type ParseOutput = (
    HashMap<(usize, usize), usize>,
    (usize, usize),
    (usize, usize),
    usize,
    usize,
);
fn parse(input: &str) -> ParseOutput {
    let input = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let w = input.len();
    let h = input[0].len();
    let mut start = (usize::MAX, usize::MAX);
    let mut end = (usize::MAX, usize::MAX);
    let mut track = HashMap::new();
    for i in 0..h {
        for j in 0..w {
            let cell = input[i][j];
            match cell {
                'S' => start = (i, j),
                'E' => end = (i, j),
                '.' => {
                    let _ = track.insert((i, j), None);
                }
                _ => {}
            }
        }
    }

    track.insert(start, Some(0));
    track.insert(end, None);

    let mut p = start;
    'outer: while p != end {
        let (pi, pj) = p;
        let p_dist = track.get(&p).unwrap().map(|dist| dist + 1);
        const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        for (i, j) in DIRS.iter() {
            let next = ((pi as i32 + *i) as usize, (pj as i32 + *j) as usize);
            if let Some(next_dist @ None) = track.get_mut(&next) {
                *next_dist = p_dist;
                p = next;
                continue 'outer;
            }
        }
        panic!();
    }

    let track = track
        .into_iter()
        .map(|(key, value)| (key, value.unwrap()))
        .collect::<HashMap<_, _>>();

    (track, start, end, h, w)
}
type Units = HashMap<usize, usize>;
fn handle_puzzle1(input: &str) -> Units {
    let (track, start, end, h, w) = parse(input);
    let mut count = HashMap::new();
    for (loc, current) in &track {
        for i in -2..=2 {
            let j_limit = 2 - i32::abs(i);
            for j in (-j_limit..=j_limit).step_by((2 * j_limit as usize).max(1)) {
                let other = ((loc.0 as i32 + i), (loc.1 as i32 + j));
                if other.0 >= 1 && other.1 >= 1 {
                    let other = (other.0 as usize, other.1 as usize);
                    if let Some(cheat) = track.get(&other) {
                        let savings = *cheat as i32 - *current as i32 - 2;
                        if savings > 0 {
                            *count.entry(savings as usize).or_default() += 1;
                        }
                    }
                }
            }
        }
    }

    count
}

fn handle_puzzle2(input: &str) -> HashMap<usize, usize> {
    let (track, _, _, h, w) = parse(input);
    track
        .clone()
        .into_iter()
        .par_bridge()
        .map(move |(loc, current)| {
            let mut count = Units::new();
            // constrain search to the height of the puzzle
            let ilobd = -(20.min(loc.0 as i32 + 1));
            let iupbd = 20.min(h as i32 - loc.0 as i32 - 1);

            for i in ilobd..=iupbd {
                // constrain search to the width of the puzzle
                let jlim = 20 - i32::abs(i);
                let jlobd = -(jlim.min(loc.1 as i32));
                let jhibd = jlim.min(w as i32 - loc.1 as i32);

                for j in jlobd..=jhibd {
                    let other = ((loc.0 as i32 + i) as usize, (loc.1 as i32 + j) as usize);

                    if let Some(cheat) = track.get(&other) {
                        let savings = cheat.saturating_sub(current);
                        let cost = (i32::abs(i) + i32::abs(j)) as usize;
                        if savings > cost {
                            *count.entry(savings - cost).or_default() += 1;
                        }
                    }
                }
            }

            count
        })
        .reduce_with(|mut acc, next| {
            for (k, v) in next {
                *acc.entry(k).or_default() += v;
            }
            acc
        })
        .unwrap()
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

    let res = handle_puzzle1(input);
    assert_eq!(res.get(&2), Some(&14));
    assert_eq!(res.get(&4), Some(&14));
    assert_eq!(res.get(&6), Some(&2));
    assert_eq!(res.get(&8), Some(&4));
    assert_eq!(res.get(&10), Some(&2));
    assert_eq!(res.get(&12), Some(&3));
    assert_eq!(res.get(&20), Some(&1));
    assert_eq!(res.get(&36), Some(&1));
    assert_eq!(res.get(&38), Some(&1));
    assert_eq!(res.get(&40), Some(&1));
    assert_eq!(res.get(&64), Some(&1));

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

    let res = handle_puzzle2(input);

    assert_eq!(res.get(&50), Some(&32));
    assert_eq!(res.get(&52), Some(&31));
    assert_eq!(res.get(&54), Some(&29));
    assert_eq!(res.get(&56), Some(&39));
    assert_eq!(res.get(&58), Some(&25));
    assert_eq!(res.get(&60), Some(&23));
    assert_eq!(res.get(&62), Some(&20));
    assert_eq!(res.get(&64), Some(&19));
    assert_eq!(res.get(&66), Some(&12));
    assert_eq!(res.get(&68), Some(&14));
    assert_eq!(res.get(&70), Some(&12));
    assert_eq!(res.get(&72), Some(&22));
    assert_eq!(res.get(&74), Some(&4));
    assert_eq!(res.get(&76), Some(&3));

    Ok(())
}

#[test]
fn test_parse() {
    let input = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;
    let (track, start, end, _, _) = parse(input);

    assert_eq!(track.len(), 85);
    assert_eq!(*track.get(&start).unwrap(), 0);
    assert_eq!(*track.get(&end).unwrap(), 84);
    assert_eq!(start, (3, 1));
    assert_eq!(end, (7, 5));
}
