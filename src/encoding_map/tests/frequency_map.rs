use std::collections::HashMap;

use crate::encoding_map;

#[test]
fn test_build_frequency_map() {
    let input_data: Vec<u8> = Vec::from("this is a test string!");

    let expected_data: HashMap<u8, i64> = [
        (b'h', 1),
        (b'a', 1),
        (b' ', 4),
        (b'g', 1),
        (b'i', 3),
        (b's', 4),
        (b'!', 1),
        (b'n', 1),
        (b'r', 1),
        (b't', 4),
        (b'e', 1),
    ]
    .iter()
    .copied()
    .collect();

    let test_output = encoding_map::frequency_map::build(&input_data);

    assert_eq!(expected_data, test_output);
}
