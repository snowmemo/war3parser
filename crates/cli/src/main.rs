use std::path::Path;

use clap::Parser;
use war3parser::parser::w3x::War3MapW3x;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
enum Command {
    /// Extract a file from a MPQ archive and save it
    #[command(visible_alias = "x")]
    ExtractFile {
        /// Path to the MPQ archive
        map_path: String,

        /// File name to extract
        #[arg(short, long)]
        file_name: String,

        /// Path to the output directory
        #[arg(short, long, default_value = ".")]
        out_dir: String,
    },

    /// Extract images with *.tga and *.blp extensions
    #[command(visible_alias = "i")]
    ExtractImages {
        /// Path to the MPQ archive
        map_path: String,

        /// Path to the output directory
        #[arg(short, long, default_value = ".")]
        out_dir: String,

        /// Keep the original file format. Default is to convert to PNG
        #[arg(short, long, default_value = "false")]
        keep_ori: bool,
    },

    /// List files in a MPQ archive
    #[command(visible_alias = "l")]
    ListFiles {
        /// Path to the MPQ archive
        map_path: String,
    },
}

fn extract_file_buffer(w3x: &mut War3MapW3x, file_name: &str) -> anyhow::Result<Vec<u8>> {
    let out_file_file = w3x.get(file_name)?;
    let mut out_file_content = vec![0u8; out_file_file.size() as usize];
    out_file_file.read(&mut w3x.archive, &mut out_file_content)?;
    Ok(out_file_content)
}

fn save_file(file_name: &str, out_dir: &Path, file_content: &[u8]) -> anyhow::Result<()> {
    let out_file_path = out_dir.join(file_name.replace("\\", "/"));
    if let Some(parent) = out_file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(out_file_path, file_content)?;
    Ok(())
}

fn main() {
    let args = Command::parse();
    match args {
        Command::ExtractFile {
            map_path,
            file_name,
            out_dir,
        } => {
            let map_path = Path::new(&map_path);
            let out_dir = Path::new(&out_dir);

            let mut w3x =
                War3MapW3x::new(map_path.to_path_buf()).expect("Failed to parse map file");
            let file_content =
                extract_file_buffer(&mut w3x, &file_name).expect("Failed to extract file");
            save_file(&file_name, out_dir, &file_content).expect("Failed to save file");
        }
        Command::ListFiles { map_path } => {
            let map_path = Path::new(&map_path);
            let w3x = War3MapW3x::new(map_path.to_path_buf()).expect("Failed to parse map file");
            let files = w3x.files.expect("Failed to get files from MPQ");
            println!("{:#?}", files);
        }
        Command::ExtractImages {
            map_path,
            out_dir,
            keep_ori,
        } => {
            let map_path = Path::new(&map_path);
            let out_dir = Path::new(&out_dir);

            let mut w3x =
                War3MapW3x::new(map_path.to_path_buf()).expect("Failed to parse map file");
            let files = w3x.files.clone().unwrap_or_default();

            /// Handle image extraction
            fn handle_image(
                w3x: &mut War3MapW3x,
                file: &str,
                out_dir: &Path,
                keep_ori: bool,
            ) -> anyhow::Result<()> {
                let file_content = extract_file_buffer(w3x, file)?;
                if keep_ori {
                    save_file(file, out_dir, &file_content)?;
                } else {
                    let image = War3MapW3x::buffer_to_image(&file_content)?;
                    let out_file_path = out_dir.join(
                        file.replace("\\", "/")
                            .replace(".blp", ".png")
                            .replace(".tga", ".png"),
                    );
                    if let Some(parent) = out_file_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    image.data.save(out_file_path)?;
                }
                Ok(())
            }

            files
                .iter()
                .filter(|file| file.ends_with(".tga") || file.ends_with(".blp"))
                .for_each(
                    |file| match handle_image(&mut w3x, file, out_dir, keep_ori) {
                        Ok(_) => println!("✅ Extracted image: {}", file),
                        Err(e) => println!("❌ Failed to extract '{}': {}", file, e),
                    },
                );
        }
    }
}
