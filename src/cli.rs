use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "lint-ai")]
pub struct Args {
    pub path: String,
    #[arg(long)]
    pub show_concepts: bool,
    #[arg(long)]
    pub show_headings: bool,
    #[arg(long)]
    pub debug_matches: bool,
    #[arg(long)]
    pub config: Option<String>,
    #[arg(long)]
    pub analyze: bool,
}

pub fn parse() -> Args {
    Args::parse()
}
