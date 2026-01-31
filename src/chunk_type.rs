use anyhow::{Error, bail};
use std::fmt::{Display, Error as FmtError, Formatter};
use std::str::FromStr;

#[derive(PartialEq, Clone, Eq, Debug)]
pub(crate) struct ChunkType {
    bytes: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        let mut chunk_type = Self { bytes: [0; 4] };

        bytes.iter().enumerate().for_each(|(i, b)| {
            chunk_type.bytes[i] = *b;
        });

        Ok(chunk_type)
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            bail!("Invalid length of byte.");
        }

        let mut chunk_type = Self { bytes: [0; 4] };

        if !s.bytes().enumerate().all(|(i, b)| {
            if !b.is_ascii_alphabetic() {
                return false;
            }
            chunk_type.bytes[i] = b;
            true
        }) {
            bail!("Not a valid utf-8 string.");
        }

        Ok(chunk_type)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        if let Some(s) = str::from_utf8(&self.bytes).ok() {
            return write!(f, "{}", s);
        }
        Ok(())
    }
}

impl ChunkType {
    pub(crate) fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.bytes.iter().all(|b| b.is_ascii_alphabetic()) && self.is_reserved_bit_valid()
    }

    pub(crate) fn is_critical(&self) -> bool {
        self.bytes[0].is_ascii_uppercase()
    }

    pub(crate) fn is_public(&self) -> bool {
        self.bytes[1].is_ascii_uppercase()
    }

    pub(crate) fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2] & 0b00100000 == 0
    }

    pub(crate) fn is_safe_to_copy(&self) -> bool {
        self.bytes[3].is_ascii_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
