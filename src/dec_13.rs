use nalgebra::{Matrix2, Vector2};
use regex::Regex;

type PuzzleOutput = f64;

/// Explanation of approach:
/// 
/// At it's heart, this puzzle is a linear algebra problem.
/// Buttons A and B define a change of basis matrix from "input space" to "location space":
///
///  |A_x B_x|  = X
///  |A_y B_y|
/// 
/// Conveniently, "input space" itself is just represented by the identity matrix:
/// 
///  | 1  0 |  = I
///  | 0  1 |
/// 
/// The first column of input space represents inputs from button A.
/// The second column represents inputs from button B.
/// Additionally, any point in input space is a measure of how many times each button has been clicked.
/// 
/// We can map *from* location space *back to* input space, so long as X is an invertible matrix, that is
/// 
/// p = X⁻¹ * p'
/// 
/// where p ∈ input space, and p' ∈ location space.
/// ---
///
/// Additionally, we define a cost function cf(p) over the input space,
///
///  cf(p) = 3 * p_x + 1 * p_y.
/// 
/// which represents the cost (in tokens) of any given "play"
/// ---
/// 
/// The final trick comes from recognizing that after mapping from location space back to input space,
/// the only "winnable" games are the ones where p ∈ input space is purely composed of integer components,
/// p ∈ (ℤ, ℤ). That is to say, there are no "fractional button inputs", only whole-valued inputs.
/// ---
/// 
/// In conclusion, the answer to our problem reduces to:
/// 
/// total_cost = ∑ cf(X⁻¹p') for all p' | p' ∈ game prizes locations from G && p = X⁻¹p' ∈ (ℤ, ℤ),
///              G
/// 
/// where G = the set of all games.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./inputs/dec13.txt")?;
    let now = std::time::Instant::now();

    let result = handle_puzzle1(input.as_str())?;
    println!(
        "Puzzle 1: ans={}, ({} us)",
        result,
        now.elapsed().as_micros()
    );

    let result = handle_puzzle2(input.as_str())?;
    println!(
        "Puzzle 2: ans={}, ({} us)",
        result,
        now.elapsed().as_micros()
    );

    Ok(())
}

fn parse(input: &str) -> Vec<(Matrix2<f64>, Vector2<f64>)> {
    let chunks = input.split("\n\n");
    let btn_re = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    chunks
        .map(|chunk| {
            let mut lines = chunk.lines();

            let btn_a = btn_re.captures(lines.next().unwrap()).unwrap();
            let (ax, ay) = (
                btn_a.get(1).unwrap().as_str().parse().unwrap(),
                btn_a.get(2).unwrap().as_str().parse().unwrap(),
            );

            let btn_b = btn_re.captures(lines.next().unwrap()).unwrap();
            let (bx, by) = (
                btn_b.get(1).unwrap().as_str().parse().unwrap(),
                btn_b.get(2).unwrap().as_str().parse().unwrap(),
            );

            let prize = prize_re.captures(lines.next().unwrap()).unwrap();
            let (px, py) = (
                prize.get(1).unwrap().as_str().parse().unwrap(),
                prize.get(2).unwrap().as_str().parse().unwrap(),
            );

            let basis_matrix = Matrix2::new(ax, bx, ay, by);
            let prize_coord = Vector2::new(px, py);

            (basis_matrix, prize_coord)
        })
        .collect::<Vec<_>>()
}

const THR: f64 = 1e-3_f64;

fn handle_puzzle1(input: &str) -> Result<PuzzleOutput, Box<dyn std::error::Error>> {
    Ok(parse(input)
        .into_iter()
        .map(|(X, p)| {
            let X_inv = X.try_inverse().unwrap();
            let p_prime = X_inv * p;
            let (ppx, ppy) = (p_prime.x, p_prime.y);

            // apologize for any floating point rounding
            let is_Z_enough = (ppx.round() - ppx).abs() < THR
                && (ppy.round() - ppy).abs() < THR;

            if is_Z_enough {
                3_f64 * ppx + 1_f64 * ppy
            } else {
                0_f64
            }
        })
        .sum::<PuzzleOutput>())
}

fn handle_puzzle2(input: &str) -> Result<PuzzleOutput, Box<dyn std::error::Error>> {
    Ok(parse(input)
        .into_iter()
        .map(|(X, mut p)| {
            // trivially handled
            p += Vector2::new(10_000_000_000_000_f64, 10_000_000_000_000_f64);
            let X_inv = X.try_inverse().unwrap();
            let p_prime = X_inv * p;
            let (ppx, ppy) = (p_prime.x, p_prime.y);

            // apologize for any floating point rounding
            let is_Z_enough = (ppx.round() - ppx).abs() < THR
                && (ppy.round() - ppy).abs() < THR;

            if is_Z_enough {
                3_f64 * ppx + 1_f64 * ppy
            } else {
                0_f64
            }
        })
        .sum::<PuzzleOutput>())
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    assert_eq!(handle_puzzle1(input)?, 480_f64);

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    assert_eq!(handle_puzzle2(input)?, 875318608908_f64);

    Ok(())
}
