use std::{
    fmt::Display,
    io::{BufReader, Read},
};

use anyhow::{anyhow, Result};
use crc::{Crc, CRC_32_ISO_HDLC};

use crate::chunk_type::ChunkType;

const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
    typ: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        // CRC is calculated from chunk type and chunk data
        let bytes = [chunk_type.bytes().as_slice(), data.as_slice()].concat();
        Self {
            length: data.len() as u32,
            typ: chunk_type,
            data,
            crc: CRC.checksum(&bytes),
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn typ(&self) -> &ChunkType {
        &self.typ
    }

    pub fn data(&self) -> &[u8] {
        &self.data.as_slice()
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.data.clone())?)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        [
            &self.length.to_be_bytes(),
            &self.typ.bytes(),
            self.data.as_slice(),
            &self.crc.to_be_bytes(),
        ]
        .concat()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 12 {
            return Err(anyhow!("input length should be at least 12 bytes"));
        }

        let mut reader = BufReader::new(bytes);
        let mut buff = [0u8; 4];

        // first 4 bytes is the data length
        reader.read_exact(&mut buff)?;
        let length = u32::from_be_bytes(buff);

        // next 4 bytes is the chunk type
        reader.read_exact(&mut buff)?;
        let chunk_type = ChunkType::try_from(buff)?;

        // next bytes are the chunk data
        let mut chunk_data: Vec<u8> = vec![0; length as usize];
        reader.read_exact(&mut chunk_data)?;

        // last 4 bytes is the CRC
        reader.read_exact(&mut buff)?;
        let crc = u32::from_be_bytes(buff);

        let chunk = Chunk::new(chunk_type, chunk_data);

        // make sure our chunk's CRC matches the given CRC
        if chunk.crc() == crc {
            Ok(chunk)
        } else {
            Err(anyhow!("invalid CRC value"))
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{")?;
        writeln!(f, "  length: {}", self.length)?;
        writeln!(f, "  chunk_type: {}", &self.typ)?;
        writeln!(f, "  data: {:?}", &self.data)?;
        writeln!(f, "  crc: {}", self.crc)?;
        writeln!(f, "}}")
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
        assert_eq!(chunk.typ().to_string(), String::from("RuSt"));
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
        assert_eq!(chunk.typ().to_string(), String::from("RuSt"));
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
