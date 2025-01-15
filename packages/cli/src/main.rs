use std::path::Path;

use clap::Parser;
use war3parser::parser::w3x::War3MapW3x;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
enum Command {
    /// Extract a file from a MPQ archive
    #[command(about, long_about = None, visible_alias = "x")]
    Extract {
        /// Path to the MPQ archive
        #[arg(short, long)]
        map_path: String,

        /// File name to extract
        #[arg(short, long)]
        file_name: String,

        /// Path to the output directory
        #[arg(short, long, default_value = ".")]
        out_dir: String,
    },

    /// List all files in a MPQ archive
    #[command(about, long_about = None, visible_alias = "l")]
    List {
        /// Path to the MPQ archive
        #[arg(short, long)]
        map_path: String,
    },
}

fn main() {
    let args = Command::parse();
    match args {
        Command::Extract {
            map_path,
            file_name,
            out_dir,
        } => {
            let map_path = Path::new(&map_path);
            let out_dir = Path::new(&out_dir);

            let mut w3x =
                War3MapW3x::new(map_path.to_path_buf()).expect("Failed to parse map file");
            let out_file_file = w3x.get(&file_name).expect("Failed to get file from MPQ");
            let mut out_file_content = vec![0u8; out_file_file.size() as usize];
            out_file_file
                .read(&mut w3x.archive, &mut out_file_content)
                .expect("Failed to read file");

            let out_file_path = out_dir.join(file_name);
            std::fs::write(out_file_path, out_file_content).expect("Failed to write file");
        }
        Command::List { map_path } => {
            let map_path = Path::new(&map_path);
            let w3x = War3MapW3x::new(map_path.to_path_buf()).expect("Failed to parse map file");
            let files = w3x.files.expect("Failed to get files from MPQ");
            println!("{:#?}", files);
        }
    }
}
