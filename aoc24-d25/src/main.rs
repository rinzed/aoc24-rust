use aoc24_tools::*;
use std::fs::read_to_string;

const DAY: u8 = 25;

fn main() {
    init_measurements!();
    print_header(DAY, "Code Chronicle");

    let data = read_to_string("input.txt").unwrap();
    let part1 = measure_total!({ solve(&data) });

    println!("Number of unique fitting lock/key pairs: (Part 1): {part1}");
    print_summary(DAY);

    println!("Ho ho ho, Merry Christmas everyone!");
}

fn solve(input: &str) -> u32 {
    let (keys, locks) = parse(input);
    count_fitting_pairs(&keys, &locks)
}

type Key = [u8; 5];
type Lock = [u8; 5];

fn parse(input: &str) -> (Vec<Key>, Vec<Lock>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    let lines: Vec<_> = input.lines().collect();
    for block in lines.chunks(8) {
        let top_row = block[0];
        let pattern = parse_pattern(&block[1..7]);
        if top_row.starts_with('.') {
            // checking the first character is enough to see if it's a key or lock
            keys.push(pattern);
        } else {
            locks.push(pattern);
        }
    }
    (keys, locks)
}

fn parse_pattern(block: &[&str]) -> [u8; 5] {
    // the parsing of a key and a lock are essentially the same.
    // simply count the number of # in a column to know its size
    let mut result = <[u8; 5]>::default();
    for j in 0..5 {
        let line = &block[j];
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                result[i] += 1;
            }
        }
    }
    result
}

fn count_fitting_pairs(keys: &Vec<Key>, locks: &Vec<Lock>) -> u32 {
    let mut count = 0;
    // just brute force check everything, there might be a faster method, but not with less code (right??)
    for key in keys {
        for lock in locks {
            if does_key_fit_in_lock(key, lock) {
                count += 1;
            }
        }
    }
    count
}

fn does_key_fit_in_lock(key: &Key, lock: &Lock) -> bool {
    for i in 0..5 {
        if key[i] + lock[i] > 5 {
            return false;
        }
    }
    true
}
