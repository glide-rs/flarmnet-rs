use clap::Parser;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Options {
    /// Path to the XCSoar format FLN file
    input: PathBuf,

    /// Path to which the LX format FLN file will be written
    #[arg(long)]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    let input_path = &options.input;
    let content = std::fs::read_to_string(input_path)?;
    let decoded = flarmnet::xcsoar::decode_file(&content)?;

    let file = flarmnet::File {
        version: decoded.version,
        records: decoded
            .records
            .into_iter()
            .filter_map(|it| it.ok())
            .collect(),
    };

    let new_path = options
        .output
        .unwrap_or_else(|| input_path.with_file_name("lx.fln"));
    let new_file = File::create(new_path)?;

    let mut writer = flarmnet::lx::Writer::new(BufWriter::new(new_file));
    writer.write(&file)?;

    Ok(())
}
