mod arg;

use arg::Args;
use clap::Parser;
use exif::{Reader, Tag};
use std::fs::File;
use std::path::Path;

fn process_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exif = Reader::new().read_from_container(&mut bufreader)?;

    println!("📜 Available EXIF metadata for '{}':", filename);

    for field in exif.fields() {
        let tag_name = format!("{:?}", field.tag);
        let value = field.display_value().to_string();
        println!("🔹 {}: {}", tag_name, value);
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    if args.files.is_empty() {
        eprintln!("❌ No files provided!");
        std::process::exit(1);
    }

    for file in &args.files {
        let path = Path::new(file);

        if !path.exists() {
            eprintln!("⚠️ Warning: File '{}' does not exist.", file);
            continue;
        }

        if !path.is_file() {
            eprintln!("⚠️ Warning: '{}' is not a valid file.", file);
            continue;
        }
        println!("📂 Processing file: {}", file);
        if let Err(e) = process_file(file) {
            eprintln!("❌ Error processing '{}': {}", file, e);
        }
    }
}
