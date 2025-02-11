use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    pub url: String,

    #[arg(short = 'r', long = "recursive")]
    pub rec: bool,

    #[arg(short = 'l', long = "depth-level", default_value_t = 5)]
    pub depth: u8,

    #[arg(short = 'p', long = "path", default_value = "./data/")]
    pub path: String,
}
