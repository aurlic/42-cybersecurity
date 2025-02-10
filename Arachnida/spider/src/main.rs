use clap::Parser;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
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
            eprintln!("❌ Erreur : URL invalide -> {}", url);
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
                "⚠️ L'URL existe mais a renvoyé un statut HTTP : {}",
                resp.status()
            );
            None
        }
        Err(_) => {
            eprintln!("❌ Erreur : Impossible d'accéder à l'URL -> {}", parsed_url);
            None
        }
    }
}

fn main() {
    let args = Args::parse();

    let valid_url = match check_url(&args.url) {
        Some(url) => url,
        None => std::process::exit(1),
    };

    println!("✅ URL valide et accessible: {}", valid_url);
    println!("Récursif: {}", args.rec);
    println!("Profondeur: {}", args.depth);
    println!("Chemin: {}", args.path);
    // let response = get(args.url).expect("Error during request");
    // let body = response.text().expect("Impossible to read request answer");

    // println!("Page content :\n{}", body);
}
