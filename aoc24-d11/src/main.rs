use aoc24_tools::*;
use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

const DAY: u8 = 11;

fn main() {
    init_measurements!();
    print_header(DAY, "Plutonian Pebbles");

    let data = read_to_string("input.txt").unwrap();
    let (part1, part2) = measure_total!({ solve(&data) });

    println!("Number of stones after 25 blinks (Part 1): {part1}");
    println!("Number of stones after 75 blinks (Part 2): {part2}");

    print_summary(DAY);
}

fn solve(input: &str) -> (usize, usize) {
    let part1 = measure!(
        {
            let stones = parse_as_vector(input);
            brute_force_blinking(&stones, 25)
        },
        "brute-25x"
    );
    let _ = measure!(
        {
            let stones = parse_as_hashmap(input);
            smarter_blinking(stones, 25)
        },
        "smart-25x"
    );
    let part2 = measure!(
        {
            let stones = parse_as_hashmap(input);
            smarter_blinking(stones, 75)
        },
        "smart-75x"
    );
    (part1, part2)
}

fn parse_as_vector(input: &str) -> Vec<&str> {
    input.split_whitespace().into_iter().collect()
}

fn brute_force_blinking(stones: &Vec<&str>, number_of_blinks: u8) -> usize {
    // put all stones in a queue with the amount of remaining blinks for that stone
    let mut queue = stones
        .iter()
        .map(|&s| (s.to_string(), number_of_blinks))
        .collect::<VecDeque<_>>();

    let mut result = 0;
    // keep working the queue until empty (with a lot of repeating work...)
    while let Some((engraving, remaining_blinks)) = queue.pop_front() {
        if remaining_blinks == 0 {
            result += 1;
        } else {
            blink_single_stone_using_queue(&mut queue, engraving, remaining_blinks - 1);
        }
    }

    result
}

fn blink_single_stone_using_queue(
    queue: &mut VecDeque<(String, u8)>,
    engraving: String,
    blinks: u8,
) {
    if engraving == "0" || engraving == "" {
        queue.push_back(("1".to_string(), blinks));
    } else if engraving.len() % 2 == 0 {
        let half = engraving.len() / 2;
        let left_stone = engraving[..half].to_string();
        let right_stone = engraving[half..].trim_start_matches('0').to_string();
        queue.push_back((left_stone, blinks));
        queue.push_back((right_stone, blinks));
    } else {
        let value = engraving.parse::<u64>().unwrap() * 2024;
        queue.push_back((value.to_string(), blinks));
    }
}

fn parse_as_hashmap(data: &str) -> HashMap<String, usize> {
    let mut stones = HashMap::new();
    for value in data.split_whitespace() {
        stones
            .entry(value.to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    stones
}

fn smarter_blinking(stones: HashMap<String, usize>, number_of_blinks: u8) -> usize {
    let mut stones = stones;
    for _ in 0..number_of_blinks {
        stones = blink_hashmap_of_stones(&stones);
    }

    stones.values().sum()
}

fn blink_hashmap_of_stones(stones: &HashMap<String, usize>) -> HashMap<String, usize> {
    let mut result = HashMap::with_capacity(stones.len());

    for (engraving, count) in stones {
        let new_stones = blink_single_stone(engraving, *count);
        for (new_engraving, new_count) in new_stones {
            result
                .entry(new_engraving)
                .and_modify(|count| *count += new_count)
                .or_insert(new_count);
        }
    }

    result
}

fn blink_single_stone(engraved_number: &String, count: usize) -> Vec<(String, usize)> {
    if engraved_number == "0" || engraved_number == "" {
        Vec::from([("1".to_string(), count)])
    } else if engraved_number.len() % 2 == 0 {
        let half = engraved_number.len() / 2;
        let left_stone = engraved_number[..half].to_string();
        let right_stone = engraved_number[half..].trim_start_matches('0').to_string();

        Vec::from([(left_stone, count), (right_stone, count)])
    } else {
        let value = engraved_number.parse::<u64>().unwrap() * 2024;
        Vec::from([(value.to_string(), count)])
    }
}