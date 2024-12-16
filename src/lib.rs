mod encoding_map;
pub mod encoding_stats;
mod error;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use encoding_map::huffman_tree::Node;
use encoding_stats::EncodingStats;
use error::{HuffmanError, Result};

/// Huffman encoded data
#[derive(Serialize, Deserialize, Debug)]
pub struct HuffmanData {
    /// The encoded data as a `Vec<u8>`
    pub encoded_data: Vec<u8>,
    /// Encoding map stored as a `HashMap<u8,String>` required for decoding the data
    pub encoding_map: HashMap<u8, String>,
    pub stats: EncodingStats,
}

impl HuffmanData {
    /// Huffman encodes a `Vec<u8>` returning a `HuffmanData` struct
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to `Vec<u8>` containing the data you want to encode
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate huff_tree_tap;
    /// use  huff_tree_tap::*;
    /// use std::collections::HashMap;
    ///
    /// let data: Vec<u8> = Vec::<u8>::from("this is a test string!");
    /// let huffman_data: HuffmanData = HuffmanData::new(&data).unwrap();
    /// let decoded_data: Vec<u8> = huffman_data.decode().unwrap();
    /// assert_eq!(decoded_data,data);
    /// ```
    pub fn new(data: &[u8]) -> Result<HuffmanData> {
        let frequency_map = encoding_map::frequency_map::build(data);
        let huffman_tree = encoding_map::huffman_tree::build(&frequency_map)?;
        let mut encoding_map: HashMap<u8, String> = HashMap::new();
        Self::build_encoding_map(&huffman_tree, &mut encoding_map, "");

        let encoded_data = Self::huffman_encode_string(data, &encoding_map);
        let encoded_data = Self::pad_encoded_data(&encoded_data);
        let encoded_data = Self::bin_string_to_u8_vec(&encoded_data)?;
        let stats = EncodingStats::new(data, &encoded_data);

        let huffman_encoded_data = HuffmanData {
            encoded_data,
            encoding_map,
            stats,
        };
        Ok(huffman_encoded_data)
    }

    /// Huffman decodes a `HuffmanData` struct and returns a decoded `Vec<u8>`
    ///
    /// # Arguments
    ///
    /// * `huffman_encoded_data` - A reference to `HuffmanData` containing the encoded data and encoding map
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate huff_tree_tap;
    /// use  huff_tree_tap::*;
    /// use std::collections::HashMap;
    ///
    /// let data: Vec<u8> = Vec::from("this is a test string!");
    /// let huffman_data: HuffmanData = HuffmanData::new(&data).unwrap();
    /// let decoded_data: Vec<u8> = huffman_data.decode().unwrap();
    /// assert_eq!(decoded_data,data);
    /// ```
    pub fn decode(&self) -> Result<Vec<u8>> {
        let encoded_data_bin_string_padded = Self::u8_vec_to_bin_string(&self.encoded_data);
        let encoded_data_bin_string = Self::unpad_encoded_data(&encoded_data_bin_string_padded);
        let decoded_data =
            Self::huffman_decode_bin_string(&encoded_data_bin_string, &self.encoding_map);

        Ok(decoded_data)
    }

    /// Inverts Keys and values for a given Encoding Map
    fn invert_encoding_map(encoding_map: &HashMap<u8, String>) -> HashMap<String, u8> {
        let mut inverted_encoding_map: HashMap<String, u8> = HashMap::new();

        for (&key, value) in encoding_map {
            inverted_encoding_map.insert(value.to_owned(), key);
        }
        inverted_encoding_map
    }

    /// Decodes Huffman encoded binary string using provided encoding HashMap
    fn huffman_decode_bin_string(
        encoded_data: &str,
        encoding_map: &HashMap<u8, String>,
    ) -> Vec<u8> {
        let inverted_encoding_map = Self::invert_encoding_map(encoding_map);
        let mut data: Vec<u8> = Vec::new();
        let mut temp_code = String::new();
        let mut encoded_data_rev = encoded_data.chars().rev().collect::<String>();
        loop {
            match inverted_encoding_map.get(&temp_code) {
                Some(&byte) => {
                    temp_code = String::from("");
                    data.push(byte);
                }
                None => match encoded_data_rev.pop() {
                    Some(code) => {
                        temp_code.push(code);
                    }
                    None => {
                        break;
                    }
                },
            }
        }
        data
    }

