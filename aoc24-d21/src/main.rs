use aoc24_tools::*;
use std::collections::{BinaryHeap, HashMap};
use std::fs::read_to_string;

const DAY: u8 = 21;

fn main() {
    init_measurements!();
    print_header(DAY, "Keypad Conundrum");

    let data = read_to_string("input.txt").unwrap();
    let (part1, part2) = measure_total!({ solve(&data) });
    println!("Sum of the complexities using TWO directional robots (Part 1): {part1}");
    println!("Sum of the complexities using TWENTY-FIVE directional robots  (Part 2): {part2}");

    print_summary(DAY);
}

fn solve(input: &str) -> (u64, u64) {
    std::sync::Once::new().call_once(|| {
        lazy_static::initialize(&DIRECTIONAL);
        lazy_static::initialize(&NUMERIC);
    });
    let part1 = calculate_complexity_for_numeric_codes_with_two_directional_keypads(input);
    let part2 = calculate_complexity_for_numeric_codes(input, 25);
    (part1, part2)
}

/****************************/
/* Structs & static keypads */
/****************************/
struct Keypad {
    // a keypad has a set of buttons, put them in a hashmap to be able to look up information about the buttons
    buttons: HashMap<char, SequenceCache>,
}

struct SequenceCache {
    sequences_to_button: HashMap<char, Vec<String>>,
    best_sequence_to_button: HashMap<char, String>,
}

lazy_static::lazy_static! {
    static ref DIRECTIONAL: Keypad = Keypad::directional();
    static ref NUMERIC: Keypad = Keypad::numeric();
}

/**********/
/* Part 1 */
/**********/
fn calculate_complexity_for_numeric_codes_with_two_directional_keypads(codes: &str) -> u64 {
    // Set up a cache to store how a sequence from 'A' to 'A' translates to the next directional keypad
    let mut cache = HashMap::new();

    let mut result = 0;
    for code in codes.lines() {
        let min_button_presses =
            get_min_presses_for_numeric_code_with_two_directional_keypads(code, &mut cache);
        result += calculate_complexity(code, min_button_presses);
    }
    result
}

fn get_min_presses_for_numeric_code_with_two_directional_keypads(
    numeric_code: &str,
    cache: &mut HashMap<String, String>,
) -> u64 {
    // step 1: find the possible routes on the numeric key pad
    let button_sequences = NUMERIC.find_all_sequences(numeric_code);

    // step 2: process the directional robots/keypads
    let mut min_sequence_length = u64::MAX;
    for button_sequence in button_sequences {
        // let's only keep unique sequences and count how many there are:
        let mut sub_sequence_counts = HashMap::new();
        update_sub_sequence_count_with_sequence(&mut sub_sequence_counts, button_sequence, 1);

        // apply now process the directional keypads between us and the numeric:
        for _ in 0..2 {
            sub_sequence_counts =
                DIRECTIONAL.get_sub_sequences_for_next_robot(sub_sequence_counts, cache);
        }

        let sequence_length = sub_sequence_counts
            .iter()
            .map(|(fragment, amount)| (fragment.len() + 1) as u64 * amount)
            .sum();
        if sequence_length < min_sequence_length {
            min_sequence_length = sequence_length;
        }
    }
    min_sequence_length
}

fn update_sub_sequence_count_with_sequence(
    sub_sequence_counts: &mut HashMap<String, u64>,
    button_sequence: String,
    count: u64,
) {
    // split the whole sequence up in parts from A to A (ignore the last, otherwise it's an empty sequence)
    for sub_sequence in button_sequence[0..button_sequence.len() - 1].split('A') {
        sub_sequence_counts
            .entry(sub_sequence.to_string())
            .and_modify(|total| *total += count)
            .or_insert(count);
    }
}

fn calculate_complexity(input: &str, instruction_len: u64) -> u64 {
    let value = input[0..3].parse::<u64>().unwrap();
    instruction_len * value
}

