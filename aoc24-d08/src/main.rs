use aoc24_tools::*;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::ops::{Add, Sub};

const DAY: u8 = 8;

fn main() {
    init_measurements!();
    print_header(DAY, "Resonant Collinearity");

    let data = read_to_string("input.txt").unwrap();

    let (part1, part2) = measure_total!({ solve(&data) });

    println!("Number of unique antinode locations:");
    println!("- when in line & exactly twice as far as another (Part 1): {part1}");
    println!("- when in line at any distance (Part 2): {part2}");

    print_summary(DAY);
}

fn solve(input: &str) -> (usize, usize) {
    let map = measure!({ AntennaMap::from_string(input) }, "parsing");
    let part1 = measure!({ map.count_antinodes_at_twice_distance() }, "part1");
    let part2 = measure!({ map.count_antinodes_at_any_distance() }, "part2");

    (part1, part2)
}

struct AntennaMap {
    antenna_sets: Vec<Vec<Point>>,
    width: i16,
    height: i16,
}

impl AntennaMap {
    fn from_string(input: &str) -> AntennaMap {
        let mut antenna_sets = HashMap::with_capacity(input.len());
        let lines = input.lines();
        let mut x = 0;
        let mut y = 0;
        for line in lines {
            x = 0;
            for c in line.chars() {
                if c != '.' {
                    antenna_sets
                        .entry(c)
                        .or_insert_with(Vec::new)
                        .push(Point { x, y });
                }
                x += 1;
            }
            y += 1;
        }
        AntennaMap {
            // clean-up names of the antenna's, because they were only needed during parsing
            antenna_sets: antenna_sets.into_iter().map(|(_, set)| set).collect(),
            width: x,
            height: y,
        }
    }

    fn count_antinodes_at_twice_distance(self: &Self) -> usize {
        let mut unique_antinodes = HashSet::with_capacity(self.antenna_sets.len());
        for antenna_set in self.antenna_sets.iter() {
            self.find_antinodes_at_twice_distance(antenna_set, &mut unique_antinodes);
        }
        unique_antinodes.len()
    }

    fn find_antinodes_at_twice_distance(
        self: &Self,
        antennas: &Vec<Point>,
        antinode_register: &mut HashSet<Point>,
    ) {
        let count = antennas.len();
        for index_a in 0..count {
            let antenna_a = &antennas[index_a];
            for index_b in index_a + 1..count {
                let antenna_b = &antennas[index_b];
                let distance = antenna_b - antenna_a;

                let antinode = antenna_b + &distance;
                if self.is_within_bounds(antinode) {
                    antinode_register.insert(antinode);
                }
                let antinode = antenna_a - &distance;
                if self.is_within_bounds(antinode) {
                    antinode_register.insert(antinode);
                }
            }
        }
    }

    fn count_antinodes_at_any_distance(self: &Self) -> usize {
        let mut unique_antinodes = HashSet::with_capacity(self.height as usize * self.width as usize);
        for antenna_set in self.antenna_sets.iter() {
            self.find_antinodes_at_any_distance(antenna_set, &mut unique_antinodes);
        }
        unique_antinodes.len()
    }

    fn find_antinodes_at_any_distance(
        self: &Self,
        antennas: &Vec<Point>,
        antinode_register: &mut HashSet<Point>,
    ) {
        let count = antennas.len();
        for index_a in 0..count {
            let antenna_a = &antennas[index_a];
            for index_b in index_a + 1..count {
                let antenna_b = &antennas[index_b];
                let distance = antenna_b - antenna_a; //We are in luck, we do not have to account for finding the smallest distance :)

                let mut antinode = *antenna_b;
                while self.is_within_bounds(antinode) {
                    antinode_register.insert(antinode);
                    antinode = &antinode + &distance;
                }

                let mut antinode = *antenna_a;
                while self.is_within_bounds(antinode) {
                    antinode_register.insert(antinode);
                    antinode = &antinode - &distance;
                }
            }
        }
    }

    fn is_within_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.width && point.y < self.height
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i16,
    y: i16,
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
