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

    let xml_path = path.with_extension("xml");
    let decrypted = flarmnet::lx::cipher::decrypt(&content)?;
    std::fs::write(&xml_path, &decrypted)?;

    Ok(())
}
