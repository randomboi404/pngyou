use anyhow::{Result, bail};
use pngyou::{Chunk, ChunkType, Png};
use std::fs;
use std::path::PathBuf;

fn get_png(input: &PathBuf) -> Result<Png> {
    let file_bytes = fs::read(input)?;
    Png::try_from(file_bytes.as_slice())
}

pub fn encode(
    input: &PathBuf,
    output: &Option<PathBuf>,
    chunk_type: &ChunkType,
    message: &str,
) -> Result<()> {
    let mut png = get_png(input)?;
    let data = message.bytes().collect::<Vec<u8>>();

    let chunk_to_append = Chunk::new(chunk_type.clone(), data);
    png.append_chunk(chunk_to_append);

    match output {
        Some(output) => Ok(fs::write(output, png.as_bytes())?),
        None => {
            println!("{}", png);
            Ok(())
        }
    }
}

pub fn decode(input: &PathBuf, chunk_type: &ChunkType) -> Result<()> {
    let png = get_png(input)?;

    let chunks = png.chunks_by_type(chunk_type);
    if chunks.is_empty() {
        bail!("No chunk found of type:\n{}", chunk_type);
    }

    chunks.into_iter().for_each(|chunk| {
        if let Ok(message) = String::from_utf8(chunk.data().to_vec()) {
            println!("{}", message);
        } else {
            println!("[Hex data]: {:?}", chunk.data());
        }
    });
    Ok(())
}

pub fn remove(input: &PathBuf, output: &Option<PathBuf>, chunk_type: &ChunkType) -> Result<()> {
    let mut png = get_png(input)?;
    png.remove_first_chunk(chunk_type)?;

    match output {
        Some(output) => Ok(fs::write(output, png.as_bytes())?),
        None => Ok(fs::write(input, png.as_bytes())?),
    }
}

pub fn print(input: &PathBuf) -> Result<()> {
    let png = get_png(input)?;

    println!("{}", png);
    Ok(())
}
