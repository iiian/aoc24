use itertools::iproduct;
use rayon::prelude::*;
use std::{collections::HashSet, sync::Arc};

pub fn puzzle1() -> Result<u32, Box<dyn std::error::Error>> {
    handle_puzzle1(std::fs::read_to_string("./inputs/dec06.txt")?.as_str())
}

pub fn puzzle2() -> Result<u32, Box<dyn std::error::Error>> {
    handle_puzzle2(std::fs::read_to_string("./inputs/dec06.txt")?.as_str())
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy)]
enum Square {
    Guard(Direction),
    Empty,
    Terrain,
}

impl Square {
    pub fn parse(c: &u8) -> Self {
        match c {
            b'.' => Square::Empty,
            b'^' => Square::Guard(Direction::North),
            b'v' => Square::Guard(Direction::South),
            b'<' => Square::Guard(Direction::West),
            b'>' => Square::Guard(Direction::East),
            b'#' => Square::Terrain,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct Engine {
    state: Vec<Vec<Square>>,
    guard_location: Option<(usize, usize)>,
    guard_visited: HashSet<(usize, usize)>,
}

impl Engine {
    pub fn load_from_string(input: &str) -> Self {
        let mut state = vec![];
        let mut guard_location = None;
        for (i, line) in input.lines().enumerate() {
            let mut next = vec![];
            for (j, cell) in line.as_bytes().iter().enumerate() {
                let square = Square::parse(cell);
                if let Square::Guard(_) = square {
                    guard_location = Some((i, j));
                }
                next.push(square);
            }

            state.push(next);
        }

        Self {
            state,
            guard_location,
            guard_visited: HashSet::new(),
        }
    }

    pub fn count_distinct_positions(self: &Self) -> usize {
        self.guard_visited.len()
    }
}

impl Iterator for Engine {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let height = self.state.len();
        let width = self.state[0].len();
        let Some(guard_location) = self.guard_location else {
            panic!("guard_location unexpectedly empty")
        };
        self.guard_visited.insert(guard_location);
        let (i, j) = guard_location;
        let Square::Guard(mut direction) = self.state[i][j] else {
            panic!("guard unexpectedly missing")
        };
        let (mut new_i, mut new_j) = (i as i64, j as i64);
        match direction {
            Direction::North => {
                if 0 < i {
                    match self.state[i - 1][j] {
                        Square::Terrain => direction = Direction::East,
                        Square::Empty => new_i -= 1,
                        _ => unreachable!(),
                    }
                } else {
                    new_i -= 1;
                }
            }
            Direction::South => {
                if i < height - 1 {
                    match self.state[i + 1][j] {
                        Square::Terrain => direction = Direction::West,
                        Square::Empty => new_i += 1,
                        _ => unreachable!(),
                    }
                } else {
                    new_i += 1;
                }
            }
            Direction::East => {
                if j < width - 1 {
                    match self.state[i][j + 1] {
                        Square::Terrain => direction = Direction::South,
                        Square::Empty => new_j += 1,
                        _ => unreachable!(),
                    }
                } else {
                    new_j += 1;
                }
            }
            Direction::West => {
                if 0 < j {
                    match self.state[i][j - 1] {
                        Square::Terrain => direction = Direction::North,
                        Square::Empty => new_j -= 1,
                        _ => unreachable!(),
                    }
                } else {
                    new_j -= 1;
                }
            }
        }
        self.state[i][j] = Square::Empty;
        if new_i < 0 || new_i as usize >= height || new_j < 0 || new_j as usize >= width {
            None // guard has left the map, so terminate the iterator
        } else {
            self.guard_location = Some((new_i as usize, new_j as usize));
            self.state[new_i as usize][new_j as usize] = Square::Guard(direction);
            Some(())
        }
    }
}

fn handle_puzzle1(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut engine = Engine::load_from_string(input);

    // step through the engine
    for () in &mut engine {}

    Ok(engine.count_distinct_positions() as u32)
}

#[derive(Clone)]
struct Engine2 {
    state: Vec<Vec<Square>>,
    guard_location: Option<(usize, usize)>,
    guard_path: HashSet<(usize, usize, Direction)>,
    has_loop: Option<bool>,
}

impl Engine2 {
    pub fn load_from_string(input: &str) -> Self {
        let mut state = vec![];
        let mut guard_location = None;
        for (i, line) in input.lines().enumerate() {
            let mut next = vec![];
            for (j, cell) in line.as_bytes().iter().enumerate() {
                let square = Square::parse(cell);
                if let Square::Guard(_) = square {
                    guard_location = Some((i, j));
                }
                next.push(square);
            }

            state.push(next);
        }

        Self {
            state,
            guard_location,
            guard_path: HashSet::new(),
            has_loop: None,
        }
    }

    fn loop_detected(&self) -> Option<bool> {
        self.has_loop
    }
}

impl Iterator for Engine2 {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let height = self.state.len();
        let width = self.state[0].len();
        let Some(guard_location) = self.guard_location else {
            panic!("guard_location unexpectedly empty")
        };
        let (i, j) = guard_location;
        let Square::Guard(mut direction) = self.state[i][j] else {
            panic!("guard unexpectedly missing")
        };
        self.guard_path.insert((i, j, direction));
        let (mut new_i, mut new_j) = (i as i64, j as i64);
        match direction {
            Direction::North => {
                if 0 < i {
                    match self.state[i - 1][j] {
                        Square::Terrain => direction = Direction::East,
                        Square::Empty => new_i -= 1,
                        _ => unreachable!(),
                    }
                } else {
                    new_i -= 1;
                }
            }
            Direction::South => {
                if i < height - 1 {
                    match self.state[i + 1][j] {
                        Square::Terrain => direction = Direction::West,
                        Square::Empty => new_i += 1,
                        _ => unreachable!(),
                    }
                } else {
                    new_i += 1;
                }
            }
            Direction::East => {
                if j < width - 1 {
                    match self.state[i][j + 1] {
                        Square::Terrain => direction = Direction::South,
                        Square::Empty => new_j += 1,
                        _ => unreachable!(),
                    }
                } else {
                    new_j += 1;
                }
            }
            Direction::West => {
                if 0 < j {
                    match self.state[i][j - 1] {
                        Square::Terrain => direction = Direction::North,
                        Square::Empty => new_j -= 1,
                        _ => unreachable!(),
                    }
                } else {
                    new_j -= 1;
                }
            }
        }
        self.state[i][j] = Square::Empty;
        if new_i < 0 || new_i as usize >= height || new_j < 0 || new_j as usize >= width {
            self.has_loop = Some(false);
            None // guard has left the map, so terminate the iterator
        } else if self
            .guard_path
            .contains(&(new_i as usize, new_j as usize, direction))
        {
            self.has_loop = Some(true);
            None
        } else {
            self.guard_location = Some((new_i as usize, new_j as usize));
            self.state[new_i as usize][new_j as usize] = Square::Guard(direction);
            Some(())
        }
    }
}

fn handle_puzzle2(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let base_engine = Engine2::load_from_string(input);
    let height = base_engine.state.len();
    let width = base_engine.state[0].len();
    let search_space = iproduct!(0..height, 0..width);

    let sum = search_space
        .par_bridge()
        .map(|(y, x)| {
            // Technically, time could factor into this, as loops
            // with dynamically added terrain may differ from loops with obstacle
            // added only at the start.

            // Todo: possibly need to add check that obstacle is not directly in
            //       front of guard.
            if matches!(base_engine.state[y][x], Square::Empty) {
                let mut engine = base_engine.clone();
                engine.state[y][x] = Square::Terrain;

                for () in &mut engine {}

                if let Some(true) = engine.loop_detected() {
                    return 1;
                }
            }

            0
        })
        .sum();

    Ok(sum)
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    assert_eq!(handle_puzzle1(input)?, 41);

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    assert_eq!(handle_puzzle2(input)?, 6);

    Ok(())
}
