use anyhow::Context;
use clap::Parser;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Options {
    /// Path to the LX format FLN file
    input: PathBuf,

    /// Path to which the decrypted XML file will be written
    #[arg(long)]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    let input_path = &options.input;
    let input = File::open(input_path).context("failed to open input file")?;
    let mut reader = flarmnet::lx::cipher::Reader::new(input);

    let xml_path = options
        .output
        .unwrap_or_else(|| input_path.with_extension("xml"));
    let mut output = File::create(xml_path).context("failed to open output file")?;
    std::io::copy(&mut reader, &mut output).context("failed to write output file")?;

    Ok(())
}
