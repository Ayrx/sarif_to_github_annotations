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

    for run in sarif.runs {
        if let Some(results) = run.results {
            for result in results {
                print_results(result);
            }
        }
    }

    Ok(())
}

fn print_results(r: serde_sarif::sarif::Result) {
    // Assume that the `text` property is always available.
    //
    // 3.11.2 states that either the `text` or `id` property must be present
    // in a message. If this expect ever throws, the lookup logic in 3.11.7
    // needs to be implemented.
    let message = r.message.text.expect("missing `text` from `message`");

    let rule_id = r.rule_id.unwrap();

    // This should never fail as documented in 3.27.12.
    let locations = r.locations.unwrap();

    for location in locations {
        // This tool is meant to be used in the context of GitHub actions,
        // which means that the scan results _most likely_ will have a physical
        // location set.
        let physical_loc = location
            .physical_location
            .expect("missing `physical_location`");

        // Assume the artifact location is present as per 3.29.2. If this
        // expect ever throws, support for the `address` property needs to be
        // implemented.
        let artifact_loc = physical_loc
            .artifact_location
            .expect("missing `artifact_location`");

        // Assume the uri is present as per 3.4.2. If this expect ever throws,
        // support for the `index` property needs to be implemented.
        let uri = artifact_loc.uri.expect("missing `uri`");

        // This should never fail as documented in 3.29.4.
        let region = physical_loc.region.unwrap();

        let start_line = region.start_line.unwrap();
        let end_line = region.end_line.unwrap();

        println!(
            "::notice file={uri},line={start_line},endLine={end_line},title={rule_id}::{message}"
        );
    }
}
