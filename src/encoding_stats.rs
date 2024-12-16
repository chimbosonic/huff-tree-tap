use serde::{Deserialize, Serialize};


/// Encoding stats for a given data size and endcoded data size
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EncodingStats {
    /// Size of the data
    pub data_size: f32,
    /// Size of the encoded data
    pub encoded_size: f32,
    /// Compression ratio
    pub ratio: f32,
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