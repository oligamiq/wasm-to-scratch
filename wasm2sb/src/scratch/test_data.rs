use std::io::Read;

use anyhow::Result;
use sb_itchy::target;

use super::sb3::ProjectZip;

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
    let data = include_bytes!("../../../scratch/wasmの元.sb3");
    let project = ProjectZip::new_from_data(project_data_path.to_string(), data.to_vec())?;

    Ok(project)
}

pub fn test_wasm_binary() -> Vec<u8> {
    let bytes = include_bytes!(
        "../../wasm_sb_bindgen_testcode.wasm"
    );
    bytes.to_vec()
}
