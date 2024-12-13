extern crate hex;
extern crate serde;

use std::collections::HashMap;

use self::serde::{Deserialize, Serialize};

/// Huffman encoded data
#[derive(Serialize, Deserialize, Debug)]
pub struct HuffmanData {
    /// The encoded data as a `Vec<u8>`
    pub encoded_data: Vec<u8>,
    /// Encoding map stored as a `HashMap<u8,String>` required for decoding the data
    pub encoding_map: HashMap<u8, String>,
    pub stats: EncodingStats,
}

/// Encoding stats for a given data size and endcoded data size
#[derive(Serialize, Deserialize, Debug)]
pub struct EncodingStats {
    /// Size of the data
    pub data_size: f32,
    /// Size of the encoded data
    pub encoded_size: f32,
    /// Compression ratio
    pub ratio: f32,
}

/// INTERNAL ONLY: Represents a Node of a Tree
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    freq: i64,
    value: Option<u8>,
}

impl EncodingStats {
    /// Returns the `EncodingStats` for a given set of data and its encoded version
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to `Vec<u8>` containing the data
    /// * `encoded_data` - A reference to `Vec<u8>` containing the data encoded
    pub fn new(data: &[u8], encoded_data: &[u8]) -> EncodingStats {
        let data_size = (data.len() * 8) as f32;
        let encoded_size = (encoded_data.len() * 8) as f32;
        let ratio = (1_f32 - (encoded_size / data_size)) * 100_f32;
        EncodingStats {
         data_size,
         encoded_size,
         ratio,
        }
    }
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
    /// let data: Vec<u8> = "this is a test string!".to_string().into_bytes();
    /// let huffman_data: HuffmanData = HuffmanData::new(&data).unwrap();
    /// let decoded_data: Vec<u8> = huffman_data.decode().unwrap();
    /// assert_eq!(decoded_data,data);
    /// ```
    pub fn new(data: &[u8]) -> Result<HuffmanData, &'static str> {
        let frequency_map = Self::build_frequency_map(data);
        let huffman_tree = Self::build_huffman_tree(&frequency_map);
        let mut encoding_map: HashMap<u8, String> = HashMap::new();
        Self::build_encoding_map(&huffman_tree, &mut encoding_map, "".to_string());

        let encoded_data = Self::huffman_encode_string(data, &encoding_map);
        let encoded_data = Self::pad_encoded_data(&encoded_data);
        let encoded_data = Self::bin_string_to_u8_vec(&encoded_data);
        let stats = EncodingStats::new(data, &encoded_data);

        let huffman_encoded_data = HuffmanData {
            encoded_data,
            encoding_map,
            stats
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
    /// let data: Vec<u8> = "this is a test string!".to_string().into_bytes();
    /// let huffman_data: HuffmanData = HuffmanData::new(&data).unwrap();
    /// let decoded_data: Vec<u8> = huffman_data.decode().unwrap();
    /// assert_eq!(decoded_data,data);
    /// ```
    pub fn decode(&self) -> Result<Vec<u8>, &'static str> {
        let encoded_data_bin_string_padded = Self::u8_vec_to_bin_string(&self.encoded_data);
        let encoded_data_bin_string = Self::unpad_encoded_data(&encoded_data_bin_string_padded);
        let decoded_data =
            Self::huffman_decode_bin_string(&encoded_data_bin_string, &self.encoding_map);

        Ok(decoded_data)
    }

    /// Creates a HashMap containing Nodes with the frequency of every u8 in given String
    fn build_frequency_map(data: &[u8]) -> HashMap<u8, i64> {
        let mut frequency_map: HashMap<u8, i64> = HashMap::new();
        for &byte in data {
            frequency_map.entry(byte).and_modify(|e| *e+=1).or_insert(1);
        }
        frequency_map
    }

