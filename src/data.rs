use crate::error::{HuffmanError, Result};

pub type Bit = u8;

pub type BitVec = Vec<Bit>;

pub trait ToFromChar {
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

pub trait BitVector {
    fn to_string(&self) -> String;
    fn from_string(s: &str) -> BitVec;
}

impl BitVector for BitVec {
    fn to_string(&self) -> String {
        self.iter().map(|bit| bit.to_char()).collect()
    }

    fn from_string(s: &str) -> BitVec {
        s.chars().map(Bit::from_char).collect()
    }
}

pub type Byte = BitVec;

pub type PaddedBits = BitVec;

pub type UnPaddedBits = BitVec;

pub trait UnPadded {
    fn pad(&self) -> PaddedBits;
}

trait ToByte {
    fn to_byte(&self) -> Result<u8>;
    fn from_byte(byte: u8) -> Self;
}

impl ToByte for Byte {
    fn to_byte(&self) -> Result<u8> {
        let mut byte = 0u8;
        for &bit in self {
            if bit != 0 && bit != 1 {
                return Err(HuffmanError::ByteStringConversionError(
                    "Non-bit value encountered",
                ));
            }
            byte = (byte << 1) | bit;
        }
        Ok(byte)
    }

    fn from_byte(byte: u8) -> Self {
        let mut byte_vec = BitVec::with_capacity(8);
        for i in 0..8 {
            let bit = (byte >> i) & 1;
            byte_vec.push(bit);
        }

        byte_vec.reverse();

        while byte_vec.first() == Some(&0) {
            byte_vec.remove(0);
        }
        byte_vec
    }
}

pub trait Padded {
    fn unpad(&self) -> UnPaddedBits;
    fn from_vec_u8(data: &[u8]) -> Self;
    fn to_vec_u8(&self) -> Result<Vec<u8>>;
}

impl Padded for PaddedBits {
    fn unpad(&self) -> UnPaddedBits {
        let mut data = UnPaddedBits::with_capacity(self.len());
        let mut temp_padded_byte = PaddedBits::with_capacity(8);
        for bit in self {
            if temp_padded_byte.len() > 7 {
                let (_, byte) = temp_padded_byte.split_at(1);
                data.extend_from_slice(byte);
                temp_padded_byte.clear();
            }
            temp_padded_byte.push(*bit);
        }
        let (_, byte) = temp_padded_byte.split_at(1);
        data.extend_from_slice(byte);
        data
    }

    fn from_vec_u8(u8_vec: &[u8]) -> PaddedBits {
        let mut bit_vec = PaddedBits::with_capacity(8 * u8_vec.len());

        for byte in u8_vec {
            bit_vec.extend_from_slice(&Byte::from_byte(*byte));
        }
        bit_vec
    }

    fn to_vec_u8(&self) -> Result<Vec<u8>> {
        let mut temp_byte = Byte::with_capacity(8);
        let mut u8_vec: Vec<u8> = Vec::with_capacity(self.len() / 8);

        for bit in self {
            if temp_byte.len() == 8 {
                u8_vec.push(temp_byte.to_byte()?);
                temp_byte.clear();
            }
            temp_byte.push(*bit);
        }
        u8_vec.push(temp_byte.to_byte()?);
        Ok(u8_vec)
    }
}

impl UnPadded for UnPaddedBits {
    fn pad(&self) -> PaddedBits {
        let mut padded_bits = PaddedBits::new();
        let mut temp_padded_byte = Byte::with_capacity(8);
        temp_padded_byte.push(1);

        for bit in self {
            if temp_padded_byte.len() > 7 {
                padded_bits.append(&mut temp_padded_byte);
                temp_padded_byte.push(1);
            }
            temp_padded_byte.push(*bit);
        }
        padded_bits.append(&mut temp_padded_byte);
        padded_bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpadded_bits_pad() {
        let input_data = UnPaddedBits::from_string("1011100101010000010100000110100101110101001010011011111000111001111011101001001010111010111111100001100");

        let expected_data = PaddedBits::from_string("1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100");

        let test_output = input_data.pad();

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_padded_bits_unpad() {
        let input_data = PaddedBits::from_string("1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100");

        let expected_data = BitVec::from_string("1011100101010000010100000110100101110101001010011011111000111001111011101001001010111010111111100001100");

        let test_output = input_data.unpad();

        assert_eq!(expected_data, test_output);
    }

    #[test]
    fn test_padded_bits_to_u8_vec() {
        let input_data = PaddedBits::from_string("1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100");

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

        let expected_data = PaddedBits::from_string("1101110011010100100010101000011011001011110101001101001110111110100111001111101111010010101010111101011111111000101100");

        let test_output = PaddedBits::from_vec_u8(&input_data);

        assert_eq!(expected_data, test_output);
    }
}
