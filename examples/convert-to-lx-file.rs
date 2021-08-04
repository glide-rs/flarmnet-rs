use anyhow::anyhow;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let path = match env::args().skip(1).next() {
        None => return Err(anyhow!("Missing PATH argument")),
        Some(path) => path,
    };

    let path = PathBuf::from(path);
    let content = std::fs::read_to_string(&path)?;
    let decoded = flarmnet::xcsoar::decode_file(&content)?;

    let file = flarmnet::File {
        version: decoded.version,
        records: decoded
            .records
            .into_iter()
            .filter_map(|it| it.ok())
            .collect(),
    };

    let new_path = path.with_file_name("lx.fln");
    let new_file = File::create(new_path)?;

    let mut writer = flarmnet::lx::Writer::new(BufWriter::new(new_file));
    writer.write(&file)?;

    Ok(())
}