    /// Inverts Keys and values for a given Encoding Map
    fn invert_encoding_map(encoding_map: &HashMap<u8, String>) -> HashMap<String, u8> {
        let mut inverted_encoding_map: HashMap<String, u8> = HashMap::new();

        for (key, value) in encoding_map {
            inverted_encoding_map.insert(value.to_owned(), *key);
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
                Some(byte) => {
                    temp_code = "".to_string();
                    data.push(*byte);
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
                encoded_data = encoded_data + code;
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
            temp_padded_byte += &bit.to_string();
        }
        let (_, byte) = temp_padded_byte.split_at(1);
        data += byte;
        data
    }

    /// Creates a a Huffman Coding Tree with given Frequency Map
    /// We sort the frequency list alphabetically then we sort it by frequency to give us consitancy in the tree we generate
    fn build_huffman_tree(frequency_map: &HashMap<u8, i64>) -> Node {
        //Create a Vector of Nodes containing each u8 and their frequency
        let mut freq_list: Vec<Node> = Vec::new();
        for (data, freq) in frequency_map {
            freq_list.push(Node {
                left: None,
                right: None,
                value: Some(*data),
                freq: *freq,
            });
        }
        //Sort the Vector
        freq_list.sort_by(|a, b| b.value.cmp(&a.value));
        freq_list.sort_by(|a, b| b.freq.cmp(&a.freq));

        while freq_list.len() != 1 {
            let left_node = freq_list.pop().unwrap();
            let right_node = freq_list.pop().unwrap();
            let new_node_freq = left_node.freq + right_node.freq;
            let new_node = Node {
                left: Some(Box::new(left_node)),
                right: Some(Box::new(right_node)),
                value: None,
                freq: new_node_freq,
            };
            freq_list.push(new_node);
            freq_list.sort_by(|a, b| b.freq.cmp(&a.freq));
        }
        freq_list.pop().unwrap()
    }

    /// Creates a Hash Map of the encoding of every u8 within a given Huffman Tree. Left node edges are 0s and right node edges are 1s
    fn build_encoding_map(node: &Node, encoding_map: &mut HashMap<u8, String>, code: String) {
        match node.value {
            Some(value) => {
                encoding_map.insert(value, code);
            }
            None => {
                match &node.left {
                    Some(left) => {
                        Self::build_encoding_map(left, encoding_map, code.clone() + "0");
                    }
                    None => {}
                }
                match &node.right {
                    Some(right) => {
                        Self::build_encoding_map(right, encoding_map, code.clone() + "1");
                    }
                    None => {}
                }
            }
        }
    }

    /// Decodes a Binary string to a Vector of u8
    fn bin_string_to_u8_vec(bin_string: &str) -> Vec<u8> {
        let mut temp_byte: String = String::new();
        let mut u8_vec: Vec<u8> = Vec::new();

        for bit in bin_string.chars() {
            if temp_byte.len() == 8 {
                let u8_byte = u8::from_str_radix(temp_byte.as_str(), 2).unwrap();
                u8_vec.push(u8_byte);
                temp_byte = "".to_string();
            }
            temp_byte.push(bit);
        }
        let u8_value = u8::from_str_radix(temp_byte.as_str(), 2).unwrap();
        u8_vec.push(u8_value);
        u8_vec
    }

    /// Encodes a Vector of u8 to a Binary string
    fn u8_vec_to_bin_string(u8_vec: &[u8]) -> String {
        let mut bin_string: String = String::new();
        for byte in u8_vec {
            bin_string += format!("{:b}", byte).as_str();
        }
        bin_string
    }

    /// Pads a given binary string by prefixing a 1 to every 7 bits
    fn pad_encoded_data(encoded_data: &str) -> String {
        let mut padded_encoded_data: String = String::new();
        let mut temp_padded_byte: String = "1".to_string();

        for bit in encoded_data.chars() {
            if temp_padded_byte.len() > 7 {
                padded_encoded_data += temp_padded_byte.as_str();
                temp_padded_byte = "1".to_string();
            }
            temp_padded_byte = temp_padded_byte + &bit.to_string();
        }
        padded_encoded_data += temp_padded_byte.as_str();
        padded_encoded_data
    }
}

// Unit Tests all internal functions must be tested here. One test per function unless impossible
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_encoded_data() {
        let input_data = "1011100101010000010100000110100101110101001010011011111000111001111011101001001010111010111111100001100".to_string();

        let expected_data = "1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100".to_string();

        let test_output = HuffmanData::pad_encoded_data(&input_data);

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_unpad_encoded_data() {
        let input_data = "1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100".to_string();

        let expected_data = "1011100101010000010100000110100101110101001010011011111000111001111011101001001010111010111111100001100".to_string();

        let test_output = HuffmanData::unpad_encoded_data(&input_data);

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_bin_string_to_u8_vec() {
        let input_data = "1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100".to_string();

        let expected_data: Vec<u8> = vec![
            220, 212, 138, 134, 203, 212, 211, 190, 156, 251, 210, 171, 215, 248, 44,
        ];

        let test_output = HuffmanData::bin_string_to_u8_vec(&input_data);

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_u8_vec_to_bin_string() {
        let input_data: Vec<u8> = vec![
            220, 212, 138, 134, 203, 212, 211, 190, 156, 251, 210, 171, 215, 248, 44,
        ];

        let expected_data = "1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100".to_string();

        let test_output = HuffmanData::u8_vec_to_bin_string(&input_data);

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_build_frequency_map() {
        let input_data: Vec<u8> = "this is a test string!".to_string().into_bytes();

        let mut expected_data: HashMap<u8, i64> = HashMap::new();
        expected_data.insert(b'h', 1);
        expected_data.insert(b'a', 1);
        expected_data.insert(b' ', 4);
        expected_data.insert(b'g', 1);
        expected_data.insert(b'i', 3);
        expected_data.insert(b's', 4);
        expected_data.insert(b'!', 1);
        expected_data.insert(b'n', 1);
        expected_data.insert(b'r', 1);
        expected_data.insert(b't', 4);
        expected_data.insert(b'e', 1);

        let test_output = HuffmanData::build_frequency_map(&input_data);

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_build_huffman_tree_build_encoding_map() {
        let mut input_data: HashMap<u8, i64> = HashMap::new();
        input_data.insert(b'h', 1);
        input_data.insert(b'a', 1);
        input_data.insert(b' ', 4);
        input_data.insert(b'g', 1);
        input_data.insert(b'i', 3);
        input_data.insert(b's', 4);
        input_data.insert(b'!', 1);
        input_data.insert(b'n', 1);
        input_data.insert(b'r', 1);
        input_data.insert(b't', 4);
        input_data.insert(b'e', 1);

        let mut expected_data: HashMap<u8, String> = HashMap::new();
        expected_data.insert(b'h', "10010".to_string());
        expected_data.insert(b'a', "0011".to_string());
        expected_data.insert(b' ', "01".to_string());
        expected_data.insert(b'g', "0001".to_string());
        expected_data.insert(b'i', "101".to_string());
        expected_data.insert(b's', "110".to_string());
        expected_data.insert(b'!', "0010".to_string());
        expected_data.insert(b'n', "10011".to_string());
        expected_data.insert(b'r', "1000".to_string());
        expected_data.insert(b't', "111".to_string());
        expected_data.insert(b'e', "0000".to_string());

        // Create a huffman tree (Can't really test the output of this without coming up with a way to print it and build it manually)
        let test_output_tree = HuffmanData::build_huffman_tree(&input_data);

        // Create a encoding map from the tree this we can test better
        let mut test_output: HashMap<u8, String> = HashMap::new();
        HuffmanData::build_encoding_map(&test_output_tree, &mut test_output, "".to_string());

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_invert_encoding_map() {
        let input_data: HashMap<u8, String> = [
            (b'h', "10010".to_string()),
            (b'a', "0011".to_string()),
            (b' ', "01".to_string()),
            (b'g', "0001".to_string()),
            (b'i', "101".to_string()),
            (b's', "110".to_string()),
            (b'!', "0010".to_string()),
            (b'n', "10011".to_string()),
            (b'r', "1000".to_string()),
            (b't', "111".to_string()),
            (b'e', "0000".to_string()),
        ]
        .iter()
        .cloned()
        .collect();

        let expected_data: HashMap<String, u8> = [
            ("10010".to_string(), b'h'),
            ("0011".to_string(), b'a'),
            ("01".to_string(), b' '),
            ("0001".to_string(), b'g'),
            ("101".to_string(), b'i'),
            ("110".to_string(), b's'),
            ("0010".to_string(), b'!'),
            ("10011".to_string(), b'n'),
            ("1000".to_string(), b'r'),
            ("111".to_string(), b't'),
            ("0000".to_string(), b'e'),
        ]
        .iter()
        .cloned()
        .collect();
        let test_output = HuffmanData::invert_encoding_map(&input_data);

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_huffman_encode_string() {
        let input_data: Vec<u8> = "this is a test string!".to_string().into_bytes();
        let input_encoding_map: HashMap<u8, String> = [
            (b'h', "10010".to_string()),
            (b'a', "0011".to_string()),
            (b' ', "01".to_string()),
            (b'g', "0001".to_string()),
            (b'i', "101".to_string()),
            (b's', "110".to_string()),
            (b'!', "0010".to_string()),
            (b'n', "10011".to_string()),
            (b'r', "1000".to_string()),
            (b't', "111".to_string()),
            (b'e', "0000".to_string()),
        ]
        .iter()
        .cloned()
        .collect();

        let expected_data: String =
            "11110010101110011011100100110111100001101110111011110001011001100010010".to_string();

        let test_output = HuffmanData::huffman_encode_string(&input_data, &input_encoding_map);

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_huffman_decode_bin_string() {
        let input_data: String =
            "11110010101110011011100100110111100001101110111011110001011001100010010".to_string();
        let input_encoding_map: HashMap<u8, String> = [
            (b'h', "10010".to_string()),
            (b'a', "0011".to_string()),
            (b' ', "01".to_string()),
            (b'g', "0001".to_string()),
            (b'i', "101".to_string()),
            (b's', "110".to_string()),
            (b'!', "0010".to_string()),
            (b'n', "10011".to_string()),
            (b'r', "1000".to_string()),
            (b't', "111".to_string()),
            (b'e', "0000".to_string()),
        ]
        .iter()
        .cloned()
        .collect();

        let expected_data: Vec<u8> = "this is a test string!".to_string().into_bytes();

        let test_output = HuffmanData::huffman_decode_bin_string(&input_data, &input_encoding_map);

        assert_eq!(expected_data, test_output);
    }
}
