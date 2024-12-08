use rayon::prelude::*;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

pub fn puzzle1() -> Result<u64, Box<dyn std::error::Error>> {
    handle_puzzle::<Puzzle1Ops>(std::fs::read_to_string("./inputs/dec07.txt")?.as_str())
}

pub fn puzzle2() -> Result<u64, Box<dyn std::error::Error>> {
    handle_puzzle::<Puzzle2Ops>(std::fs::read_to_string("./inputs/dec07.txt")?.as_str())
}

/// A permutation generator over some set of operations and operands
struct Perms<O>
where
    O: OpsSet,
{
    /// The nums that must be searched for a companion
    nums: Vec<u64>,
    ///
    ops_perm: Option<Vec<usize>>,
    _marker: std::marker::PhantomData<O>,
}

impl<O> Perms<O>
where
    O: OpsSet,
{
    pub fn new(base_str: &str) -> Self {
        let nums = base_str
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect::<Vec<_>>();
        let ops_perm = Some(vec![0; nums.len() - 1]);

        Self {
            nums,
            ops_perm,
            _marker: std::marker::PhantomData,
        }
    }

    /// Compute the product of the current permutation
    pub fn compute(&self) -> Option<u64>
    where
        O: OpsSet,
    {
        if let Some(ref ops_perm) = self.ops_perm {
            Some(
                self.nums
                    .iter()
                    .skip(1)
                    .zip(ops_perm)
                    .fold(self.nums[0], |acc, (num, op)| O::apply(*op, acc, *num)),
            )
        } else {
            None
        }
    }
}

impl<O> Iterator for Perms<O>
where
    O: OpsSet,
{
    type Item = u64;

    /// Iterate all x^k permutations, where x := |ops_set|, k := |operands_list|
    fn next(&mut self) -> Option<Self::Item>
    where
        O: OpsSet,
    {
        if let Some(result) = self.compute() {
            let Some(ref mut ops_perm) = self.ops_perm else {
                panic!()
            };
            let mut i = 0_usize;

            loop {
                ops_perm[i] = (ops_perm[i] + 1) % O::len();
                if ops_perm[i] != 0 {
                    break;
                }
                i += 1;
                if i >= ops_perm.len() {
                    self.ops_perm = None;
                    break;
                }
            }

            Some(result)
        } else {
            None
        }
    }
}

/// A set of operations
trait OpsSet {
    /// Perform the computation
    fn apply(op_id: usize, a: u64, b: u64) -> u64;

    /// Get # of ops
    fn len() -> usize;
}
#[derive(Clone, Copy)]
struct Puzzle1Ops;
unsafe impl Send for Puzzle1Ops {}
const OPS_PUZZLE1: [char; 2] = ['+', '*'];
impl OpsSet for Puzzle1Ops {
    fn apply(op_id: usize, a: u64, b: u64) -> u64 {
        match OPS_PUZZLE1[op_id] {
            '+' => a + b,
            '*' => a * b,
            _ => unreachable!(),
        }
    }

    fn len() -> usize {
        OPS_PUZZLE1.len()
    }
}

#[derive(Clone, Copy)]

struct Puzzle2Ops;
unsafe impl Send for Puzzle2Ops {}
const OPS_PUZZLE2: [char; 3] = ['+', '*', '|'];
impl OpsSet for Puzzle2Ops {
    fn apply(op_id: usize, a: u64, mut b: u64) -> u64 {
        match OPS_PUZZLE2[op_id] {
            '+' => a + b,
            '*' => a * b,
            '|' => {
                let mut exp = 0_u32;
                let mut product = 0_u64;

                while b != 0 {
                    product += (b % 10) * 10u64.pow(exp);
                    exp += 1;
                    b /= 10;
                }

                product += a * 10u64.pow(exp);

                product
            }
            _ => unreachable!(),
        }
    }

    fn len() -> usize {
        OPS_PUZZLE2.len()
    }
}

/// Run the puzzle for some input and OpsSet
fn handle_puzzle<O>(input: &str) -> Result<u64, Box<dyn Error>>
where
    O: OpsSet + Send,
{
    let lines = input.lines();
    let answers: Arc<Mutex<Vec<Option<u64>>>> =
        Arc::new(Mutex::new(vec![None; lines.clone().count()]));
    lines
        .enumerate()
        .par_bridge()
        .flat_map(|(index, line)| {
            let (expected, operands) = line.split_once(": ").unwrap();
            let expected = expected.parse::<u64>().unwrap();
            Perms::<O>::new(operands)
                .map(move |actual| (index, expected, actual))
                // take only the successful solutions
                .filter(|(_, expected, actual)| expected == actual)
                .par_bridge()
        })
        // now, all permutations of all lines are running in parallel
        .for_each(|(index, _, actual)| answers.lock().unwrap()[index] = Some(actual));

    let result = answers
        .lock()
        .unwrap()
        .iter()
        .filter_map(|e| e.map(|actual| actual))
        .sum();

    Ok(result)
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    assert_eq!(handle_puzzle::<Puzzle1Ops>(input)?, 3749);

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    assert_eq!(handle_puzzle::<Puzzle2Ops>(input)?, 11387);

    Ok(())
}
