use config::CommandLineArgs;
use sb_sbity::target::SpriteOrStage;
use scratch::rewrite_dependency::rewrite_list;
use scratch::test_data::test_project;

use crate::util::get_type_from_func;
use eyre::{Result, WrapErr};

pub mod config;
pub mod scratch;
pub mod util;
pub mod wasm;
pub use util::GenCtx;

fn main() -> Result<()> {
    env_logger::init_from_env(
        env_logger::Env::new()
            .filter("WASM2SB_LOG")
            .write_style("WASM2SB_LOG_STYLE"),
    );

    let (config, path) =
        CommandLineArgs::parse_and_check().wrap_err("failed to parse command line arguments")?;

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

        let data = std::fs::read(&path).wrap_err(format!("failed to read file: {:?}", path))?;

        let ty = wasm::get_ty(&data).wrap_err(format!("failed to get type from wasm"))?;

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
