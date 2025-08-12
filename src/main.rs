use clap::Parser;
use std::fs::{File, read_dir};
use std::io::{Read, Write};
use zip::write::FileOptions;
use zip::read::ZipArchive;

#[derive(Parser)]
#[command(author, version, about = "Simple ZIP archiver")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Create a ZIP archive
    Create {
        /// Output ZIP file
        output: String,
        /// Input file or directory to compress
        input: String,
    },
    /// Extract a ZIP archive
    Extract {
        /// Input ZIP file
        input: String,
        /// Output directory
        output: String,
    },
}

fn create_zip(output_path: &str, input_path: &str) -> std::io::Result<()> {
    let output_file = File::create(output_path)?;
    let mut zip_writer = zip::ZipWriter::new(output_file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    let input = std::path::Path::new(input_path);

    if input.is_file() {
        let mut file = File::open(input)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let filename = input.file_name().unwrap().to_str().unwrap();
        zip_writer.start_file(filename, options)?;
        zip_writer.write_all(&buffer)?;
    } else if input.is_dir() {
        for entry in read_dir(input)? {
            let entry = entry?;
            let path = entry.path();
            let name_in_zip = path.strip_prefix(input).unwrap();

            if path.is_file() {
                let mut file = File::open(&path)?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;

                zip_writer.start_file(name_in_zip.to_str().unwrap(), options)?;
                zip_writer.write_all(&buffer)?;
            }
        }
    }

    zip_writer.finish()?;
    Ok(())
}

fn extract_zip(input_path: &str, output_dir: &str) -> std::io::Result<()> {
    let file = File::open(input_path)?;
    let mut archive = ZipArchive::new(file)?;

    std::fs::create_dir_all(output_dir)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let output_path = std::path::Path::new(output_dir).join(file.name());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&output_path)?;
        } else {
            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut output = File::create(&output_path)?;
            std::io::copy(&mut file, &mut output)?;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Create { output, input } => {
            if !std::path::Path::new(&input).exists() {
                eprintln!("Error: Input does not exist: {}", input);
                std::process::exit(1);
            }
            create_zip(&output, &input)?;
            println!("Created ZIP: {}", output);
        }
        Command::Extract { input, output } => {
            if !std::path::Path::new(&input).exists() {
                eprintln!("Error: ZIP file not found: {}", input);
                std::process::exit(1);
            }
            extract_zip(&input, &output)?;
            println!("Extracted to: {}", output);
        }
    }

    Ok(())
}