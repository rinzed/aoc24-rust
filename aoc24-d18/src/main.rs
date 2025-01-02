use aoc24_tools::*;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;

const DAY: u8 = 18;

fn main() {
    init_measurements!();
    print_header(DAY, "RAM Run");

    let data = read_to_string("input.txt").unwrap();
    let (part1, part2) = measure_total!({ solve(&data, 1024, 70) });

    println!("Minimum number of steps to exit (Part 1): {part1}");
    println!("First byte to prevent reaching the exit (Part 2): {part2}");

    print_summary(DAY);
}

fn solve(input: &str, number_of_bytes: usize, size: usize) -> (usize, Point) {
    let start = Point::new(0, 0);
    let end = Point::new(size as i8, size as i8);
    let (mut memory_space, next_bytes) = MemorySpace::parse(input, number_of_bytes, size);
    let (part1, safe_path) = memory_space.find_path(&start, &end).unwrap();
    let part2 = memory_space.find_blockade(safe_path, &start, &end, next_bytes).unwrap();
    (part1, part2)
}

struct MemorySpace {
    corrupted: HashSet<Point>,
    width: usize,
    height: usize,
}

impl MemorySpace {
    fn parse(input: &str, bytes: usize, size: usize) -> (MemorySpace, Vec<Point>) {
        let mut remaining_bytes = Vec::new();
        let mut corrupted = HashSet::new();
        for (i, line) in input.lines().enumerate() {
            let parts = line.split(',').collect::<Vec<&str>>();
            let x = parts[0].parse::<i8>().unwrap();
            let y = parts[1].parse::<i8>().unwrap();
            if i < bytes {
                corrupted.insert(Point::new(x, y));
            } else {
                remaining_bytes.push(Point::new(x, y));
            }
        }
        (
            MemorySpace {
                corrupted,
                height: size + 1,
                width: size + 1,
            },
            remaining_bytes,
        )
    }

    fn find_blockade(
        &mut self,
        safe_path: Vec<Point>,
        start: &Point,
        end: &Point,
        next_bytes: Vec<Point>,
    ) -> Option<Point> {
        let mut safe_path : HashSet<_> = safe_path.into_iter().collect();
        for next_corruption in next_bytes {
            let path_blocked = safe_path.contains(&next_corruption);
            self.corrupted.insert(next_corruption.clone());
            if path_blocked {
                // find a new path
                if let Some((_, path)) = self.find_path(start, end) {
                    safe_path = path.into_iter().collect();
                } else {
                    // no path found, so we found the blockade
                    self.print_with_path_and_block(&safe_path, Some(&next_corruption));
                    return Some(next_corruption);
                }
            }
        }
        None //path is never blocked!
    }

    fn print_with_path_and_block(&self, path: &HashSet<Point>, block: Option<&Point>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let loc = Point::new(x as i8, y as i8);
                if Some(&loc) == block {
                    print!("\x1B[31mX\x1B[0m") //red
                } else {
                    match self.corrupted.contains(&loc) {
                        true => print!("\x1B[32m#\x1B[0m"), //green
                        false => {
                            if path.contains(&loc) {
                                print!("\x1B[33mO\x1B[0m") //yellow
                            } else {
                                print!(".")
                            }
                        }
                    }
                }
            }
            println!();
        }
        println!();
    }

    fn find_path(&self, start: &Point, end: &Point) -> Option<(usize, Vec<Point>)> {
        // using Dijkstra's algorithm with a BinaryHeap as a priority queue
        let starting_step = Step {
            node: start.clone(),
            distance: 0,
            path: Vec::new(),
        };
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::from([starting_step]);
        while let Some(current) = queue.pop() {
            if &current.node == end {
                let mut path: Vec<Point> = current.path.clone();
                path.push(current.node);
                return Some((current.distance, path));
            }
            if !visited.contains(&current.node) {
                visited.insert(current.node.clone());
                for neighbour in self.get_neighbours(current) {
                    queue.push(neighbour);
                }
            }
        }
        None
    }

    fn get_neighbours(&self, current: Step) -> Vec<Step> {
        // build the path, that's handy for part 2 (again)
        let mut new_path = Vec::from(current.path);
        new_path.push(current.node.clone());

        let distance = current.distance + 1;

        let neighbours = [
            Step {
                distance,
                node: current.node.go(NORTH),
                path: new_path.clone(),
            },
            Step {
                distance,
                node: current.node.go(WEST),
                path: new_path.clone(),
            },
            Step {
                distance,
                node: current.node.go(SOUTH),
                path: new_path.clone(),
            },
            Step {
                distance,
                node: current.node.go(EAST),
                path: new_path,
            },
        ];
        // filter out options that are out of bounds (None) or corrupted (Some(false)
        neighbours
            .into_iter()
            .filter(|s| self.is_safe(&s.node) == Some(true))
            .collect::<Vec<Step>>()
    }

    fn is_safe(&self, p: &Point) -> Option<bool> {
        if p.x > -1 && p.y > -1 && (p.x as usize) < self.width && (p.y as usize) < self.height {
            Some(!self.corrupted.contains(&p))
        } else {
            None
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Point {
    x: i8,
    y: i8,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{},{}", self.x, self.y))
    }
}

impl Point {
    fn new(x: i8, y: i8) -> Point {
        Point { x, y }
    }

    fn go(&self, direction: Direction) -> Point {
        match direction {
            NORTH => Point {
                x: self.x,
                y: self.y - 1,
            },
            EAST => Point {
                x: self.x + 1,
                y: self.y,
            },
            SOUTH => Point {
                x: self.x,
                y: self.y + 1,
            },
            WEST => Point {
                x: self.x - 1,
                y: self.y,
            },
            _ => panic!("invalid direction"),
        }
    }
}

type Direction = u8;
const NORTH: Direction = 0;
const EAST: Direction = 1;
const SOUTH: Direction = 2;
const WEST: Direction = 3;

#[derive(Eq, PartialEq)]
struct Step { //borrowed from a couple of days ago
    node: Point,
    distance: usize,
    path: Vec<Point>,
}

// implement Ord & PartialOrd to make the BinaryHeap sort the shortest distance to the front.
impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}
impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
