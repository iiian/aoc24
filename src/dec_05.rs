use core::panic;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
};

pub fn puzzle1() -> Result<u32, Box<dyn std::error::Error>> {
    handle_puzzle1(std::fs::read_to_string("./inputs/dec05.txt")?.as_str())
}

pub fn puzzle2() -> Result<u32, Box<dyn std::error::Error>> {
    handle_puzzle2(std::fs::read_to_string("./inputs/dec05.txt")?.as_str())
}

pub struct Constraints {
    top: TopologicalSort,
    constraints: Vec<(u8, u8)>,
}

impl Constraints {
    pub fn new(constraints: Vec<(u8, u8)>) -> Self {
        Self {
            top: TopologicalSort::from_constraints(constraints.clone()),
            constraints,
        }
    }

    pub fn is_correct(self: &Self, update: &Vec<u8>) -> bool {
        for (prior, posterior) in &self.constraints {
            let prior_index = update.iter().position(|n| n == prior);
            let posterior_index = update.iter().position(|n| n == posterior);

            if let Some(prior_index) = prior_index {
                if let Some(posterior_index) = posterior_index {
                    if prior_index >= posterior_index {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn reorder(self: &Self, update: &mut Vec<u8>) -> Vec<u8> {
        self.top.sort(update)
    }
}

pub struct TopologicalSort {
    forward: HashMap<u8, HashSet<u8>>,
    backward: HashMap<u8, HashSet<u8>>,
}

impl TopologicalSort {
    pub fn sort(self: &Self, list: &Vec<u8>) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        let list_as_set = list.iter().map(|e| *e).collect::<HashSet<_>>();
        let mut backward = list
            .iter()
            .filter_map(|key| {
                self.backward.get(key).map(|value| {
                    (
                        *key,
                        value
                            .intersection(&list_as_set)
                            .map(|e| *e)
                            .to_owned()
                            .collect::<HashSet<_>>(),
                    )
                })
            })
            .collect::<HashMap<_, _>>();

        let mut forward = list
            .iter()
            .filter_map(|key| {
                self.forward.get(key).map(|value| {
                    (
                        *key,
                        value
                            .intersection(&list_as_set)
                            .map(|e| *e)
                            .to_owned()
                            .collect::<HashSet<_>>(),
                    )
                })
            })
            .collect::<HashMap<_, _>>();
        let mut frontier: VecDeque<u8> = {
            let frontier: VecDeque<u8> = backward
                .iter()
                .filter_map(|(key, value)| if value.is_empty() { Some(*key) } else { None })
                .collect::<VecDeque<u8>>();

            frontier
        };

        while !frontier.is_empty() {
            let u = frontier.pop_front().unwrap();
            out.push(u);
            for (key_v, v_parents) in backward.iter_mut().filter(|(_, h)| h.contains(&u)) {
                v_parents.remove(&u);
                if let Some(children) = forward.get_mut(&u) {
                    children.remove(key_v);
                }
                if v_parents.is_empty() {
                    frontier.push_back(*key_v);
                }
            }
        }
        let rest = list
            .iter()
            .filter(|c| !out.contains(c))
            .map(|c| *c)
            .collect::<Vec<_>>();
        [out, rest].concat()
    }

    fn from_constraints(constraints: Vec<(u8, u8)>) -> TopologicalSort {
        // edge is .1 -> .2
        let mut backward: HashMap<u8, HashSet<u8>> = HashMap::new();
        let mut forward: HashMap<u8, HashSet<u8>> = HashMap::new();
        for (parent, child) in constraints {
            backward.entry(child).or_default().insert(parent);
            backward.entry(parent).or_default();
            forward.entry(parent).or_default().insert(child);
            forward.entry(child).or_default();
        }

        Self { backward, forward }
    }
}

fn load_puzzle(raw: &str) -> Result<(Vec<Vec<u8>>, Constraints), Box<dyn Error>> {
    let Some((constraints, sequences)) = raw.split_once("\n\n") else {
        panic!()
    };

    let constraints = constraints
        .split("\n")
        .map(|line| {
            line.split_once('|')
                .map(|(a, b)| (a.parse::<u8>().unwrap(), b.parse::<u8>().unwrap()))
                .unwrap()
        })
        .collect::<Vec<_>>();
    let constraints = Constraints::new(constraints);

    let updates = sequences
        .split('\n')
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    Ok((updates, constraints))
}

fn handle_puzzle1(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let Ok((updates, constraints)) = load_puzzle(input) else {
        panic!()
    };

    let mut sum = 0;
    for update in &updates {
        if update.len() % 2 == 0 {
            panic!("unexpected even length input");
        }

        if constraints.is_correct(update) {
            sum += update[update.len() / 2] as u32;
        }
    }

    Ok(sum)
}

fn handle_puzzle2(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let Ok((mut updates, constraints)) = load_puzzle(input) else {
        panic!()
    };

    let mut sum = 0;
    for update in &mut updates {
        if update.len() % 2 == 0 {
            panic!("unexpected even length input");
        }

        if !constraints.is_correct(update) {
            let update = constraints.reorder(update);
            sum += update[update.len() / 2] as u32;
        }
    }

    Ok(sum)
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    assert_eq!(handle_puzzle1(input)?, 143);

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    assert_eq!(handle_puzzle2(input)?, 123);

    Ok(())
}
