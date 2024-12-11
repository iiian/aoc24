use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

pub fn puzzle1() -> Result<usize, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec10.txt")?;
    let now = Instant::now();

    let ans = handle_puzzle1(&input);
    println!("({}µs)", now.elapsed().as_micros());

    ans
}

pub fn puzzle2() -> Result<usize, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec10.txt")?;
    let now = Instant::now();

    let ans = handle_puzzle2(&input);
    println!("({}µs)", now.elapsed().as_micros());

    ans
}

const DIRECTIONS: [Off; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[inline]
fn get_pos(input: &[Vec<u32>], (mut i, mut j): Coord, off: Off) -> Option<(u32, Coord)> {
    match off {
        (-1, 0) => {
            if i == 0 {
                return None;
            }
            i -= 1;
        }
        (1, 0) => {
            if i + 1 == input.len() {
                return None;
            }
            i += 1;
        }
        (0, -1) => {
            if j == 0 {
                return None;
            }
            j -= 1;
        }
        (0, 1) => {
            if j + 1 == input.len() {
                return None;
            }
            j += 1;
        }
        _ => {
            return None;
        }
    }
    Some((input[i][j], (i, j)))
}

fn handle_puzzle1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    // Approach: expand a topological frontier

    type ReachablePeaks = HashSet<Coord>;
    type Frontier = HashMap<Coord, ReachablePeaks>;
    let input = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or(u32::MAX))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut frontier = Frontier::new();
    for (i, line) in input.iter().enumerate() {
        for (j, level) in line.iter().enumerate() {
            if *level == 9 {
                frontier.entry((i, j)).or_default().insert((i, j));
            }
        }
    }

    for target_level in (0..9).rev() {
        let mut new_frontier = Frontier::new();
        for (pos, nines) in frontier {
            for dir in DIRECTIONS {
                if let Some((adj_level, adj)) = get_pos(&input, pos, dir) {
                    if adj_level == target_level {
                        new_frontier.entry(adj).or_default().extend(nines.clone());
                    }
                }
            }
        }
        frontier = new_frontier;
    }

    Ok(frontier.values().map(|set| set.len()).sum())
}

fn handle_puzzle2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    // Approach: expand a topological frontier

    type NumPaths = usize;
    type Frontier = HashMap<Coord, NumPaths>;

    let input = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or(u32::MAX))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut frontier = Frontier::new();
    for (i, line) in input.iter().enumerate() {
        for (j, level) in line.iter().enumerate() {
            if *level == 0 {
                frontier.insert((i, j), 1);
            }
        }
    }

    for target_level in 1..=9 {
        let mut new_frontier = Frontier::new();
        for (pos, paths) in frontier {
            for dir in DIRECTIONS {
                if let Some((adj_level, adj)) = get_pos(&input, pos, dir) {
                    if adj_level == target_level {
                        *new_frontier.entry(adj).or_default() += paths;
                    }
                }
            }
        }
        frontier = new_frontier;
    }

    Ok(frontier.values().sum())
}

type Coord = (usize, usize);
type Off = (i8, i8);

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"0123
1234
8765
9876"#;

    assert_eq!(handle_puzzle1(input)?, 1);

    let input = r#"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"#;

    assert_eq!(handle_puzzle1(input)?, 2);

    let input = r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#;

    assert_eq!(handle_puzzle1(input)?, 4);

    let input = r#"10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01"#;

    assert_eq!(handle_puzzle1(input)?, 3);

    let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    assert_eq!(handle_puzzle1(input)?, 36);

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9...."#;

    assert_eq!(handle_puzzle2(input)?, 3);
    let input = r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#;

    assert_eq!(handle_puzzle2(input)?, 13);

    let input = r#"012345
123456
234567
345678
4.6789
56789."#;

    assert_eq!(handle_puzzle2(input)?, 227);

    let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    assert_eq!(handle_puzzle2(input)?, 81);

    Ok(())
}
