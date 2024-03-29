use clap::Parser as _;
use config::CommandLineArgs;
use sb_sbity::target::SpriteOrStage;
use scratch::rewrite_dependency::rewrite_list;
use scratch::test_data::test_project;

use miette::{NamedSource, Result};
use scratch::wasm_binary;
use util::get_preview_rect_from_block;

use crate::error::Wasm2SbError;
use crate::scratch::test_data::test_wasm_binary;
use crate::util::get_type_from_func;

pub mod config;
pub mod error;
pub mod scratch;
pub mod util;
pub mod wasm;
pub use util::GenCtx;

fn main() -> Result<()> {
    env_logger::init_from_env(
        env_logger::Env::new()
            .filter("DINGHY_LOG")
            .write_style("DINGHY_LOG_STYLE"),
    );

    let (config, path) = CommandLineArgs::parse_and_check()?;

    let mut project = test_project().unwrap();
    let mut sprite = None;

    let internal_project = project.project.clone();
    let mut internal_project = internal_project.write();
    for target in internal_project.targets.iter_mut() {
        match target {
            SpriteOrStage::Sprite(sprite_impl) => {
                sprite = Some(sprite_impl);
                break;
            }
            SpriteOrStage::Stage(stage_impl) => {
                rewrite_list(&mut stage_impl.target.lists);
            }
        }
    }

    if let Some(sprite) = sprite {
        // println!("{:#?}", blocks);

        let data = match std::fs::read(&path) {
            Ok(data) => data,
            Err(e) => {
                return Err(Wasm2SbError {
                    src: NamedSource::new("main.rs", "source2\n  text\n    here".into()),
                    bad_bit: (2, 1).into(),
                }
                .into());
            }
        };

        let ty = wasm::get_ty(&data)?;

        println!("ty: {:?}", ty);

        let mut module = walrus::Module::from_buffer(&data).unwrap();

        let function_types = &module.types;

        let mut ctx = GenCtx::new();

        let (utf8_block, _) = scratch::block::to_utf8::generator::to_utf8_generator();

        for function in module.funcs.iter() {
            // println!("{:?}", function.idx);
            // println!("{:?}", function.start);

            let func_type = get_type_from_func(&function, function_types);
            // println!("function: {:?}", function);
            // println!("func_type: {:?}", func_type);

            match &function.kind {
                walrus::FunctionKind::Import(import) => {
                    println!("import {:?}", function.id());
                    println!("{:?}", import);
                }
                walrus::FunctionKind::Local(locals) => {
                    println!("local {:?}", function.id());
                    // println!("{:?}", locals);
                    println!("{:?}", func_type);
                    println!("");
                }
                walrus::FunctionKind::Uninitialized(_) => todo!(),
            };

            let block = project.generate_func_block(function, func_type, &mut ctx);
            sprite.target.blocks.0.extend(block.0);

            // println!("{}", serde_json::to_string(&blocks).unwrap());

            // println!("func: {:#?}", function);

            // if i == 1 {
            //     break;
            // }
        }

        sprite.target.blocks.0.extend(utf8_block.0);
    }
    // for function in module.functions() {
    //     println!("{:?}", function);
    //     break;
    // }

    #[cfg(not(target_arch = "wasm32"))]
    project.zip_file("scratch/out.sb3")?;

    Ok(())
}
