use std::path::Path;

use clap::{Parser, Subcommand};
use war3parser::war3map_metadata::War3MapMetadata;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
#[command(about, long_about = None)]
enum Command {
    /// Extract a file from a MPQ archive
    #[command(about, long_about = None, visible_alias = "x")]
    Extract {
        /// Path to the MPQ archive
        #[arg(short, long)]
        map_path: String,

        /// Output directory
        #[arg(short, long)]
        out_dir: String,

        /// File name to extract
        #[arg(default_value = "(listfile)")]
        file_name: String,
    },

    /// Export bindings for the wasm module
    #[command(about, long_about = None, visible_alias = "e")]
    ExportBindings {
        /// Output directory
        #[arg(default_value = "bindings")]
        out_dir: String,
    },
}

fn main() {
    let cli: Cli = Cli::parse();
    match cli.command {
        Command::Extract {
            map_path,
            file_name,
            out_dir,
        } => {
            unimplemented!()
        }

        _ => {
            println!("Not implemented")
        }
    }
}
