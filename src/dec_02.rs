use std::fs::read_to_string;

pub fn load_input(path: &str) -> Result<Vec<Vec<u8>>, std::io::Error> {
    let raw = read_to_string(path)?;
    let mut out = vec![];

    for line in raw.lines() {
        let mut next = vec![];
        for raw_num in line.split_whitespace() {
            next.push(raw_num.parse().unwrap())
        }
        out.push(next);
    }

    Ok(out)
}

/// count "unsafe" reports
pub fn puzzle1() -> Result<u16, Box<dyn std::error::Error>> {
    let reports = load_input("./inputs/dec02.txt")?;

    Ok(compute_safe_total(reports))
}
pub fn puzzle2() -> Result<u16, Box<dyn std::error::Error>> {
    let reports = load_input("./inputs/dec02.txt")?;

    Ok(compute_safe_total_with_dampening_lazy(reports))
}

fn compute_safe_total(reports: Vec<Vec<u8>>) -> u16 {
    let mut safe_total = 0 as u16;
    for report in reports {
        safe_total += if is_safe(report) { 1 } else { 0 };
    }

    safe_total
}

fn is_safe(report: Vec<u8>) -> bool {
    enum State {
        Undefined,
        Increasing,
        Decreasing,
    }
    let mut state = State::Undefined;
    'inner: for i in 1..report.len() {
        let [x, y] = &report[i - 1..=i] else { panic!() };
        match state {
            State::Undefined => {
                if x < y {
                    state = State::Increasing;
                }
                if x > y {
                    state = State::Decreasing;
                }
                if x.abs_diff(*y) > 3 || x == y {
                    return false;
                }
            }
            State::Decreasing => {
                if x.abs_diff(*y) > 3 || x <= y {
                    return false;
                }
            }
            State::Increasing => {
                if x.abs_diff(*y) > 3 || x >= y {
                    return false;
                }
            }
        }
    }

    true
}

fn compute_safe_total_with_dampening_lazy(reports: Vec<Vec<u8>>) -> u16 {
    let mut safe_total = 0;
    for report in reports {
        for i in 0..report.len() {
            let brute_force_damp_report = report
                .iter()
                .take(i)
                .chain(report.iter().skip(i + 1))
                .map(|e| *e)
                .collect();

            // we just need one; there's not that much comp to do
            if is_safe(brute_force_damp_report) {
                safe_total += 1;
                break;
            }
        }
    }

    safe_total
}

#[test]
fn test_compute_unsafe_total() {
    let expected = 2;
    let actual = compute_safe_total(vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![1, 3, 2, 4, 5],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9],
    ]);
    assert_eq!(expected, actual);
}

#[test]
fn test_with_dampener() {
    let expected = 4;
    let actual = compute_safe_total_with_dampening_lazy(vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![1, 3, 2, 4, 5],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9],
    ]);
    assert_eq!(expected, actual);
}
