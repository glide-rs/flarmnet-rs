use anyhow::{anyhow, Context};
use std::env;
use std::fs::File;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let path = match env::args().nth(1) {
        None => return Err(anyhow!("Missing PATH argument")),
        Some(path) => path,
    };

    let path = PathBuf::from(path);
    let input = File::open(&path).context("failed to open input file")?;
    let mut reader = flarmnet::lx::cipher::Reader::new(input);

    let xml_path = path.with_extension("xml");
    let mut output = File::create(xml_path).context("failed to open output file")?;
    std::io::copy(&mut reader, &mut output).context("failed to write output file")?;

    Ok(())
}
