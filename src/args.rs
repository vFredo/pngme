use clap::{Args, Subcommand, Parser};
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
#[command(author, version, about, long_about = None)]
pub struct EncodeArgs {
    #[clap(value_parser)]
    pub file: PathBuf,
    #[clap(value_parser)]
    pub chunk_type: String,
    #[clap(value_parser)]
    pub message: String,
    #[clap(value_parser)]
    pub output: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    #[clap(value_parser)]
    pub file: PathBuf,
    #[clap(value_parser)]
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    #[clap(value_parser)]
    pub file: PathBuf,
    #[clap(value_parser)]
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    #[clap(value_parser)]
    pub file: PathBuf,
}
