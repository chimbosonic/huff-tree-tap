use huff_tree_tap::*;
use std::collections::HashMap;

// All public functions must be tested here. One test per function unless impossible.
#[test]
fn test_huffman_encode(){
    let input_data = "My super test string".to_string().into_bytes();

    let expected_encoded_data = vec![182, 188, 239, 160, 190, 196, 223, 148, 209, 87];
    let expected_data_encoding_map: HashMap<u8, String> = [
        (b'M',"0110".to_string()),
        (b'g',"0111".to_string()),
        (b' ',"111".to_string()),
        (b'y',"1100".to_string()),
        (b'u',"11011".to_string()),
        (b'p',"11010".to_string()),
        (b'e',"000".to_string()),
        (b'n',"0101".to_string()),
        (b't',"101".to_string()),
        (b'r',"001".to_string()),
        (b'i',"0100".to_string()),
        (b's',"100".to_string()),
    ].iter().cloned().collect();
    let test_output = huffman_encode(&input_data);

    assert_eq!(expected_encoded_data,test_output.encoded_data);
    assert_eq!(expected_data_encoding_map,test_output.encoding_map);
}

#[test]
fn test_huffman_decode(){
    let input_encoded_data = vec![182, 188, 239, 160, 190, 196, 223, 148, 209, 87];
    let input_encoding_map: HashMap<u8, String> = [
        (b'M',"0110".to_string()),
        (b'g',"0111".to_string()),
        (b' ',"111".to_string()),
        (b'y',"1100".to_string()),
        (b'u',"11011".to_string()),
        (b'p',"11010".to_string()),
        (b'e',"000".to_string()),
        (b'n',"0101".to_string()),
        (b't',"101".to_string()),
        (b'r',"001".to_string()),
        (b'i',"0100".to_string()),
        (b's',"100".to_string()),
    ].iter().cloned().collect();
    let input_data = HuffmanData{encoded_data: input_encoded_data,encoding_map: input_encoding_map};

    let expected_data = "My super test string".to_string().into_bytes();

    let test_output: Vec<u8> = huffman_decode(&input_data);

    assert_eq!(expected_data,test_output);
}

