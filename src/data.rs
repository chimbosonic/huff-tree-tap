use crate::error::{HuffmanError, Result};

pub type ByteString = String;

pub type PaddedBits = String;

pub type UnPaddedBits = String;

pub trait UnPadded {
    fn pad(&self) -> PaddedBits;
}

trait ToByte {
    fn to_byte(&self) -> Result<u8>;
}

impl ToByte for ByteString {
    fn to_byte(&self) -> Result<u8> {
        u8::from_str_radix(self, 2).map_err(|_| {
            HuffmanError::ByteStringConversionError("Binary String passed contained a non-bit 0/1")
        })
    }
}

pub trait Padded {
    fn unpad(&self) -> UnPaddedBits;
    fn from_vec_u8(data: &[u8]) -> Self;
    fn to_vec_u8(&self) -> Result<Vec<u8>>;
}

impl Padded for PaddedBits {
    fn unpad(&self) -> UnPaddedBits {
        let mut data: String = String::new();
        let mut temp_padded_byte: String = String::new();

        for bit in self.chars() {
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

    fn from_vec_u8(u8_vec: &[u8]) -> PaddedBits {
        let mut bin_string = PaddedBits::new();
        for byte in u8_vec {
            bin_string.push_str(format!("{:b}", byte).as_str());
        }
        bin_string
    }

    fn to_vec_u8(&self) -> Result<Vec<u8>> {
        let mut temp_byte = ByteString::new();
        let mut u8_vec: Vec<u8> = Vec::new();

        for bit in self.chars() {
            if temp_byte.len() == 8 {
                u8_vec.push(temp_byte.to_byte()?);
                temp_byte = String::new();
            }
            temp_byte.push(bit);
        }
        u8_vec.push(temp_byte.to_byte()?);
        Ok(u8_vec)
    }
}

impl UnPadded for UnPaddedBits {
    fn pad(&self) -> PaddedBits {
        let mut padded_encoded_data = PaddedBits::new();
        let mut temp_padded_byte: String = String::from("1");

        for bit in self.chars() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpadded_bits_pad() {
        let input_data = UnPaddedBits::from("1011100101010000010100000110100101110101001010011011111000111001111011101001001010111010111111100001100");

        let expected_data = PaddedBits::from("1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100");

        let test_output = input_data.pad();

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_padded_bits_unpad() {
        let input_data = PaddedBits::from("1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100");

        let expected_data = UnPaddedBits::from("1011100101010000010100000110100101110101001010011011111000111001111011101001001010111010111111100001100");

        let test_output = input_data.unpad();

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_padded_bits_to_u8_vec() {
        let input_data = PaddedBits::from("1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100");

        let expected_data: Vec<u8> = vec![
            220, 212, 138, 134, 203, 212, 211, 190, 156, 251, 210, 171, 215, 248, 44,
        ];

        let test_output = input_data.to_vec_u8().unwrap();

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_padded_bits_from_vec_u8() {
        let input_data: Vec<u8> = vec![
            220, 212, 138, 134, 203, 212, 211, 190, 156, 251, 210, 171, 215, 248, 44,
        ];

        let expected_data = PaddedBits::from("1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100");

        let test_output = PaddedBits::from_vec_u8(&input_data);

        assert_eq!(expected_data, test_output);
    }
}