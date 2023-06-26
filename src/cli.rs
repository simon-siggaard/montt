use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version)]
/// Probabilistically forecasting your Gantt chart.
pub struct Cli {
    /// The .montt project description file to use.
    #[arg(short, long, default_value = "project.montt", global = true)]
    pub project: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    CriticalPath,
    Forecast {
        /// The number of samples to use for the forecast.
        #[arg(short = 'n', long, default_value = "1000000", global = true)]
        sample_size: usize,

        #[command(flatten)]
        filter: ForecastFilter,

        #[command(subcommand)]
        commands: ForecastCommands,
    },
}

#[derive(Args, Clone)]
#[group(multiple = false)]
pub struct ForecastFilter {
    #[arg(long, global = true, group = "output_filter")]
    pub most_likely: bool,
    #[arg(long, global = true, group = "output_filter")]
    pub quantile: Option<Vec<f64>>,
}

#[derive(Subcommand, Clone)]
pub enum ForecastCommands {
    /// Forecast the critical path of the project.
    CriticalPath {
        /// Output the critical paths as a Sankey diagram in an HTML-document.
        #[arg(long, conflicts_with = "output_filter")]
        sankey: bool,
    },

    /// Forecast the duration of the project.
    Duration,

    /// Forecast a specific task in the project.
    Task(TaskArgs),
}

#[derive(Args, Clone)]
pub struct TaskArgs {
    #[command(flatten)]
    pub args: TaskCriticalPathArgs,

    /// The name of the task to forecast.
    pub task: String,
}

#[derive(Args, Clone)]
#[group(multiple = false, conflicts_with = "output_filter")]
pub struct TaskCriticalPathArgs {
    /// Output the critical paths this task appears in, as well as the likelihood of each path.
    #[arg(long, conflicts_with = "output_filter")]
    pub critical_paths: bool,

    /// Output the percentage of critical paths this task appears in, as a number between 0 and 1.
    #[arg(long, conflicts_with = "output_filter")]
    pub critical_paths_percentage: bool,
}
