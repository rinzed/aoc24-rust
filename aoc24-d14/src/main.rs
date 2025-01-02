use aoc24_tools::*;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

const DAY: u8 = 14;

fn main() {
    init_measurements!();
    print_header(DAY, "Restroom Redoubt");

    let data = read_to_string("input.txt").unwrap();
    let (part1, part2, robots) = measure_total!({ solve(&data) });
    print_robots(&robots);
    println!("Safety factor after 100s (Part 1): {part1}");
    println!("Seconds to find Easter Egg (Part 2): {part2}");

    print_summary(DAY);
}

fn solve(input: &str) -> (u32, u16, Vec<Robot>) {
    let mut robots = measure!({ parse(input) }, "parse");
    let part1 = measure!({ calculate_safety_factor(&robots, 100) }, "100s");
    let part2 = measure!({ find_easter_egg(&mut robots) }, "easter egg");
    (part1, part2, robots)
}

const SPACE_WIDTH: i16 = 101;
const SPACE_HEIGHT: i16 = 103;
const HALF_WIDTH: i16 = SPACE_WIDTH / 2;
const HALF_HEIGHT: i16 = SPACE_HEIGHT / 2;

fn calculate_safety_factor(robots: &Vec<Robot>, seconds: i16) -> u32 {
    let mut quadrant_count = [0u32, 0, 0, 0];
    for robot in robots {
        let px = (robot.location.x + robot.vector.x * seconds) % SPACE_WIDTH;
        let py = (robot.location.y + robot.vector.y * seconds) % SPACE_HEIGHT;

        if px < HALF_WIDTH && py < HALF_HEIGHT {
            quadrant_count[0] += 1
        } else if px < HALF_WIDTH && py > HALF_HEIGHT {
            quadrant_count[1] += 1
        } else if px > HALF_WIDTH && py < HALF_HEIGHT {
            quadrant_count[2] += 1
        } else if px > HALF_WIDTH && py > HALF_HEIGHT {
            quadrant_count[3] += 1
        }
    }
    quadrant_count[0] * quadrant_count[1] * quadrant_count[2] * quadrant_count[3]
}

fn find_easter_egg(robots: &mut Vec<Robot>) -> u16 {
    let mut seconds = 0;
    loop {
        let mut map = HashSet::with_capacity(robots.len());
        let mut is_unique = true;
        for i in 0..robots.len() {
            let robot = &mut robots[i];

            robot.location.x = (robot.location.x + robot.vector.x) % SPACE_WIDTH;
            robot.location.y = (robot.location.y + robot.vector.y) % SPACE_HEIGHT;

            is_unique &= map.insert((robot.location.x, robot.location.y));
        }
        seconds += 1;

        // Shortcut: works most of the time: when no robot's overlap, the form a Christ mass tree...
        if is_unique {
            break;
        }
    }

    seconds
}

fn parse(input: &str) -> Vec<Robot> {
    let mut robots = vec![];
    for line in input.lines() {
        let i_comma1 = line.find(',').unwrap();
        let i_space = line.find(' ').unwrap();
        let i_comma2 = line[i_space..].find(',').unwrap() + i_space;

        let px = line[2..i_comma1].parse::<i16>().unwrap();
        let py = line[i_comma1 + 1..i_space].parse::<i16>().unwrap();
        let vx = line[i_space + 3..i_comma2].parse::<i16>().unwrap() + SPACE_WIDTH;
        let vy = line[i_comma2 + 1..].parse::<i16>().unwrap() + SPACE_HEIGHT;

        robots.push(Robot {
            location: Point { x: px, y: py },
            vector: Point { x: vx, y: vy },
        });
    }

    robots
}

fn print_robots(robots: &Vec<Robot>) {
    let mut hashmap = HashMap::new();
    for robot in robots {
        hashmap
            .entry(&robot.location)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }
    for y in 0..SPACE_HEIGHT {
        for x in 0..SPACE_WIDTH {
            let p = Point { x, y };
            if hashmap.contains_key(&p) {
                let value = hashmap.get(&p).unwrap().to_string();
                print!("\x1B[32m{value:.1}\x1B[0m"); //use ANSI_escape_code to make it green
            } else {
                print!("\x1B[2m.\x1B[0m"); //use ANSI_escape_code to dim
            }
        }
        println!();
    }
    println!();
}

struct Robot {
    location: Point,
    vector: Point,
}

#[derive(Eq, PartialEq, Hash)]
struct Point {
    x: i16,
    y: i16,
}
