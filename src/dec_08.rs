use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn puzzle1() -> Result<u32, Box<dyn std::error::Error>> {
    handle_puzzle1(std::fs::read_to_string("./inputs/dec08.txt")?.as_str())
}

pub fn puzzle2() -> Result<u32, Box<dyn std::error::Error>> {
    handle_puzzle2(std::fs::read_to_string("./inputs/dec08.txt")?.as_str())
}

pub fn within_boundary((x, y): (i32, i32), width: i32, height: i32) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}

fn handle_puzzle1(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let (antennae, width, height) = load_input(input);

    let mut antinodes = HashSet::<(i32, i32)>::new();
    for (_, antennae) in antennae {
        for pair in antennae.iter().combinations(2) {
            let [&(x1, y1), &(x2, y2)] = pair[0..2] else {
                panic!()
            };

            let (mx, my) = (x2 - x1, y2 - y1);
            let p1 = (x2 + mx, y2 + my);
            let p2 = (x1 - mx, y1 - my);

            if within_boundary(p1, width, height) {
                antinodes.insert(p1);
            }
            if within_boundary(p2, width, height) {
                antinodes.insert(p2);
            }
        }
    }

    Ok(antinodes.len() as u32)
}

fn load_input(input: &str) -> (HashMap<u8, HashSet<(i32, i32)>>, i32, i32) {
    let mut antennae = HashMap::<u8, HashSet<(i32, i32)>>::new();
    let width = input.lines().count() as i32;
    let height = input.lines().next().unwrap().len() as i32;
    let every_antenna = input.lines().enumerate().flat_map(|(y, line)| {
        line.as_bytes()
            .iter()
            .enumerate()
            .filter(|(_, c)| **c != b'.')
            .map(move |(x, char)| (*char, y, x))
    });
    for (fq, y, x) in every_antenna {
        antennae.entry(fq).or_default().insert((x as i32, y as i32));
    }
    (antennae, width, height)
}

fn handle_puzzle2(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let (antennae, width, height) = load_input(input);

    let mut antinodes = HashSet::<(i32, i32)>::new();
    for (_, antennae) in antennae {
        if antennae.len() > 2 {
            antennae.iter().for_each(|point| {
                antinodes.insert(*point);
            });
        }
        for pair in antennae.iter().combinations(2) {
            let [&(x1, y1), &(x2, y2)] = pair[0..2] else {
                panic!()
            };
            let (mx, my) = (x2 - x1, y2 - y1);
            let mut p = (x2, y2);
            loop {
                p = (p.0 + mx, p.1 + my);

                if !within_boundary(p, width, height) {
                    break;
                }
                antinodes.insert(p);
            }
            p = (x1, y1);
            loop {
                p = (p.0 - mx, p.1 - my);

                if !within_boundary(p, width, height) {
                    break;
                }
                antinodes.insert(p);
            }
        }
    }

    Ok(antinodes.len() as u32)
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    assert_eq!(handle_puzzle1(input)?, 14);

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    assert_eq!(handle_puzzle2(input)?, 34);

    let input = r#"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."#;

    assert_eq!(handle_puzzle2(input)?, 9);

    Ok(())
}
