use std::{collections::HashMap, slice::Iter};

use itertools::{iproduct, Itertools};

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

/// After trying to just brute force with the solution above for pt1, around a `ClickChain` composed of 13 steps,
/// we're looking at ~10 sec to expand a single layer. Another 12 chains would result in another 12 orders of magnitude, which is about 31,000 years.
/// I don't have that much time, so a craftier solution must be employed.
///
/// Rather than directly computing a 26-step string and then taking its length, there are two aspects of the problem that can be exploited to compute
/// directly the *length* of the 26th input sequence in a tractable time.
///
/// The first aspect is the fact that, for any given transition -- that is numpad(3, 6) or dpad(<, A) on the same robot's input pad -- "the shortest length
/// the parent must travel to produce this result in the child layer" does not vary as the child layer's sequence progresses; it is always a fixed length. This applies across all layers.
///
/// The second aspect is that, for all of these transitions across all layers, any transition is completely and totally linearly independent of its siblings in that layer,
/// in terms of its contributions to the final length. This is because the parent robot controlling this robot's movements must always return to the 'A' button
/// in order to finalize the click input. This means that whenever a click occurs in the child robot, the parent has recentered itself, and so there can be no "drift"
/// in what it means to transition from A to B on the child robot's input pad.
///
/// Using these two facts, we abandon thinking of the problem in terms of "sequences of button presses". In its lieu, we can think of "weighted maps of button transitions".
/// Consider some child robot's sequence; it might need to perform say (<, ^) seven times. We know that the shortest path for moving a robot from < to ^ is fixed, and furthermore
/// we can precompute what its constituent transitions and weights would be in the parent controlling robot's layer.
///
/// Therefore, the solution to the problem is crunching our way through 26 layers of accreting a weighted map of button transition, from the previous layer into the next layer.
/// Then we take the sum of all those transition weights, and multiply it by the input_code. sum all of those, and that's the final answer.
fn handle_puzzle2(input: &str) -> usize {
    // initialize inputs
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

    // construct the 1numpad -> 25dpad chain
    let num = ClickMatrixv2::numpad().transition_expansion_maps();
    let dpad = ClickMatrixv2::dpad().transition_expansion_maps();
    let mut expansion_chain = vec![&num];
    for _ in 0..25 {
        expansion_chain.push(&dpad);
    }

    let mut sum = 0;
    for (input, input_code) in inputs.iter().zip(input_codes) {
        // accreting weighted transitions
        let mut trans_weights = HashMap::new();

        let initial_transitions = ['A'].into_iter().chain(input.chars()).tuple_windows();
        for next_transition in initial_transitions {
            *trans_weights.entry(next_transition).or_default() += 1;
        }

        // expand through the layers of the click chain, replacing the input_cost_map @ the end of each chain step
        for trans_expans_map in expansion_chain.iter() {
            let mut next_trans_weights = HashMap::new();

            // build the next layer map, based on how any given transition results in next steps to take,
            // and multiplied by the number of times the current layer performs that transition
            for (next_trans, next_repeats) in trans_weights {
                if let Some(default_trans_weights) = trans_expans_map.get(&next_trans) {
                    for (parent_trans, base_weight) in default_trans_weights.iter() {
                        *next_trans_weights.entry(*parent_trans).or_default() +=
                            next_repeats * base_weight;
                    }
                }
            }
            trans_weights = next_trans_weights;
        }
        let input_len = trans_weights.values().sum::<usize>();
        sum += input_len * input_code;
    }

    sum
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

    fn transition_expansion_maps(&self) -> HashMap<(char, char), HashMap<(char, char), usize>> {
        let all_dirs = self.key.keys().collect::<Vec<_>>();
        let mut out = HashMap::new();

        for c in all_dirs.into_iter().combinations(2) {
            let (from, to) = (**c.get(0).unwrap(), **c.get(1).unwrap());

            let from_to_chars = self
                .get(from, to)
                .iter()
                .min_by_key(|s| s.len())
                .unwrap()
                .chars();
            let best = ['A']
                .into_iter()
                .chain(from_to_chars)
                .tuple_windows()
                .into_iter()
                .fold(HashMap::<(char, char), usize>::new(), |mut acc, next| {
                    *acc.entry(next).or_default() += 1;
                    acc
                });

            out.insert((from, to), best);
        }

        out
    }
}

impl ClickMatrixv2 {
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
    let dpad = ClickMatrixv2::dpad();

    let DIRS = ['<', '^', '>', 'v', 'A'];
    for c in DIRS.iter().combinations(2) {
        let (from, to) = (**c.get(0).unwrap(), **c.get(1).unwrap());

        let from_to_chars = dpad
            .get(from, to)
            .iter()
            .min_by_key(|s| s.len())
            .unwrap()
            .chars();
        let best = ['A']
            .into_iter()
            .chain(from_to_chars)
            .tuple_windows()
            .into_iter()
            .fold(HashMap::<(char, char), usize>::new(), |mut acc, next| {
                *acc.entry(next).or_default() += 1;
                acc
            });
    }

    Ok(())
}
