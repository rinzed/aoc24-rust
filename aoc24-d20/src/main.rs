use aoc24_tools::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::read_to_string;
use colored::Colorize;

const DAY: u8 = 20;

fn main() {
    init_measurements!();
    print_header(DAY, "Race Condition");

    let data = read_to_string("input.txt").unwrap();
    let (part1, part2) = measure_total!({ solve(&data) });

    println!("Number of cheats with 2 picoseconds cheat rule (Part 1): {part1}");
    println!("Number of cheats with 20 picoseconds cheat rule (Part 2): {part2}");

    print_summary(DAY);
}

fn solve(input: &str) -> (u32, u32) {
    let threshold = 100;

    let map = measure!({ Map::parse(input) }, "parse");
    // re-use Dijkstra as the path finding algorithm,
    // it's a bit overkill, because there is only one path between the walls
    // but the overhead is minimal.
    let racetrack = measure!({ map.dijkstra().unwrap() }, "dijkstra");
    map.print_path(&racetrack);

    // convert race_path to a look-up to make it easy to create an easy and fast way to check if a cheat lands on the racetrack
    // and to find out how far we are along the track after cheating
    let racetrack_map: HashMap<_, _> = racetrack
        .into_iter()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect();

    let part1 = measure!({ find_good_cheats_2ps(&racetrack_map, threshold) }, "part1");

    let part2 = measure!(
        { find_good_cheats_with(&racetrack_map, threshold, 20) },
        "part2"
    );
    (part1, part2)
}

fn find_good_cheats_2ps(racetrack_map: &HashMap<Point, usize>, threshold: usize) -> u32 {
    let cheat_time = 2;

    let mut num_of_cheats = 0;
    let race_length = racetrack_map.len();
    // look for each position if there are spots that are within skip reach:
    for (cheat_from, &time_from_start) in racetrack_map {
        if time_from_start >= race_length - threshold {
            continue; // to close to finish, no need to check for cheats
        }

        // difference between part 1 and part 2, is how simple we can make the cheat-finding method
        let cheat_destinations = get_2ps_cheat_destinations(cheat_from);
        for cheat_destination in cheat_destinations {
            // check if destination is on the racetrack
            if let Some(&destination_time_from_start) = racetrack_map.get(&cheat_destination) {
                let time_to_beat = time_from_start + cheat_time;
                // check if we improved our time by cheating
                if destination_time_from_start > time_to_beat {
                    let cheated_steps = destination_time_from_start - time_to_beat;
                    // is the cheat at least our threshold
                    if cheated_steps >= threshold {
                        num_of_cheats += 1;
                    }
                }
            }
        }
    }

    num_of_cheats
}

fn get_2ps_cheat_destinations(current: &Point) -> [Point; 4] {
    [
        Point::new(current.x - 2, current.y),
        Point::new(current.x + 2, current.y),
        Point::new(current.x, current.y - 2),
        Point::new(current.x, current.y + 2),
    ]
}

fn find_good_cheats_with(
    racetrack_map: &HashMap<Point, usize>,
    threshold: usize,
    max_cheat_time: u8,
) -> u32 {
    let mut num_of_cheats = 0;
    let race_length = racetrack_map.len();
    // look for each position if there are spots that are within skip reach:
    for (cheat_from, &time_from_start) in racetrack_map {
        if time_from_start >= race_length - threshold {
            continue; // to close to finish, no need to check for cheats
        }

        let cheat_destinations = get_cheat_destinations(cheat_from, max_cheat_time);
        for (cheat_destination, cheat_time) in cheat_destinations {
            // check if destination is on the racetrack
            if let Some(&destination_time_from_start) = racetrack_map.get(&cheat_destination) {
                let time_to_beat = time_from_start + cheat_time;
                // check if we improved our time by cheating
                if destination_time_from_start > time_to_beat {
                    let cheated_steps = destination_time_from_start - time_to_beat;
                    // is the cheat at least our threshold
                    if cheated_steps >= threshold {
                        num_of_cheats += 1;
                    }
                }
            }
        }
    }

    num_of_cheats
}

