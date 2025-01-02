use aoc24_tools::*;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::hash::RandomState;

const DAY: u8 = 23;

fn main() {
    init_measurements!();
    print_header(DAY, "LAN Party");

    let data = read_to_string("input.txt").unwrap();
    let (part1, part2) = measure_total!({ solve(&data) });

    println!("Number of 3 connected computers starting with 't' (Part 1): {part1}");
    println!("LAN party password (Part 2): {part2}");

    print_summary(DAY);
}

fn solve(input: &str) -> (usize, String) {
    let part1 = count_interconnected_computers_with_a_t(input);
    let part2 = find_largest_lan_party(input);
    (part1, part2)
}

fn count_interconnected_computers_with_a_t(input: &str) -> usize {
    let mut count = 0;
    let sets = parse_as_connections(input);

    // top to bottom approach: take first set of computers
    // look ahead for all connections either has and remember those
    // now check if any overlap, if they do, and any of the 3 computers start with a t, then count it.

    for i in 0..sets.len() {
        let computer_a = sets[i].0;
        let computer_b = sets[i].1;

        let mut connected_to_a = HashSet::new();
        let mut connected_to_b = HashSet::new();

        // store all connections to a or b
        for j in (i + 1)..sets.len() {
            let computer_c = sets[j].0;
            let computer_d = sets[j].1;

            if computer_a == computer_c {
                connected_to_a.insert(computer_d);
            }
            if computer_a == computer_d {
                connected_to_a.insert(computer_c);
            }
            if computer_b == computer_c {
                connected_to_b.insert(computer_d);
            }
            if computer_b == computer_d {
                connected_to_b.insert(computer_c);
            }
        }

        // now check if any connections overlap & and any of the pc's in the 3 pc set start with t
        count += connected_to_b
            .intersection(&connected_to_a)
            .filter(|&third_computer| any_start_with_t([computer_a, computer_b, third_computer]))
            .count();
    }

    count
}

fn parse_as_connections(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split('-').into_iter();
            let from = parts.next().unwrap();
            let to = parts.next().unwrap();
            (from, to)
        })
        .collect()
}

fn any_start_with_t(computer_set: [&str; 3]) -> bool {
    computer_set[0].starts_with('t')
        || computer_set[1].starts_with('t')
        || computer_set[2].starts_with('t')
}

fn find_largest_lan_party(input: &str) -> String {
    // let's parse the input as a hashmap storing all direct connections,
    // that way we can quickly see of any computer is connected to another
    let connections = parse_as_hashset(input);

    // start by building networks of 2 pc's (HashMap will prevent duplication)
    let mut networks = HashMap::new();
    for &computer in connections.keys() {
        for &other_computer in &connections[&computer] {
            let network = Network::new(&[computer, other_computer]);
            networks.entry(network.key.clone()).or_insert(network);
        }
    }

    // loop over the network and try extending any we have, if we can, store the extended networks
    while networks.len() > 1 {
        let mut bigger_networks = HashMap::new();

        'network: for network in networks.values() {
            let first_computer = &network.computers[0];
            let connected_to = &connections[first_computer.as_str()];
            let mut shared_connections: HashSet<&str, RandomState> =
                HashSet::from_iter(connected_to.into_iter().map(|&s| s));

            // find pc's that are connected to the current pc's in the network
            for i in 1..network.computers.len() {
                let other_computer = &network.computers[i];
                let other_connected_to = &connections[other_computer.as_str()];
                let other_connected_to =
                    HashSet::from_iter(other_connected_to.into_iter().map(|&s| s));
                shared_connections = HashSet::from_iter(
                    shared_connections
                        .intersection(&other_connected_to)
                        .into_iter()
                        .map(|&s| s),
                );

                if shared_connections.is_empty() {
                    continue 'network;
                }
            }

            // any pc's are part of a network with the previous, but we are not sure of they connect to each other
            // this we will check in the next loop.
            for new_computer in shared_connections {
                let new_network = Network::extend_width(network, new_computer);
                bigger_networks.insert(new_network.key.clone(), new_network);
            }
        }

        networks = bigger_networks;
    }

    // take the key from the remaining network!
    networks.keys().cloned().next().unwrap()
}

fn parse_as_hashset(input: &str) -> HashMap<&str, Vec<&str>> {
    let raw_connections = parse_as_connections(input);
    let mut result = HashMap::new();

    for (pc_a, pc_b) in raw_connections {
        result.entry(pc_a).or_insert_with(Vec::new).push(pc_b);
        result.entry(pc_b).or_insert_with(Vec::new).push(pc_a);
    }

    result
}

struct Network {
    key: String,
    computers: Vec<String>,
}

impl Network {
    fn new(computers: &[&str]) -> Self {
        let mut sorted_computers = computers
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        sorted_computers.sort();
        Network {
            key: sorted_computers.join(","),
            computers: sorted_computers,
        }
    }

    fn extend_width(current: &Network, new_computer: &str) -> Network {
        let mut new_computers = current
            .computers
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>();
        new_computers.push(new_computer);
        new_computers.sort();

        Network::new(&new_computers)
    }
}