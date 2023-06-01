use std::convert::TryFrom;
use std::fmt;

use crate::chunk_type::ChunkType;
use crate::{Error, Result};
use crc::{Crc, CRC_32_ISO_HDLC};

const CRC_ALGORITHM: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

#[derive(Debug, Clone)]
pub struct Chunk {
    size: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
}

impl Chunk {
    pub const LENGTH_BYTES: usize = 4;
    pub const CHUNK_TYPE_BYTES: usize = 4;
    pub const CRC_BYTES: usize = 4;
    pub const MIN_BYTES: usize = Chunk::LENGTH_BYTES + Chunk::CHUNK_TYPE_BYTES + Chunk::CRC_BYTES;

    /// Construct a new Chunk with a type and a data
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Chunk {
            size: data.len() as u32,
            chunk_type,
            data,
        }
    }

    /// The length of the data in the Chunk
    pub fn length(&self) -> u32 {
        self.size
    }

    /// The `ChunkType` of this chunk
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// The raw data of the Chunk in bytes
    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }

    /// The CRC (Cyclic Redundancy Check) of this chunk
    pub fn crc(&self) -> u32 {
        let bytes: Vec<u8> = self
            .chunk_type
            .bytes()
            .iter()
            .chain(self.data.iter())
            .copied()
            .collect();

        CRC_ALGORITHM.checksum(&bytes)
    }

    /// Returns the data stored in this chunk as a `String`.
    /// This function will return an error if the stored data is not valid UTF-8.
    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.data.to_vec())?)
    }

    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
        self.size
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect()
    }
}

#[derive(Debug)]
pub enum ChunkError {
    /// The input crc do not match the real crc
    InvalidCrc(u32, u32),

    /// The input is to small for the Chunk specifications
    InvalidInput(usize),
}

impl std::error::Error for ChunkError {}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < Chunk::MIN_BYTES {
            return Err(Box::new(ChunkError::InvalidInput(bytes.len())));
        }

        let mut iter = bytes.iter().copied();

        let size: [u8; 4] = iter
            .by_ref()
            .take(Chunk::LENGTH_BYTES)
            .collect::<Vec<u8>>()
            .try_into()
            .map_err(|_| fmt::Error)?;

        let size: u32 = u32::from_be_bytes(size);

        let chunk_type: [u8; 4] = iter
            .by_ref()
            .take(Chunk::CHUNK_TYPE_BYTES)
            .collect::<Vec<u8>>()
            .try_into()
            .map_err(|_| fmt::Error)?;

        let chunk_type: ChunkType = ChunkType::try_from(chunk_type)?;

        let data: Vec<u8> = iter.by_ref().take(size as usize).collect();
        let input_crc: [u8; 4] = iter
            .by_ref()
            .take(Chunk::CRC_BYTES)
            .collect::<Vec<u8>>()
            .try_into()
            .map_err(|_| fmt::Error)?;

        let input_crc: u32 = u32::from_be_bytes(input_crc);
        let chunk: Chunk = Chunk::new(chunk_type, data);

        if chunk.crc() != input_crc {
            return Err(Box::new(ChunkError::InvalidCrc(chunk.crc(), input_crc)));
        }

        Ok(chunk)
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChunkError::InvalidCrc(expected, actual) => write!(
                f,
                "Invalid CRC constructing Chunk. Expected {} but found {}",
                expected, actual
            ),
            ChunkError::InvalidInput(actual) => write!(
                f,
                "Minimun lenght of bytes expected {} but found {}",
                Chunk::MIN_BYTES,
                actual
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
