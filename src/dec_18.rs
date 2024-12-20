use std::collections::HashSet;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec18.txt")?;
    const WIDTH: usize = 71;
    const HEIGHT: usize = 71;
    let now = std::time::Instant::now();
    let result = handle_puzzle1(input.as_str(), 1_024, WIDTH, HEIGHT);
    println!(
        "Puzzle 1: ans {:?}, ({} us)",
        result,
        now.elapsed().as_micros()
    );

    let now = std::time::Instant::now();
    let result = handle_puzzle2(input.as_str(), WIDTH, HEIGHT);
    println!(
        "Puzzle 2: ans {:?}, ({} us)",
        result,
        now.elapsed().as_micros()
    );

    Ok(())
}

type ParseOutput = Vec<(usize, usize)>;
fn parse(input: &str) -> ParseOutput {
    input
        .lines()
        .map(|line| {
            line.split_once(',')
                .map(|(a, b)| (b.parse::<usize>().unwrap(), a.parse::<usize>().unwrap()))
                .unwrap()
        })
        .collect()
}
type Units = Option<usize>;

fn handle_puzzle1(input: &str, take: usize, w: usize, h: usize) -> Units {
    let obs = parse(input).into_iter().take(take).collect::<HashSet<_>>();

    let mut v = HashSet::<(usize, usize)>::from([(0, 0)]);
    let mut fr = vec![(0, 0)];
    let mut dist = 0;

    loop {
        if fr.is_empty() {
            break;
        }
        let mut n_fr = vec![];
        while let Some(next @ (i, j)) = fr.pop() {
            if obs.contains(&next) {
                continue;
            }
            if i == h - 1 && j == w - 1 {
                return Some(dist);
            }

            if i > 0 {
                let pt = (i - 1, j);
                if is_valid(&v, &obs, &pt) {
                    v.insert(pt);
                    n_fr.push(pt);
                }
            }

            if j > 0 {
                let pt = (i, j - 1);
                if is_valid(&v, &obs, &pt) {
                    v.insert(pt);
                    n_fr.push(pt);
                }
            }
            if i < h - 1 {
                let pt = (i + 1, j);
                if is_valid(&v, &obs, &pt) {
                    v.insert(pt);
                    n_fr.push(pt);
                }
            }

            if j < w - 1 {
                let pt = (i, j + 1);
                if is_valid(&v, &obs, &pt) {
                    v.insert(pt);
                    n_fr.push(pt);
                }
            }
        }
        dist += 1;
        fr = n_fr;

        //println!("{dist}");
        //show(&fr, &obs, &v, w, h);
    }

    None
}

fn is_valid(
    v: &HashSet<(usize, usize)>,
    obs: &HashSet<(usize, usize)>,
    pt: &(usize, usize),
) -> bool {
    !(v.contains(pt) || obs.contains(pt))
}

fn handle_puzzle2(input: &str, w: usize, h: usize) -> Option<(usize, usize)> {
    let all_obs = parse(input);

    'outer: for i in 1..all_obs.len() {
        let obs = all_obs.iter().take(i).copied().collect::<HashSet<_>>();

        let mut v = HashSet::<(usize, usize)>::from([(0, 0)]);
        let mut fr = vec![(0, 0)];

        while !fr.is_empty() {
            let mut n_fr = vec![];
            while let Some(next @ (i, j)) = fr.pop() {
                if obs.contains(&next) {
                    continue;
                }
                if i == h - 1 && j == w - 1 {
                    continue 'outer;
                }

                if i > 0 {
                    let pt = (i - 1, j);
                    if is_valid(&v, &obs, &pt) {
                        v.insert(pt);
                        n_fr.push(pt);
                    }
                }

                if j > 0 {
                    let pt = (i, j - 1);
                    if is_valid(&v, &obs, &pt) {
                        v.insert(pt);
                        n_fr.push(pt);
                    }
                }
                if i < h - 1 {
                    let pt = (i + 1, j);
                    if is_valid(&v, &obs, &pt) {
                        v.insert(pt);
                        n_fr.push(pt);
                    }
                }

                if j < w - 1 {
                    let pt = (i, j + 1);
                    if is_valid(&v, &obs, &pt) {
                        v.insert(pt);
                        n_fr.push(pt);
                    }
                }
            }

            fr = n_fr;

            //println!("{dist}");
            //show(&fr, &obs, &v, w, h);
        }
        let (y, x) = all_obs[i - 1];
        return Some((x, y));
    }

    None
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;

    assert_eq!(handle_puzzle1(input, 12, 7, 7), Some(22));

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;

    assert_eq!(handle_puzzle2(input, 7, 7), Some((6, 1)));

    Ok(())
}
