use std::{collections::HashMap, fs::File, io::Write};

use regex::Regex;

type Units = i64;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec14.txt")?;
    let now = std::time::Instant::now();

    let result = handle_puzzle1(input.as_str(), 101, 103, 100);
    println!(
        "Puzzle 1: ans={}, ({} us)",
        result,
        now.elapsed().as_micros()
    );

    // let result = handle_puzzle2(input.as_str());
    // println!(
    //     "Puzzle 2: ans={}, ({} us)",
    //     result,
    //     now.elapsed().as_micros()
    // );

    Ok(())
}

type ParseOutput = Vec<((i64, i64), (i64, i64))>;
fn parse(input: &str) -> ParseOutput {
    let line_re = Regex::new(r"p=([+-]?\d+),([+-]?\d+) v=([+-]?\d+),([+-]?\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = line_re.captures(line).unwrap();
            let (px, py) = (caps.get(1).unwrap(), caps.get(2).unwrap());
            let (vx, vy) = (caps.get(3).unwrap(), caps.get(4).unwrap());

            (
                (px.as_str().parse().unwrap(), py.as_str().parse().unwrap()),
                (vx.as_str().parse().unwrap(), vy.as_str().parse().unwrap()),
            )
        })
        .collect()
}

fn handle_puzzle1(input: &str, width: i64, height: i64, runtime: i64) -> Units {
    let input = parse(input);

    input
        .into_iter()
        .map(|((ix, iy), (vx, vy))| {
            let ex = ((ix + (vx * runtime)) % width + width) % width;
            let ey = ((iy + (vy * runtime)) % height + height) % height;

            if ex == width / 2 || ey == height / 2 {
                0
            } else {
                match (ex < width / 2, ey < height / 2) {
                    (true, true) => 1,
                    (true, false) => 2,
                    (false, true) => 3,
                    (false, false) => 4,
                }
            }
        })
        .filter(|a| *a > 0)
        .fold(HashMap::<i64, i64>::new(), |mut acc, next| {
            *acc.entry(next).or_default() += 1;
            acc
        })
        .into_values()
        .reduce(|a, b| a * b)
        .unwrap()
}

fn handle_puzzle2(input: &str) -> Units {
    let mut file = File::create("output.txt").unwrap();
    let mut state = parse(input);
    let mut draw = vec![vec![' '; 103]; 103];
    for ((pt_x, pt_y), _) in &state {
        draw[*pt_x as usize][*pt_y as usize] = '*';
    }
    let width = 101;
    let height = 103;

    // the system *must* have a period of 101 * 103 cycles
    for i in (0..(width * height)) {
        for ((px, py), (vx, vy)) in &mut state {
            draw[*px as usize][*py as usize] = ' ';
            *px = ((*px + (i) * (*vx)) % width + width) % width;
            *py = ((*py + (i) * (*vy)) % height + height) % height;
            draw[*px as usize][*py as usize] = '*';
        }

        // search for patterns? maybe regex matches?
        writeln!(file, "iteration #{}:", i + 1).unwrap();
        print_state(&mut file, &draw);
    }

    0
}

fn print_state(file: &mut dyn Write, draw: &[Vec<char>]) {
    for row in draw {
        for c in row {
            write!(file, "{c}").unwrap();
        }
        writeln!(file).unwrap();
    }
}

#[test]
fn test_parse() {
    let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    let result = parse(input);
    assert_eq!(result[0].0 .0, 0);
    assert_eq!(result[2].0 .1, 3);
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    assert_eq!(handle_puzzle1(input, 11, 7, 100), 12);

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#""#;

    assert_eq!(handle_puzzle2(input), todo!());

    Ok(())
}
