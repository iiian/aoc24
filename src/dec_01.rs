use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use super::helper::load_input;

pub fn puzzle1() -> Result<u32, std::io::Error> {
    let (mut left, mut right) = load_input("./inputs/dec01.txt")?;

    left.sort();
    right.sort();

    let mut distance = 0;

    for (l, r) in left.into_iter().zip(right.into_iter()) {
        distance += l.abs_diff(r);
    }

    Ok(distance)
}

pub fn puzzle2() -> Result<u32, std::io::Error> {
    let (mut left, mut right) = load_input("./inputs/dec01.txt")?;
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
