use anyhow::Result;
use clap::Parser;
use serde_sarif::sarif::Sarif;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file = File::open(args.file)?;
    let reader = BufReader::new(file);

    let sarif: Sarif = serde_json::from_reader(reader)?;

    Ok(())
}
