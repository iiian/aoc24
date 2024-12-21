use std::collections::{HashMap, HashSet, VecDeque};

use itertools::iproduct;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec21.txt")?;

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

type ParseOutput = Vec<String>;
fn parse(input: &str) -> ParseOutput {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<ParseOutput>()
}

type Units = usize;
fn handle_puzzle1(input: &str) -> Units {
    let inputs = parse(input);
    let num = ClickMatrix::from(vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ]);
    let dpad = ClickMatrix::from(vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']]);

    // ♫♪♬ It's       beginning-to-feel         a-lot          like         ... pY      toRCh
    let layer_door_r1 = expand_layer(&num, inputs.clone());
    let layer_r1_r2 = expand_layer(&dpad, layer_door_r1);
    let layer_r2_r3 = expand_layer(&dpad, layer_r1_r2);
    let layer_r3_me = expand_layer(&dpad, layer_r2_r3);

    layer_r3_me
        .into_iter()
        .zip(inputs.into_iter())
        .map(|(path, input)| {
            path.len()
                * input
                    .chars()
                    .filter(|c| c.is_numeric())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap()
        })
        .sum()
}

fn expand_layer(matr: &ClickMatrix, layer_doorpad_robot_one: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    for input in layer_doorpad_robot_one {
        let input = input.chars();
        let mut step = String::new();
        let mut from = &'A';
        for to in input {
            let Some(frag) = matr.get(from, &to) else {
                panic!()
            };

            step += frag;
        }

        out.push(step);
    }
    out
}

/// A look up table for shortest "click" paths (or one thereof pseudo-randomly selected) from button to button
/// in AOC Day 21 DPAD notation, that is [<^>vA]+
struct ClickMatrix {
    /// the square matrix representing (row) => (column) moving from the ith button to the jth button
    layout: Vec<Vec<String>>,
    /// key into the layout by button id
    key: HashMap<char, usize>,
}

impl ClickMatrix {
    fn from(keypad_layout: Vec<Vec<char>>) -> Self {
        let h = keypad_layout.len();
        let w = keypad_layout[0].len();
        let dim = h * w - 1;

        let key = keypad_layout
            .iter()
            .flatten()
            .filter(|k| **k != ' ')
            .enumerate()
            .map(|(i, c)| (*c, i))
            .collect::<HashMap<_, _>>();

        let mut result = vec![vec![Option::<String>::None; dim]; dim];
        for (i, j) in iproduct!(0..h, 0..w).filter(|(i, j)| keypad_layout[*i][*j] != ' ') {
            let k_src = *key.get(&keypad_layout[i][j]).unwrap();
            let mut frontier =
                VecDeque::<((usize, usize), String)>::from([((i, j), String::from(""))]);
            let mut map = HashMap::<(usize, usize), String>::new();

            while let Some((next @ (ni, nj), path)) = frontier.pop_front() {
                if map.contains_key(&next) {
                    continue;
                }
                if let Some(k_dest) = key.get(&keypad_layout[ni][nj]) {
                    result[k_src][*k_dest] = Some(path.clone() + "A");
                    map.insert(next, path.clone());
                    if ni > 0 {
                        frontier.push_back(((ni - 1, nj), path.clone() + "^"));
                    }
                    if nj > 0 {
                        frontier.push_back(((ni, nj - 1), path.clone() + "<"));
                    }
                    if ni < h - 1 {
                        frontier.push_back(((ni + 1, nj), path.clone() + "v"));
                    }
                    if nj < w - 1 {
                        frontier.push_back(((ni, nj + 1), path.clone() + ">"));
                    }
                }
            }
        }

        let layout = result
            .into_iter()
            .map(|row| row.into_iter().filter_map(|k| k).collect())
            .collect();

        Self { key, layout }
    }

    pub fn get(&self, from: &char, to: &char) -> Option<&String> {
        if let Some(i) = self.key.get(from) {
            if let Some(j) = self.key.get(to) {
                return Some(&self.layout[*i][*j]);
            }
        }

        return None;
    }
}

fn handle_puzzle2(input: &str) -> Units {
    todo!()
}

#[test]
fn test_clickmatrix() {
    let cm = ClickMatrix::from(vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ]);

    println!("{:?}", cm.key);
    for row in cm.layout {
        println!("{:?}", row);
    }
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"029A
980A
179A
456A
379A"#;

    assert_eq!(handle_puzzle1(input), 126384);

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#""#;

    assert_eq!(handle_puzzle2(input), todo!());

    Ok(())
}
