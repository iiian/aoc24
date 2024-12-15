use std::collections::{HashSet, VecDeque};

type Units = usize;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec15.txt")?;
    let now = std::time::Instant::now();

    let result = handle_puzzle1(input.as_str());
    println!(
        "Puzzle 1: ans={}, ({} us)",
        result,
        now.elapsed().as_micros()
    );

    let result = handle_puzzle2(transform(&input).as_str());
    println!(
        "Puzzle 2: ans={}, ({} us)",
        result,
        now.elapsed().as_micros()
    );

    Ok(())
}

/// total number robot inputs in file: 20_020
type ParseOutput = (Vec<Vec<char>>, Vec<char>, (i32, i32));
fn parse(input: &str) -> ParseOutput {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let (mut ix, mut iy) = (0, 0);
    let map = map
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .inspect(|(j, c)| {
                    if *c == '@' {
                        (ix, iy) = (i as i32, *j as i32);
                    }
                })
                .map(|(_, c)| c)
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    (
        map,
        moves
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .collect::<Vec<_>>(),
        (ix, iy),
    )
}

fn transform(input: &str) -> String {
    let (a, b) = input.split_once("\n\n").unwrap();
    let mut out = String::new();
    for c in a.chars() {
        match c {
            'O' => out.push_str("[]"),
            '.' => out.push_str(".."),
            '@' => out.push_str("@."),
            '#' => out.push_str("##"),
            '\n' => out.push('\n'),
            _ => unreachable!(),
        }
    }

    String::from(out.to_string() + "\n\n" + b)
}

fn handle_puzzle1(input: &str) -> Units {
    let (mut map, moves, (mut i, mut j)) = parse(input);

    for action in moves {
        let (di, dj) = match action {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => unreachable!(),
        };

        if map[(i + di) as usize][(j + dj) as usize] == '.' {
            map[i as usize][j as usize] = '.';
            i += di;
            j += dj;
            map[i as usize][j as usize] = '@';
        } else if map[(i + di) as usize][(j + dj) as usize] == 'O' {
            let (dii, dji) = (di, dj);
            let (mut di, mut dj) = (di, dj);
            while map[(i + di) as usize][(j + dj) as usize] == 'O' {
                di += dii;
                dj += dji;
            }
            if map[(i + di) as usize][(j + dj) as usize] == '.' {
                map[i as usize][j as usize] = '.';
                map[(i + dii) as usize][(j + dji) as usize] = '@';
                map[(i + di) as usize][(j + dj) as usize] = 'O';
                i += dii;
                j += dji;
            }
        }
        // for line in &map {
        //     println!("{:?}", line.iter().collect::<String>());
        // }
        // println!();

        // assert_eq!(map[i as usize][j as usize], '@');
    }

    gpsm(map, 'O')
}

fn gpsm(map: Vec<Vec<char>>, c: char) -> usize {
    let mut sum = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == c {
                sum += 100 * i + j;
            }
        }
    }

    sum
}

