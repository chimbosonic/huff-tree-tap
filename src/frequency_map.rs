use std::collections::HashMap;

pub type FrequencyMap = HashMap<u8, i64>;

pub trait FrequencyMapping {
    fn build(data: &[u8]) -> Self;
}

impl FrequencyMapping for FrequencyMap {
    fn build(data: &[u8]) -> Self {
        let mut frequency_map: FrequencyMap = FrequencyMap::new();
        for &byte in data {
            *frequency_map.entry(byte).or_insert(0) += 1;
        }
        frequency_map
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_build_frequency_map() {
        let input_data: Vec<u8> = Vec::from("this is a test string!");
        let expected_data: FrequencyMap = vec![
            (116, 4),
            (103, 1),
            (104, 1),
            (32, 4),
            (33, 1),
            (110, 1),
            (97, 1),
            (114, 1),
            (105, 3),
            (101, 1),
            (115, 4),
        ]
        .iter()
        .cloned()
        .collect();
        let test_ouput = FrequencyMap::build(&input_data);

        assert_eq!(expected_data, test_ouput);
    }
}
