use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::{Error, Result};

/// A validated PNG chunk type. See the PNG spec for more details.
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    identifier: [u8; 4],
}

impl ChunkType {
    /// Returns the raw bytes contained in the ChunkType
    pub fn bytes(&self) -> [u8; 4] {
        self.identifier
    }

    /// Returns the property state of the first byte as described in the PNG spec
    /// 0 (uppercase) = critical, 1 (lowercase) = ancillary.
    pub fn is_critical(&self) -> bool {
        self.identifier[0] & 1 << 5 == 0
    }

    /// Returns the property state of the second byte as described in the PNG spec
    /// 0 (uppercase) = public, 1 (lowercase) = private.
    pub fn is_public(&self) -> bool {
        self.identifier[1] & 1 << 5 == 0
    }

    /// Returns the property state of the third byte as described in the PNG spec
    /// Must be 0 (uppercase) in files conforming to this version of PNG.
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.identifier[2] & 1 << 5 == 0
    }

    /// Returns the property state of the fourth byte as described in the PNG spec
    /// 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy.
    pub fn is_safe_to_copy(&self) -> bool {
        self.identifier[3] & 1 << 5 != 0
    }

    /// Returns true if the reserved byte is valid
    /// and all four bytes are represented by the characters A-Z or a-z.
    pub fn is_valid(&self) -> bool {
        let valid_bytes: bool = self
            .identifier
            .iter()
            .all(|&byte| ChunkType::is_valid_byte(byte));
        valid_bytes && self.is_reserved_bit_valid()
    }

    /// Valid bytes are represented by the characters A-Z or a-z
    pub fn is_valid_byte(byte: u8) -> bool {
        (byte >= b'a' && byte <= b'z') || (byte >= b'A' && byte <= b'Z')
    }
}


impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        let valid_bytes: bool = bytes.iter().all(|&byte| Self::is_valid_byte(byte));

        if valid_bytes {
            Ok(Self { identifier: bytes })
        } else {
            Err(Box::new(ChunkTypeError::InvalidCharacter))
        }
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let bytes = s.as_bytes();

        if bytes.len() == 4 {
            let bytes: [u8; 4] = bytes.try_into().unwrap();
            Ok(Self::try_from(bytes)?)
        } else {
            Err(Box::new(ChunkTypeError::ByteLengthError(bytes.len())))
        }
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = String::from_utf8(self.identifier.to_vec()).map_err(|_| fmt::Error)?;
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum ChunkTypeError {
    /// Chunk has incorrect number of bytes (4 expected)
    ByteLengthError(usize),

    /// The input string contains an invalid character
    InvalidCharacter,
}

impl std::error::Error for ChunkTypeError {}

impl fmt::Display for ChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChunkTypeError::ByteLengthError(actual) => write!(
                f,
                "Expected 4 bytes but received {} when creating chunk type",
                actual
            ),
            ChunkTypeError::InvalidCharacter => {
                write!(f, "Input contains one or more invalid characters")
            }
        }
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
