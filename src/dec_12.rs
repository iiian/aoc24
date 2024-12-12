pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec12.txt")?;
    let now = std::time::Instant::now();

    let result = part1::handle_puzzle1(input.as_str())?;
    println!(
        "Puzzle 1: ans={}, ({} us)",
        result,
        now.elapsed().as_micros()
    );

    let result = part2::handle_puzzle2(input.as_str())?;
    println!(
        "Puzzle 2: ans={}, ({} us)",
        result,
        now.elapsed().as_micros()
    );

    Ok(())
}

mod part1 {
    use std::collections::hash_map::Entry::Vacant;

    use std::collections::VecDeque;

    use std::collections::HashMap;
    type PuzzleOutput = usize;

    pub fn handle_puzzle1(input: &str) -> Result<PuzzleOutput, Box<dyn std::error::Error>> {
        let space = input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<_>>>();

        let mut frontier = VecDeque::<(usize, usize)>::from([(0, 0)]);
        let mut visited = HashMap::<(usize, usize), bool>::new();

        let mut total_cost = 0;

        // Approach: for each point discovered on the frontier, attempt to build a region from it.
        //           if the point has already been visited, we may skip the attempt to visit it.
        //
        //           given any unvisited starting point, let it represent our first encounter with
        //           some new region. to get the full picture of the area and perimeter of that region,
        //           we construct an inner MAP tracking points that necessarily are part of this
        //           region. the points are keys, the values are a state: unprocessed/processed.
        //           any non-region points are added to the outer frontier.
        //           calculation happens after new region points are added.
        //           then the keys from the inner map are appended to the outer visited set, and we
        //           continue. This process will fill the space in its entirety, leaving us with a
        //           result.
        while let Some(next) = frontier.pop_front() {
            if is_visited(&visited, &next) {
                continue;
            }
            let mut region_frontier = VecDeque::<(usize, usize)>::from([next]);
            let mut region_visited = HashMap::<(usize, usize), bool>::new();
            region_visited.insert(next, false);

            let c = space[next.0][next.1];
            let mut perimeter = 0;
            while let Some(point) = region_frontier.pop_front() {
                if is_visited(&region_visited, &point) {
                    continue;
                }

                add_new_points(
                    point,
                    c,
                    &space,
                    &mut region_visited,
                    &mut region_frontier,
                    &mut visited,
                    &mut frontier,
                );

                perimeter += compute_change(c, point, &space);
                *region_visited.get_mut(&point).unwrap() = true;
            }

            assert!(region_visited.values().all(|b| *b));

            let area = region_visited.len();
            total_cost += perimeter * area;
            for (point, _) in region_visited {
                visited.insert(point, true);
            }
        }

        Ok(total_cost)
    }

    #[inline]
    fn compute_change(c: char, (i, j): (usize, usize), space: &[Vec<char>]) -> usize {
        let mut perim = 4;
        if i > 0 && space[i - 1][j] == c {
            perim -= 1;
        }
        if j > 0 && space[i][j - 1] == c {
            perim -= 1;
        }
        if i < space.len() - 1 && space[i + 1][j] == c {
            perim -= 1;
        }
        if j < space[0].len() - 1 && space[i][j + 1] == c {
            perim -= 1;
        }

        perim
    }

    #[inline]
    pub(crate) fn add_new_points(
        (i, j): (usize, usize),
        c: char,
        space: &[Vec<char>],
        region_visited: &mut HashMap<(usize, usize), bool>,
        region_frontier: &mut VecDeque<(usize, usize)>,
        visited: &mut HashMap<(usize, usize), bool>,
        frontier: &mut VecDeque<(usize, usize)>,
    ) {
        if i > 0 {
            try_extend(
                (i - 1, j),
                c,
                space[i - 1][j],
                region_visited,
                region_frontier,
                visited,
                frontier,
            );
        }
        if j > 0 {
            try_extend(
                (i, j - 1),
                c,
                space[i][j - 1],
                region_visited,
                region_frontier,
                visited,
                frontier,
            );
        }
        if i < space.len() - 1 {
            try_extend(
                (i + 1, j),
                c,
                space[i + 1][j],
                region_visited,
                region_frontier,
                visited,
                frontier,
            );
        }
        if j < space[0].len() - 1 {
            try_extend(
                (i, j + 1),
                c,
                space[i][j + 1],
                region_visited,
                region_frontier,
                visited,
                frontier,
            );
        }
    }

