mod chunk;
mod chunk_type;
mod commands;
mod png;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Opts {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Encode a message with the chunk type into a PNG file")]
    Encode {
        file_path: PathBuf,
        chunk_type: String,
        message: String,
        output_file: Option<PathBuf>,
    },
    #[command(about = "Decode a message with the given chunk type from a PNG file")]
    Decode {
        file_path: PathBuf,
        chunk_type: String,
    },
    #[command(about = "Remove a message with the specified chunk type from a PNG file")]
    Remove {
        file_path: PathBuf,
        chunk_type: String,
    },
    #[command(about = "Print the file contents as lossy UTF-8")]
    Print { file_path: PathBuf },
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    match &opts.command {
        Commands::Encode {
            file_path,
            chunk_type,
            message,
            output_file,
        } => commands::encode(file_path, chunk_type, message, output_file)?,

        Commands::Decode {
            file_path,
            chunk_type,
        } => commands::decode(file_path, chunk_type)?,

        Commands::Remove {
            file_path,
            chunk_type,
        } => commands::remove(file_path, chunk_type)?,

        Commands::Print { file_path } => commands::print(file_path)?,
    }
    Ok(())
}
