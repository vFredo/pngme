use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: PngMeArgs,
}

#[derive(Subcommand, Debug)]
pub enum PngMeArgs {
    /// Encode a message to a specify file
    Encode(EncodeArgs),
    /// Decode a message from a file knowing the chunk_type
    Decode(DecodeArgs),
    /// Remove a chunk from a file knowing the chunk_type
    Remove(RemoveArgs),
    /// Print Chunks from a file
    Print(PrintArgs),
}

#[derive(Args, Debug)]
#[command(author, version, about)]
pub struct EncodeArgs {
    pub file: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    pub file: PathBuf,
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    pub file: PathBuf,
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    pub file: PathBuf,
}
