mod cli;
mod montt;
mod statistics;

use std::collections::HashMap;

use clap::Parser;
use cli::{Cli, Commands, ForecastFilter};
use montt::{Montt, Sample};

impl Cli {
    fn run(self, montt: Montt) -> Result<(), Box<dyn std::error::Error>> {
        match self.command {
            Commands::CriticalPath => {
                let cp = montt.critical_path();
                println!("Critical path:\n{:?}", cp);
            }
            Commands::Forecast {
                sample_size,
                filter:
                    ForecastFilter {
                        most_likely,
                        quantile,
                    },
                commands,
            } => {
                let mut samples = HashMap::<usize, usize>::new();
                for i in 0..sample_size {
                    let estimate = montt.sample() as usize;
                    if let Some(s) = samples.get_mut(&estimate) {
                        *s += 1;
                    } else {
                        samples.insert(estimate, 1);
                    }
                    if i % 10000 == 0 {
                        println!("Sampled {} of {}.", i, sample_size);
                    }
                }

                println!("{:?}", samples);
            }
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let montt = Montt::parse(&cli.project)?;
    cli.run(montt)
}
