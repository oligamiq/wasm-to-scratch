use std::io::Read;

use anyhow::Result;

use super::sb3::ProjectZip;
pub fn test_wasm_binary() -> Result<Vec<u8>> {
    let filepath = "./testcode/pkg/testcode_bg.wasm";
    let wasm = std::fs::File::open(filepath)?;
    let data = wasm.bytes().fold(Vec::new(), |mut m, data| {
        m.extend(data);
        m
    });
    Ok(data)
}

pub fn test_project() -> Result<ProjectZip> {
    // let project = Project {
    //     meta: Meta {
    //         semver: "0.0.1".to_string(),
    //         vm: "wasm".to_string(),
    //         agent: "0.0.1".to_string(),
    //     },
    //     targets: Default::default(),
    //     extensions: Default::default(),
    //     monitors: Default::default(),
    // };
    // let project_data_path = "scratch/default.sb3";
    let project_data_path = "scratch/wasmの元.sb3";
    let project = ProjectZip::new(project_data_path.to_string())?;

    Ok(project)
}
