pub mod block;
pub mod generate_id;
pub mod rewrite_dependency;
pub mod sb3;
pub mod test_data;

use std::{io::Read as _, path::PathBuf};

use eyre::{Context, Result};

pub fn wasm_binary<T: Into<PathBuf>>(path: T) -> Result<Vec<u8>> {
    let path: PathBuf = path.into();
    let wasm = std::fs::File::open(path).wrap_err("failed to open wasm file")?;
    let data = wasm.bytes().fold(Vec::new(), |mut m, data| {
        m.extend(data);
        m
    });
    Ok(data)
}
