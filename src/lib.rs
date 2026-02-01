//! # PNG You!
//!
//! A simple, high-performance library to hide secret messages in a PNG file using steganography.
//!
//! ## How it works
//! Every PNG file is divided into chunks following specific rules (see [PNG Struct Spec]).
//! It is, therefore, possible to append our own chunks of data inside a PNG without removing its validity.
//! The crate makes use of this to "hide" data inside a PNG file easily.
//!
//! ## Quick Example
//! ```no_run
//! use pngyou::{Png, Chunk, ChunkType};
//! use std::str::FromStr;
//! use std::convert::TryFrom;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   // Load your PNG file from bytes
//!   let input_bytes = std::fs::read("input.png")?;
//!   let mut png = Png::try_from(input_bytes.as_slice())?;
//!
//!   // Create a custom chunk type and your secret message
//!   let chunk_type = ChunkType::from_str("RuSt")?;
//!   let message = "This is a secret message".as_bytes().to_vec();
//!
//!   // Create the chunk and append it to the PNG
//!   let new_chunk = Chunk::new(chunk_type, message);
//!   png.append_chunk(new_chunk);
//!
//!   // Save the modified PNG back to a file
//!   std::fs::write("output.png", png.as_bytes())?;
//!
//!   Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! - **PNG Specification Compliance:** Strict parsing and validation of PNG
//!   structure and chunk types according to the PNG specification.
//!
//! - **Chunk Abstractions:** Strongly-typed representations for PNG chunks
//!   and chunk types, with semantic validation (critical, public, safe-to-copy).
//!
//! - **Safe Chunk Manipulation:** Create, insert, remove, and query chunks
//!   without corrupting the PNG file structure.
//!
//! - **CRC Integrity:** Automatic computation and verification of CRC values
//!   to preserve file correctness.
//!
//! - **Steganography-Friendly:** Designed to support custom chunks
//!   for data embedding without breaking image decoders.
//!
//! - **Extensible Design:** Can be used for general PNG inspection, editing,
//!   and tooling beyond steganography use cases.
//!
//! ## CLI Interface
//!
//! See the `pngyou` binary for end user usage examples.
//!
//! [PNG Struct Spec]: https://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html

mod chunk;
mod chunk_type;
mod png;

pub use chunk::Chunk;
pub use chunk_type::ChunkType;
pub use png::Png;
