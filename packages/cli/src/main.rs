use std::path::Path;

use clap::{Parser, Subcommand};
use war3parser::extractor::Extractor;

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
            let buf = std::fs::read(map_path).expect("Failed to read map");
            let mut map = Extractor::new(&buf);
            let w3raw = map
                .extract_with_filename(&file_name)
                .expect("Failed to extract file");
            let out_path = Path::new(&out_dir).join(&w3raw.filename);
            std::fs::write(&out_path, w3raw.data).expect("Failed to write file");
            println!("Extracted {} to {}", w3raw.filename, out_path.display());
        }

        _ => {
            println!("Not implemented")
        }
    }
}
