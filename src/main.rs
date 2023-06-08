use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod cipher;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = args::Cli::parse();
    match cli.command {
        args::PngMeArgs::Encode(args) => commands::encode(args),
        args::PngMeArgs::Decode(args) => commands::decode(args),
        args::PngMeArgs::Find(args) => commands::find(args),
        args::PngMeArgs::Remove(args) => commands::remove(args),
        args::PngMeArgs::Print(args) => commands::print_chunks(args),
    }
    .unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
    });
    Ok(())
}
