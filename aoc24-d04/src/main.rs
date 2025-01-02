use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let day = 4;
    println!(
        "\n/* {:40} */ \n/* Day {:02}: {:32} */",
        "Advent of Code 2024", day, "Ceres Search"
    );

    let data = read_to_string("input.txt").unwrap();

    let start = std::time::Instant::now();
    let (part1, part2) = solve(&data);
    let time = start.elapsed();

    println!("XMAS count (Part 1): {part1}");
    println!("Crossed-MAS count (Part 2): {part2}");

    // Print summary
    let ns = time.as_nanos();
    let version = rustc_version::version().unwrap();
    let lines = read_to_string("src/main.rs").unwrap().lines().count();
    let os_arch = format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH);
    println!("\n| Day {day} | \u{1F980} Rust {version} | \u{23F1}\u{FE0F} {time:?} ({ns:?} ns) | \u{1F4DC} {lines} lines | \u{2699}\u{FE0F} {os_arch} |");
}

fn solve(input: &str) -> (usize, usize) {
    let part1 = count_xmas_words(input);
    let part2 = count_crossed_mas(input);

    (part1, part2)
}

fn count_xmas_words(input: &str) -> usize {
    let horizontal_lines = input.lines().collect::<Vec<&str>>();
    let other_lines = transform_into_vertical_and_diagonals(&horizontal_lines);

    // use to regexes because XMAS and SAMX can overlap like XMASAMX and count as two
    let xmas = Regex::new(r"XMAS").unwrap();
    let revered = Regex::new(r"SAMX").unwrap();

    let mut count = 0;
    for line in horizontal_lines {
        count += xmas.find_iter(line).count() + revered.find_iter(line).count();
    }
    for line in other_lines {
        let line = line.as_str();
        count += xmas.find_iter(line).count() + revered.find_iter(line).count();
    }
    count
}

fn transform_into_vertical_and_diagonals(lines: &Vec<&str>) -> Vec<String> {
    let mut vertical = HashMap::new();
    let mut diagonal_1 = HashMap::new();
    let mut diagonal_2 = HashMap::new();

    let mut y = 0;
    for line in lines.into_iter() {
        let mut x = 0;
        for char in line.chars() {
            vertical.entry(x).or_insert(vec![]).push(char);
            diagonal_1.entry(y + x).or_insert(vec![]).push(char); //shift forward
            diagonal_2.entry(y - x).or_insert(vec![]).push(char); //shift back
            x += 1;
        }
        y += 1;
    }

    vertical
        .into_iter()
        .chain(diagonal_1.into_iter())
        .chain(diagonal_2.into_iter())
        .map(|(_, c)| c.iter().collect::<String>())
        .collect::<Vec<String>>()
}

fn count_crossed_mas(text: &str) -> usize {
    let mut count = 0;
    let lines = text.lines().collect::<Vec<&str>>();
    let max = lines.len(); // assume the word search is a square grid.

    let mut y = 1;
    let mut prev_line: &str = lines[0];
    for &line in lines.iter().skip(1).take(max - 2) {
        let &next_line = lines.get(y + 1).unwrap();

        let mut x = 1;
        for char in line.chars().skip(1).take(max - 2) {
            if char == 'A' {
                let top_left = prev_line.chars().nth(x - 1).unwrap();
                let top_right = prev_line.chars().nth(x + 1).unwrap();
                let bottom_left = next_line.chars().nth(x - 1).unwrap();
                let bottom_right = next_line.chars().nth(x + 1).unwrap();

                if is_ms(top_left, bottom_right) && is_ms(top_right, bottom_left) {
                    count += 1;
                }
            }
            x += 1;
        }
        y += 1;
        prev_line = line;
    }

    count
}

fn is_ms(a: char, b: char) -> bool {
    match (a, b) {
        ('M', 'S') => true,
        ('S', 'M') => true,
        _ => false,
    }
}
