use super::args::InputImage;
use anyhow::{Result, bail};
use pngyou::{Chunk, ChunkType, Png};
use std::fs;
use std::path::PathBuf;

fn parse_input(input: &InputImage) -> Result<Vec<u8>> {
    let mut bytes = Vec::<u8>::new();

    match input {
        InputImage::File(path) => {
            let file_bytes = fs::read(path)?;
            bytes.extend_from_slice(&file_bytes.as_slice());
        }
        InputImage::Url(url) => {
            let mut response = ureq::get(url).call()?;
            let body_bytes = response.body_mut().read_to_vec()?;

            bytes.extend_from_slice(&body_bytes);
        }
    }

    Ok(bytes)
}

pub fn encode(
    input: &InputImage,
    output: &Option<PathBuf>,
    chunk_type: &ChunkType,
    message: &str,
) -> Result<()> {
    let parsed_input = parse_input(input)?;
    let mut png = Png::try_from(parsed_input.as_slice())?;

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

pub fn decode(input: &InputImage, chunk_type: &ChunkType) -> Result<()> {
    let parsed_input = parse_input(input)?;
    let png = Png::try_from(parsed_input.as_slice())?;

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

pub fn remove(input: &InputImage, output: &Option<PathBuf>, chunk_type: &ChunkType) -> Result<()> {
    let parsed_input = parse_input(input)?;
    let mut png = Png::try_from(parsed_input.as_slice())?;

    png.remove_first_chunk(chunk_type)?;

    match output {
        Some(output) => Ok(fs::write(output, png.as_bytes())?),
        None => match input {
            InputImage::File(path) => Ok(fs::write(path, png.as_bytes())?),
            InputImage::Url(_) => {
                println!("{}", png);
                Ok(())
            }
        },
    }
}

pub fn print(input: &InputImage) -> Result<()> {
    let parsed_input = parse_input(input)?;
    let png = Png::try_from(parsed_input.as_slice())?;

    println!("{}", png);
    Ok(())
}
