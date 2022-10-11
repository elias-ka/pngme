use std::{fs, path::PathBuf, str::FromStr};

use anyhow::Result;

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};

pub fn encode(
    file_path: &PathBuf,
    chunk_type: &str,
    message: &str,
    output_file: &Option<PathBuf>,
) -> Result<()> {
    let input = fs::read(file_path)?;
    let output = if let Some(path) = output_file {
        path
    } else {
        file_path
    };
    let mut png = Png::try_from(input.as_slice())?;
    let chunk_type = ChunkType::from_str(chunk_type)?;
    let chunk = Chunk::new(chunk_type, message.as_bytes().into());
    png.append_chunk(chunk);
    fs::write(output, png.as_bytes())?;
    println!("Message encoded successfully");
    Ok(())
}

pub fn decode(file_path: &PathBuf, chunk_type: &str) -> Result<()> {
    let input = fs::read(file_path)?;
    let png = Png::try_from(input.as_slice())?;
    let chunk = png.chunk_by_type(chunk_type);
    if let Some(chunk) = chunk {
        println!("Found message: {}", String::from_utf8(chunk.data().into())?);
    } else {
        println!("Nothing found");
    }
    Ok(())
}

pub fn remove(file_path: &PathBuf, chunk_type: &str) -> Result<()> {
    let input = fs::read(file_path)?;
    let mut png = Png::try_from(input.as_slice())?;
    match png.remove_chunk(chunk_type) {
        Ok(c) => println!("Removed message: {}", String::from_utf8(c.data().into())?),
        Err(e) => println!("Could not remove chunk, {}", e),
    }
    Ok(())
}

pub fn print(file_path: &PathBuf) -> Result<()> {
    let input = fs::read(file_path)?;
    let png = Png::try_from(input.as_slice())?;
    println!("{}", String::from_utf8_lossy(&png.as_bytes()));
    Ok(())
}
