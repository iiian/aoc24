use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
    io::{Seek, SeekFrom, Write},
    usize,
};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec16.txt")?;
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

type ProblemSpace = (HashSet<(usize, usize)>, (usize, usize), (usize, usize));
fn parse(input: &str) -> ProblemSpace {
    let mut out = HashSet::<(usize, usize)>::new();
    let lines = input.lines();
    let width = lines.clone().next().unwrap().len();
    let height = lines.clone().count();
    for (i, line) in lines.enumerate() {
        for (j, c) in line.char_indices() {
            if c != '#' {
                out.insert((i, j));
            }
        }
    }

    (out, (height - 2, 1), (1, width - 2))
}
#[derive(PartialEq, Eq, Clone, Copy, Ord, PartialOrd)]
enum Dir {
    N = 0,
    E = 1,
    S = 2,
    W = 3,
}

impl Dir {
    pub fn turn(&self, direction: i8) -> Dir {
        let current = *self as i8;
        let new_value = (current + direction).rem_euclid(4); // Ensures wrapping within 0-3
        Dir::try_from(new_value as u8).unwrap()
    }
}

impl TryFrom<u8> for Dir {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Dir::N),
            1 => Ok(Dir::E),
            2 => Ok(Dir::S),
            3 => Ok(Dir::W),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Point {
    pt: (usize, usize),
    dir: Dir,
}

impl Point {
    pub fn new(coord: (usize, usize), dir: Dir) -> Self {
        Self { pt: coord, dir }
    }

    pub fn move_turn(&self, x: i8) -> Self {
        let Point { pt: (i, j), dir } = self;
        let dir = dir.turn(x);
        let pt = match dir {
            Dir::N => (*i - 1, *j),
            Dir::S => (*i + 1, *j),
            Dir::E => (*i, *j + 1),
            Dir::W => (*i, *j - 1),
        };

        Self::new(pt, dir)
    }

    pub fn turn(&self, x: i8) -> Self {
        let dir = self.dir.turn(x);
        Self::new(self.pt, dir)
    }
}

type Units = Option<usize>;
fn handle_puzzle1(input: &str) -> Units {
    let (traversable, start, goal) = parse(input);
    let mut heap = BinaryHeap::<State>::new();
    let mut distances = HashMap::<(usize, usize), usize>::new();
    for pt in traversable {
        distances.insert(pt, usize::MAX);
    }
    *distances.get_mut(&start).unwrap() = 0;

    heap.push(State {
        cost: 0,
        pos: Point::new(start, Dir::E),
    });

    while let Some(State { pos, cost }) = heap.pop() {
        if pos.pt == goal {
            return Some(distances[&goal]);
        }
        if cost > *distances.get(&pos.pt).unwrap() {
            continue;
        }

        for (pos, dcost) in [
            (pos.move_turn(-1), 1000),
            (pos.move_turn(0), 0),
            (pos.move_turn(1), 1000),
        ] {
            if !distances.contains_key(&pos.pt) {
                continue;
            }
            let cost = cost + dcost + 1;
            if cost <= *distances.get(&pos.pt).unwrap() {
                heap.push(State { pos, cost });
                *distances.get_mut(&pos.pt).unwrap() = cost;
            }
        }
    }

    None
}

fn handle_puzzle2(input: &str) -> Units {
    let (traversable, start, goal) = parse(input);
    let mut heap = BinaryHeap::<State>::new();
    let mut dist = HashMap::<(usize, usize), usize>::new();
    for pt in traversable {
        dist.insert(pt, usize::MAX);
    }
    *dist.get_mut(&start).unwrap() = 0;

    heap.push(State {
        cost: 0,
        pos: Point::new(start, Dir::E),
    });

    while let Some(State { pos, cost }) = heap.pop() {
        if cost > *dist.get(&pos.pt).unwrap() {
            continue;
        }

        for (pt, dcost) in [
            (pos.move_turn(-1), 1000),
            (pos.move_turn(0), 0),
            (pos.move_turn(1), 1000),
        ] {
            if !dist.contains_key(&pt.pt) {
                continue;
            }
            let cost = cost + dcost + 1;
            if cost <= *dist.get(&pt.pt).unwrap() {
                heap.push(State { pos: pt, cost });
                *dist.get_mut(&pt.pt).unwrap() = cost;
            }
        }
    }

    let mut ways = 1;
    let mut visited = HashSet::<(usize, usize)>::new();
    visited.insert(goal);
    let mut frontier = VecDeque::<(Point, usize)>::new();
    if let Some(goal_dist) = dist.get(&(goal.0 + 1, goal.1)) {
        if *goal_dist != usize::MAX {
            let pt = Point::new((goal.0 + 1, goal.1), Dir::S);
            frontier.push_back((pt, dist[&goal]));
        }
    }
    if let Some(goal_dist) = dist.get(&(goal.0, goal.1 - 1)) {
        if *goal_dist != usize::MAX {
            let pt = Point::new((goal.0, goal.1 - 1), Dir::W);
            frontier.push_back((pt, dist[&goal]));
        }
    }

    while let Some((p1, target)) = frontier.pop_front() {
        if p1.pt == (7, 4) {
            println!("");
        }
        let d = dist[&p1.pt];
        if !(d + 1001 == target || d + 1 == target) || visited.contains(&p1.pt) {
            continue;
        }
        ways += 1;
        visited.insert(p1.pt);
        for (p2, cost) in [
            (p1.move_turn(0), 1000),
            (p1.move_turn(-1), 0),
            (p1.move_turn(1), 0),
        ] {
            if let Some(di) = dist.get(&p2.pt) {
                if *di == usize::MAX {
                    continue;
                }
                frontier.push_front((p2, d));
                frontier.push_front((p2, d + cost));
            }
        }
    }

    Some(ways)
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    assert_eq!(handle_puzzle1(input), Some(7036));

    let input = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    assert_eq!(handle_puzzle1(input), Some(11048));

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    assert_eq!(handle_puzzle2(input), Some(45));

    let input = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    assert_eq!(handle_puzzle2(input), Some(64));

    Ok(())
}