fn handle_puzzle2(input: &str) -> Units {
    let (mut map, moves, (mut i, mut j)) = parse(input);

    'outer: for (e, action) in moves.into_iter().enumerate() {
        let (di, dj) = match action {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => unreachable!(),
        };

        let c = map[(i + di) as usize][(j + dj) as usize];
        if c == '.' {
            map[i as usize][j as usize] = '.';
            i += di;
            j += dj;
            map[i as usize][j as usize] = '@';
        } else if c == '#' {
            continue;
        } else {
            match action {
                '<' => {
                    let mut ju = j as usize;
                    while 3 < ju && map[i as usize][ju - 1] == ']' {
                        ju -= 2;
                    }
                    if map[i as usize][ju - 1] == '.' {
                        map[i as usize][j as usize] = '.';
                        while ju < j as usize {
                            map[i as usize][ju - 1] = '[';
                            map[i as usize][ju] = ']';
                            ju += 2;
                        }
                        map[i as usize][(j - 1) as usize] = '@';
                        j -= 1;
                    }
                }
                '>' => {
                    let mut ju = j as usize;
                    while ju < map[0].len() - 4 && map[i as usize][ju + 1] == '[' {
                        ju += 2;
                    }
                    if map[i as usize][ju + 1] == '.' {
                        map[i as usize][j as usize] = '.';
                        while (j as usize) < ju {
                            map[i as usize][ju + 1] = ']';
                            map[i as usize][ju] = '[';
                            ju -= 2;
                        }
                        map[i as usize][(j + 1) as usize] = '@';
                        j += 1;
                    }
                }
                '^' => {
                    if i <= 1 {
                        continue;
                    }
                    let mut q = VecDeque::<(usize, usize)>::new();
                    q.push_back((i as usize - 1, j as usize));
                    let mut visited = HashSet::<(usize, usize)>::new();
                    let mut move_set = HashSet::<(usize, usize, char)>::new();
                    while let Some((i, j)) = q.pop_front() {
                        if map[i][j] == '.' {
                            continue;
                        } else if map[i][j] == '#' || i <= 1 {
                            continue 'outer;
                        }

                        if map[i][j] == ']' {
                            if !visited.contains(&(i - 1, j)) {
                                q.push_back((i - 1, j));
                                visited.insert((i - 1, j));
                            }
                            if j >= 1 && !visited.contains(&(i - 1, j - 1)) {
                                q.push_back((i - 1, j - 1));
                                visited.insert((i - 1, j - 1));
                            }
                            if j >= 1 && !visited.contains(&(i, j - 1)) {
                                q.push_back((i, j - 1));
                                visited.insert((i, j - 1));
                            }
                        } else if map[i][j] == '[' {
                            if !visited.contains(&(i - 1, j)) {
                                q.push_back((i - 1, j));
                                visited.insert((i - 1, j));
                            }
                            if j <= map[0].len() - 2 && !visited.contains(&(i - 1, j + 1)) {
                                q.push_back((i - 1, j + 1));
                                visited.insert((i - 1, j + 1));
                            }
                            if j <= map[0].len() - 2 && !visited.contains(&(i, j + 1)) {
                                q.push_back((i, j + 1));
                                visited.insert((i, j + 1));
                            }
                        }
                        move_set.insert((i, j, map[i][j]));
                    }

                    map[i as usize][j as usize] = '.';
                    for (i, j, _) in &move_set {
                        map[*i][*j] = '.';
                    }
                    map[i as usize - 1][j as usize] = '@';
                    for (i, j, c) in move_set {
                        map[i - 1][j] = c;
                    }
                    i -= 1;
                }
                'v' => {
                    if i as usize >= map.len() - 2 {
                        continue;
                    }
                    let mut q = VecDeque::<(usize, usize)>::new();
                    q.push_back((i as usize + 1, j as usize));
                    let mut visited = HashSet::<(usize, usize)>::new();
                    let mut move_set = HashSet::<(usize, usize, char)>::new();
                    while let Some((i, j)) = q.pop_front() {
                        if map[i][j] == '.' {
                            continue;
                        } else if map[i][j] == '#' || i as usize >= map.len() - 2 {
                            continue 'outer;
                        }

                        if map[i][j] == ']' {
                            if !visited.contains(&(i + 1, j)) {
                                q.push_back((i + 1, j));
                                visited.insert((i + 1, j));
                            }
                            if j >= 1 && !visited.contains(&(i + 1, j - 1)) {
                                q.push_back((i + 1, j - 1));
                                visited.insert((i + 1, j - 1));
                            }
                            if j >= 1 && !visited.contains(&(i, j - 1)) {
                                q.push_back((i, j - 1));
                                visited.insert((i, j - 1));
                            }
                        } else if map[i][j] == '[' {
                            if !visited.contains(&(i + 1, j)) {
                                q.push_back((i + 1, j));
                                visited.insert((i + 1, j));
                            }
                            if j <= map[0].len() - 2 && !visited.contains(&(i + 1, j + 1)) {
                                q.push_back((i + 1, j + 1));
                                visited.insert((i + 1, j + 1));
                            }
                            if j <= map[0].len() - 2 && !visited.contains(&(i, j + 1)) {
                                q.push_back((i, j + 1));
                                visited.insert((i, j + 1));
                            }
                        }
                        move_set.insert((i, j, map[i][j]));
                    }

                    map[i as usize][j as usize] = '.';
                    for (i, j, _) in &move_set {
                        map[*i][*j] = '.';
                    }
                    map[i as usize + 1][j as usize] = '@';
                    for (i, j, c) in move_set {
                        map[i + 1][j] = c;
                    }
                    i += 1;
                }
                _ => unreachable!(),
            }
        }

        // println!("iteration #{e}");
        // for line in &map {
        //     println!("{:?}", line.iter().collect::<String>());
        // }
        // println!();

        // assert_eq!(map[i as usize][j as usize], '@');
    }

    gpsm(map, '[')
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########

"#;

    assert_eq!(handle_puzzle1(input), 10_092);

    let input = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

    assert_eq!(handle_puzzle1(input), 2_028);

    let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    assert_eq!(handle_puzzle1(input), 10_092);

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#;

    let binding = transform(input);
    let input = binding.as_str();

    assert_eq!(handle_puzzle2(input), 618);

    let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    let binding = transform(input);
    let input = binding.as_str();

    assert_eq!(handle_puzzle2(input), 9_021);

    Ok(())
}
