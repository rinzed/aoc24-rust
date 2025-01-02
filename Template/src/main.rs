use aoc24_tools::*;
use std::fs::read_to_string;

const DAY: u8 = {{day}};

fn main() {
    init_measurements!();
    print_header(DAY, "{{title}}");

    let data = read_to_string("input.txt").unwrap();
    let (part1, part2) = measure_total!({ solve(&data) });

    println!("... (Part 1): {part1}");
    println!("... (Part 2): {part2}");

    print_summary(DAY);
}

fn solve(input: &str) -> (u32,u32) {
    dbg!(input);
    let part1 = 0;
    let part2 = 0;
    (part1, part2)
}
