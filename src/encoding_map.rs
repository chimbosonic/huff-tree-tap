use crate::error::Result;
use crate::huffman_tree::Node;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type Map = HashMap<u8, String>;
type InverseMap = HashMap<String, u8>;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EncodingMap {
    map: Map,
    inverse_map: InverseMap,
}

pub type Bit = u8;

pub type Code = Vec<Bit>;


trait ToFromChar {
    fn to_char(&self) -> char;
    fn from_char(c: char) -> Self;
}

impl ToFromChar for Bit {
    fn to_char(&self) -> char {
        match self {
            0 => '0',
            1 => '1',
            _ => panic!("Invalid bit"),
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '0' => 0,
            '1' => 1,
            _ => panic!("Invalid bit"),
        }
    }
}

trait Codeable {
    fn new_code() -> Code;
    fn extend_code(&self, bit: char) -> Code;
    fn to_string(&self) -> String;
}
impl Codeable for Code {
    fn new_code() -> Code {
        Vec::<u8>::new()
    }

    fn to_string(&self) -> String {
        self.iter()
            .map(|bit| {
                bit.to_char()
            })
            .collect()
    }

    fn extend_code(&self, bit: char) -> Code {
        let mut code = self.clone();
        code.push(Bit::from_char(bit));
        code
    }
}

impl EncodingMap {
    pub fn new(huffman_tree: &Node) -> Result<Self> {
        let mut map = Map::new();
        Self::build_encoding_map(huffman_tree, &mut map, &Code::new_code());

        let inverse_map = map.iter().map(|(k, v)| (v.clone(), *k)).collect();

        Ok(Self { map, inverse_map })
    }

    pub fn extract(&self) -> (HashMap<u8, String>, HashMap<String, u8>) {
        (self.map.clone(), self.inverse_map.clone())
    }

    pub fn from(map: Map) -> Self {
        let inverse_map = map.iter().map(|(k, v)| (v.clone(), *k)).collect();
        Self { map, inverse_map }
    }

    pub fn get(&self, key: &u8) -> Option<&String> {
        self.map.get(key)
    }

    pub fn get_inverse(&self, key: &str) -> Option<&u8> {
        self.inverse_map.get(key)
    }

    /// Creates a Hash Map of the encoding of every u8 within a given Huffman Tree. Left node edges are 0s and right node edges are 1s
    fn build_encoding_map(node: &Node, map: &mut Map, code: &Code) {
        match node.value {
            Some(value) => {
                map.insert(value, Codeable::to_string(code));
            }
            None => {
                if let Some(left) = &node.left {
                    Self::build_encoding_map(left, map, &code.extend_code('0'));
                }
                if let Some(right) = &node.right {
                    Self::build_encoding_map(right, map, &code.extend_code('1'));
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

        assert_eq!(expected_data, test_output);
    }
}
