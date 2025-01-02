use aoc24_tools::*;
use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;
use std::str::FromStr;

const DAY: u8 = 12;

fn main() {
    init_measurements!();
    print_header(DAY, "Garden Groups");

    let data = read_to_string("input.txt").unwrap();
    let (part1, part2) = measure_total!({ solve(&data) });
    assert_eq!(part2, 870202);
    println!("Price of fencing without discount   (Part 1): {part1}");
    println!("Price of fencing with bulk discount (Part 2): {part2}");

    print_summary(DAY);
}

fn solve(input: &str) -> (usize, usize) {
    let mut part1 = 0;
    let mut part2 = 0;
    let map = Map::from_str(input).unwrap();

    let mut visited = HashSet::new();
    for y in 0..map.rows {
        for x in 0..map.cols {
            let point = Point::new(x as i16, y as i16);
            if !visited.contains(&point) {
                let (area, perimeter, region) = calculate_region(&map, point);
                part1 += perimeter * area;
                if area > 2 {
                    let discount = calculate_bulk_discount(&region);
                    let number_of_sides = perimeter - discount;
                    part2 += number_of_sides * area;
                } else { // for 1 or 2 squares the perimeter is always 4:
                    part2 += 4 * area;
                }

                region.into_iter().for_each(|point| {
                    visited.insert(point);
                });
            }
        }
    }
    (part1, part2)
}

fn calculate_bulk_discount(region: &HashSet<Point>) -> usize {
    let mut no_corner = 0;
    // scan over the 'shadowed' region, always look at a block of 2x2
    // when exactly two are in within a block, this is a straight fence and not a corner
    // examples: oo  xo
    //           xx  xo
    // except for checkerboard pattern
    // like: xo ox
    //       ox xo
    // counting and using xor is an option, but pattern matching is faster...

    let mut queue = region
        .iter()
        .map(|p| (p.clone(), false)) //boolean: not a shadow
        .collect::<VecDeque<(Point, bool)>>();

    while let Some((point, is_shadow)) = queue.pop_front() {
        //for point in unique_with_shadow {
        let x = point.x;
        let y = point.y;

        let top_left = Point::new(x - 1, y - 1);
        let top_right = Point::new(x - 0, y - 1);
        let bottom_left = Point::new(x - 1, y - 0);
        let bottom_right = Point::new(x + 0, y + 0);

        let mut area_bits = !region.contains(&top_left) as u8;
        area_bits |= (!region.contains(&top_right) as u8) << 1;
        area_bits |= (!region.contains(&bottom_left) as u8) << 2;
        area_bits |= (!region.contains(&bottom_right) as u8) << 3;

        if area_bits == 0b0000_1100
            || area_bits == 0b0000_0011
            || area_bits == 0b0000_1010
            || area_bits == 0b0000_0101
        {
            no_corner += 1;
        }

        if !is_shadow {
            let shadow = Point::new(x + 1, y + 1);
            if !region.contains(&shadow) {
                queue.push_back((shadow, true));
            }
        }
    }

    no_corner
}

fn calculate_region(map: &Map, start: Point) -> (usize, usize, HashSet<Point>) {
    let mut area = 0;
    let mut perimeter = 0;
    let mut known_region: HashSet<Point> = HashSet::new();

    let character = map.get(&start).unwrap();
    let mut queue = VecDeque::from([(start, character)]);

    while let Some((point, character)) = queue.pop_front() {
        // add to region, if not yet processed before
        if !known_region.contains(&point) {
            for neighbour_point in point.get_neighbours() {
                if let Some(neighbour_char) = map.get(&neighbour_point) {
                    if neighbour_char == character {
                        queue.push_back((neighbour_point, neighbour_char)); //add to search
                        continue;
                    }
                }
                // not the same region or out of bounds, so increase perimeter
                perimeter += 1;
            }
            known_region.insert(point); // move ownership here
            area += 1;
        }
    }
    (area, perimeter, known_region)
}

struct Map {
    data: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn new(data: Vec<char>, cols: usize, rows: usize) -> Map {
        Map { data, cols, rows }
    }

    fn get(&self, p: &Point) -> Option<&char> {
        if p.x > -1 && p.y > -1 {
            let x = p.x as usize;
            let y = p.y as usize;
            if y < self.rows && x < self.cols {
                let index = y * self.cols + x;
                return Some(&self.data[index]);
            }
        }
        None
    }
}

impl FromStr for Map {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines();
        let mut data = Vec::with_capacity(input.len());
        for line in lines.into_iter() {
            for ch in line.chars() {
                data.push(ch);
            }
        }

        let height = input.lines().count();
        let width = input.find('\r').ok_or(())?;
        Ok(Map::new(data, width, height))
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

    fn get_neighbours(&self) -> [Point; 4] {
        [
            Point::new(self.x - 1, self.y - 0),
            Point::new(self.x + 1, self.y - 0),
            Point::new(self.x + 0, self.y - 1),
            Point::new(self.x + 0, self.y + 1),
        ]
    }
}
