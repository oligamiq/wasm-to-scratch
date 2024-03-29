use tempfile::NamedTempFile;
use walrus::Module;
use eyre::Result;
use wasm_opt::OptimizationOptions;

pub fn rm_export_fn(module: &mut Module, rm_types: Vec<String>) -> Result<()> {
    let prefix = "__wasm_sb_bindgen_describe_";

    for ty in rm_types {
        let name = format!("{}{}", prefix, ty);
        module.exports.remove(&name).map_err(|e| eyre::eyre!("{:?}", e.to_string()))?;
    }

    module.exports.remove("__wasm_sb_bindgen_placeholder_anchor__").map_err(|e| eyre::eyre!("{:?}", e.to_string()))?;

    Ok(())
}

pub fn check_rm_import_fn(module: &Module) -> Result<()> {
    let scheme_version_id = match module
        .imports.iter().find(|import| {
            import.name.starts_with("schema_version_")
    }) {
        Some(_) => return Err(eyre::eyre!("schema_version_ found")),
        None => (),
    };

    let wasm_sb_bindgen_version_id = match module
        .imports.iter().find(|import| {
            import.name.starts_with("wasm_sb_bindgen_version_")
    }) {
        Some(_) => return Err(eyre::eyre!("wasm_sb_bindgen_version_ found")),
        None => (),
    };

    let describe_id = match module
        .imports.iter().find(|export| {
            export.name.starts_with("__wasm_sb_bindgen_describe")
    }) {
        Some(_) => return Err(eyre::eyre!("__wasm_sb_bindgen_describe found")),
        None => (),
    };

    Ok(())
}

pub fn wasm_opt_module(mut module: Module) -> Result<Module> {
    let tmp = NamedTempFile::new()?;

    // save
    module.emit_wasm_file(tmp.path()).map_err(|e| eyre::eyre!("{:?}", e.to_string()))?;

    OptimizationOptions::new_optimize_for_size_aggressively().run(
        tmp.path(),
        tmp.path(),
    )?;

    // load
    let module = walrus::Module::from_file(tmp.path()).map_err(|e| eyre::eyre!("{:?}", e.to_string()))?;

    Ok(module)
}
