use arg::Args;
use clap::Parser;

mod arg;
mod error;
mod g_flag;

fn main() {
    let args = Args::parse();

    if let Some(key) = args.generate {
        match g_flag::handle_g(key) {
            Ok(_) => println!("File read successfully!"),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
