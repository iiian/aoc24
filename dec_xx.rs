pub fn puzzle1() -> Result<u32, Box<dyn std::error::Error>> {
    handle_puzzle1(std::fs::read_to_string("./inputs/decxx.txt"))
}

pub fn puzzle2() -> Result<u32, Box<dyn std::error::Error>> {
    handle_puzzle2(std::fs::read_to_string("./inputs/decxx.txt"))
}

fn handle_puzzle1(input: String) -> Result<u32, Box<dyn std::error::Error>> {
    todo!()
}

fn handle_puzzle2(input: String) -> Result<u32, Box<dyn std::error::Error>> {
    todo!()
}

#[test]
fn test_puzzle1() {
    let input = r#""#;

    assert_eq!(handle_puzzle1(input), 0);
}

#[test]
fn test_puzzle2() {
    let input = r#""#;

    assert_eq!(handle_puzzle2(input), 0);
}
