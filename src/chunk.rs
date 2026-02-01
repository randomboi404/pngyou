use super::chunk_type::ChunkType;
use anyhow::{Error, Result, bail};
use crc::{CRC_32_ISO_HDLC, Crc};
use std::fmt::{Display, Error as FmtError, Formatter};

/// The [Chunk] struct represents a particular chunk
/// for a PNG file.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Chunk {
    length: [u8; 4],
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: [u8; 4],
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 12 {
            bail!("Invalid chunk. Minimum chunk size must be of 12 bytes.");
        }

        let mut length = [0u8; 4];
        let mut chunk_type_bytes = [0u8; 4];
        let mut data = Vec::<u8>::with_capacity(bytes.len() - 12);

        length.copy_from_slice(&bytes[0..4]);
        chunk_type_bytes.copy_from_slice(&bytes[4..8]);
        data.extend_from_slice(&bytes[8..(bytes.len() - 4)]);

        let expected_length = u32::from_be_bytes(length) as usize;
        if data.len() != expected_length {
            bail!(
                "Mismatched length: header says {}, but found {} bytes",
                expected_length,
                data.len()
            );
        }

        let crc_handler = Crc::<u32>::new(&CRC_32_ISO_HDLC);

        let mut crc_input = Vec::with_capacity(4 + data.len());
        crc_input.extend_from_slice(&chunk_type_bytes);
        crc_input.extend_from_slice(&data);

        let crc_expected = crc_handler.checksum(&crc_input).to_be_bytes();

        let mut crc = [0u8; 4];
        crc.copy_from_slice(&bytes[(bytes.len() - 4)..bytes.len()]);

        if crc != crc_expected {
            bail!("CRC mismatched!");
        }

        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;

        Ok(Self {
            length,
            chunk_type,
            data,
            crc,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(
            f,
            "Length: {}\nChunk Type: {}\nData (Bytes): {:#?}\nData (String): {}\nCRC: {}\n",
            self.length(),
            self.chunk_type,
            self.data,
            self.data.iter().map(|b| *b as char).collect::<String>(),
            self.crc()
        )
    }
}

impl Chunk {
    /// Creates a new [Chunk] instance from chunk type and data bytes.
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len();
        u32::try_from(length)
            .unwrap_or_else(|_| panic!("Chunk data is too big! Max size is: {} bytes", u32::MAX));

        let mut bytes = Vec::with_capacity(4 + length);

        bytes.extend_from_slice(&chunk_type.bytes());
        bytes.extend_from_slice(&data);

        let crc_handler = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let crc = crc_handler.checksum(&bytes).to_be_bytes();

        Self {
            length: (length as u32).to_be_bytes(),
            chunk_type,
            data,
            crc,
        }
    }

    /// Returns the length of the chunk.
    pub fn length(&self) -> u32 {
        u32::from_be_bytes(self.length)
    }

    /// Returns the type of the chunk.
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// Returns the data of the chunk as slice of bytes.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Returns the CRC of the chunk.
    pub fn crc(&self) -> u32 {
        u32::from_be_bytes(self.crc)
    }

    /// Returns the data of the chunk as a String.
    pub fn data_as_string(&self) -> Result<String> {
        Ok(str::from_utf8(&self.data)?.to_string())
    }

    /// Returns the chunk as a list of bytes.
    pub fn as_bytes(&self) -> Vec<u8> {
        let data_len = self.data.len();
        let mut bytes = Vec::with_capacity(12 + data_len);

        bytes.extend_from_slice(&self.length);
        bytes.extend_from_slice(&self.chunk_type.bytes());
        bytes.extend_from_slice(&self.data);
        bytes.extend_from_slice(&self.crc);

        bytes
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
