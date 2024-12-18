use crate::data::ToFromChar;
use crate::data::{Bit, BitVector};
use crate::huffman_tree::Node;
use crate::{data::BitVec, error::Result};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type Map = HashMap<u8, BitVec>;
type InverseMap = HashMap<BitVec, u8>;

trait MapTrait {
    fn to_string_map(&self) -> HashMap<u8, String>;
}

trait InverseMapTrait {
    fn to_string_map(&self) -> HashMap<String, u8>;
}

impl MapTrait for Map {
    fn to_string_map(&self) -> HashMap<u8, String> {
        self.iter().map(|(k, v)| (*k, v.to_string())).collect()
    }
}

impl InverseMapTrait for InverseMap {
    fn to_string_map(&self) -> HashMap<String, u8> {
        self.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EncodingMap {
    map: Map,
    inverse_map: InverseMap,
}

impl EncodingMap {
    pub fn new(huffman_tree: &Node) -> Result<Self> {
        let mut map = Map::new();
        Self::build_encoding_map(huffman_tree, &mut map, &BitVec::new());

        let inverse_map = map.iter().map(|(k, v)| (v.clone(), *k)).collect();

        Ok(Self { map, inverse_map })
    }

    pub fn extract(&self) -> (HashMap<u8, String>, HashMap<String, u8>) {
        (self.map.to_string_map(), self.inverse_map.to_string_map())
    }

    pub fn from(map: HashMap<u8, String>) -> Self {
        let map: Map = map
            .iter()
            .map(|(k, v)| (*k, BitVec::from_string(v)))
            .collect();
        let inverse_map = map.iter().map(|(k, v)| (v.clone(), *k)).collect();
        Self { map, inverse_map }
    }

    pub fn get(&self, key: &u8) -> Option<&BitVec> {
        self.map.get(key)
    }

    pub fn get_shortest_code(&self) -> usize {
        if let Some(el) = self.inverse_map.keys().min_by_key(|v| v.len()) {
            el.len()
        } else {
            0
        }
    }

    pub fn get_longest_code(&self) -> usize {
        if let Some(el) = self.inverse_map.keys().max_by_key(|v| v.len()) {
            el.len()
        } else {
            0
        }
    }

    pub fn get_inverse(&self, key: &BitVec) -> Option<&u8> {
        self.inverse_map.get(key)
    }

    /// Creates a Hash Map of the encoding of every u8 within a given Huffman Tree. Left node edges are 0s and right node edges are 1s
    fn build_encoding_map(node: &Node, map: &mut Map, code: &BitVec) {
        match node.value {
            Some(value) => {
                map.insert(value, code.clone());
            }
            None => {
                if let Some(left) = &node.left {
                    let mut code = code.clone();
                    code.push(Bit::from_char('0'));
                    Self::build_encoding_map(left, map, &code);
                }
                if let Some(right) = &node.right {
                    let mut code = code.clone();
                    code.push(Bit::from_char('1'));
                    Self::build_encoding_map(right, map, &code);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        frequency_map::{FrequencyMap, FrequencyMapping},
        huffman_tree,
    };

    use super::*;

    #[test]
    fn test_encoding_map() {
        let input_data: Vec<u8> = Vec::from("this is a test string!");
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
        let expected_data = EncodingMap::from(expected_data);

        let frequency_map = FrequencyMap::build(&input_data);
        let huffman_tree = huffman_tree::build(&frequency_map).unwrap();

        // Create a encoding map from the tree this we can test better
        let test_output = EncodingMap::new(&huffman_tree).unwrap();
        assert_eq!(test_output.get_shortest_code(), 2);
        assert_eq!(expected_data, test_output);
    }
}
