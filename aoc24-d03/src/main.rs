use regex::Regex;
use std::fs::read_to_string;

fn solve(input: &str) {
    let part1 = do_multiplication(input);
    println!("Sum of all multiplications (Part1): {part1}");

    let part2 = do_enabled_multiplications(input);
    println!("Sum of enabled multiplications (Part2) {part2}");
}

fn do_multiplication(input: &str) -> u32 {
    let regex = Regex::new(r"mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\)").unwrap();
    let mut result = 0;
    for capture in regex.captures_iter(input) {
        let x = capture.name("x").unwrap().as_str().parse::<u32>().unwrap();
        let y = capture.name("y").unwrap().as_str().parse::<u32>().unwrap();
        result += x * y;
    }
    result
}

fn do_enabled_multiplications(input: &str) -> u32 {
    let regex =
        r"(?<mul>mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))";
    let regex = Regex::new(regex).unwrap();

    let mut result = 0;
    let mut enabled = true;
    for capture in regex.captures_iter(input) {
        if enabled {
            match capture.name("mul") {
                Some(_) => {
                    let x = capture.name("x").unwrap().as_str().parse::<u32>().unwrap();
                    let y = capture.name("y").unwrap().as_str().parse::<u32>().unwrap();
                    result += x * y;
                }
                None => {}
            }
            match capture.name("dont") {
                Some(_) => {
                    enabled = false;
                }
                None => {}
            }
        } else {
            match capture.name("do") {
                Some(_) => {
                    enabled = true;
                }
                None => {}
            }
        }
    }
    result
}

fn main() {
    let day = 3;
    println!(
        "\n/* {:40} */ \n/* Day {:02}: {:32} */",
        "Advent of Code 2024", day, "Mull It Over"
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
