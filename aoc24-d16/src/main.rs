use aoc24_tools::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::read_to_string;
use std::usize;

const DAY: u8 = 16;

fn main() {
    init_measurements!();
    print_header(DAY, "Reindeer Maze");

    let file = "input.txt";
    let data = read_to_string(file).unwrap();
    let (part1, part2) = measure_total!({ solve(&data) });

    println!("What is the lowest score a Reindeer could possibly get? (Part 1): {part1}");
    println!("How many tiles are part of at least one of the best paths through the maze? (Part 2): {part2}");

    print_summary(DAY);
}

fn solve(input: &str) -> (usize, usize) {
    let (maze, start, end) = Maze::parse(input);

    let (part1, _) = measure!(
        { find_lowest_score(&maze, start.clone(), &end).unwrap() },
        "part 1"
    );
    let part2_path = measure!(
        { find_all_tiles_on_the_best_paths(&maze, start, end).unwrap() },
        "part 2"
    );
    let part2 = part2_path.len();

    (part1, part2)
}

fn find_lowest_score(maze: &Maze, start: Point, end: &Point) -> Option<(usize, Vec<Point>)> {
    // using Dijkstra's algorithm with a BinaryHeap as a priority queue
    let starting_step = Step {
        node: start,
        direction: EAST,
        distance: 0,
        path: Vec::new(),
    };
    let mut visited = HashMap::new();
    let mut queue = BinaryHeap::from([starting_step]);
    while let Some(current) = queue.pop() {
        if &current.node == end {
            let mut path: Vec<Point> = current.path.clone();
            path.push(current.node);
            return Some((current.distance, path));
        }
        if !visited.contains_key(&current.node) {
            visited.insert(current.node.clone(), current.distance);
            for neighbour in get_neighbours(current, maze) {
                queue.push(neighbour);
            }
        }
    }
    None
}

fn find_all_tiles_on_the_best_paths(
    maze: &Maze,
    start: Point,
    end: Point,
) -> Option<HashSet<Point>> {
    let starting_step = Step {
        node: start,
        distance: 0,
        direction: EAST,
        path: Vec::new(),
    };

    let mut visited: HashMap<(Point, Direction), usize> = HashMap::new();
    let mut queue = BinaryHeap::from([starting_step]);

    let mut shortest = u32::MAX as usize;
    let mut on_the_best_paths: HashSet<Point> = HashSet::new();

    while let Some(current) = queue.pop() {
        if current.node == end {
            // found a path to the end, now remember all steps that were part of it.
            // the hashset will help with deduplication.
            shortest = current.distance;
            on_the_best_paths.insert(current.node);
            for point in current.path {
                on_the_best_paths.insert(point);
            }
            continue;
        } else if current.distance > shortest {
            // stop, because we are now processing paths longer than the shortest.
            return Some(on_the_best_paths);
        }

        // check if we have been to this location, using that direction.
        // we might have found an alternative route to this state, which is fine.
        // to keep the algorithm simple: just continue resolving it.
        let key = (current.node.clone(), current.direction);
        let visited_distance = visited.entry(key).or_insert(current.distance);
        // when the new route is worse, no need to continue down this path:
        if *visited_distance >= current.distance {
            for neighbour in get_neighbours(current, maze) {
                queue.push(neighbour);
            }
        }
    }
    None
}

fn get_neighbours(current: Step, maze: &Maze) -> Vec<Step> {
    // build the path, that's handy for part 2.
    let mut new_path = Vec::from(current.path);
    new_path.push(current.node.clone());

    let direction_cw = rotate(&current.direction, 1);
    let direction_ccw = rotate(&current.direction, 3);
    let neighbours = [
        // options are: move in current direction
        Step {
            distance: current.distance + 1,
            direction: current.direction,
            node: current.node.apply(current.direction),
            path: new_path.clone(),
        },
        // move after rotating once
        Step {
            distance: current.distance + 1001,
            direction: direction_cw,
            node: current.node.apply(direction_cw),
            path: new_path.clone(),
        },
        Step {
            distance: current.distance + 1001,
            direction: direction_ccw,
            node: current.node.apply(direction_ccw),
            path: new_path,
        },
    ];
    // filter out options that collide with a wall
    neighbours
        .into_iter()
        .filter(|s| !maze.walls.contains(&s.node))
        .collect::<Vec<Step>>()
}

#[derive(Debug, Eq, PartialEq)]
struct Step {
    distance: usize,
    node: Point,
    direction: Direction,
    // path is handy for printing the path at the end and is needed for part 2
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

struct Maze {
    walls: HashSet<Point>,
}

impl Maze {
    fn parse(input: &str) -> (Self, Point, Point) {
        let lines = input.lines().into_iter();
        let mut walls = HashSet::new();
        let mut start = Point::new(0, 0);
        let mut end = Point::new(0, 0);
        let mut y = 0;

        for line in lines {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    walls.insert(Point::new(x as u8, y));
                } else if c == 'S' {
                    start = Point::new(x as u8, y);
                } else if c == 'E' {
                    end = Point::new(x as u8, y);
                }
            }
            y += 1;
        }

        (
            Maze {
                walls,
            },
            start,
            end,
        )
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new(x: u8, y: u8) -> Point {
        Point { x, y }
    }

    fn apply(&self, direction: Direction) -> Point {
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

fn rotate(direction: &Direction, steps: u8) -> Direction {
    (direction + steps) % 4
}

type Direction = u8;
const NORTH: Direction = 0;
const EAST: Direction = 1;
const SOUTH: Direction = 2;
const WEST: Direction = 3;
