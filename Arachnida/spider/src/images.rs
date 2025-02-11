use scraper::{Html, Selector};

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
