use huff_tree_tap::encoding_stats::*;
use huff_tree_tap::*;
use std::collections::HashMap;

#[test]
fn test_huffmandata_decode() {
    let input_encoded_data = vec![182, 188, 239, 160, 190, 196, 223, 148, 209, 87];
    let input_encoding_map: HashMap<u8, String> = [
        (b'M', "0110".to_string()),
        (b'g', "0111".to_string()),
        (b' ', "111".to_string()),
        (b'y', "1100".to_string()),
        (b'u', "11011".to_string()),
        (b'p', "11010".to_string()),
        (b'e', "000".to_string()),
        (b'n', "0101".to_string()),
        (b't', "101".to_string()),
        (b'r', "001".to_string()),
        (b'i', "0100".to_string()),
        (b's', "100".to_string()),
    ]
    .iter()
    .cloned()
    .collect();
    let input_data = HuffmanData {
        encoded_data: input_encoded_data,
        encoding_map: input_encoding_map,
        stats: EncodingStats {
            data_size: 1.0,
            encoded_size: 1.0,
            ratio: 1.0,
        },
    };

    let expected_data = "My super test string".to_string().into_bytes();

    let test_output: Vec<u8> = input_data.decode().unwrap();

    assert_eq!(expected_data, test_output);
}

#[test]
fn test_huffmandata_encode() {
    let input_data = "My super test string".to_string().into_bytes();

    let expected_encoded_data = vec![182, 188, 239, 160, 190, 196, 223, 148, 209, 87];
    let expected_data_encoding_map: HashMap<u8, String> = [
        (b'M', "0110".to_string()),
        (b'g', "0111".to_string()),
        (b' ', "111".to_string()),
        (b'y', "1100".to_string()),
        (b'u', "11011".to_string()),
        (b'p', "11010".to_string()),
        (b'e', "000".to_string()),
        (b'n', "0101".to_string()),
        (b't', "101".to_string()),
        (b'r', "001".to_string()),
        (b'i', "0100".to_string()),
        (b's', "100".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    let test_output = HuffmanData::new(&input_data).unwrap();

    assert_eq!(expected_encoded_data, test_output.encoded_data);
    assert_eq!(expected_data_encoding_map, test_output.encoding_map);
}

#[test]
fn test_huffmandata_stats() {
    let input_data = "My super test string".to_string().into_bytes();
    let expected_stats = EncodingStats {
        data_size: 160.0,
        encoded_size: 80.0,
        ratio: 50.0,
    };
    let test_output = HuffmanData::new(&input_data).unwrap();

    assert_eq!(expected_stats.data_size, test_output.stats.data_size);
    assert_eq!(expected_stats.encoded_size, test_output.stats.encoded_size);
    assert_eq!(expected_stats.ratio, test_output.stats.ratio);
}

#[test]
fn test_bench() {
    let mut unencoded_data = Vec::<u8>::new();
    for _ in 0..1000 {
        unencoded_data.append(&mut vec![b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h']);
    }
    let encoded_data = HuffmanData::new(&unencoded_data).unwrap();

    assert_eq!(
        encoded_data.stats,
        EncodingStats {
            data_size: 64000.0,
            encoded_size: 27432.0,
            ratio: 57.1375,
        }
    );

    assert_eq!(encoded_data.decode().unwrap(), unencoded_data);
}
