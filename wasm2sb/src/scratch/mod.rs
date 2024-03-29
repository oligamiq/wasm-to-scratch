pub mod block;
pub mod generate_id;
pub mod rewrite_dependency;
pub mod sb3;
pub mod test_data;

use std::{io::Read as _, path::PathBuf};

use miette::{NamedSource, Result};

use crate::error::Wasm2SbError;

pub fn wasm_binary<T: Into<PathBuf>>(path: T) -> Result<Vec<u8>> {
    let path: PathBuf = path.into();
    let wasm = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(e) => {
            return Err(Wasm2SbError {
                src: NamedSource::new("mod.rs", "source\n  text\n    here".into()),
                bad_bit: (0, 0).into(),
            }.into());
        }
    };
    let data = wasm.bytes().fold(Vec::new(), |mut m, data| {
        m.extend(data);
        m
    });
    Ok(data)
}
