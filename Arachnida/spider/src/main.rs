use clap::Parser;
use reqwest::blocking::{get, Client};
use reqwest::header::USER_AGENT;
use std::error::Error;
use url::Url;

mod images;

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

fn main() {
    let args = Args::parse();

    let valid_url = match check_url(&args.url) {
        Some(url) => url.to_string(),
        None => std::process::exit(1),
    };

    match fetch_page(&valid_url) {
        Ok(html) => {
            println!("✅ Successfully downloaded html");

            let images = images::extract_images(&html);
            let valid_images = images::filter_images(images);

            if valid_images.is_empty() {
                println!("⚠️ No valid image found.");
            } else {
                println!("📸 Valid image(s) found :");
                for url in &valid_images {
                    println!("  - {}", url);
                }

                for url in valid_images {
                    let full_url = images::get_full_url(&valid_url, &url); // Ajout de la gestion des URL complètes
                    println!("📸 Downloading image: {}", full_url);
                    match images::download_image(full_url.as_str(), &args.path) {
                        Ok(_) => println!("✅ Successfully downloaded: {}", full_url),
                        Err(e) => eprintln!("{}", e),
                    }
                }
            }
        }
        Err(e) => println!("❌ Error during download : {}", e),
    }
}
