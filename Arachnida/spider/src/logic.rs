use crate::args::Args;
use crate::images;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::{get, Client};
use reqwest::header::USER_AGENT;
use std::error::Error;
use url::Url;

pub fn check_url(url: &str) -> Option<Url> {
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

pub fn fetch_page(url: &str) -> Result<String, Box<dyn Error>> {
    let response = get(url)?;
    if !response.status().is_success() {
        return Err(format!("Request failure: {}", response.status()).into());
    }
    let body = response.text()?;
    Ok(body)
}

pub fn process_page(url: &str, args: &Args) {
    match fetch_page(url) {
        Ok(html) => {
            println!("✅ Successfully downloaded HTML");

            let images = images::extract_images(&html);
            let valid_images = images::filter_images(images);

            if valid_images.is_empty() {
                println!("⚠️ No valid image found.");
                return;
            }

            println!("📸 Found {} valid image(s)", valid_images.len());

            let pb = ProgressBar::new(valid_images.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{wide_bar} {pos}/{len}")
                    .expect("Error setting progress bar style"),
            );

            for img_url in valid_images {
                let full_url = images::get_full_url(url, &img_url);
                let _ = images::download_image(full_url.as_str(), &args.path);
                pb.inc(1);
            }

            pb.finish_with_message("Download complete");
        }
        Err(e) => println!("❌ Error downloading page: {}", e),
    }
}

pub fn recursive_crawl(url: &str, args: &Args, depth: usize) {
    if depth > args.depth.into() {
        return;
    }

    println!("🌍 Crawling depth {}: {}", depth, url);
    process_page(url, args);

    match fetch_page(url) {
        Ok(html) => {
            let links = images::extract_links(&html);
            for link in links {
                let full_link = images::get_full_url(url, &link);
                recursive_crawl(&full_link, args, depth + 1);
            }
        }
        Err(e) => println!("❌ Error fetching linked page: {}", e),
    }
}
