use std::{collections::HashMap, slice::Iter};

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

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn handle_puzzle1(input: &str) -> usize {
    let inputs = parse(input);
    let input_codes = inputs.clone().into_iter().map(|input| {
        input
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
    });
    let num = ClickMatrixv2::numpad();
    let dpad = ClickMatrixv2::dpad();

    let chain = [&num, &dpad, &dpad];
    let chain = ClickChain::from(&chain);

    inputs
        .iter()
        .map(|input| chain.find_shortest(input.as_str()))
        .clone()
        .into_iter()
        .zip(input_codes)
        .map(|(path, code)| path.len() * code)
        .sum()
}

fn handle_puzzle2(input: &str) -> usize {
    let inputs = parse(input);
    let input_codes = inputs
        .clone()
        .into_iter()
        .map(|input| {
            input
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<_>>();
    let num = ClickMatrixv2::numpad();
    let dpad = ClickMatrixv2::dpad();

    let mut chain = Vec::from([&num]);
    for i in 0..11 {
        chain.push(&dpad);
    }
    for i in 10..11 {
        let chain = ClickChain::from(&chain);

        println!("dpad count = {i}");
        for (i, input) in inputs.iter().enumerate() {
            let shortest = chain.find_shortest(input.as_str()).len();
            println!("\t{input} len = {shortest}");
            println!("\tcomplexity = {}", input_codes[i] * shortest);
        }
        // inputs
        //     .iter()
        //     .map(|input| chain.find_shortest(input.as_str()))
        //     .clone()
        //     .into_iter()
        //     .zip(input_codes)
        //     .map(|(path, code)| path.len() * code)
        //     .sum()
    }

    0
}

/// Allows one to query "from"/"to" for any two keys on any keypad,
/// and in return one receives the comprehensive set of possible routes
struct ClickMatrixv2 {
    matrix: Vec<Vec<Vec<String>>>,
    key: HashMap<char, usize>,
}
impl ClickMatrixv2 {
    /// Generate an (h*w-1)**2 matrix for a given keypad, `base`
    fn from(base: Vec<Vec<char>>, null_sq: (usize, usize)) -> Self {
        let h = base.len();
        let w = base[0].len();
        let hw = h * w - 1;

        type Pt = (usize, usize);
        fn generate(curs: Pt, dest: Pt, s: &mut String, l: &mut Vec<String>, null_sq: &Pt) {
            if &curs == null_sq {
                return;
            }
            if curs == dest {
                s.push('A');
                l.push(s.clone());
                s.pop();
                return;
            }
            if curs.0 < dest.0 {
                s.push('v');
                generate((curs.0 + 1, curs.1), dest, s, l, null_sq);
                s.pop();
            } else if curs.0 > dest.0 {
                s.push('^');
                generate((curs.0 - 1, curs.1), dest, s, l, null_sq);
                s.pop();
            }

            if curs.1 < dest.1 {
                s.push('>');
                generate((curs.0, curs.1 + 1), dest, s, l, null_sq);
                s.pop();
            } else if curs.1 > dest.1 {
                s.push('<');
                generate((curs.0, curs.1 - 1), dest, s, l, null_sq);
                s.pop();
            }
        };

        let mut matrix = vec![vec![Option::<Vec<String>>::None; hw]; hw];
        let mut key = HashMap::<char, usize>::new();
        let mut k = 0;
        for row in &base {
            for ch in row {
                if *ch == ' ' {
                    continue;
                }
                key.insert(*ch, k);
                k += 1;
            }
        }

        for pt_src @ (i1, j1) in iproduct!(0..h, 0..w) {
            if pt_src == null_sq {
                continue;
            }
            if let Some(k1) = key.get(&base[i1][j1]) {
                for pt_dest @ (i2, j2) in iproduct!(0..h, 0..w) {
                    if pt_dest == null_sq {
                        continue;
                    }
                    if let Some(k2) = key.get(&base[i2][j2]) {
                        let mut list = vec![];
                        generate(pt_src, pt_dest, &mut String::new(), &mut list, &null_sq);

                        matrix[*k1][*k2] = Some(list);
                    }
                }
            }
        }

        let matrix = matrix
            .into_iter()
            .map(|row| row.into_iter().filter_map(|o| o).collect())
            .collect();

        Self { matrix, key }
    }

    /// default constructor for the `numpad` input entry mechanism
    fn numpad() -> Self {
        Self::from(
            vec![
                vec!['7', '8', '9'],
                vec!['4', '5', '6'],
                vec!['1', '2', '3'],
                vec![' ', '0', 'A'],
            ],
            (3, 0),
        )
    }

    /// default constructor for `dpad` input entry mechanism
    fn dpad() -> Self {
        Self::from(vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']], (0, 0))
    }

    /// Look up the "ways" to get to `dest` from `src` for the given keypad
    pub fn get(&self, src: char, dest: char) -> &[String] {
        let ksrc = *self.key.get(&src).unwrap();
        let kdest = *self.key.get(&dest).unwrap();

        &self.matrix[ksrc][kdest]
    }
}

/// Given a daisy-chain of keypads, computes the shortest path to achieving the primary input
/// using the `find_shortest` method.
struct ClickChain<'a> {
    steps: &'a [&'a ClickMatrixv2],
}

impl<'a> ClickChain<'a> {
    pub fn from(steps: &'a [&'a ClickMatrixv2]) -> Self {
        Self { steps }
    }

    /// Finds a minimal length path that satisfies the keypad chain.
    pub fn find_shortest(&self, input: &str) -> String {
        let mut it = self.steps.iter();
        self.find_shortest_inner(input, &mut it).unwrap()
    }

    fn find_shortest_inner(&self, input: &str, it: &mut Iter<&ClickMatrixv2>) -> Option<String> {
        if let Some(keypad_matrix) = it.next() {
            let mut from = 'A';
            let mut best = String::new();
            for to in input.chars() {
                let paths = keypad_matrix.get(from, to);
                let shortest = paths
                    .iter()
                    .map(|opt| {
                        if let Some(shortest) =
                            self.find_shortest_inner(opt.as_str(), &mut it.clone())
                        {
                            shortest
                        } else {
                            opt.clone()
                        }
                    })
                    .min_by_key(|s| s.len())
                    .unwrap();

                best.extend(shortest.chars());
                from = to;
            }

            return Some(best);
        }

        // If the iterator is dead, that means we're as deep as it gets.
        None
    }
}

fn align_lines(mut lines: Vec<String>) -> String {
    for i in (0..lines.len() - 1).rev() {
        let (unaligned, base) = (&lines[i], &lines[i + 1]);
        let mut realigned = String::new();
        let mut unaligned = unaligned.chars();
        let mut base = base.chars();
        while let Some(c) = base.next() {
            let next = if c == 'A' {
                unaligned.next().unwrap()
            } else {
                ' '
            };
            realigned.push(next);
        }
        lines[i] = realigned;
    }

    lines.join("\n")
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
