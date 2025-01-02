use aoc24_tools::*;
use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

const DAY: u8 = 24;

fn main() {
    init_measurements!();
    print_header(DAY, "Crossed Wires");

    let data = read_to_string("input.txt").unwrap();
    let (part1, part2) = measure_total!({ solve(&data) });

    println!("The z-wires represent decimal number (Part 1): {part1} | {part1:46b}");
    println!("The wires that need to be swapped to fix the system (Part 2): {part2}");

    print_summary(DAY);
}

fn solve(input: &str) -> (u64, String) {
    let (wires, mut gates) = measure!({ parse(input) }, "parse");
    let part1 = measure!({ process_logic_system(&wires, &gates).unwrap() }, "part1");

    let mut part2 = measure!(
        { find_gates_to_swap(&mut gates, 1, &Vec::new()).unwrap() },
        "part2"
    );
    part2.sort();
    (part1, part2.join(","))
}

/*******************/
/* Types & parsing */
/*******************/
enum Operation {
    AND,
    OR,
    XOR,
}
struct Gate<'a> {
    inputs: [&'a str; 2],
    operation: Operation,
    output: &'a str,
    // used for outputs like z00 and z10
    is_z: bool,
    output_id: usize,
}

impl<'a> Gate<'a> {
    fn set_output(&mut self, value: &'a str) {
        self.output = value;
        self.is_z = &value[0..1] == "z";
        if self.is_z {
            self.output_id = parse_to_usize(&value[1..3]);
        }
    }
}

fn parse(input: &str) -> (HashMap<&str, Bit>, Vec<Gate>) {
    let mut lines = input.lines();
    // first read until empty line for values on wire
    let mut wires = HashMap::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let name = &line[0..3];
        let bit = if &line[5..6] == "1" { 1 } else { 0 };
        wires.insert(name, bit);
    }

    let mut gates = Vec::new();
    while let Some(line) = lines.next() {
        let input1 = &line[0..3];
        let operation = line[4..7].trim(); //OR is 2 letters, XOR/AND are 3, so use trim to normalize
        let input2 = &line[7..11].trim();
        let &output = &line[14..line.len()].trim();
        let is_z = &output[0..1] == "z";
        let output_id = if is_z {
            parse_to_usize(&output[1..3])
        } else {
            0
        };

        let gate = Gate {
            inputs: [input1, input2],
            operation: match operation {
                "OR" => Operation::OR,
                "AND" => Operation::AND,
                "XOR" => Operation::XOR,
                _ => panic!("Failed to parse operation {operation}"),
            },
            output,
            output_id,
            is_z,
        };
        gates.push(gate);
    }

    (wires, gates)
}

/**********/
/* Part 1 */
/**********/
fn process_logic_system(wires: &HashMap<&str, Bit>, gates: &Vec<Gate>) -> Option<u64> {
    let mut known_wires = wires.clone();
    let mut remaining_outputs = gates.iter().filter(|g| g.is_z).count();
    let mut remaining_gates = VecDeque::from_iter(gates.iter());
    while let Some(gate) = remaining_gates.pop_front() {
        if let Some(value) = gate.process_with_short_circuit(&known_wires) {
            if gate.is_z {
                remaining_outputs -= 1;
            }
            known_wires.insert(gate.output, value);
            if remaining_outputs == 0 {
                break;
            }
        } else {
            remaining_gates.push_back(gate);
        }
    }

    if remaining_outputs == 0 {
        Some(read_value_from_wires(&known_wires, "z"))
    } else {
        None
    }
}

type Bit = u8;
const TRUE: Bit = 1;
const FALSE: Bit = 0;

