use std::collections::HashMap;
use crate::{calculate_complexity, get_inputs_for_last_robot_michael, Keypad};

fn get_inputs_for_last_robot(input: &str) -> (Vec<String>, usize) {
    let directional = Keypad::directional();
    let numeric = Keypad::numeric();
    let routes = numeric.find_paths(input, 'A');

    let mut routes2 = Vec::new();
    let mut min_length = usize::MAX;
    for route_input in routes.iter() {
        let new_routes = directional.find_paths(route_input, 'A');
        new_routes.into_iter().for_each(|s| {
            if s.len() <= min_length {
                min_length = s.len();
                routes2.push(s)
            }
        });
    }
    routes2 = routes2
        .into_iter()
        .filter(|s| s.len() <= min_length)
        .collect();

    let mut routes3 = Vec::new();
    let mut min_length = usize::MAX;
    for route_input in routes2.iter() {
        let new_routes = directional.find_paths(route_input, 'A');
        new_routes.into_iter().for_each(|s| {
            if s.len() <= min_length {
                min_length = s.len();
                routes3.push(s)
            }
        });
    }
    routes3 = routes3
        .into_iter()
        .filter(|s| s.len() <= min_length)
        .collect();
    (routes3, min_length)
}


#[test]
fn check_numeric_keypad() {
    let keypad = Keypad::numeric();
    assert_eq!(keypad.buttons.len(), 11);
    for button in keypad.buttons.iter() {
        assert_eq!(button.1.routes_to_other_buttons.len(), 10);
    }
    keypad.print();
}

#[test]
fn check_directional_keypad() {
    let keypad = Keypad::directional();
    assert_eq!(keypad.buttons.len(), 5);
    for button in keypad.buttons.iter() {
        assert_eq!(button.1.routes_to_other_buttons.len(), 4);
    }
    keypad.print();
}

#[test]
fn first_example_029a_3_routes() {
    let input = "029A";
    let _expected = ["<A^A>^^AvvvA", "<A^A^>^AvvvA", "<A^A^^>AvvvA"];
    let keypad = Keypad::numeric();

    let routes = keypad.find_paths(input, 'A');

    assert_eq!(routes.len(), 3);
    println!("{:#?}", routes);
}

#[test]
fn second_example_029a_2_robots() {
    let input = "029A";
    let expected = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".to_string();

    let dir_control_num = Keypad::directional();
    let numeric = Keypad::numeric();
    let routes = numeric.find_paths(input, 'A');

    let mut cache: HashMap<String, String> = HashMap::new();
    for route_input in routes.iter().skip(2).take(1) {
        println!("Proccesing: {}", route_input);

        let mut input2 = HashMap::new();
        for value in route_input[0..route_input.len() - 1].split('A') {
            input2
                .entry(value.to_string())
                .and_modify(|e| *e += 1u64)
                .or_insert(1u64);
        }
        println!("After split: {:#?}", input2);

        let best_route = dir_control_num.find_paths_with_cache(input2, &mut cache, false);

        println!("After processing: {:#?}", best_route);

        let actual: u64 = best_route
            .iter()
            .map(|(fragment, amount)| (fragment.len() + 1) as u64 * amount)
            .sum();
        assert_eq!(actual, expected.len() as u64);
    }
}

#[test]
fn third_example_029a_3_robots() {
    //let mut cache = HashMap::new();
    let numeric = Keypad::numeric();
    let directional = Keypad::directional();

    let input = "029A";
    let expected =
        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".to_string();
    let (routes, _) = get_inputs_for_last_robot(input);
    let min_length = get_inputs_for_last_robot_michael(input, 2, &directional, &numeric);
    assert!(routes.contains(&expected));
    assert_eq!(min_length, expected.len() as u64);

    let input = "980A";
    let expected = "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".to_string();
    let (routes, _) = get_inputs_for_last_robot(input);
    let min_length = get_inputs_for_last_robot_michael(input, 2, &directional, &numeric);
    assert!(routes.contains(&expected));
    assert_eq!(min_length, expected.len() as u64);

    let input = "179A";
    let expected =
        "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".to_string();
    let (routes, _) = get_inputs_for_last_robot(input);
    let min_length = get_inputs_for_last_robot_michael(input, 2, &directional, &numeric);
    assert!(routes.contains(&expected));
    assert_eq!(min_length, expected.len() as u64);

    let input = "456A";
    let expected = "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".to_string();
    let (routes, _) = get_inputs_for_last_robot(input);
    let min_length = get_inputs_for_last_robot_michael(input, 2, &directional, &numeric);
    assert!(routes.contains(&expected));
    assert_eq!(min_length, expected.len() as u64);

    let input = "379A";
    let expected = "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".to_string();
    let (routes, _) = get_inputs_for_last_robot(input);
    let min_length = get_inputs_for_last_robot_michael(input, 2, &directional, &numeric);
    assert!(routes.contains(&expected));
    println!();
    println!("{:#?}", routes);
    println!();
    assert_eq!(min_length, expected.len() as u64);
}

// 029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
// 980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
// 179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
// 456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
// 379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

#[test]
fn find_paths_with_cache_experiment() {
    let input = "029A\n980A\n179A\n456A\n379A\n671A\n826A\n670A\n085A\n283A";
    //let mut cache = HashMap::new();
    let numeric = Keypad::numeric();
    let directional = Keypad::directional();

    let lines = input.lines();
    for line in lines {
        let min_length = get_inputs_for_last_robot_michael(line, 17, &directional, &numeric);
        _ = calculate_complexity(line, min_length) as u32;
    }
    //print_cache(&cache);
    // assert!(routes.contains(&expected));
    // assert_eq!(min_length, expected.len());
}

#[test]
fn complexity_check() {
    let input = "029A";
    let route = "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A";

    let score = calculate_complexity(input, route.len() as u64);

    assert_eq!(score, 68 * 29);
}


#[test]
fn test_a_25_robots() {
    let result = get_inputs_for_last_robot_opt(
        "A",
        20,
        &Keypad::directional(),
        &Keypad::numeric(),
        &mut HashMap::new(),
    );
    assert_eq!(result, 1);
}

#[test]
fn test_3a_25_robots2() {
    let result = get_inputs_for_last_robot_opt(
        "3A",
        25,
        &Keypad::directional(),
        &Keypad::numeric(),
        &mut HashMap::new(),
    );
    assert_eq!(result, 57870179098);
}
