mod montt;

use crate::montt::Montt;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// The .montt project description file to use.
    #[arg(short, long, default_value = "project.montt")]
    project: String,

    /// Don't forecast, even if estimate quantiles are provided.
    #[arg(long, default_value = "false")]
    no_forecast: bool,

    /// The number of random samples to take when forecasting.
    #[arg(short = 'n', long, default_value = "1000000")]
    sample_size: usize,

    /// The output format to use.
    #[arg(short, long, default_value = "json")]
    output: Output,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Output {
    /// Output a Gantt chart in the form of a mermaid diagram.
    Mermaid,
    /// Output an HTML document containing a sankey representation of the forecast.
    Sankey,
    /// Output the forecast as a JSON document.
    Json,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let montt = Montt::parse(&cli.project)?;
    Ok(())
}
