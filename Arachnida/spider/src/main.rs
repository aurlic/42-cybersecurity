use clap::Parser;
use reqwest::blocking::{get, Client};
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use std::error::Error;
use url::Url;

#[derive(Parser, Debug)]
struct Args {
    url: String,

    #[arg(short = 'r', long = "recursive")]
    rec: bool,

    #[arg(short = 'l', long = "depth-level", default_value_t = 5)]
    depth: u8,

    #[arg(short = 'p', long = "path", default_value = "./data/")]
    path: String,
}

fn check_url(url: &str) -> Option<Url> {
    let parsed_url = match Url::parse(url) {
        Ok(valid_url) => valid_url,
        Err(_) => {
            eprintln!("❌ Error : invalid URL -> {}", url);
            return None;
        }
    };

    let client = Client::new();
    let response = client
        .head(parsed_url.clone())
        .header(USER_AGENT, "Mozilla/5.0")
        .send();

    match response {
        Ok(resp) if resp.status().is_success() => Some(parsed_url),
        Ok(resp) => {
            eprintln!(
                "⚠️ The URL exists but returned an HTTP status. : {}",
                resp.status()
            );
            None
        }
        Err(_) => {
            eprintln!("❌ Error : Impossible to access the URL -> {}", parsed_url);
            None
        }
    }
}

fn fetch_page(url: &str) -> Result<String, Box<dyn Error>> {
    let response = get(url)?;
    if !response.status().is_success() {
        return Err(format!("Request failure: {}", response.status()).into());
    }
    let body = response.text()?;
    Ok(body)
}

fn extract_images(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("img").unwrap();
    let mut image_urls = Vec::new();

    for element in document.select(&selector) {
        if let Some(src) = element.value().attr("src") {
            image_urls.push(src.to_string());
        }
    }

    image_urls
}

fn filter_images(images: Vec<String>) -> Vec<String> {
    let mut valid_images = Vec::new();
    let allowed_extensions = ["jpg", "jpeg", "png", "gif", "bmp"];

    for image in images {
        for ext in allowed_extensions {
            if image.ends_with(ext) {
                valid_images.push(image);
                break;
            }
        }
    }

    valid_images
}

fn main() {
    let args = Args::parse();

    let valid_url = match check_url(&args.url) {
        Some(url) => url,
        None => std::process::exit(1),
    };

    match fetch_page(&args.url) {
        Ok(html) => println!("✅ Page téléchargée avec succès !"),
        Err(e) => println!("❌ Erreur lors du téléchargement : {}", e),
    }

    // println!("✅ URL valide et accessible: {}", valid_url);
    // println!("Récursif: {}", args.rec);
    // println!("Profondeur: {}", args.depth);
    // println!("Chemin: {}", args.path);
}
