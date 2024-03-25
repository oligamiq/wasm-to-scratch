use std::collections::HashMap;

use crate::wasm::descriptor::Descriptor;

use self::interpreter_descriptor::interpreter_descriptor;

pub mod adjust;
pub mod decode;
pub mod descriptor;
pub mod interpreter_descriptor;
pub mod sb;

use anyhow::Result;

pub fn get_ty(buff: &Vec<u8>) -> Result<HashMap<String, Descriptor>> {
    let module = match wain_syntax_binary::parse(buff) {
        Ok(m) => m,
        Err(err) => {
            return Err(anyhow::anyhow!(err.to_string()));
        }
    }
    .module;

    let prefix = "__wasm_sb_bindgen_describe_";
    let exports = module
        .exports
        .iter()
        .flat_map(|export| {
            if !export.name.0.to_string().starts_with(prefix) {
                return None;
            }

            match &export.kind {
                wain_ast::ExportKind::Func(_) => Some(export.name.0.to_string()),
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    let d = interpreter_descriptor(&module, exports);
    let tys = d
        .iter()
        .map(|(name, d)| {
            let descriptor = Descriptor::decode(d);
            (name[prefix.len()..].to_string(), descriptor)
        })
        .collect::<HashMap<_, _>>();

    for (name, ty) in &tys {
        println!("{}: {:?}", name, ty);
    }

    Ok(tys)
}
