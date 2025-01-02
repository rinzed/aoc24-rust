use crate::{write_value_to_wires, read_value_from_wires};

#[test]
fn test_convert_to_wires() {
    let result = write_value_to_wires('z', 5, 2);

    assert_eq!(result.len(), 3);
    assert_eq!(Some(&true), result.get("z00"));
    assert_eq!(Some(&false), result.get("z01"));
    assert_eq!(Some(&true), result.get("z02"));

    let converted_back = read_value_from_wires(&result, "z");
    assert_eq!(converted_back, 5);
}
