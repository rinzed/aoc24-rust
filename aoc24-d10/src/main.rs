use std::collections::HashSet;
use std::fs::read_to_string;
use std::str::FromStr;

const DAY: u8 = 10;

fn main() {
    println!(
        "\n/* {:40} */ \n/* Day {:02}: {:32} */",
        "Advent of Code 2024", DAY, "Hoof It"
    );

    let data = read_to_string("input.txt").unwrap();
    let start = std::time::Instant::now();
    let (part1, part2) = solve(&data);
    let time = start.elapsed();

    println!("Total trail score, based on reachable peeks (Part 1): {part1}");
    println!("Total trail score, based on number of distinct trails to peeks (Part 2): {part2}");

    let ns = time.as_nanos();
    let version = rustc_version::version().unwrap();
    let lines = read_to_string("src/main.rs").unwrap().lines().count();
    let os_arch = format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH);
    println!("\n| Day {DAY}-base | \u{1F980} Rust {version} | \u{23F1}\u{FE0F} {time:?} ({ns} ns) | \u{1F4DC} {lines} lines | \u{2699}\u{FE0F} {os_arch} |");
}

pub fn solve(input: &str) -> (usize, u16) {
    let map = Map::from_str(input).expect("Unable to parse input");
    map.find_trails_and_calculate_score()
}

struct Map {
    rows: usize,
    cols: usize,
    data: Vec<u8>,
    trailheads: Vec<Point>,
}

impl Map {
    fn get(&self, x: i8, y: i8) -> Option<&u8> {
        if x > -1 && y > -1 {
            let x = x as usize;
            let y = y as usize;
            if y < self.rows && x < self.cols {
                let index = y * self.cols + x;
                return Some(&self.data[index]);
            }
        }
        None
    }

    pub fn find_trails_and_calculate_score(self: &Self) -> (usize, u16) {
        let mut found_trails = HashSet::new();
        let mut part2 = 0;
        for trailhead in &self.trailheads {
            part2 += self.find_paths_up(&trailhead, 0, &trailhead, &mut found_trails);
        }

        (found_trails.len(), part2)
    }

    fn find_paths_up(
        self: &Self,
        current: &Point,
        height: u8,
        trailhead: &Point,
        found_trails: &mut HashSet<(Point, Point)>,
    ) -> u16 {
        if height == 9 {
            // collect all combinations of trailhead and peeks for part 1:
            found_trails.insert((trailhead.clone(), current.clone()));
            // found a new path to a peek, so return 1 for part 2:
            return 1;
        }

        let mut result = 0;

        let height = height + 1;
        let up = Point {
            x: current.x,
            y: current.y - 1,
        };
        let down = Point {
            x: current.x,
            y: current.y + 1,
        };
        let left = Point {
            x: current.x - 1,
            y: current.y,
        };
        let right = Point {
            x: current.x + 1,
            y: current.y,
        };
        let directions = [up, down, left, right];

        for direction in directions.iter() {
            let current = self.get(direction.x, direction.y);
            if let Some(found_height) = current {
                if found_height == &height {
                    result += self.find_paths_up(&direction, height, trailhead, found_trails);
                }
            }
        }

        result
    }
}

impl FromStr for Map {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let total = input.len();
        let lines = input.lines();
        let mut data = Vec::with_capacity(total);
        let mut trailheads = Vec::with_capacity(total);
        for (y, line) in lines.into_iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if let Some(height) = ch.to_digit(10) {
                    data.push(height as u8);
                    if height == 0 {
                        trailheads.push(Point {
                            x: x as i8,
                            y: y as i8,
                        })
                    }
                }
            }
        }

        let height = input.lines().count();
        let width = input.find('\r').ok_or(())?;
        Ok(Map {
            data,
            rows: height,
            cols: width,
            trailheads,
        })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Point {
    x: i8,
    y: i8,
}
