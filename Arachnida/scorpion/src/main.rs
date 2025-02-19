mod arg;

use arg::Args;
use clap::Parser;
use exif::{Reader, Tag};
use gif::DecodeOptions;
use image::ImageReader;
use png::Decoder as pngDecoder;
use std::fs::File;
use std::path::Path;

fn process_jpg(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exif = Reader::new().read_from_container(&mut bufreader)?;

    let mut camera_model = None;
    let mut datetime = None;
    let mut shutter_speed = None;
    let mut aperture = None;
    let mut iso = None;
    let mut focal_length = None;
    let mut flash = None;
    let mut width = None;
    let mut height = None;
    let mut resolution_x = None;
    let mut resolution_y = None;
    let mut color_space = None;

    for field in exif.fields() {
        match field.tag {
            Tag::Model => camera_model = Some(field.display_value().to_string()),
            Tag::DateTimeOriginal => datetime = Some(field.display_value().to_string()),
            Tag::ExposureTime => shutter_speed = Some(field.display_value().to_string()),
            Tag::FNumber => aperture = Some(field.display_value().to_string()),
            Tag::PhotographicSensitivity => iso = Some(field.display_value().to_string()),
            Tag::FocalLength => focal_length = Some(field.display_value().to_string()),
            Tag::Flash => flash = Some(field.display_value().to_string()),
            Tag::PixelXDimension => width = Some(field.display_value().to_string()),
            Tag::PixelYDimension => height = Some(field.display_value().to_string()),
            Tag::XResolution => resolution_x = Some(field.display_value().to_string()),
            Tag::YResolution => resolution_y = Some(field.display_value().to_string()),
            Tag::ColorSpace => color_space = Some(field.display_value().to_string()),
            _ => {}
        }
    }

    println!(
        "📷 Camera : {}",
        camera_model.unwrap_or("Unknown".to_string())
    );
    println!("🕒 Date : {}", datetime.unwrap_or("Unknown".to_string()));
    println!(
        "📸 Settings : {}s, f/{}, ISO {}, {}mm",
        shutter_speed.unwrap_or("?".to_string()),
        aperture.unwrap_or("?".to_string()),
        iso.unwrap_or("?".to_string()),
        focal_length.unwrap_or("?".to_string())
    );
    println!("🔦 Flash : {}", flash.unwrap_or("No".to_string()));
    println!(
        "🖼 Dimensions : {} x {} px",
        width.unwrap_or("?".to_string()),
        height.unwrap_or("?".to_string())
    );
    println!(
        "📏 Resolution : {} DPI x {} DPI",
        resolution_x.unwrap_or("?".to_string()),
        resolution_y.unwrap_or("?".to_string())
    );
    println!(
        "🎨 Colorimetric space : {}",
        color_space.unwrap_or("Unknown".to_string())
    );

    Ok(())
}

fn process_png(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let decoder = pngDecoder::new(file);

    let reader = decoder.read_info()?;

    let (width, height) = reader.info().size();
    let (color_type, bit_depth) = reader.output_color_type();

    println!("📏 Dimensions : {} x {} px", width, height);
    println!("🎨 Color type: {:?}", color_type);
    println!("🖼 Bit depth : {:?}", bit_depth);

    Ok(())
}

fn process_bmp(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(filename)?
        .with_guessed_format()?
        .decode()?;
    println!("📏 Dimensions : {} x {} px", img.width(), img.height());
    println!("🎨 Color type: {:?}", img.color());

    Ok(())
}

fn process_gif(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filename)?;

    let mut decoder = DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);

    let mut decoder = decoder.read_info(file)?;

    let width = decoder.width();
    let height = decoder.height();
    let global_palette = decoder.global_palette().is_some();
    let repeat = decoder.repeat();

    println!("📏 Dimensions: {} x {} px", width, height);
    println!(
        "🎨 Global palette: {}",
        if global_palette { "Yes" } else { "No" }
    );
    println!("🔁 Repeat: {:?}", repeat);

    let mut frame_count = 0;
    while decoder.read_next_frame()?.is_some() {
        frame_count += 1;
    }
    println!("🎞 Number of frames: {}", frame_count);

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

        let file_lower = file.to_lowercase();

        if file_lower.ends_with(".jpg") || file_lower.ends_with(".jpeg") {
            println!("\n📂 Processing JPG file: {}", file);
            if let Err(e) = process_jpg(file) {
                eprintln!("❌ Error processing '{}': {}", file, e);
            }
        } else if file_lower.ends_with(".png") {
            println!("\n📂 Processing PNG file: {}", file);
            if let Err(e) = process_png(file) {
                eprintln!("❌ Error processing '{}': {}", file, e);
            }
        } else if file_lower.ends_with(".bmp") {
            println!("\n📂 Processing BMP file: {}", file);
            if let Err(e) = process_bmp(file) {
                eprintln!("❌ Error processing '{}': {}", file, e);
            }
        } else if file_lower.ends_with(".gif") {
            println!("\n📂 Processing GIF file: {}", file);
            if let Err(e) = process_gif(file) {
                eprintln!("❌ Error processing '{}': {}", file, e);
            }
        } else {
            eprintln!("⚠️ Unsupported file type for file '{}'", file);
        }
    }
}
