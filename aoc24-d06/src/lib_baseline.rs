use std::collections::{HashMap, HashSet};
use std::ops::Add;
use std::str::FromStr;
use crate::lib_baseline::WalkOutcome::{InALoop, LeftMappedArea, Obstructed};

#[inline]
pub fn solve(input: &str) -> (usize, usize) {
    let start_map = Map::from_str(input).unwrap();
    let mut part1_map = start_map.clone();
    let part1 = solve_part1(&mut part1_map);
    let part2 = solve_part2(&start_map, part1_map.visited);
    (part1, part2)
}

fn solve_part1(map: &mut Map) -> usize {
    _ = map.do_guard_walking();
    map.visited.len() + 1 //plus 1 for the initial guard position
}

fn solve_part2(map: &Map, visited : HashMap<Point, HashSet<DirectionIndex>>) -> usize {

    let mut part2 = 0;
    for (new_obstruction, _) in visited.iter() {
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
    width: i16,
    height: i16,
    guard: Guard,
    visited: HashMap<Point, HashSet<usize>>,
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
            let next_location = self.guard.location + self.guard.get_direction();
            if self.obstructions.contains(&next_location) {
                return Obstructed;
            } else if self.is_out_of_bounds(&next_location) {
                return LeftMappedArea;
            } else if self.was_visited_before(&next_location, &self.guard.direction_index) {
                return InALoop;
            }
            self.guard.move_to(next_location);

            self.visited
                .entry(next_location)
                .or_insert(HashSet::with_capacity(2))
                .insert(self.guard.direction_index);
        }
    }

    fn was_visited_before(&self, location: &Point, direction: &DirectionIndex) -> bool {
        match self.visited.get(&location) {
            Some(v) => v.contains(direction),
            None => false,
        }
    }

    fn is_out_of_bounds(&self, point: &Point) -> bool {
        point.x < 0 || point.y < 0 || point.x >= self.width || point.y >= self.height
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
        let mut width = 0i16;
        let mut obstructions = HashSet::new();
        let mut y = 0i16;
        let mut guard = Guard {
            location: Point { x: -1, y: -1 },
            direction_index: 0,
        };
        for line in lines {
            width = line.len() as i16;
            let mut x = 0i16;
            for c in line.chars() {
                match c {
                    '#' => _ = obstructions.insert(Point { x, y }),
                    '^' => {
                        guard = Guard {
                            location: Point { x, y },
                            direction_index: 0,
                        }
                    }
                    _ => {}
                }
                x += 1;
            }
            y += 1;
        }
        let visited = HashMap::new();
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
    direction_index: DirectionIndex,
}

type DirectionIndex = usize;
const DIRECTIONS: [Point; 4] = [UP, RIGHT, DOWN, LEFT];

impl Guard {
    fn move_to(&mut self, location: Point) {
        self.location = location;
    }

    fn rotate(&mut self) {
        self.direction_index = (self.direction_index + 1) % 4;
    }

    fn get_direction(&self) -> Point {
        DIRECTIONS[self.direction_index]
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i16,
    y: i16,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

const UP: Point = Point { x: 0, y: -1 };
const DOWN: Point = Point { x: 0, y: 1 };
const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };