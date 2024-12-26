use itertools::iproduct;
use std::collections::HashMap;

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

#[inline]
fn parse(input: &str) -> Vec<(usize, String)> {
    input
        .lines()
        .map(|line| {
            (
                line.chars()
                    .filter(|c| c.is_numeric())
                    .collect::<String>()
                    .parse()
                    .unwrap(),
                line.chars().collect(),
            )
        })
        .collect()
}

#[inline]
fn handle_puzzle1(input: &str) -> usize {
    let inputs = parse(input);
    let num = ClickMatrixv2::numpad();
    let dpad = ClickMatrixv2::dpad();

    let chain = [&num, &dpad, &dpad];
    let chain = ClickChain::from(&chain);

    inputs
        .iter()
        .map(|(code, input)| code * chain.find_shortest_len(input.as_str()))
        .sum()
}

#[inline]
fn handle_puzzle2(input: &str) -> usize {
    let inputs = parse(input);
    let num = ClickMatrixv2::numpad();
    let dpad = ClickMatrixv2::dpad();

    let mut chain = vec![&num];
    for _ in 0..25 {
        chain.push(&dpad);
    }
    let chain = ClickChain::from(&chain);

    inputs
        .iter()
        .map(|(code, input)| code * chain.find_shortest_len(input.as_str()))
        .sum()
}

/// Allows querying "from"/"to" for any two keys on any keypad,
/// yielding the set of possible (direct) routes between keys
struct ClickMatrixv2 {
    matrix: Vec<Vec<Vec<String>>>,
    key: HashMap<char, usize>,
}

/// static methods
impl ClickMatrixv2 {
    /// Generate an (h*w-1)**2 matrix for a given keypad, `base`
    fn from(base: Vec<Vec<char>>, null_sq: (usize, usize)) -> Self {
        let h = base.len();
        let w = base[0].len();
        let hw = h * w - 1;

        type Pt = (usize, usize);
        // walk-about the keypad
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

            match curs.0.cmp(&dest.0) {
                std::cmp::Ordering::Less => {
                    s.push('v');
                    generate((curs.0 + 1, curs.1), dest, s, l, null_sq);
                    s.pop();
                }
                std::cmp::Ordering::Greater => {
                    s.push('^');
                    generate((curs.0 - 1, curs.1), dest, s, l, null_sq);
                    s.pop();
                }
                _ => {}
            }

            match curs.1.cmp(&dest.1) {
                std::cmp::Ordering::Less => {
                    s.push('>');
                    generate((curs.0, curs.1 + 1), dest, s, l, null_sq);
                    s.pop();
                }
                std::cmp::Ordering::Greater => {
                    s.push('<');
                    generate((curs.0, curs.1 - 1), dest, s, l, null_sq);
                    s.pop();
                }
                _ => {}
            }
        }

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
            .map(|row| row.into_iter().flatten().collect())
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
}

/// instance methods
impl ClickMatrixv2 {
    /// Look up the "ways" to get to `dest` from `src` for the given keypad
    pub fn get(&self, src: char, dest: char) -> &[String] {
        let ksrc = *self.key.get(&src).unwrap();
        let kdest = *self.key.get(&dest).unwrap();
        &self.matrix[ksrc][kdest]
    }
}

/// Given a daisy-chain of keypads, can compute shortest path to achieving the primary input
/// using the `find_shortest_len` method.
struct ClickChain<'a> {
    steps: &'a [&'a ClickMatrixv2],
}

impl<'a> ClickChain<'a> {
    pub fn from(steps: &'a [&'a ClickMatrixv2]) -> Self {
        Self { steps }
    }

    pub fn find_shortest_len(&self, input: &str) -> usize {
        let mut it = self.steps.iter().enumerate();

        ClickChain::find_shortest_len_inner(input, &mut it, &mut HashMap::new()).unwrap()
    }

    fn find_shortest_len_inner<'b, I>(
        input: &'b str,
        it: &mut I,
        cache: &mut HashMap<(&'b str, usize), usize>,
    ) -> Option<usize>
    where
        I: Iterator<Item = (usize, &'b &'b ClickMatrixv2)> + Clone,
    {
        if let Some((depth, keypad_matrix)) = it.next() {
            if let Some(len) = cache.get(&(input, depth)) {
                return Some(*len);
            }
            let mut from = 'A';
            let mut best = 0_usize;
            for to in input.chars() {
                let paths = keypad_matrix.get(from, to);
                let shortest = paths
                    .iter()
                    .map(|opt| {
                        if let Some(shortest) = ClickChain::find_shortest_len_inner(
                            opt.as_str(),
                            &mut it.clone(),
                            cache,
                        ) {
                            shortest
                        } else {
                            opt.len()
                        }
                    })
                    .min()
                    .unwrap();

                best += shortest;
                from = to;
            }

            cache.insert((input, depth), best);
            Some(best)
        } else {
            // If the iterator is dead, that means we're as deep as it gets.
            None
        }
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
