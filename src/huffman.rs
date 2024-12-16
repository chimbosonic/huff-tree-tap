use crate::data::{Padded, PaddedBits, UnPadded, UnPaddedBits};
use crate::encoding_map::EncodingMap;
use crate::encoding_stats::EncodingStats;
use crate::error::Result;
use crate::frequency_map::{FrequencyMap, FrequencyMapping};
use crate::huffman_tree::{self, Node};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Huffman encoded data
#[derive(Serialize, Deserialize, Debug)]
pub struct HuffmanData {
    /// The encoded data as a `Vec<u8>`
    pub encoded_data: Vec<u8>,
    /// Encoding map stored as a `EncodingMap` required for decoding the data
    pub encoding_map: HashMap<u8, String>,
    /// Encoding stats for the data
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
        let frequency_map: FrequencyMap = FrequencyMap::build(data);
        let huffman_tree: Node = huffman_tree::build(&frequency_map)?;
        let encoding_map: EncodingMap = EncodingMap::new(&huffman_tree)?;

        let encoded_data: UnPaddedBits = Self::huffman_encode(data, &encoding_map);
        let encoded_data: PaddedBits = encoded_data.pad();
        let encoded_data = encoded_data.to_vec_u8()?;
        let stats: EncodingStats = EncodingStats::new(data, &encoded_data);

        let huffman_encoded_data = HuffmanData {
            encoded_data,
            encoding_map: encoding_map.extract().0,
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
        let encoded_data: PaddedBits = PaddedBits::from_vec_u8(&self.encoded_data);
        let encoded_data: UnPaddedBits = encoded_data.unpad();
        let encoding_map: EncodingMap = EncodingMap::from(self.encoding_map.clone());
        let decoded_data = Self::huffman_decode(&encoded_data, &encoding_map);

        Ok(decoded_data)
    }

    fn huffman_decode(encoded_data: &UnPaddedBits, encoding_map: &EncodingMap) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        let mut temp_code = String::new();
        let mut encoded_data_rev = encoded_data.chars().rev().collect::<String>();
        loop {
            match encoding_map.get_inverse(&temp_code) {
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

    fn huffman_encode(data: &[u8], encoding_map: &EncodingMap) -> UnPaddedBits {
        let mut encoded_data = String::new();
        for c in data {
            if let Some(code) = encoding_map.get(c) {
                encoded_data += code;
            }
        }
        encoded_data
    }
}

// Unit Tests all internal functions must be tested here. One test per function unless impossible
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huffman_encode() {
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
        let input_encoding_map = EncodingMap::from(input_encoding_map);

        let expected_data = UnPaddedBits::from(
            "11110010101110011011100100110111100001101110111011110001011001100010010",
        );

        let test_output = HuffmanData::huffman_encode(&input_data, &input_encoding_map);

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_huffman_decode() {
        let input_data = UnPaddedBits::from(
            "11110010101110011011100100110111100001101110111011110001011001100010010",
        );
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
        let input_encoding_map = EncodingMap::from(input_encoding_map);

        let expected_data: Vec<u8> = Vec::from("this is a test string!");

        let test_output = HuffmanData::huffman_decode(&input_data, &input_encoding_map);

        assert_eq!(expected_data, test_output);
    }
}