fn get_cheat_destinations(center: &Point, max_cheat_time: u8) -> Vec<(Point, usize)> {
    let max_cheat_time = max_cheat_time as i16; //convert for easy math later

    // based on the max_cheat_time we know that we have exactly X results:
    // the inner 9 of the diamond (Rhombus) can be ignored
    let mut vector: Vec<(Point, usize)> = Vec::new();

    for offset_i in 1..max_cheat_time + 1 {
        let remaining = max_cheat_time - offset_i;
        for offset_j in 0..remaining + 1 {
            let distance = (offset_i + offset_j) as usize;
            // ignore any step of less than 1 distance and the first diagonal
            if distance > 1 && !(offset_i == 1 && offset_j == 1) {
                let point_a = Point::new(center.x - offset_i, center.y - offset_j);
                let point_b = Point::new(center.x + offset_j, center.y - offset_i);
                let point_c = Point::new(center.x + offset_i, center.y + offset_j);
                let point_d = Point::new(center.x - offset_j, center.y + offset_i);
                vector.push((point_a, distance));
                vector.push((point_b, distance));
                vector.push((point_c, distance));
                vector.push((point_d, distance));
            }
        }
    }

    vector
}

struct Map {
    walls: HashSet<Point>,
    width: usize,
    height: usize,
    start: Point,
    end: Point,
}

impl Map {
    fn parse(input: &str) -> Map {
        let mut walls = HashSet::new();
        let mut start = Point { x: 0, y: 0 };
        let mut end = Point { x: 0, y: 0 };

        let mut y = 0;
        let lines = input.lines();
        for line in lines {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    walls.insert(Point::new(x as i16, y));
                } else if c == 'S' {
                    start = Point::new(x as i16, y);
                } else if c == 'E' {
                    end = Point::new(x as i16, y);
                }
            }
            y += 1;
        }

        Map {
            walls,
            height: y as usize,
            width: y as usize,
            start,
            end,
        }
    }

    fn print_path(&self, path: &[Point]) {
        let length = path.len();
        // just for fun: create a rainbow road like path
        let color_map : HashMap<_,_>= path.iter().enumerate().map(|(i, p)| (p, ((i * 255usize) / length, i % 255))).collect();

        for y in 0..self.height {
            for x in 0..self.width {
                let loc = Point::new(x as i16, y as i16);
                if self.start == loc {
                    print!("{}", "S".blue())
                } else if self.end == loc {
                    print!("{}", "E".red())
                } else {
                    match self.walls.contains(&loc) {
                        true => print!("{}", "#".bright_green()),
                        false => {
                            if let Some((color,other)) = color_map.get(&loc) {
                                print!("{}", "O".truecolor(*color as u8, *other as u8, 255-(*color as u8)))
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

    fn dijkstra(&self) -> Option<Vec<Point>> {
        let starting_step = Step {
            node: self.start.clone(),
            distance: 0,
            path: Vec::new(),
        };
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::from([starting_step]);
        while let Some(current) = queue.pop() {
            if current.node == self.end {
                let mut path: Vec<Point> = current.path.clone();
                path.push(current.node);
                return Some(path);
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
        let mut new_path = Vec::from(current.path);
        new_path.push(current.node.clone());

        let distance = current.distance + 1;

        let neighbours = [
            Step {
                distance,
                node: Point::new(current.node.x - 1, current.node.y),
                path: new_path.clone(),
            },
            Step {
                distance,
                node: Point::new(current.node.x + 1, current.node.y),
                path: new_path.clone(),
            },
            Step {
                distance,
                node: Point::new(current.node.x, current.node.y - 1),
                path: new_path.clone(),
            },
            Step {
                distance,
                node: Point::new(current.node.x, current.node.y + 1),
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
            Some(!self.walls.contains(&p))
        } else {
            None
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn new(x: i16, y: i16) -> Point {
        Point { x, y }
    }
}

#[derive(Eq, PartialEq)]
struct Step {
    //borrowed from a couple of days ago
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
