use anyhow::Result;
use pngyou::{Chunk, ChunkType, Png};
use std::path::PathBuf;

pub fn encode(
    input: &PathBuf,
    output: &Option<PathBuf>,
    chunk_type: &ChunkType,
    message: &str,
) -> Result<()> {
    todo!()
}

pub fn decode(input: &PathBuf, chunk_type: &ChunkType) -> Result<()> {
    todo!()
}

pub fn remove(input: &PathBuf, output: &Option<PathBuf>, chunk_type: &ChunkType) -> Result<()> {
    todo!()
}

pub fn print(input: &PathBuf) -> Result<()> {
    todo!()
}
