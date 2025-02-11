mod args;
mod images;
mod logic;
use clap::Parser;

use args::Args;
// fn main() {
//     let args = Args::parse();

//     let valid_url = match check_url(&args.url) {
//         Some(url) => url.to_string(),
//         None => std::process::exit(1),
//     };

//     match fetch_page(&valid_url) {
//         Ok(html) => {
//             println!("✅ Successfully downloaded html");

//             let images = images::extract_images(&html);
//             let valid_images = images::filter_images(images);

//             if valid_images.is_empty() {
//                 println!("⚠️ No valid image found.");
//             } else {
//                 println!("📸 Found {} valid image(s)", valid_images.len());

//                 let pb = ProgressBar::new(valid_images.len() as u64);
//                 pb.set_style(
//                     ProgressStyle::default_bar()
//                         .template("{msg} {wide_bar} {pos}/{len}")
//                         .expect("Error setting progress bar style"),
//                 );

//                 for url in valid_images {
//                     let full_url = images::get_full_url(&valid_url, &url);
//                     match images::download_image(full_url.as_str(), &args.path) {
//                         Ok(_) => pb.inc(1),
//                         Err(_) => pb.inc(1),
//                     }
//                 }

//                 pb.finish_with_message("Download complete");
//             }
//         }
//         Err(e) => println!("❌ Error during download : {}", e),
//     }
// }

fn main() {
    let args = Args::parse();

    let valid_url = match logic::check_url(&args.url) {
        Some(url) => url.to_string(),
        None => std::process::exit(1),
    };

    if args.rec {
        logic::recursive_crawl(&valid_url, &args, 0);
    } else {
        logic::process_page(&valid_url, &args);
    }
}
