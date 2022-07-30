use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    file: PathBuf
}

fn main() {
    let args = Args::parse();
}
