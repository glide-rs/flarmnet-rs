use anyhow::Context;
use clap::Parser;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Options {
    /// Path to the decrypted LX format XML file
    input: PathBuf,

    /// Path to which the decrypted XML file will be written
    #[arg(long)]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    let input_path = &options.input;
    let mut input = File::open(input_path).context("failed to open input file")?;

    let xml_path = options
        .output
        .unwrap_or_else(|| input_path.with_extension("fln"));
    let output = File::create(xml_path).context("failed to open output file")?;
    let mut writer = flarmnet::lx::cipher::Writer::new(output);
    std::io::copy(&mut input, &mut writer).context("failed to write output file")?;

    Ok(())
}