impl Gate<'_> {
    fn process_strict(&self, wires: &HashMap<&str, Bit>) -> Option<Bit> {
        let &wire_a = wires.get(self.inputs[0])?;
        let &wire_b = wires.get(self.inputs[1])?;
        let result = match self.operation {
            Operation::AND => wire_a & wire_b,
            Operation::OR => wire_a | wire_b,
            Operation::XOR => wire_a ^ wire_b,
        };
        Some(result)
    }

    /// This method uses short-circuiting to try to get an answer before all inputs are known.
    /// This speeds-up part 1, because fewer loops are needed.
    /// But for part 2, this is killing, because we are working with partial input's to find a partial output,
    /// this short-circuit approach results in a lot more permutations to consider and test. This is because
    /// the strict eval will short-circuit because will get stuck in an infinite loop because a wire is never
    /// calculated using the limited inputs, but with this approach, a lot more wires will get a value because
    /// a part of the input as already known.
    /// For part 1 this approach halves the processing time.
    fn process_with_short_circuit(&self, wires: &HashMap<&str, Bit>) -> Option<Bit> {
        let wire_a = wires.get(self.inputs[0]);
        let wire_b = wires.get(self.inputs[1]);
        let result = match self.operation {
            Operation::AND => match (&wire_a, &wire_b) {
                (Some(&FALSE), _) => Some(FALSE),
                (_, Some(&FALSE)) => Some(FALSE),
                (Some(&TRUE), Some(&TRUE)) => Some(TRUE),
                _ => None,
            },
            Operation::OR => match (wire_a, wire_b) {
                (Some(&TRUE), _) => Some(TRUE),
                (_, Some(&TRUE)) => Some(TRUE),
                (Some(&FALSE), Some(&FALSE)) => Some(FALSE),
                _ => None,
            },
            Operation::XOR => match (wire_a, wire_b) {
                (Some(&a), Some(&b)) => Some(a ^ b),
                _ => None,
            },
        }?;
        Some(result)
    }
}

/**********/
/* Part 2 */
/**********/
fn find_gates_to_swap(
    gates: &mut Vec<Gate>,
    number_of_bits: usize,
    active_swaps: &Vec<&str>,
) -> Option<Vec<String>> {
    if number_of_bits == 46 {
        return Some(active_swaps.into_iter().map(|&s| s.to_string()).collect());
    }
    if active_swaps.len() > 8 {
        return None;
    }

    if validate_sum(&gates, number_of_bits) == Some(true) {
        return find_gates_to_swap(gates, number_of_bits + 1, active_swaps);
    }

    // try swaps to find a valid alternative, that works until the current depth
    let swappable_wires: Vec<_> = gates
        .iter()
        .filter(|&g| !active_swaps.contains(&g.output))
        .map(|g| g.output)
        .collect();
    for i in 0..swappable_wires.len() {
        for j in i + 1..swappable_wires.len() {
            let wire_a = swappable_wires[i];
            let wire_b = swappable_wires[j];
            swap_wires(gates, &wire_a, &wire_b);
            // test if new configuration creates a valid option until the current depth
            if validate_sums_to_bits(&gates, number_of_bits) {
                // if so, continue with recursion
                let mut recursive_active_swaps = active_swaps.clone();
                recursive_active_swaps.push(&wire_a);
                recursive_active_swaps.push(&wire_b);
                if let Some(result) =
                    find_gates_to_swap(gates, number_of_bits + 1, &recursive_active_swaps)
                {
                    return Some(result);
                }
            }
            // undo swap to continue
            swap_wires(gates, &wire_a, &wire_b);
        }
    }

    None
}

fn validate_sums_to_bits(gates: &Vec<Gate>, number_of_bits: usize) -> bool {
    for bits in (1..number_of_bits + 1).rev() {
        if validate_sum(&gates, bits) != Some(true) {
            return false;
        }
    }
    true
}

fn swap_wires(gates: &mut Vec<Gate>, swap_a: &str, swap_b: &str) {
    // This code seems a bit of a hassle, but remember, we can have multiple borrows OR one mutable borrow.
    // In this approach we aren't borrowing anything until we read both outputs. And then we're mutating the gates.
    // Also, we only need to iterate only once over the gate list to find both indexes.
    let mut gate_a = usize::MAX;
    let mut gate_b = usize::MAX;
    for i in 0..gates.len() {
        if gates[i].output == swap_a {
            gate_a = i;
        }
        if gates[i].output == swap_b {
            gate_b = i;
        }
    }

    let remember_a = gates[gate_a].output;
    let remember_b = gates[gate_b].output;
    gates[gate_a].set_output(remember_b);
    gates[gate_b].set_output(remember_a);
}

