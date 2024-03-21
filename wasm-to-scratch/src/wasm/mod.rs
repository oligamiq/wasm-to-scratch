use wain_ast::Module;

use crate::wasm::descriptor::{Descriptor, Function};

use self::interpreter_descriptor::interpreter_descriptor;

pub mod decode;
pub mod descriptor;
pub mod interpreter_descriptor;
pub mod sb;

pub fn get_ty(module: &mut Module) -> Vec<(String, Descriptor)> {
    let prefix = "__wasm_sb_bindgen_describe_";
    let exports = module
        .exports
        .iter()
        .flat_map(|export| {
            if !export.name.0.starts_with(prefix) {
                return None;
            }

            let _ = match export.kind {
                wain_ast::ExportKind::Func(_) => Some(export.name.0.to_string()),
                _ => None,
            };

            Some(export.name.0.to_string())
        })
        .collect::<Vec<_>>();
    let d = interpreter_descriptor(module, exports);
    let tys = d.iter().map(|(name, d)| {
        let descriptor = Descriptor::decode(d);
        (name[prefix.len()..].to_string(), descriptor)
    }).collect::<Vec<_>>();

    for (name, ty) in &tys {
        println!("{}: {:?}", name, ty);
    }

    tys
}
