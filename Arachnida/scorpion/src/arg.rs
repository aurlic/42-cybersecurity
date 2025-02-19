use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(required = true)]
    pub files: Vec<String>,
}