fn validate_sum<'a>(gates: &'a Vec<Gate<'a>>, number_of_bits: usize) -> Option<bool> {
    let values = get_values_to_test(number_of_bits);
    if sum(gates, values[0], values[1], number_of_bits)? != values[0] + values[1] {
        Some(false)
    } else if sum(gates, values[1], values[1], number_of_bits)? != values[1] + values[1] {
        Some(false)
    } else {
        Some(true)
    }
}

fn get_values_to_test(number_of_bits: usize) -> [u64; 2] {
    if number_of_bits == 1 {
        // with 1 bit, only 0 & 1 are possible positive numbers to test
        [0, 1]
    } else {
        //otherwise
        let base = 1 << (number_of_bits - 1);
        [base - 1, base + 1]
    }
}

fn sum(gates: &Vec<Gate>, value_x: u64, value_y: u64, number_of_bits: usize) -> Option<u64> {
    let wires_x = write_value_to_wires('x', value_x, number_of_bits);
    let wires_y = write_value_to_wires('y', value_y, number_of_bits);

    let result_number_of_bits = number_of_bits + 1;

    let mut known_wires = HashMap::new();
    for (key, &value) in wires_x.iter() {
        known_wires.insert(key.as_str(), value);
    }
    for (key, &value) in wires_y.iter() {
        known_wires.insert(key.as_str(), value);
    }

    let mut remaining_gates = VecDeque::from_iter(gates.iter());
    process_logic_system2(
        &mut remaining_gates,
        &mut known_wires,
        result_number_of_bits,
    )
}

fn process_logic_system2<'a>(
    unprocessed_gates: &mut VecDeque<&'a Gate<'a>>,
    known_wires: &mut HashMap<&'a str, Bit>,
    result_number_of_bits: usize,
) -> Option<u64> {
    let mut z_result: u64 = 0;
    let mut z_count = 0;
    let mut breaker = 0;
    while let Some(gate) = unprocessed_gates.pop_front() {
        if let Some(value) = gate.process_strict(known_wires) {
            if gate.is_z {
                let offset = gate.output_id;
                if offset < result_number_of_bits {
                    z_result += (value as u64) << offset;
                }
                z_count += 1;
                if z_count == result_number_of_bits {
                    break;
                }
            } else {
                known_wires.insert(gate.output, value);
            }

            breaker = 0;
        } else {
            unprocessed_gates.push_back(gate);
            breaker += 1;
        }

        if breaker >= unprocessed_gates.len() {
            return None;
        }
    }
    Some(z_result)
}

fn parse_to_usize(digits: &str) -> usize {
    let mut result = 0;
    for c in digits.chars() {
        result *= 10;
        result += c as usize - 48; //48 = '0'
    }
    result
}

fn read_value_from_wires(wires: &HashMap<&str, Bit>, prefix: &str) -> u64 {
    let mut letter_gates = wires
        .iter()
        .filter(|w| w.0.starts_with(prefix))
        .collect::<Vec<_>>();
    letter_gates.sort_by(|a, b| b.0.cmp(&a.0));
    let mut result = 0;
    for (_, &value) in letter_gates {
        result <<= 1;
        result += value as u64;
    }
    result
}

fn write_value_to_wires(char: char, value: u64, number_of_bits: usize) -> HashMap<String, Bit> {
    let mut wires = HashMap::new();

    let mut value = value;
    for i in 0..number_of_bits + 1 {
        let bit = (value % 2) as Bit;
        value /= 2;

        let key = format!("{char}{:02}", i);
        wires.insert(key, bit);
    }

    wires
}