    /// Encodes string with given HashMap
    fn huffman_encode_string(data: &[u8], encoding_map: &HashMap<u8, String>) -> String {
        let mut encoded_data = String::new();
        for c in data {
            if let Some(code) = encoding_map.get(c) {
                encoded_data += code;
            }
        }
        encoded_data
    }

    /// Removes padding
    fn unpad_encoded_data(padded_data: &str) -> String {
        let mut data: String = String::new();
        let mut temp_padded_byte: String = String::new();

        for bit in padded_data.chars() {
            if temp_padded_byte.len() > 7 {
                let (_, byte) = temp_padded_byte.split_at(1);
                data += byte;
                temp_padded_byte = String::new();
            }
            temp_padded_byte.push(bit);
        }
        let (_, byte) = temp_padded_byte.split_at(1);
        data += byte;
        data
    }

    /// Creates a Hash Map of the encoding of every u8 within a given Huffman Tree. Left node edges are 0s and right node edges are 1s
    fn build_encoding_map(node: &Node, encoding_map: &mut HashMap<u8, String>, code: &str) {
        match node.value {
            Some(value) => {
                encoding_map.insert(value, code.to_string());
            }
            None => {
                match &node.left {
                    Some(left) => {
                        Self::build_encoding_map(left, encoding_map, &format!("{}{}", code, "0"));
                    }
                    None => {}
                }
                match &node.right {
                    Some(right) => {
                        Self::build_encoding_map(right, encoding_map, &format!("{}{}", code, "1"));
                    }
                    None => {}
                }
            }
        }
    }

    /// Decodes a Binary string to a Vector of u8
    fn bin_string_to_u8_vec(bin_string: &str) -> Result<Vec<u8>> {
        let mut temp_byte: String = String::new();
        let mut u8_vec: Vec<u8> = Vec::new();

        for bit in bin_string.chars() {
            if temp_byte.len() == 8 {
                let u8_byte = Self::convert_byte_string_u8(&temp_byte)?;
                u8_vec.push(u8_byte);
                temp_byte = String::new();
            }
            temp_byte.push(bit);
        }
        let u8_value = Self::convert_byte_string_u8(&temp_byte)?;
        u8_vec.push(u8_value);
        Ok(u8_vec)
    }

    fn convert_byte_string_u8(byte_string: &str) -> Result<u8> {
        u8::from_str_radix(byte_string, 2).map_err(|_| {
            HuffmanError::ByteStringConversionError("Binary String passed contained a non-bit 0/1")
        })
    }

    /// Encodes a Vector of u8 to a Binary string
    fn u8_vec_to_bin_string(u8_vec: &[u8]) -> String {
        let mut bin_string: String = String::new();
        for byte in u8_vec {
            bin_string.push_str(format!("{:b}", byte).as_str());
        }
        bin_string
    }

    /// Pads a given binary string by prefixing a 1 to every 7 bits
    fn pad_encoded_data(encoded_data: &str) -> String {
        let mut padded_encoded_data: String = String::new();
        let mut temp_padded_byte: String = String::from("1");

        for bit in encoded_data.chars() {
            if temp_padded_byte.len() > 7 {
                padded_encoded_data.push_str(&temp_padded_byte);
                temp_padded_byte = String::from("1");
            }
            temp_padded_byte.push(bit);
        }
        padded_encoded_data.push_str(&temp_padded_byte);
        padded_encoded_data
    }
}

