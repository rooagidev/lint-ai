use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "lint-ai")]
pub struct Args {
    pub path: String,
}

pub fn parse() -> Args {
    Args::parse()
}
