use clap::{Parser, Subcommand};
use pngyou::ChunkType;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "PNG You!")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// encode the given file.
    Encode {
        /// path of file to encode.
        input: PathBuf,

        /// path to output encoded file.
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// chunk type to use encoding on.
        #[arg(short, long)]
        chunk_type: ChunkType,

        /// secret message to be encoded.
        #[arg(short, long)]
        message: String,
    },

    /// decode the given file.
    Decode {
        /// path of file to decode.
        input: PathBuf,

        /// chunk type to decode.
        #[arg(short, long)]
        chunk_type: ChunkType,
    },

    /// remove encoded message from the given file.
    Remove {
        /// path of file to remove the message from.
        input: PathBuf,

        /// path to output modified file.
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// chunk type to find and remove.
        #[arg(short, long)]
        chunk_type: ChunkType,
    },

    /// print the file as a raw sequence of bytes.
    Print {
        /// path of file to print.
        input: PathBuf,
    },
}
