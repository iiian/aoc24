use std::fs::read_to_string;

pub fn load_input(path: &str) -> Result<(Vec<u32>, Vec<u32>), std::io::Error> {
    let raw = read_to_string(path)?;
    let lines = raw.lines().map(|line| line.split_whitespace());
    let mut a = Vec::<u32>::new();
    let mut b = Vec::<u32>::new();
    for mut nums in lines {
        if let (Some(value1), Some(value2)) = (nums.next(), nums.next()) {
            a.push(value1.parse::<u32>().unwrap());
            b.push(value2.parse::<u32>().unwrap());
        }
    }

    Ok((a, b))
}
