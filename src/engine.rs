use crate::graph::Graph;
use crate::report::Report;
use crate::rules::orphan_pages::check_orphans;
use anyhow::Result;

pub fn run(args: crate::cli::Args) -> Result<()> {
    let graph = Graph::build(&args.path)?;
    let mut report = Report::new();

    check_orphans(&graph, &mut report);

    report.print();
    Ok(())
}
