use std::collections::HashMap;

/// Creates a HashMap containing Nodes with the frequency of every u8 in given String
pub fn build(data: &[u8]) -> HashMap<u8, i64> {
    let mut frequency_map: HashMap<u8, i64> = HashMap::new();
    for &byte in data {
        frequency_map
            .entry(byte)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    frequency_map
}
