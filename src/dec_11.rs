use std::{
    collections::{HashMap, HashSet, LinkedList},
    fmt::{Debug, Display, Pointer, Write},
    time::Instant,
};

type Units = usize;

pub fn puzzle1() -> Result<Units, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec11.txt")?;
    let now = Instant::now();

    let ans = handle_puzzle1(&input, 25);
    println!("({}µs)", now.elapsed().as_micros());

    ans
}

pub fn puzzle2() -> Result<Units, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec11.txt")?;
    let now = Instant::now();

    let ans = handle_puzzle2(&input);
    println!("({}µs)", now.elapsed().as_micros());

    ans
}

fn handle_puzzle1(input: &str, rounds: usize) -> Result<Units, Box<dyn std::error::Error>> {
    let mut stones = input
        .split_whitespace()
        .map(|data| Stone::new(data.to_string()))
        .collect::<LinkedList<_>>();

    for i in 0..rounds {
        let mut new_stones = LinkedList::new();
        while let Some(mut stone) = stones.pop_front() {
            if let Some(new_stone) = stone.roll() {
                new_stones.push_back(stone);
                new_stones.push_back(new_stone);
            } else {
                new_stones.push_back(stone);
            }
        }
        stones = new_stones;
    }

    Ok(stones.len())
}

/// (number, duplicates)
type ExpansionNode = (usize, usize);

/// We can solve this problem using hypergraphs (graphs where individual edges simultaneously lead to more than one node)
/// https://en.wikipedia.org/wiki/Hypergraph credit Stephen Wolfram and his batshit crazy theory of
/// fundamental physics.
type Hypergraph = HashMap<usize, Vec<ExpansionNode>>;

fn handle_puzzle2(input: &str) -> Result<Units, Box<dyn std::error::Error>> {
    let mut cycle_space = Hypergraph::new();
    cycle_space.insert(0, vec![(1, 1)]);

    let mut nums = input
        .split_whitespace()
        .map(|n| (n.parse::<usize>().unwrap(), 1))
        .collect::<HashMap<usize, usize>>();

    for _ in 0..75 {
        let mut new_nums = HashMap::<usize, usize>::new();
        for (num, count) in nums.iter() {
            if !cycle_space.contains_key(num) {
                // handle rule processing, update hypergraph
                let num_str = num.to_string();
                let edges = if num_str.len() % 2 == 0 {
                    let (a, b) = num_str.split_at(num_str.len() / 2);
                    vec![(a.parse().unwrap(), 1), (b.parse().unwrap(), 1)]
                } else {
                    vec![(num * 2024, 1)]
                };

                cycle_space.insert(*num, edges);
            }
            let edges = cycle_space.get(num).unwrap();
            for (number, duplicates) in edges {
                *new_nums.entry(*number).or_default() += count * duplicates;
            }
        }
        nums = new_nums;
    }

    Ok(nums.values().sum())
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"0 1 10 99 999"#;

    assert_eq!(handle_puzzle1(input, 1)?, 7);

    let input = r#"125 17"#;

    assert_eq!(handle_puzzle1(input, 6)?, 22);
    assert_eq!(handle_puzzle1(input, 25)?, 55312);

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"0 1 10 99 999"#;

    assert_eq!(handle_puzzle2(input)?, 0);

    Ok(())
}

struct Stone {
    data: String,
}

impl Stone {
    pub fn new(data: String) -> Self {
        Self { data }
    }

    pub fn roll(&mut self) -> Option<Self> {
        if self.data == "0" {
            self.data = String::from("1");
            return None;
        }
        if self.data.len() % 2 == 0 {
            let (a, b) = self.data.split_at(self.data.len() / 2);
            let mut a: String = a.chars().skip_while(|c| *c == '0').collect();
            if a.is_empty() {
                a = "0".to_string();
            }
            let mut b: String = b.chars().skip_while(|c| *c == '0').collect();
            if b.is_empty() {
                b = "0".to_string();
            }
            self.data = a;
            return Some(Stone::new(b));
        }
        self.data = (self.data.parse::<Units>().unwrap() * 2024).to_string();
        None
    }
}
