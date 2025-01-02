use std::fs::read_to_string;
use aoc24_d06::solve;

fn main() {
    let day = 6;
    println!(
        "\n/* {:40} */ \n/* Day {:02}: {:32} */",
        "Advent of Code 2024", day, "Guard Gallivant"
    );
    let data = read_to_string("input.txt").unwrap();

    let start = std::time::Instant::now();
    let (part1, part2) = solve(&data);
    let time = start.elapsed();

    println!("Number of distinct positions visited before leaving (Part 1): {part1}");
    println!("Number of possible positions to create a loop       (Part 2): {part2}");
    assert_eq!(part1, 5162);
    assert_eq!(part2, 1909);

    let ns = time.as_nanos();
    let version = rustc_version::version().unwrap();
    let lines = read_to_string("src/main.rs").unwrap().lines().count();
    let os_arch = format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH);
    println!("\n| Day {day} | \u{1F980} Rust {version} | \u{23F1}\u{FE0F} {time:?} ({ns} ns) | \u{1F4DC} {lines} lines | \u{2699}\u{FE0F} {os_arch} |");
}
