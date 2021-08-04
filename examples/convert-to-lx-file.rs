use anyhow::anyhow;
use std::env;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let path = match env::args().skip(1).next() {
        None => return Err(anyhow!("Missing PATH argument")),
        Some(path) => path,
    };

    let path = PathBuf::from(path);
    let content = std::fs::read_to_string(&path)?;
    let decoded = flarmnet::decode_file(&content)?;

    let file = flarmnet::File {
        version: decoded.version,
        records: decoded
            .records
            .into_iter()
            .filter_map(|it| it.ok())
            .collect(),
    };

    let new_path = path.with_file_name("lx.fln");
    let new_content = flarmnet::lx::encode_file(&file)?;
    std::fs::write(&new_path, &new_content)?;

    Ok(())
}
