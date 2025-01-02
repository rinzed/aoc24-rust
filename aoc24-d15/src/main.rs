use aoc24_tools::*;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::Lines;
use Object::*;

const DAY: u8 = 15;

fn main() {
    init_measurements!();
    print_header(DAY, "Warehouse Woes");

    let path = "input.txt";
    let data = read_to_string(path).unwrap();
    let (part1, part2, warehouse, robot) = measure_total!({ solve(&data) });

    warehouse.print(&robot);
    println!("Sum of boxes' GPS in 1st warehouse (Part 1): {part1}");
    println!("Sum of boxes' GPS in 2nd warehouse (Part 2): {part2}");

    print_summary(DAY);
}

fn solve(input: &str) -> (u32, u32, Warehouse, Point) {
    let (mut robot, mut warehouse, moves) = measure!({ parse(input) }, "parse");
    let part1 = measure!(
        { execute_robot_movements(&mut robot, &mut warehouse, &moves) },
        "1st,normal"
    );
    let (mut robot, mut warehouse) = measure!({ parse_wide(input) }, "parse_wide");
    let part2 = measure!(
        { execute_robot_movements_on_wide_map(&mut robot, &mut warehouse, &moves) },
        "2nd,wide"
    );
    (part1, part2, warehouse, robot)
}

fn execute_robot_movements(
    robot: &mut Point,
    warehouse: &mut Warehouse,
    moves: &Vec<Direction>,
) -> u32 {
    for robot_move in moves.into_iter() {
        if let Some(destination) = try_move(robot, &mut warehouse.map, robot_move) {
            robot.move_to(&destination);
        }
    }
    warehouse.calculate_gps_sum()
}

fn try_move(
    location: &Point,
    warehouse_map: &mut HashMap<Point, Object>,
    direction: &Direction,
) -> Option<Point> {
    let next_location = location.get(direction);
    if let Some(object) = warehouse_map.get(&next_location) {
        match object {
            Wall => return None, //stop on wall
            Box(_) => {
                // let's try to move the box
                let moved_box_location = try_move(&next_location, warehouse_map, direction)?;
                warehouse_map
                    .remove(&next_location)
                    .and_then(|a_box| warehouse_map.insert(moved_box_location, a_box));
            }
        }
    }
    // empty spot or created one.
    Some(next_location)
}

fn execute_robot_movements_on_wide_map(
    robot: &mut Point,
    warehouse: &mut Warehouse,
    moves: &Vec<Direction>,
) -> u32 {
    for robot_move in moves.into_iter() {
        if let Some(moved_robot_location) =
            move_robot_on_wide_map(robot, &mut warehouse.map, robot_move)
        {
            robot.move_to(&moved_robot_location);
        }
    }

    warehouse.calculate_gps_sum()
}

fn move_robot_on_wide_map(
    robot: &Point,
    warehouse_map: &mut HashMap<Point, Object>,
    direction: &Direction,
) -> Option<Point> {
    // maybe not optimal, but for more easy thinking about this, separate the left, right and up/down movements
    match direction {
        Direction::Left => try_move_left_wide(&robot, warehouse_map, direction),
        Direction::Right => try_move_right_wide(&robot, 1, warehouse_map, direction),
        _ => {
            let robot = Vec::from([(Point::new(robot.x, robot.y), 1)]);
            try_move_up_or_down_wide(&robot, warehouse_map, direction)?
                .into_iter()
                .next()
        }
    }
}

fn try_move_left_wide(
    location: &Point,
    warehouse_map: &mut HashMap<Point, Object>,
    direction: &Direction,
) -> Option<Point> {
    // when moving left, account for that a boxes are registered one position further
    let next_location = location.get(direction);
    // check for wall (because boxes are wide, we should never find those)
    if let Some(Wall) = warehouse_map.get(&next_location) {
        return None;
    }

    // next check for box:
    let check_box_location = next_location.get(direction);
    if let Some(Box(_)) = warehouse_map.get(&check_box_location) {
        // found a box, so lets check if the box can still move: (use None propagation operator ?)
        let moved_box_location = try_move_left_wide(&check_box_location, warehouse_map, direction)?;
        warehouse_map
            .remove(&check_box_location)
            .and_then(|a_box| warehouse_map.insert(moved_box_location, a_box));
    }
    Some(next_location)
}

fn try_move_right_wide(
    location: &Point,
    my_width: u8,
    warehouse_map: &mut HashMap<Point, Object>,
    direction: &Direction,
) -> Option<Point> {
    // when moving right, account for the width of the object we are currently checking
    // a robot has width = 1, but a box has a width = 2
    let next_location = location.get(direction);
    let check_location = Point::new(location.x + my_width, location.y);

    // check for wall or box
    if let Some(object) = warehouse_map.get(&check_location) {
        match object {
            Wall => return None,
            Box(box_width) => {
                // check if the box can move, using its width (None propagations using ?)
                let moved_box_location =
                    try_move_right_wide(&check_location, *box_width, warehouse_map, direction)?;
                warehouse_map
                    .remove(&check_location)
                    .and_then(|a_box| warehouse_map.insert(moved_box_location, a_box));
            }
        }
    }
    Some(next_location)
}

