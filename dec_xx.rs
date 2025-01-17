pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/decxx.txt")?;

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

type ParseOutput = todo!();
fn parse(input: &str) -> ParseOutput {
    todo!()
}
type Units = todo!();
fn handle_puzzle1(input: &str) -> Units {
    todo!()
}

fn handle_puzzle2(input: &str) -> Units {
    todo!()
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#""#;

    assert_eq!(handle_puzzle1(input), todo!());

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#""#;

    assert_eq!(handle_puzzle2(input), todo!());

    Ok(())
}
