mod window_iterator;
use regex::Regex;
use std::fs::read_to_string;

use window_iterator::WindowIterator;

pub fn puzzle1() -> Result<u32, Box<dyn std::error::Error>> {
    handle_puzzle1(read_to_string("./inputs/dec04.txt")?)
}

pub fn puzzle2() -> Result<u32, Box<dyn std::error::Error>> {
    handle_puzzle2(read_to_string("./inputs/dec04.txt")?)
}

static XMAS: &[u8] = "XMAS".as_bytes();
static DIRECTIONS: [(i16, i16, &str); 8] = [
    (-1, 0, "upward"),            // up
    (1, 0, "downward"),           // down
    (0, -1, "backward"),          // left
    (0, 1, "to the right"),       // right
    (-1, -1, "backward up"),      // top-left
    (-1, 1, "forwards up"),       // top-right
    (1, -1, "backward downward"), // bottom-left
    (1, 1, "forward downward"),   // bottom-right
];

fn get_xmases(input: &Vec<Vec<u8>>, v: usize, h: usize) -> u32 {
    if input[v][h] != b'X' {
        return 0;
    }

    let mut count = 0;

    'directions: for (dv, dh, direction_name) in DIRECTIONS {
        let check_v = v as i16 + 3 * dv;
        let check_h = h as i16 + 3 * dh;
        if check_v >= 0
            && check_v < input.len() as i16
            && check_h >= 0
            && check_h < input[0].len() as i16
        {
            let mut xv = v as i16;
            let mut xh = h as i16;
            for i in 1..=3_usize {
                xv += dv;
                xh += dh;

                if input[xv as usize][xh as usize] != XMAS[i] {
                    continue 'directions;
                }
            }
            count += 1;
        }
    }

    count
}

fn handle_puzzle1(input: String) -> Result<u32, Box<dyn std::error::Error>> {
    let cols = input.find('\n').unwrap();
    let rows = input.bytes().filter(|c| *c == b'\n').count() + 1;

    let input = input
        .split('\n')
        .map(|each| each.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut count = 0;

    for v in 0..rows {
        for h in 0..cols {
            // 12 o'clock
            count += get_xmases(&input, v, h);
        }
    }

    Ok(count)
}

fn is_super_xmas(window: String) -> bool {
    Regex::new(r"(M.S.A.M.S)|(M.M.A.S.S)|(S.M.A.S.M)|(S.S.A.M.M)")
        .unwrap()
        .is_match(window.as_str())
}

fn handle_puzzle2(input: String) -> Result<u32, Box<dyn std::error::Error>> {
    let input = input
        .split('\n')
        .map(|each| each.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut count = 0;

    // cpu fan goes brrrrrrr ✈️
    for window in WindowIterator::new(&input) {
        let is_super_xmas = is_super_xmas(
            window
                .into_iter()
                .map(|row| row.into_iter().map(|byte| byte as char).collect::<String>())
                .collect::<Vec<String>>()
                .join(""),
        );
        if is_super_xmas {
            count += 1;
        }
    }

    Ok(count)
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let raw = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    // ....XXMAS.
    // .SAMXMS...
    // ...S..A...
    // ..A.A.MS.X
    // XMASAMX.MM
    // X.....XA.A
    // S.S.S.S.SS
    // .A.A.A.A.A
    // ..M.M.M.MM
    // .X.X.XMASX

    assert_eq!(handle_puzzle1(raw.to_string())?, 18);

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let raw = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    // .M.S......
    // ..A..MSMS.
    // .M.S.MAA..
    // ..A.ASMSM.
    // .M.S.M....
    // ..........
    // S.S.S.S.S.
    // .A.A.A.A..
    // M.M.M.M.M.
    // ..........

    assert_eq!(handle_puzzle2(raw.to_string())?, 9);

    Ok(())
}