impl Keypad {
    fn find_all_sequences(&self, button_sequence: &str) -> Vec<String> {
        // This method is shared between part 1 and part 2, because it works fine for the numeric keypad in both cases
        let mut from_button = 'A';
        let mut all_sequences = Vec::from([String::new()]);
        for to_button in button_sequence.chars() {
            let mut appended_sequences = Vec::new();
            if from_button == to_button {
                for prev_sequence in &all_sequences {
                    appended_sequences.push(prev_sequence.clone() + "A");
                }
            } else {
                let sequences_to_next_button =
                    &self.buttons[&from_button].sequences_to_button[&to_button];
                for prev_sequence in &all_sequences {
                    for sequence_to_next_button in sequences_to_next_button {
                        appended_sequences
                            .push(prev_sequence.clone() + sequence_to_next_button + "A");
                    }
                }
            }

            all_sequences = appended_sequences;
            from_button = to_button;
        }
        all_sequences
    }

    fn get_sub_sequences_for_next_robot(
        &self,
        sub_sequences_with_counter: HashMap<String, u64>,
        cache: &mut HashMap<String, String>,
    ) -> HashMap<String, u64> {
        let mut sub_sequence_count_result = HashMap::new();
        for (sub_sequence, count) in sub_sequences_with_counter {
            let sub_sequence_with_a = sub_sequence.to_string() + "A";

            let min_sequence = self.get_min_sequence_for_sub_sequence(&sub_sequence_with_a, cache);
            update_sub_sequence_count_with_sequence(
                &mut sub_sequence_count_result,
                min_sequence,
                count,
            );
        }
        sub_sequence_count_result
    }

    fn get_min_sequence_for_sub_sequence(
        &self,
        sub_sequence: &str,
        cache: &mut HashMap<String, String>,
    ) -> String {
        let key = sub_sequence.to_string();
        if let Some(output) = cache.get(&key) {
            return output.clone();
        }

        let mut sequence = String::new();
        let mut from_button = 'A';
        for to_button in sub_sequence.chars() {
            if from_button != to_button {
                sequence += &self.buttons[&from_button].best_sequence_to_button[&to_button]
            };
            sequence.push('A');
            from_button = to_button;
        }

        cache.insert(key, sequence.clone());
        sequence
    }
}

/**************************************/
/* Part 2 with 25 directional keypads */
/**************************************/
fn calculate_complexity_for_numeric_codes(codes: &str, directional_keypads: u8) -> u64 {
    // create a cache to remember sequences we already found
    let mut cache = HashMap::new();

    let mut result = 0;
    for code in codes.lines() {
        let min_button_presses =
            get_min_button_presses_for_numeric_code(code, directional_keypads, &mut cache);
        result += calculate_complexity(code, min_button_presses);
    }
    result
}

fn get_min_button_presses_for_numeric_code(
    numeric_code: &str,
    robots: u8,
    cache: &mut HashMap<(String, u8), u64>,
) -> u64 {
    let button_sequences = NUMERIC.find_all_sequences(numeric_code);

    let mut min_button_presses = u64::MAX;
    for button_sequences in button_sequences {
        let button_presses = DIRECTIONAL.find_min_button_presses(&button_sequences, robots, cache);
        if button_presses < min_button_presses {
            min_button_presses = button_presses;
        }
    }
    min_button_presses
}

impl Keypad {
    fn find_min_button_presses(
        &self,
        button_sequence: &String,
        depth: u8,
        cache: &mut HashMap<(String, u8), u64>,
    ) -> u64 {
        // Credits to Michael for confirming that this approach would work.
        // I had to peek at his solution for a breakthrough.
        // After tinkering a lot using my sub_sequence counting approach couldn't find a correct answer
        if depth == 0 {
            return button_sequence.len() as u64;
        }

        let key = (button_sequence.clone(), depth);
        if let Some(&result) = cache.get(&key) {
            return result;
        }

        let mut total_sequence_length = 0;
        let mut from_button = 'A';
        for to_button in button_sequence.chars() {
            // for each button to the next, find the shortest sequence that works
            let mut min_sub_sequence_length = u64::MAX;
            for button_sequence in self.get_button_sequences_between_buttons(from_button, to_button)
            {
                // use recursion to find the shortest length that the next robot down the line can take:
                let new_sub_sequence_length =
                    self.find_min_button_presses(&button_sequence, depth - 1, cache);
                if new_sub_sequence_length < min_sub_sequence_length {
                    min_sub_sequence_length = new_sub_sequence_length;
                }
            }

            total_sequence_length += min_sub_sequence_length;

            from_button = to_button;
        }

        cache.insert(key, total_sequence_length);
        total_sequence_length
    }

