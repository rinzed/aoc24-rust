use std::fs::read_to_string;

fn main() {
    let day = 7;
    println!(
        "\n/* {:40} */ \n/* Day {:02}: {:32} */",
        "Advent of Code 2024", day, "Bridge Repair"
    );
    let data = read_to_string("input.txt").unwrap();

    let start = std::time::Instant::now();
    let (part1, part2) = solve(&data);
    let time = start.elapsed();

    println!("Total calibration result for + and *      (Part 1): {part1}");
    println!("Total calibration result for +, * and ||  (Part 2): {part2}");

    let ns = time.as_nanos();
    let version = rustc_version::version().unwrap();
    let lines = read_to_string("src/main.rs").unwrap().lines().count();
    let os_arch = format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH);
    println!("\n| Day {day} | \u{1F980} Rust {version} | \u{23F1}\u{FE0F} {time:?} ({ns} ns) | \u{1F4DC} {lines} lines | \u{2699}\u{FE0F} {os_arch} |");
}

fn solve(input: &str) -> (u64, u64) {
    // Parse to raw equations, these will be the owners of the data.
    let raw_equations = Equation::from_string(input);
    let equations = raw_equations
        .iter()
        .map(|raw| Equation::from_raw(raw))
        .collect();
    let (part1, invalid_eq) = get_total_calibration_result_and_invalid(equations, false);
    let (part2, _) = get_total_calibration_result_and_invalid(invalid_eq, true);
    (part1, part1 + part2)
}

fn get_total_calibration_result_and_invalid(
    equations: Vec<Equation>,
    enable_concat: bool,
) -> (u64, Vec<Equation>) {
    let mut invalid = Vec::with_capacity(equations.len());
    let mut sum = 0;
    equations
        .into_iter()
        .for_each(|equation| match equation.can_be_true(enable_concat) {
            true => sum += equation.result,
            false => invalid.push(equation),
        });
    (sum, invalid)
}

struct RawEquation {
    result: u64,
    all_values: Vec<u64>,
}

struct Equation<'a> {
    result: u64,
    first_value: u64,
    other_values: &'a [u64],
}

impl Equation<'_> {
    fn from_string(input: &str) -> Vec<RawEquation> {
        let mut result = Vec::new();
        for line in input.lines() {
            let parts = line.split(": ").collect::<Vec<&str>>();
            let sum = parts[0].parse::<u64>().unwrap();
            let values: Vec<u64> = parts[1]
                .split(' ')
                .map(|v| v.parse::<u64>().unwrap())
                .collect();
            let parsed = RawEquation {
                result: sum,
                all_values: values,
            };
            result.push(parsed);
        }
        result
    }

    fn can_be_true(self: &Self, enable_concat: bool) -> bool {

        if self.other_values.len() == 0 {
            return self.result == self.first_value;
        }
        let &first = &self.first_value;
        let &second = &self.other_values[0];
        // for performance: stop when result is already bigger than first or second item
        if self.result < first || self.result < second {
            return false;
        }

        let next_other_values = &self.other_values[1..];

        let next_first = first + second;
        let next_eq = Equation::new(self.result, next_first, next_other_values);
        if next_eq.can_be_true(enable_concat) {
            return true;
        }

        let next_first = first * second;
        let next_eq = Equation::new(self.result, next_first, next_other_values);
        if next_eq.can_be_true(enable_concat) {
            return true;
        }

        if enable_concat {
            let power = 10u64.pow(number_of_digits(second));
            let next_first = (first * power) + second;
            let next_eq = Equation::new(self.result, next_first, next_other_values);
            if next_eq.can_be_true(enable_concat) {
                return true;
            }
        }

        false
    }

    fn from_raw(raw: &RawEquation) -> Equation {
        let other_values = &raw.all_values[1..];
        Equation {
            result: raw.result,
            first_value: raw.all_values[0],
            other_values,
        }
    }

    fn new(result: u64, first_value: u64, other_values: &[u64]) -> Equation {
        Equation {
            result,
            first_value,
            other_values,
        }
    }
}

fn number_of_digits(number: u64) -> u32 {
    if number == 0 {
        1
    } else {
        // mathematical approach for some more speed
        (number as f64).log10().floor() as u32 + 1
    }
}
