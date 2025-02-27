use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(
        short = 'g',
        conflicts_with = "key_flag",
        required_unless_present = "key_flag"
    )]
    pub generate: Option<String>,

    #[arg(
        short = 'k',
        conflicts_with = "generate",
        required_unless_present = "generate"
    )]
    pub key_flag: Option<String>,
}
