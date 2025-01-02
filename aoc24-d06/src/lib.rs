pub mod lib_baseline;

use std::collections::HashSet;
use std::str::FromStr;
use crate::WalkOutcome::{InALoop, LeftMappedArea, Obstructed};

#[inline]
pub fn solve(input: &str) -> (usize, usize) {
    let start_map = Map::from_str(input).unwrap();
    let mut part1_map = start_map.clone();
    let (part1, visited) = solve_part1(&mut part1_map);
    let part2 = solve_part2(&start_map, visited);
    (part1, part2)
}

fn solve_part1(map: &mut Map) -> (usize, HashSet<Point>)  {
    _ = map.do_guard_walking();
    let unique : HashSet<Point> = HashSet::from_iter(map.visited.iter().map(|(p,_)| *p));
    (unique.len() + 1, unique) //plus 1 for the initial guard position
}

fn solve_part2(map: &Map, visited : HashSet<Point>) -> usize {

    let mut part2 = 0;
    for new_obstruction in visited.iter() {
        let mut map_with_obstruction = map.clone();
        map_with_obstruction.obstructions.insert(new_obstruction.clone());
        match map_with_obstruction.do_guard_walking() {
            InALoop => {
                part2 += 1;
            }
            _ => {}
        }
    }
    part2
}

#[derive(Clone)]
struct Map {
    obstructions: HashSet<Point>,
    width: u8,
    height: u8,
    guard: Guard,
    visited: HashSet<(Point, Direction)>,
}

impl Map {
    fn do_guard_walking(&mut self) -> WalkOutcome {
        loop {
            let outcome = self.walk_straight();
            match outcome {
                Obstructed => {
                    self.guard.rotate();
                }
                _ => {
                    return outcome;
                }
            }
        }
    }

    fn walk_straight(&mut self) -> WalkOutcome {
        loop {
            let next_location = self.guard.location.apply(self.guard.direction);
            if self.obstructions.contains(&next_location) {
                return Obstructed;
            } else if self.is_out_of_bounds(&next_location) {
                return LeftMappedArea;
            } else if self.was_visited_before(&next_location, &self.guard.direction) {
                return InALoop;
            }
            self.guard.move_to(next_location);

            self.visited.insert((next_location, self.guard.direction));
        }
    }

    fn was_visited_before(&self, location: &Point, direction: &Direction) -> bool {
        let find = (*location, *direction);
        self.visited.contains(&find)
    }

    fn is_out_of_bounds(&self, point: &Point) -> bool {
        point.x >= self.width || point.y >= self.height
    }
}

enum WalkOutcome {
    Obstructed,
    LeftMappedArea,
    InALoop,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines();
        let mut width = 0u8;
        let mut obstructions = HashSet::with_capacity(2000);
        let mut y = 0u8;
        let mut guard = Guard {
            location: Point { x: 255, y: 255 },
            direction: NORTH,
        };
        for line in lines {
            width = line.len() as u8;
            let mut x = 0u8;
            for c in line.chars() {
                match c {
                    '#' => _ = obstructions.insert(Point { x, y }),
                    '^' => {
                        guard = Guard {
                            location: Point { x, y },
                            direction: NORTH,
                        }
                    }
                    _ => {}
                }
                x += 1;
            }
            y += 1;
        }
        let visited = HashSet::with_capacity(2000);
        Ok(Map {
            obstructions,
            width,
            height: y,
            guard,
            visited,
        })
    }
}

#[derive(Clone)]
struct Guard {

    location: Point,
    direction: Direction,
}

impl Guard {
    fn move_to(&mut self, location: Point) {
        self.location = location;
    }

    fn rotate(&mut self) {
        self.direction = (self.direction + 1) % 4;
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn apply(&self, direction: Direction) -> Point {
        match direction {
            NORTH => Point { x: self.x, y: self.y - 1 },
            SOUTH => Point { x: self.x, y: self.y + 1 },
            WEST => Point { x: self.x - 1, y: self.y },
            EAST => Point { x: self.x + 1, y: self.y },
            _ => panic!("invalid direction"),
        }
    }
}

type Direction = u8;
const NORTH: Direction = 0;
const EAST: Direction = 1;
const SOUTH: Direction = 2;
const WEST: Direction = 3;

