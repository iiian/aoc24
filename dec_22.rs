pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec22.txt")?;

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

type ParseOutput = Vec<usize>;
fn parse(input: &str) -> ParseOutput {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

type Units = usize;
fn handle_puzzle1(input: &str) -> Units {
    parse(input)
        .into_iter()
        .map(|seed| Monke { seed }.nth(1_999).unwrap())
        .sum()
}

fn handle_puzzle2(input: &str) -> Units {
    todo!()
}

struct Monke {
    seed: usize,
}

impl Iterator for Monke {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.seed ^= self.seed * 64;
        self.seed %= 16777216;

        self.seed ^= self.seed / 32;
        self.seed %= 16777216;

        self.seed ^= self.seed * 2048;
        self.seed %= 16777216;
        Some(self.seed)
    }
}

#[test]
fn test_sequence() {
    let mut iter = Monke { seed: 123 };
    let expectation = vec![
        15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254,
    ];
    for expectation in expectation {
        assert_eq!(iter.next(), Some(expectation));
    }
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"1
10
100
2024"#;

    assert_eq!(handle_puzzle1(input), 37327623);

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#""#;

    assert_eq!(handle_puzzle2(input), todo!());

    Ok(())
}