fn try_move_up_or_down_wide(
    locations: &Vec<(Point, u8)>,
    warehouse_map: &mut HashMap<Point, Object>,
    direction: &Direction,
) -> Option<Vec<Point>> {
    // receives one or more objects to move up or down, all are in the same row.
    // when moving down/up: check also for boxes in x-1, and if so, keep in mind that boxes push 2 wide.
    // also keep in mind, if one of the boxes can not move, nothing can move,
    // that's why we collect all boxes before doing recursion
    let offset_y: i16 = match direction {
        Direction::Up => -1,
        Direction::Down => 1,
        _ => panic!("This method only supports up or down"),
    };
    let check_locations = locations.into_iter().flat_map(|(point, width)| {
        (0..*width)
            .map(move |offset_x| Point::new(point.x + offset_x, (point.y as i16 + offset_y) as u8))
    });

    let mut boxes = Vec::new();
    for check_location in check_locations {
        // check directly above for wall's and boxes:
        if let Some(object) = warehouse_map.get(&check_location) {
            match object {
                Wall => {
                    return None;
                }
                Box(width) => {
                    boxes.push((check_location, *width)); // remember box, check them all at once!
                }
            }
        } else {
            // check for a box, one to the left:
            let check_for_box_location = Point::new(check_location.x - 1, check_location.y);
            if let Some(Box(width)) = warehouse_map.get(&check_for_box_location) {
                boxes.push((check_for_box_location, *width));
            }
        }
    }

    // when a box is found:
    if !boxes.is_empty() {
        // can the boxes be moved? (using none-propagation)
        let new_locations = try_move_up_or_down_wide(&boxes, warehouse_map, direction)?;
        for (new_location, (old_location, _)) in new_locations.into_iter().zip(boxes) {
            warehouse_map
                .remove(&old_location)
                .and_then(|a_box| warehouse_map.insert(new_location, a_box));
        }
    }

    // all locations allow a move, so respond with all new locations:
    let new_locations = locations
        .into_iter()
        .map(|(point, _)| point.get(direction));
    Some(new_locations.collect())
}

fn parse_wide(input: &str) -> (Point, Warehouse) {
    let mut map = HashMap::new();
    let mut robot = Point::new(0, 0);
    let mut y = 1u8; // start at 1, so we can use unsigned ints for all of this!
    let mut width = 0;
    for line in input.lines() {
        if line.is_empty() {
            break; //also stop on empty line
        }
        if width == 0 {
            width = line.len() as u8 * 2;
        }
        for (x, c) in line.chars().enumerate() {
            let x1 = (x * 2 + 1) as u8;
            let x2 = x1 + 1;
            if c == '#' {
                map.insert(Point::new(x1, y), Wall);
                map.insert(Point::new(x2, y), Wall);
            } else if c == 'O' {
                map.insert(Point::new(x1, y), Box(2));
            } else if c == '@' {
                robot = Point::new(x1, y);
            }
        }

        y += 1;
    }

    (robot, Warehouse::new(map, width, y - 1))
}

fn parse(input: &str) -> (Point, Warehouse, Vec<Direction>) {
    let mut lines = input.lines().into_iter();
    let mut map = HashMap::new();
    let mut robot = Point::new(0, 0);
    let mut y = 1u8; // start at 1, so we can use unsigned ints for all of this!
    let mut width = 0;
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break; // empty line, so continue with directions
        }
        if width == 0 {
            width = line.len() as u8;
        }
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert(Point::new((x + 1) as u8, y), Wall);
            } else if c == 'O' {
                map.insert(Point::new((x + 1) as u8, y), Box(1));
            } else if c == '@' {
                robot = Point::new((x + 1) as u8, y);
            }
        }
        y += 1;
    }

    (
        robot,
        Warehouse::new(map, width, y - 1),
        parse_moves(&mut lines),
    )
}

fn parse_moves(lines: &mut Lines) -> Vec<Direction> {
    let mut moves = Vec::new();
    while let Some(line) = lines.next() {
        for c in line.chars() {
            moves.push(match c {
                '>' => Direction::Right,
                '<' => Direction::Left,
                '^' => Direction::Up,
                'v' => Direction::Down,
                _ => panic!("Unknown direction '{c}' found"),
            })
        }
    }
    moves
}

struct Warehouse {
    map: HashMap<Point, Object>,
    width: u8,
    height: u8,
}

impl Warehouse {
    fn new(map: HashMap<Point, Object>, width: u8, height: u8) -> Warehouse {
        Warehouse { map, width, height }
    }

    fn print(&self, robot: &Point) {
        for y in 1..self.height + 1 {
            for x in 1..self.width + 1 {
                let loc = Point::new(x, y);
                if let Some(object) = self.map.get(&loc) {
                    match object {
                        Box(1) => print!("O"),
                        Box(_) => print!("["),
                        Wall => print!("#"),
                    }
                } else if loc == *robot {
                    print!("\x1B[32m@\x1B[0m");
                } else {
                    let left = Direction::Left;
                    let left = loc.get(&left);
                    if let Some(Box(2)) = self.map.get(&left) {
                        print!("]")
                    } else {
                        print!(".");
                    }
                }
            }
            println!();
        }
        println!();
    }

    fn calculate_gps_sum(&self) -> u32 {
        self.map
            .iter()
            .filter_map(|(point, object)| match object {
                Box(_) => Some(point.gps()),
                _ => None,
            })
            .sum()
    }
}

enum Object {
    Wall,
    Box(u8), //width
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Hash, Eq, PartialEq)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new(x: u8, y: u8) -> Point {
        Point { x, y }
    }

    fn get(&self, rhs: &Direction) -> Point {
        match rhs {
            Direction::Up => Point::new(self.x, self.y - 1),
            Direction::Down => Point::new(self.x, self.y + 1),
            Direction::Left => Point::new(self.x - 1, self.y),
            Direction::Right => Point::new(self.x + 1, self.y),
        }
    }

    fn move_to(&mut self, location: &Point) {
        self.x = location.x;
        self.y = location.y;
    }

    fn gps(&self) -> u32 {
        // apply offset of -1, because we start every thing from 1
        let x = self.x as u32 - 1;
        let y = self.y as u32 - 1;
        y * 100 + x
    }
}
