use std::collections::HashMap;

use crate::encoding_map;

#[test]
fn test_build_huffman_tree_build_encoding_map() {
    let input_data: HashMap<u8, i64> = [
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

    // Create a huffman tree (Can't really test the output of this without coming up with a way to print it and build it manually)
    let _test_output_tree = encoding_map::huffman_tree::build(&input_data).unwrap();
}
