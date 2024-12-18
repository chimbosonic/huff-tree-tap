use crate::error::{HuffmanError, Result};
use crate::frequency_map::FrequencyMap;

#[derive(Debug)]
pub struct Node {
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub freq: i64,
    pub value: Option<u8>,
}

impl Node {
    fn new_leaf(freq: i64, value: Option<u8>) -> Node {
        Node {
            left: None,
            right: None,
            freq,
            value,
        }
    }

    fn new_branch(left: Node, right: Node) -> Node {
        let freq = left.freq + right.freq;
        Node {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            freq,
            value: None,
        }
    }
}

/// Creates a a Huffman Coding Tree with given Frequency Map
/// We sort the frequency list alphabetically then we sort it by frequency to give us consitancy in the tree we generate
pub fn build(frequency_map: &FrequencyMap) -> Result<Node> {
    //Create a Vector of Nodes containing each u8 and their frequency
    let mut freq_list: Vec<Node> = Vec::with_capacity(frequency_map.len());
    for (&data, &freq) in frequency_map {
        freq_list.push(Node::new_leaf(freq, Some(data)));
    }

    //Sort the Vector
    freq_list.sort_by(|a, b| b.value.cmp(&a.value));
    freq_list.sort_by(|a, b| b.freq.cmp(&a.freq));

    while freq_list.len() != 1 {
        let left_node = freq_list
            .pop()
            .ok_or(HuffmanError::TreeError("Missing Left Node"))?;
        let right_node = freq_list
            .pop()
            .ok_or(HuffmanError::TreeError("Missing Right Node"))?;
        let new_node = Node::new_branch(left_node, right_node);
        freq_list.push(new_node);
        freq_list.sort_by(|a, b| b.freq.cmp(&a.freq));
    }
    freq_list
        .pop()
        .ok_or(HuffmanError::TreeError("Missing Root Node"))
}

#[cfg(test)]
mod tests {
    use crate::frequency_map::FrequencyMapping;

    use super::*;

    #[test]
    fn test_build_huffman_tree() {
        let input_data: Vec<u8> = Vec::from("this is a test string!");
        let frequency_map = FrequencyMap::build(&input_data);

        // Create a huffman tree (Can't really test the output of this without coming up with a way to print it and build it manually)
        let _test_output_tree = build(&frequency_map).unwrap();
    }
}