    fn get_button_sequences_between_buttons(
        &self,
        from_button: char,
        to_button: char,
    ) -> Vec<String> {
        if from_button == to_button {
            return Vec::from(["A".to_string()]);
        }

        self.buttons[&from_button].sequences_to_button[&to_button]
            .iter()
            .map(|s| s.clone() + "A")
            .collect::<Vec<String>>()
    }
}

/*********************************/
/* Setup of keypads with buttons */
/*********************************/
impl Keypad {
    fn numeric() -> Keypad {
        // +---+---+---+
        // | 7 | 8 | 9 |
        // +---+---+---+
        // | 4 | 5 | 6 |
        // +---+---+---+
        // | 1 | 2 | 3 |
        // +---+---+---+
        //     | 0 | A |
        //     +---+---+
        Keypad::from_text(
            "789\n\
                  456\n\
                  123\n\
                  #0A",
        )
    }

    fn directional() -> Keypad {
        //     +---+---+
        //     | ^ | A |
        // +---+---+---+
        // | < | v | > |
        // +---+---+---+
        Keypad::from_text(
            "#^A\n\
                  <v>",
        )
    }

    fn from_text(text: &str) -> Keypad {
        let grid = text
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<Vec<char>>>();

        let mut routes = HashMap::new();

        // use a mod/divide approach to iterate over all buttons
        // use a second loop to get button combinations
        // find the shortest path between to and also store the reverse of it.
        let width = grid[0].len();
        let height = grid.len();
        for from_i in 0..width * height {
            for to_i in from_i + 1..width * height {
                let from_x = from_i % width;
                let from_y = from_i / width;
                let to_x = to_i % width;
                let to_y = to_i / width;

                let from_button = grid[from_y][from_x];
                let to_button = grid[to_y][to_x];
                if from_button == '#' || to_button == '#' {
                    continue;
                }

                let routes_from_to_to = find_routes(&grid, from_x, from_y, to_x, to_y).unwrap();
                let routes_to_to_from = reverse_routes(&routes_from_to_to);
                routes
                    .entry(from_button)
                    .or_insert(Vec::new())
                    .push((to_button, routes_from_to_to));

                routes
                    .entry(to_button)
                    .or_insert(Vec::new())
                    .push((from_button, routes_to_to_from));
            }
        }

        let mut buttons = HashMap::new();
        for (key, routes) in routes {
            buttons.insert(key, SequenceCache::new(routes));
        }

        Keypad { buttons }
    }
}

fn reverse_routes(routes_from_to_to: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    for route in routes_from_to_to {
        result.push(reverse_route(route));
    }
    result
}

fn reverse_route(route: &String) -> String {
    route
        .chars()
        .rev()
        .map(|c| match c {
            '>' => '<',
            '<' => '>',
            '^' => 'v',
            'v' => '^',
            c => c,
        })
        .collect::<String>()
}

impl SequenceCache {
    fn new(routes: Vec<(char, Vec<String>)>) -> SequenceCache {
        // used for part 1: calculate a rank for each route.
        let mut best_route_to_other_buttons = HashMap::new();
        for (to_button, options) in routes.iter() {
            best_route_to_other_buttons.insert(
                *to_button,
                options
                    .iter()
                    .min_by(|a, b| rank(a).cmp(&rank(b)))
                    .unwrap()
                    .clone(),
            );
        }

        SequenceCache {
            sequences_to_button: routes.into_iter().collect(),
            best_sequence_to_button: best_route_to_other_buttons,
        }
    }
}

