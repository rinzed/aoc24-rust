use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashSet;
use std::fs::read_to_string;
use std::hash::Hash;

fn main() {
    let day = 5;
    println!(
        "\n/* {:40} */ \n/* Day {:02}: {:32} */",
        "Advent of Code 2024", day, "Print Queue"
    );
    let data = read_to_string("input.txt").unwrap();

    let start = std::time::Instant::now();
    let (part1, part2) = solve(&data);
    let time = start.elapsed();

    println!("Sum of middle page numbers that were correct        (Part 1): {part1}");
    println!("Sum of middle page numbers that have been corrected (Part 2): {part2}");

    let ns = time.as_nanos();
    let version = rustc_version::version().unwrap();
    let lines = read_to_string("src/main.rs").unwrap().lines().count();
    let os_arch = format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH);
    println!("\n| Day {day} | \u{1F980} Rust {version} | \u{23F1}\u{FE0F} {time:?} ({ns} ns) | \u{1F4DC} {lines} lines | \u{2699}\u{FE0F} {os_arch} |");
}

fn solve(input: &str) -> (u16, u16) {
    let (rules, updates) = parse(input);

    let mut part1 = 0u16;
    let mut part2 = 0u16;
    for update in updates {
        if update.is_sorted_by(|&before, &after| is_sorted(&rules, before, after)) {
            part1 += get_middle_page_number(update) as u16;
        } else {
            let mut sorted = update.clone();
            sorted.sort_by(|&before, &after| compare(&rules, before, after));
            part2 += get_middle_page_number(sorted) as u16;
        }
    }

    (part1, part2)
}

#[derive(Hash, PartialEq, Eq)]
struct Rule {
    before: u8,
    after: u8,
}

fn parse(input: &str) -> (HashSet<Rule>, Vec<Vec<u8>>) {
    let rules_lines = input.lines().take_while(|l| l.len() > 0);
    let rules = HashSet::from_iter(rules_lines.into_iter().map(|line| {
        // using shortcut parsing because all pages are 2 digits
        Rule {
            before: line[..2].parse().unwrap(),
            after: line[3..].parse().unwrap(),
        }
    }));

    let update_lines = input.lines().skip(rules.len() + 1);
    let updates = Vec::from_iter(
        update_lines.map(|mut line| {
            // using shortcut in parsing because all elements are 2 digits
            let page_count = line.len() / 3 + 1;
            let mut update = Vec::with_capacity(page_count);
            while line.len() > 2 {
                update.push(line[..(2)].parse().unwrap());
                line = &line[3..];
            }
            update.push(line[..(2)].parse().unwrap());
            update
        }),
    );
    (rules, updates)
}

fn compare(rules: &HashSet<Rule>, before: u8, after: u8) -> std::cmp::Ordering {
    if before == after {
         Equal
    } else if is_sorted(rules, before, after) {
        Less
    } else {
        Greater
    }
}

fn is_sorted(rules: &HashSet<Rule>, before: u8, after: u8) -> bool {
    let invalid = Rule {
        before: after,
        after: before,
    };
    !rules.contains(&invalid)
}

fn get_middle_page_number(update: Vec<u8>) -> u8 {
    update[update.len() / 2] // assume that each update has an uneven length
}
