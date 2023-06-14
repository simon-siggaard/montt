mod montt;

use crate::montt::Montt;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    file: String,
}

fn main() {
    let cli = Cli::parse();
    let montt = Montt::parse(&cli.file).unwrap();
}
