use std::fs;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, FindArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

use crate::Result;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let data = args.message.clone().into_bytes();
    let new_chunk = Chunk::new(chunk_type, data);

    let mut png: Png = Png::from_file(&args.file)?;
    png.append_chunk(new_chunk);

    match args.output {
        Some(file) => fs::write(file, png.as_bytes())?,
        None => fs::write(args.file, png.as_bytes())?,
    }
    println!("Chunk '{}' added", args.chunk_type);
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png: Png = Png::from_file(&args.file)?;
    let find_chunk = png.chunk_by_type(&args.chunk_type);
    match find_chunk {
        Some(chunk) => println!("Message: {}", chunk.data_as_string()?),
        None => println!("No message for Chunk '{}'", args.chunk_type),
    }
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn find(args: FindArgs) -> Result<()> {
    let png: Png = Png::from_file(&args.file)?;

    if let Some(chunks) = png.find_possible_messages() {
        println!("Chunks with possible messages: ");
        for &chunk in chunks.iter() {
            println!("{}", chunk);
        }
    } else {
        println!("Couldn't find any possible Chunk with a message");
    }
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png: Png = Png::from_file(&args.file)?;
    png.remove_chunk(&args.chunk_type)?;

    fs::write(args.file, png.as_bytes())?;
    println!("Chunk '{}' removed", args.chunk_type);
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let png: Png = Png::from_file(&args.file)?;
    println!("{}", png);
    Ok(())
}
