use anyhow::{Context, Result};

#[derive(rust_embed::RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() -> Result<()> {
    let file = Data::get("example.txt").context("file not found")?;
    let _data = std::str::from_utf8(file.data.as_ref())?;

    Ok(())
}
