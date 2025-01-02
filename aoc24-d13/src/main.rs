use aoc24_tools::*;
use std::f64::consts::PI;
use std::fs::read_to_string;

const DAY: u8 = 13;
const TEN_TRILLION: u64 = 10_000_000_000_000;

fn main() {
    init_measurements!();
    print_header(DAY, "Claw Contraption");

    let data = read_to_string("input.txt").unwrap();
    let (part1, part2) = measure_total!({ solve(&data) });

    println!("Tokens to win all (Part 1): {part1}");
    println!("Tokens to win all + 10,000,000,000,000 (Part 2): {part2}");

    print_summary(DAY);
}

fn solve(input: &str) -> (u64, u64) {
    let mut machines = measure!({ ClawMachine::from_string(input) }, "1-parsing");
    let part1 = measure!({ calculate_tokens_using_iteration(&machines) }, "1-iterate");
    measure!(
        { ClawMachine::add_to_prices(&mut machines, TEN_TRILLION) },
        "2-clone"
    );
    let part2 = measure!(
        { calculate_tokens_using_trigonometry(&machines) },
        "2-trigono."
    );
    (part1, part2)
}

fn calculate_tokens_using_trigonometry(machines: &Vec<ClawMachine>) -> u64 {
    let mut result = 0;
    for machine in machines {
        result += machine
            .calculate_tokens_to_win_using_trigonometry()
            .unwrap_or(0)
    }
    result
}

fn calculate_tokens_using_iteration(machines: &Vec<ClawMachine>) -> u64 {
    let mut result = 0;
    for machine in machines {
        result += machine
            .calculate_tokens_to_win_using_iteration()
            .unwrap_or(0)
    }
    result
}

struct ClawMachine {
    button_a: Button,
    button_b: Button,
    prize: Point,
}

struct Button {
    movement: Point,
    tokens: u64,
}

struct Point {
    x: u64,
    y: u64,
}

impl ClawMachine {
    fn calculate_tokens_to_win_using_iteration(self: &Self) -> Option<u64> {
        // just loop over all options until we find one
        // the good thing is that there is always only one solution to the problem
        for press_a in 0..100 {
            for press_b in 0..100 {
                let result = self.gamble_on(&press_a, &press_b);
                if result.is_some() {
                    return result;
                }
            }
        }
        None
    }

    fn gamble_on(&self, press_a: &u64, press_b: &u64) -> Option<u64> {
        let result_x = press_a * self.button_a.movement.x + press_b * self.button_b.movement.x;
        if result_x == self.prize.x {
            let result_y = press_a * self.button_a.movement.y + press_b * self.button_b.movement.y;
            if result_y == self.prize.y {
                return Some(press_a * self.button_a.tokens + press_b * self.button_b.tokens);
            }
        }
        None
    }

    fn calculate_tokens_to_win_using_trigonometry(self: &Self) -> Option<u64> {
        // calculate the angle's of the vectors based on the x-axis (1,0)
        let angle_prize = self.prize.angle();
        let angle_a = self.button_a.movement.angle();
        let angle_b = self.button_b.movement.angle();

        // check if the vector to the prize is between the movement of buttons A & B, if not,
        // the machine is rigged to never win.
        if (angle_prize > angle_a && angle_prize > angle_b)
            || (angle_prize < angle_a && angle_prize < angle_b)
        {
            return None;
        }

        // the vector's should form a triangle, so let's calculate the angles of each of the corners of the triangle
        let corner_prize_a = (angle_prize - angle_a).abs();
        let corner_prize_b = (angle_prize - angle_b).abs();
        let corner_a_b = PI - corner_prize_a - corner_prize_b;

        // apply the law of sines to find the required distances to move in the directions of A & B to arrive complete the triangle
        let distance_to_prize = self.prize.distance();
        let radius_x2 = distance_to_prize / corner_a_b.sin();
        let distance_for_a = radius_x2 * corner_prize_b.sin();
        let distance_for_b = radius_x2 * corner_prize_a.sin();

        // divide to find the number of button presses needed
        let presses_a = distance_for_a / self.button_a.movement.distance();
        let presses_b = distance_for_b / self.button_b.movement.distance();

        // only complete presses count, there might be some rounding issues, so ignore those
        let presses_a_rounded = presses_a.round();
        let presses_b_rounded = presses_b.round();
        if (presses_a_rounded - presses_a).abs() > 0.001
            || (presses_b_rounded - presses_b).abs() > 0.001
        {
            return None;
        }

        // to be really sure, we could use the gamble-method from part 1 to check, but I got the right answer without :)
        Some(
            presses_a_rounded as u64 * self.button_a.tokens
                + presses_b_rounded as u64 * self.button_b.tokens,
        )
    }

    fn from_string(input: &str) -> Vec<ClawMachine> {
        let mut result: Vec<ClawMachine> = Vec::with_capacity(500);

        let mut button_a = "";
        let mut button_b = "";
        for (index, line) in input.lines().enumerate() {
            match index % 4 {
                0 => button_a = line,
                1 => button_b = line,
                2 => {
                    let button_a = Button::from_string(button_a, 3);
                    let button_b = Button::from_string(button_b, 1);
                    let prize = Point::from_prize_string(line);
                    result.push(ClawMachine {
                        button_a,
                        button_b,
                        prize,
                    })
                }
                _ => {}
            }
        }

        result
    }

    fn add_to_prices(machines: &mut Vec<ClawMachine>, prize_increment: u64) {
        for machine in machines {
            machine.add_to_prize(prize_increment);
        }
    }

    fn add_to_prize(&mut self, prize_increment: u64) {
        self.prize.x += prize_increment;
        self.prize.y += prize_increment;
    }
}

impl Point {
    fn new(x: u64, y: u64) -> Point {
        Point { x, y }
    }

    fn from_prize_string(prize_line: &str) -> Point {
        let split_index = prize_line.find(',').unwrap();
        let x = prize_line[9..split_index].parse::<u64>().unwrap();
        let y = prize_line[split_index + 4..].parse::<u64>().unwrap();
        Point::new(x, y)
    }

    fn angle(&self) -> f64 {
        (self.y as f64 / self.x as f64).atan()
    }

    fn distance(&self) -> f64 {
        // apply Pythagorean theorem
        let x = self.x as u128;
        let y = self.y as u128;
        ((x * x + y * y) as f64).sqrt()
    }
}

impl Button {
    fn from_string(line: &str, tokens: u64) -> Button {
        let split_index = line.find(',').unwrap();
        let x = line[12..split_index].parse().unwrap();
        let y = line[split_index + 4..].parse().unwrap();
        Button {
            movement: Point { x, y },
            tokens,
        }
    }
}
