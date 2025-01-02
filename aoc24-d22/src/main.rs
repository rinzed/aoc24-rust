use aoc24_tools::*;
use std::collections::HashMap;
use std::fs::read_to_string;

const DAY: u8 = 22;

fn main() {
    init_measurements!();
    print_header(DAY, "Monkey Market");

    let file = "input.txt";
    let data = read_to_string(file).unwrap();
    let (part1, part2) = measure_total!({ solve(&data) });

    println!("Sum of 2000th secret number for each buyer (Part 1): {part1}");
    println!("Number of bananas I can buy (Part 2): {part2} or");
    println!("{}", "\u{1F34C}".repeat(part2 as usize));

    print_summary(DAY);
}

fn solve(input: &str) -> (u64, u16) {
    let (part1, secret_number_sequences) =
        measure!({ sum_of_2000th_secret_numbers(input) }, "part1");
    let part2 = measure!({ find_best_banana_buy(secret_number_sequences) }, "part2");
    (part1, part2)
}

const PRUNING_NUMBER: u64 = 16777216;

fn calculate_next_secret(secret_number: &mut u64) {
    // each buyer's secret number evolves into the next secret number in the sequence via the following process:
    // - Calculate the result of multiplying the secret number by 64.
    // Then, mix this result into the secret number. Finally, prune the secret number.
    *secret_number ^= *secret_number << 6;
    *secret_number %= PRUNING_NUMBER;
    // - Calculate the result of dividing the secret number by 32.
    // Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
    *secret_number ^= *secret_number >> 5;
    *secret_number %= PRUNING_NUMBER;
    // - Calculate the result of multiplying the secret number by 2048.
    // Then, mix this result into the secret number. Finally, prune the secret number.
    *secret_number ^= *secret_number << 11;
    *secret_number %= PRUNING_NUMBER;
}

fn sum_of_2000th_secret_numbers(input: &str) -> (u64, Vec<Vec<u8>>) {
    let mut sequences = Vec::new();
    let mut sum: u64 = 0;

    let secret_numbers = input.lines().map(|l| l.parse::<u32>().unwrap());
    for original_secret_number in secret_numbers {
        let mut working_secret_number = original_secret_number as u64;
        let mut sequence = Vec::with_capacity(2001);
        sequence.push((working_secret_number % 10) as u8);
        for _ in 0..2000 {
            calculate_next_secret(&mut working_secret_number);
            sequence.push((working_secret_number % 10) as u8);
        }

        sum += working_secret_number;
        sequences.push(sequence);
    }
    (sum, sequences)
}

fn find_best_banana_buy(price_sequences: Vec<Vec<u8>>) -> u16 {
    // let's collect the number of banana's for each sequence
    let mut bananas_by_sequence = HashMap::new();

    for price_sequence in price_sequences {
        // we can only buy the first time a sequence comes by for this seller
        let mut seller_bananas_by_sequence = HashMap::new();

        let mut previous_price = price_sequence[0];
        // I have tried to get the key smaller than 4 bytes, but it just doesn't seem possible
        // the more readable, but a bit slower approach is to use a 4 byte window.
        let mut key = 0u32;
        for (i, &price) in price_sequence.iter().skip(1).enumerate() {
            let difference = 9 + price - previous_price; // use 9 as base to keep numbers positive (u8)
            key *= 19;
            key += difference as u32;
            previous_price = price;

            if i >= 3 {
                // if not yet in local_cache, remember this price
                seller_bananas_by_sequence.entry(key).or_insert(price);
                key %= 19 * 19 * 19; // remove the last part
            }
        }

        // add the results for this seller to the totals
        for (key, num_bananas) in seller_bananas_by_sequence {
            bananas_by_sequence
                .entry(key)
                .and_modify(|total_bananas| *total_bananas += num_bananas as u16)
                .or_insert(num_bananas as u16);
        }
    }

    *bananas_by_sequence.values().max().unwrap()
}
