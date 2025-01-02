use std::fs::read_to_string;

fn main() {
    let day = 1;
    println!(
        "\n/* {:40} */ \n/* Day {:02}: {:32} */",
        "Advent of Code 2024", day, "Historian Hysteria"
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
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        left_list.push(parts.next().unwrap().parse().unwrap());
        right_list.push(parts.last().unwrap().parse().unwrap());
    });

    left_list.sort_by(|a, b| a.cmp(b));
    right_list.sort_by(|a, b| a.cmp(b));

    let mut right_iter = right_list.iter();
    let result_part1 = left_list
        .iter()
        .map(|left| (left, right_iter.next().unwrap()))
        .fold(0, |acc, (left, right)| acc + (left - right).abs());
    println!("Total distance (part 1): {result_part1}");

    let result_part2 = left_list.into_iter().fold(0, |acc, left| {
        acc + right_list
            .iter()
            .fold(0, |acc, &right| acc + if left == right { left } else { 0 })
    });
    println!("Similarity score (part 2): {result_part2}");
}
