use std::collections::HashMap;

use crate::wasm::descriptor::Descriptor;

use colored::Colorize as _;
use eyre::{Context, Result, eyre};

use self::interpreter_descriptor::interpreter_descriptor;

pub mod adjust;
pub mod decode;
pub mod descriptor;
pub mod interpreter_descriptor;
pub mod sb;
pub mod scheme_versions;

pub fn load_schema_version(module: &wain_ast::Module) -> Result<()> {
    for func in &module.funcs {
        match &func.kind {
            wain_ast::FuncKind::Import(import) => {
                if import.name.0.starts_with("schema_version_") {
                    let name = import.name.0[15..].to_string();
                    let version = name.replace("_", ".");
                    let version = version.parse::<semver::Version>().unwrap();
                    let versions = scheme_versions::scheme_versions();
                    let max_version = versions.last().unwrap();
                    let pre_version = &versions[versions.len() - 2];
                    if version > *max_version {
                        return Err(eyre!("schema version is too new: {} > {}", version, max_version))
                    }
                    if version < *pre_version {
                        return Err(eyre!("schema version is too old: {} < {}", version, pre_version))
                    }
                    return Ok(())
                }
            }
            _ => ()
        }
    };

    Err(eyre!("schema version not found"))
}

pub fn get_ty(buff: &Vec<u8>) -> Result<HashMap<String, Descriptor>> {
    let module = match wain_syntax_binary::parse(buff) {
        Ok(m) => m,
        Err(err) => {
            return Err(eyre::eyre!("{:?}", err.to_string()))
                .wrap_err(format!("failed to parse wasm binary"));
        }
    }
    .module;

    load_schema_version(&module)?;
    println!("{}", "schema version loaded successfully!".green().bold());

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
