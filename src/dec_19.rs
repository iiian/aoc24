use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{
    collections::{HashMap, HashSet},
    io::Write,
    sync::{Arc, Mutex},
};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec19.txt")?;

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

type ParseOutput<'a> = (HashSet<&'a str>, Vec<&'a str>);
fn parse(input: &'_ str) -> ParseOutput<'_> {
    let mut lines = input.lines();

    let patterns = lines.next().unwrap().split(", ").collect::<HashSet<_>>();
    lines.next();
    let targets = lines.collect::<Vec<_>>();

    (patterns, targets)
}

type Units = usize;
fn handle_puzzle1(input: &str) -> Units {
    let (patterns, targets) = parse(input);

    let mut pfx_cache = HashMap::new();
    for pattern in patterns {
        pfx_cache.insert(pattern, true);
    }
    targets
        .into_iter()
        .filter(|tgt| cache_contains(&mut pfx_cache, tgt))
        .count()
}

fn cache_contains<'a>(cache: &mut HashMap<&'a str, bool>, substr: &'a str) -> bool {
    if let Some(answ) = cache.get(substr) {
        return *answ;
    }

    let indices = 1..=substr.len() - 1;
    for (pfx, sfx) in indices.map(|i| substr.split_at(i)) {
        let has_pfx = cache_contains(cache, pfx);
        let has_sfx = cache_contains(cache, sfx);

        if !cache.contains_key(pfx) {
            cache.insert(pfx, has_pfx);
        }
        if !cache.contains_key(sfx) {
            cache.insert(sfx, has_sfx);
        }

        if has_pfx && has_sfx {
            cache.insert(substr, true);
            return true;
        }
    }

    cache.insert(substr, false);
    false
}

fn handle_puzzle2(input: &str) -> Units {
    let (patterns, targets) = parse(input);
    let mut cache = HashMap::new();
    targets
        .into_iter()
        .map(|tgt| ways(&patterns, tgt, &mut cache))
        .sum()
}

fn ways<'a>(
    fragments: &HashSet<&'a str>,
    substr: &'a str,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if substr.is_empty() {
        return 1;
    }

    if let Some(&result) = cache.get(substr) {
        return result;
    }

    let mut subways = 0;
    for fragment in fragments {
        if let Some(remaining) = substr.strip_prefix(fragment) {
            subways += ways(fragments, remaining, cache);
        }
    }

    cache.insert(substr, subways);
    subways
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    assert_eq!(handle_puzzle1(input), 6);

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    assert_eq!(handle_puzzle2(input), 16);

    Ok(())
}
