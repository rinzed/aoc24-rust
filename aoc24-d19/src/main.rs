use aoc24_tools::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::read_to_string;

const DAY: u8 = 19;

fn main() {
    init_measurements!();
    print_header(DAY, "Linen Layout");

    let file = "input.txt";
    let data = read_to_string(file).unwrap();
    let (part1, part2) = measure_total!({ solve(&data) });

    println!("Number of possible designs? (Part 1): {part1}");
    println!("Number of ways to create each design (Part 2): {part2}");

    print_summary(DAY);
}

fn solve(input: &str) -> (usize, usize) {
    let (towels, designs) = measure!({ parse(input) }, "parse");

    let possible_designs = measure!({ get_possible_designs(&towels, &designs) }, "Part 1");
    let sum_of_towel_arrangements = measure!(
        { get_sum_of_towel_arrangements(&towels, &possible_designs) },
        "Part 2"
    );
    (possible_designs.len(), sum_of_towel_arrangements)
}

/* PART 1: The wrong way... or at least an un-scalable way */
fn get_possible_designs<'a>(towels: &Vec<&str>, designs: &Vec<&'a str>) -> Vec<&'a str> {
    designs
        .iter()
        .filter_map(|&design| {
            if can_create_design_using(design, towels) {
                Some(design)
            } else {
                None
            }
        })
        .collect()
}

fn can_create_design_using(initial_design: &str, available_towels: &Vec<&str>) -> bool {
    // This was how I solved part 1, but I failed to scale this approach for part 2.
    // For part 2, I have gone down a dark path and tried a lot before stumbling back to recursion.
    // I knew caching was the solution, but using an approach like below, I didn't know what to cache.
    let initial_work = Work {
        design: initial_design,
        priority: 0,
    };
    let mut queue = BinaryHeap::from([initial_work]);

    while let Some(current) = queue.pop() {
        let design = current.design;
        for &start_towel in available_towels.iter() {
            // check if the towel matches what we need:
            if start_towel.len() > design.len() {
                continue;
            }
            // check if the start matches:
            let design_start = &design[0..start_towel.len()];
            if start_towel != design_start {
                continue;
            }
            let remaining_design: &str = design[start_towel.len()..design.len()].as_ref();
            if remaining_design.is_empty() {
                return true;
            }

            // now also check from the end against all towels
            // using this approach, we are needing this to speed things up,
            // a lot of designs can be quickly eliminated when no matching end can be found.
            for &end_towel in available_towels.iter() {
                // check if the towel matches what we need:
                if end_towel.len() > remaining_design.len() {
                    continue;
                }
                // check if the END matches:
                let design_end = &remaining_design
                    [remaining_design.len() - end_towel.len()..remaining_design.len()];
                if end_towel != design_end {
                    continue;
                }
                // now try recursing with what remains
                let remaining = &remaining_design[0..remaining_design.len() - end_towel.len()];
                if remaining.is_empty() {
                    return true;
                }
                queue.push(Work {
                    design: remaining,
                    priority: remaining.len(),
                });
            }
        }
    }
    false
}

#[derive(Eq, PartialEq)]
struct Work<'a> {
    priority: usize, //length, shortest remaining designs should be checked first
    design: &'a str,
}

impl Ord for Work<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

impl PartialOrd for Work<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/* PART 2: A much better way */
fn get_sum_of_towel_arrangements(towels: &Vec<&str>, designs: &Vec<&str>) -> usize {
    // convert the towels to a look-up because this is way quicker (and easier)
    let towel_set = towels.iter().map(|&s| s).collect::<HashSet<_>>();

    // with this cache we will remember the end of the design and how many solutions it has.
    let mut cache = HashMap::new();

    let mut result = 0;
    for &design in designs {
        result += get_towel_arrangements(design, &towel_set, 0, &mut cache);
    }
    result
}

// We are using string-slices for the better performance,
// but we need to tell the compiler what the life-time of those slices are.
// the slices we store in the cache are linked to the design slice
fn get_towel_arrangements<'a>(
    design: &'a str,
    towel_set: &HashSet<&str>,
    start_i: usize,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    // the end of the string is the key of the cache
    let cache_key = &design[start_i..design.len()];
    if let Some(&result) = cache.get(cache_key) {
        return result;
    }

    let mut result = 0;
    for end_i in start_i + 1..design.len() + 1 {
        let part = &design[start_i..end_i];
        if let Some(_) = towel_set.get(part) {
            if end_i == design.len() {
                result += 1;
            } else {
                result += get_towel_arrangements(design, towel_set, end_i, cache);
            }
        }
    }

    cache.insert(cache_key, result);
    result
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut towels = Vec::new();
    let mut designs = Vec::new();

    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            towels = parse_towels(line);
        } else if i > 1 {
            designs.push(line);
        }
    }
    (towels, designs)
}

fn parse_towels(line: &str) -> Vec<&str> {
    line.split(", ").collect()
}