    #[inline]
    pub(crate) fn try_extend(
        point: (usize, usize),
        c: char,
        c2: char,
        region_visited: &mut HashMap<(usize, usize), bool>,
        region_frontier: &mut VecDeque<(usize, usize)>,
        visited: &mut HashMap<(usize, usize), bool>,
        frontier: &mut VecDeque<(usize, usize)>,
    ) {
        if c2 == c {
            if let Vacant(e) = region_visited.entry(point) {
                e.insert(false);
                region_frontier.push_back(point);
            }
        } else {
            visited.entry(point).or_insert(false);
            frontier.push_back(point);
        }
    }

    pub(crate) fn is_visited(
        visited: &HashMap<(usize, usize), bool>,
        point: &(usize, usize),
    ) -> bool {
        visited.get(point).copied().unwrap_or(false)
    }

    #[test]
    fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
        let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

        assert_eq!(handle_puzzle1(input)?, 1930);

        Ok(())
    }
}


mod part2 {
    use std::collections::HashSet;

use std::collections::HashMap;
use std::collections::VecDeque;

use crate::part1;

type PuzzleOutput = usize;

pub fn handle_puzzle2(input: &str) -> Result<PuzzleOutput, Box<dyn std::error::Error>> {
    let space = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let mut frontier = VecDeque::<(usize, usize)>::from([(0, 0)]);
    let mut visited = HashMap::<(usize, usize), bool>::new();

    let mut total_cost = 0;

    // Approach: for each point discovered on the frontier, attempt to build a region from it.
    //           if the point has already been visited, we may skip the attempt to visit it.
    //
    //           given any unvisited starting point, let it represent our first encounter with
    //           some new region. to get the full picture of the area and perimeter of that region,
    //           we construct an inner MAP tracking points that necessarily are part of this
    //           region. the points are keys, the values are a state: unprocessed/processed.
    //           any non-region points are added to the outer frontier.
    //           calculation happens after new region points are added.
    //           then the keys from the inner map are appended to the outer visited set, and we
    //           continue. This process will fill the space in its entirety, leaving us with a
    //           result.
    while let Some(next) = frontier.pop_front() {
        if part1::is_visited(&visited, &next) {
            continue;
        }
        let mut region_frontier = VecDeque::<(usize, usize)>::from([next]);
        let mut region_visited = HashMap::<(usize, usize), bool>::new();
        region_visited.insert(next, false);

        let c = space[next.0][next.1];
        while let Some(point) = region_frontier.pop_front() {
            if part1::is_visited(&region_visited, &point) {
                continue;
            }

            part1::add_new_points(
                point,
                c,
                &space,
                &mut region_visited,
                &mut region_frontier,
                &mut visited,
                &mut frontier,
            );

            *region_visited.get_mut(&point).unwrap() = true;
        }

        assert!(region_visited.values().all(|b| *b));

        let area = region_visited.len();
        let num_sides = get_num_sides(&region_visited);
        total_cost += num_sides * area;
        for (point, _) in region_visited {
            visited.insert(point, true);
        }
    }

    Ok(total_cost)
}

// pick random point, make way to edge
// circumnavigate the object until one reaches the origin again.
pub(crate) fn get_num_sides(region: &HashMap<(usize, usize), bool>) -> usize {
    #[derive(PartialEq)]
    enum Dir {
        N,
        S,
        E,
        W,
    }

    if region.len() <= 2 {
        return 4;
    }
    let mut num_sides = 0;

    let mut visited = HashSet::<(usize, usize)>::new();
    for point in region.keys() {
        let (i, j) = point;
        let (i, mut j) = (*i as i64, *j as i64);
        while region.contains_key(&(i as usize, j as usize + 1)) {
            visited.insert(point.clone());
            j += 1;
        }

        // we are now outside the region, headed south, hugging the region
        // to our right until we loop around again.
        j += 1;
        if visited.contains(&(i as usize, j as usize)) {
            continue;
        }
        let (mut ci, mut cj, mut cdir) = (i, j, Dir::S);
        loop {
            visited.insert((ci as usize, cj as usize));
            match cdir {
                Dir::S => {
                    // right turn
                    if !region.contains_key(&(ci as usize, (cj - 1) as usize)) {
                        cdir = Dir::W;
                        num_sides += 1;
                        cj -= 1;
                    }
                    // left turn
                    else if region.contains_key(&((ci + 1) as usize, cj as usize)) {
                        cdir = Dir::E;
                        num_sides += 1;
                    }
                    // straight
                    else {
                        ci += 1;
                    }
                }
                Dir::N => {
                    // right turn
                    if ci == -1 || !region.contains_key(&(ci as usize, (cj + 1) as usize)) {
                        cdir = Dir::E;
                        num_sides += 1;
                        cj += 1;
                    }
                    // left turn
                    else if region.contains_key(&((ci - 1) as usize, cj as usize)) {
                        cdir = Dir::W;
                        num_sides += 1;
                    }
                    // straight
                    else {
                        ci -= 1;
                    }
                }
                Dir::E => {
                    // right turn
                    if !region.contains_key(&((ci + 1) as usize, cj as usize)) {
                        cdir = Dir::S;
                        num_sides += 1;
                        ci += 1;
                    }
                    // left turn
                    else if region.contains_key(&(ci as usize, (cj + 1) as usize)) {
                        cdir = Dir::N;
                        num_sides += 1;
                    }
                    // straight
                    else {
                        cj += 1;
                    }
                }
                Dir::W => {
                    // right turn
                    if cj == -1 || !region.contains_key(&((ci - 1) as usize, cj as usize)) {
                        cdir = Dir::N;
                        num_sides += 1;
                        ci -= 1;
                    }
                    // left turn
                    else if region.contains_key(&(ci as usize, (cj - 1) as usize)) {
                        cdir = Dir::S;
                        num_sides += 1;
                    }
                    // straight
                    else {
                        cj -= 1;
                    }
                }
            }
            if ci == i && cj == j && cdir == Dir::S {
                break;
            }
        }
    }

    num_sides
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"AAAA
BBCD
BBCC
EEEC"#;

    assert_eq!(handle_puzzle2(input)?, 80);

    let input = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;

    assert_eq!(handle_puzzle2(input)?, 236);
    let input = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;

    assert_eq!(handle_puzzle2(input)?, 368);

    let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    assert_eq!(handle_puzzle2(input)?, 1206);

    Ok(())
}

#[test]
fn test_get_num_sides() {
    let mut input = HashMap::new();
    input.insert((10, 10), true);
    assert_eq!(get_num_sides(&input), 4);

    input.insert((11, 10), true);
    input.insert((10, 11), true);
    assert_eq!(get_num_sides(&input), 6);
    input.insert((11, 11), true);
    assert_eq!(get_num_sides(&input), 4);

    input.insert((11, 12), true);
    input.insert((12, 11), true);
    assert_eq!(get_num_sides(&input), 10);

    input.insert((11, 9), true);
    assert_eq!(get_num_sides(&input), 12);
    input.insert((11, 8), true);
    assert_eq!(get_num_sides(&input), 12);
    input.insert((11, 7), true);
    assert_eq!(get_num_sides(&input), 12);

    input.insert((12, 10), true);
    input.insert((12, 9), true);
    input.insert((12, 8), true);
    input.insert((12, 7), true);
    assert_eq!(get_num_sides(&input), 10);
}

}