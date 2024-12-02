use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use std::fs::read_to_string;

pub fn puzzle1() -> Result<u32, Box<dyn std::error::Error>> {
    let (mut left, mut right) = load_input("./inputs/dec01.txt")?;

    left.sort();
    right.sort();

    let mut distance = 0;

    for (l, r) in left.into_iter().zip(right.into_iter()) {
        distance += l.abs_diff(r);
    }

    Ok(distance)
}

pub fn puzzle2() -> Result<u32, Box<dyn std::error::Error>> {
    let (left, right) = load_input("./inputs/dec01.txt")?;
    let mut score = 0;

    let right: HashMap<u32, u16> = {
        let mut new_right = HashMap::<u32, u16>::new();

        for num in right {
            let entry = new_right.entry(num).or_default();
            *entry += 1;
        }

        new_right
    };

    for num in left {
        if let Some(right_num) = right.get(&num) {
            score += num * (*right_num as u32);
        }
    }

    Ok(score)
}

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
