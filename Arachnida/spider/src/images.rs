use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::fs::{create_dir_all, File};
use std::io;
use std::path::Path;
use url::Url;

pub fn get_full_url(base_url: &str, image_url: &str) -> String {
    if image_url.starts_with("http") {
        image_url.to_string()
    } else if image_url.starts_with("//") {
        format!("http:{}", image_url)
    } else {
        let base = Url::parse(base_url).unwrap();
        base.join(image_url).unwrap().to_string()
    }
}

pub fn download_image(url: &str, path: &str) -> Result<(), String> {
    if let Err(e) = create_dir_all(path) {
        return Err(format!("❌ Error creating directory: {}", e));
    }
    let client = Client::new();

    let response = match client.get(url).send() {
        Ok(resp) => resp,
        Err(_) => return Err(format!("❌ Error downloading image: {}", url)),
    };

    if !response.status().is_success() {
        return Err(format!("❌ Error: Image download failed for {}", url));
    }

    let filename = Path::new(url)
        .file_name()
        .unwrap_or_else(|| "default_name".as_ref())
        .to_str()
        .unwrap_or("default_name");

    let save_path = format!("{}/{}", path, filename);
    let mut file = match File::create(save_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("❌ Error creating file: {}", e)),
    };

    match io::copy(&mut response.bytes().unwrap().as_ref(), &mut file) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("❌ Error writing file: {}", e)),
    }
}

pub fn extract_images(html: &str) -> Vec<String> {
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

pub fn extract_links(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    let mut links = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            links.push(href.to_string());
        }
    }

    links
}

pub fn filter_images(images: Vec<String>) -> Vec<String> {
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