// Unit Tests all internal functions must be tested here. One test per function unless impossible
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_encoded_data() {
        let input_data = "1011100101010000010100000110100101110101001010011011111000111001111011101001001010111010111111100001100";

        let expected_data = "1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100";

        let test_output = HuffmanData::pad_encoded_data(&input_data);

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_unpad_encoded_data() {
        let input_data = "1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100";

        let expected_data = "1011100101010000010100000110100101110101001010011011111000111001111011101001001010111010111111100001100";

        let test_output = HuffmanData::unpad_encoded_data(&input_data);

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_bin_string_to_u8_vec() {
        let input_data = "1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100";

        let expected_data: Vec<u8> = vec![
            220, 212, 138, 134, 203, 212, 211, 190, 156, 251, 210, 171, 215, 248, 44,
        ];

        let test_output = HuffmanData::bin_string_to_u8_vec(&input_data).unwrap();

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_u8_vec_to_bin_string() {
        let input_data: Vec<u8> = vec![
            220, 212, 138, 134, 203, 212, 211, 190, 156, 251, 210, 171, 215, 248, 44,
        ];

        let expected_data = "1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100";

        let test_output = HuffmanData::u8_vec_to_bin_string(&input_data);

        assert_eq!(expected_data, test_output);
    }

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

        let expected_data: HashMap<u8, String> = [
            (b'h', "10010"),
            (b'a', "0011"),
            (b' ', "01"),
            (b'g', "0001"),
            (b'i', "101"),
            (b's', "110"),
            (b'!', "0010"),
            (b'n', "10011"),
            (b'r', "1000"),
            (b't', "111"),
            (b'e', "0000"),
        ]
        .iter()
        .map(|(k, v)| (*k, v.to_string()))
        .collect();

        // Create a huffman tree (Can't really test the output of this without coming up with a way to print it and build it manually)
        let test_output_tree = encoding_map::huffman_tree::build(&input_data).unwrap();

        // Create a encoding map from the tree this we can test better
        let mut test_output: HashMap<u8, String> = HashMap::new();
        HuffmanData::build_encoding_map(&test_output_tree, &mut test_output, "");

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_invert_encoding_map() {
        let input_data: HashMap<u8, String> = [
            (b'h', "10010"),
            (b'a', "0011"),
            (b' ', "01"),
            (b'g', "0001"),
            (b'i', "101"),
            (b's', "110"),
            (b'!', "0010"),
            (b'n', "10011"),
            (b'r', "1000"),
            (b't', "111"),
            (b'e', "0000"),
        ]
        .iter()
        .map(|(k, v)| (*k, v.to_string()))
        .collect();

        let expected_data: HashMap<String, u8> = [
            ("10010", b'h'),
            ("0011", b'a'),
            ("01", b' '),
            ("0001", b'g'),
            ("101", b'i'),
            ("110", b's'),
            ("0010", b'!'),
            ("10011", b'n'),
            ("1000", b'r'),
            ("111", b't'),
            ("0000", b'e'),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), *v))
        .collect();
        let test_output = HuffmanData::invert_encoding_map(&input_data);

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_huffman_encode_string() {
        let input_data: Vec<u8> = Vec::from("this is a test string!");
        let input_encoding_map: HashMap<u8, String> = [
            (b'h', "10010"),
            (b'a', "0011"),
            (b' ', "01"),
            (b'g', "0001"),
            (b'i', "101"),
            (b's', "110"),
            (b'!', "0010"),
            (b'n', "10011"),
            (b'r', "1000"),
            (b't', "111"),
            (b'e', "0000"),
        ]
        .iter()
        .map(|(k, v)| (*k, v.to_string()))
        .collect();

        let expected_data =
            "11110010101110011011100100110111100001101110111011110001011001100010010";

        let test_output = HuffmanData::huffman_encode_string(&input_data, &input_encoding_map);

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_huffman_decode_bin_string() {
        let input_data = "11110010101110011011100100110111100001101110111011110001011001100010010";
        let input_encoding_map: HashMap<u8, String> = [
            (b'h', "10010"),
            (b'a', "0011"),
            (b' ', "01"),
            (b'g', "0001"),
            (b'i', "101"),
            (b's', "110"),
            (b'!', "0010"),
            (b'n', "10011"),
            (b'r', "1000"),
            (b't', "111"),
            (b'e', "0000"),
        ]
        .iter()
        .map(|(k, v)| (*k, v.to_string()))
        .collect();

        let expected_data: Vec<u8> = Vec::from("this is a test string!");

        let test_output = HuffmanData::huffman_decode_bin_string(&input_data, &input_encoding_map);

        assert_eq!(expected_data, test_output);
    }
}
