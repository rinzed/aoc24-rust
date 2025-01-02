use std::fs::read_to_string;

fn main() {
    let day = 2;
    println!(
        "\n/* {:40} */ \n/* Day {:02}: {:32} */",
        "Advent of Code 2024", day, "Red-Nosed Reports"
    );
    let start = std::time::Instant::now();

    let data = read_to_string("input.txt").unwrap();
    solve(&data);

    // Print summary
    let time = start.elapsed();
    let ns = time.as_nanos();
    let version = rustc_version::version().unwrap();
    let lines = read_to_string("src/main.rs").unwrap().lines().count();
    let os_arch = format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH);
    println!("\n| Day {day} | \u{1F980} Rust {version} | \u{23F1}\u{FE0F} {time:?} ({ns:?} ns) | \u{1F4DC} {lines} lines | \u{2699}\u{FE0F} {os_arch} |");
}

fn solve(input: &str) {
    let reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<i16>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut safe_count = 0;
    let mut safe_dampened_count = 0;
    for report in reports.into_iter() {
        if is_safe(report.iter().collect()) {
            safe_count += 1;
        } else {
            'dampener: for i in 0..report.len() {
                let before_skip = report.iter().take(i);
                let after_skip = report.iter().skip(i + 1);
                if is_safe(before_skip.chain(after_skip).collect()) {
                    safe_dampened_count += 1;
                    break 'dampener;
                }
            }
        }
    }

    println!("Safe reports (part 1): {}", safe_count);
    safe_count += safe_dampened_count;
    println!("Safe reports using Problem Dampener (part 2): {}", safe_count);
}

fn is_safe(report: Vec<&i16>) -> bool {
    let prev = report[0];
    let next = report[1];
    let ascending = next - prev > 0;

    for i in 1..report.len() {
        let prev = report[i - 1];
        let next = report[i];
        let diff = next - prev;
        if !(-4 < diff
            && diff != 0
            && diff < 4
            && ((ascending && diff > 0) || (!ascending && diff < 0)))
        {
            return false;
        }
    }
    true
}
