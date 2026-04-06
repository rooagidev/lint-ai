mod cli;
mod engine;
mod graph;
mod report;
mod rules;

use anyhow::Result;

fn main() -> Result<()> {
    let args = cli::parse();
    engine::run(args)?;
    Ok(())
}