fn rank(sequence: &String) -> usize {
    // This works if we aren't going to deep, but this idea did not work out as an approach for 25 robots.
    // moving from and pressing a certain button takes a fixed minimum number of steps.
    // this fails for a lot of robots, because a long path for a certain robot might result
    // in a better path for robot further up the chain.
    // it worked out for the two directional robots at first, so I kept it in
    let mut result = 0;
    let mut from_button = sequence.chars().nth(0).unwrap();
    for to_button in sequence.chars().skip(1) {
        if from_button == to_button {
            result += 1;
        } else {
            match (from_button, to_button) {
                ('A', '^') => result += 2,
                ('A', '>') => result += 2,
                ('A', 'v') => result += 3,
                ('A', '<') => result += 4,

                ('^', 'A') => result += 2,
                ('^', '>') => result += 3,
                ('^', 'v') => result += 2,
                ('^', '<') => result += 3,

                ('>', 'A') => result += 2,
                ('>', '^') => result += 3,
                ('>', 'v') => result += 2,
                ('>', '<') => result += 3,

                ('v', 'A') => result += 3,
                ('v', '^') => result += 2,
                ('v', '>') => result += 2,
                ('v', '<') => result += 2,

                ('<', 'A') => result += 4,
                ('<', '^') => result += 3,
                ('<', '>') => result += 3,
                ('<', 'v') => result += 2,
                _ => panic!(
                    "This combination should not exist? {}->{}",
                    from_button, to_button
                ),
            }
        }
        from_button = to_button;
    }
    result
}

/******************************************************/
/* Dijkstra's Algorithm to find paths between buttons */
/******************************************************/
#[derive(Eq, PartialEq)]
struct Work {
    node: (i8, i8),
    route: String,
}

impl Ord for Work {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.route.len().cmp(&other.route.len()).reverse()
    }
}

impl PartialOrd for Work {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_routes(
    grid: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
) -> Option<Vec<String>> {
    let end = (end_x as i8, end_y as i8);
    let starting_step = Work {
        node: (start_x as i8, start_y as i8),
        route: String::new(),
    };

    let mut result = Vec::new();
    let mut visited: HashMap<(i8, i8), usize> = HashMap::new();
    let mut queue = BinaryHeap::from([starting_step]);

    let mut shortest = u32::MAX as usize;

    while let Some(current) = queue.pop() {
        if current.node == end {
            // found a path to the end, now remember all steps that were part of it.
            // the hashset will help with deduplication.
            shortest = current.route.len();
            result.push(current.route);
            continue;
        } else if current.route.len() > shortest {
            // stop, because we are now processing paths longer than the shortest.
            return Some(result);
        }

        // check if we have been to this location, using that direction.
        // we might have found an alternative route to this state, which is fine.
        // to keep the algorithm simple: just continue resolving it.
        let key = current.node.clone();
        let visited_distance = visited.entry(key).or_insert(current.route.len());
        // when the new route is worse, no need to continue down this path:
        if *visited_distance >= current.route.len() {
            for neighbour in get_neighbours(current, grid) {
                queue.push(neighbour);
            }
        }
    }
    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

fn get_neighbours(current: Work, grid: &Vec<Vec<char>>) -> Vec<Work> {
    let neighbours = [
        Work {
            node: (current.node.0 - 1, current.node.1),
            route: current.route.clone() + "<",
        },
        Work {
            node: (current.node.0 + 1, current.node.1),
            route: current.route.clone() + ">",
        },
        Work {
            node: (current.node.0, current.node.1 - 1),
            route: current.route.clone() + "^",
        },
        Work {
            node: (current.node.0, current.node.1 + 1),
            route: current.route + "v",
        },
    ];
    neighbours
        .into_iter()
        .filter_map(|w| {
            let line = grid.get(w.node.1 as usize)?;
            let &char = line.get(w.node.0 as usize)?;
            return if char == '#' { None } else { Some(w) };
        })
        .collect()
}
