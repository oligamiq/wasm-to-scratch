use clap::Parser as _;
use config::CommandLineArgs;
use sb_sbity::target::SpriteOrStage;
use scratch::rewrite_dependency::rewrite_list;
use scratch::test_data::test_project;

use anyhow::Result;
use scratch::wasm_binary;
use util::get_preview_rect_from_block;

use crate::scratch::block::procedures_definition::generate_func_block;
use crate::scratch::test_data::test_wasm_binary;
use crate::util::get_type_from_func;

pub mod scratch;
pub mod util;
pub mod wasm;
pub mod config;

fn main() -> Result<()> {
    let opt = CommandLineArgs::parse();

    let mut project = test_project().unwrap();
    let mut sprite = None;

    for target in project.project.targets.iter_mut() {
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
        let blocks = &mut sprite.target.blocks;
        let (left_x, _right_x, top_y, _bottom_y) = get_preview_rect_from_block(blocks);
        // println!("{:#?}", blocks);

        // let data = test_wasm_binary().unwrap();
        let data = test_wasm_binary();

        let ty = wasm::get_ty(&data)?;

        println!("ty: {:?}", ty);

        let mut module = walrus::Module::from_buffer(&data).unwrap();

        let functions_count = module.functions().count() * 2;

        let function_types = &module.types;

        let mut blocks_y = top_y;
        let mut i = 0;
        for function in module.funcs.iter() {
            // len文字の長さで0埋め
            let _name = util::wrap_by_len(i, functions_count);

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

            let block = generate_func_block(
                function,
                func_type,
                left_x,
                &mut blocks_y,
                (&mut i, functions_count),
            );
            blocks.0.extend(block.0);

            // println!("{}", serde_json::to_string(&blocks).unwrap());

            // println!("func: {:#?}", function);

            // if i == 1 {
            //     break;
            // }
        }

        let (utf8_block, _) = scratch::block::to_utf8::generator::to_utf8_generator();
        blocks.0.extend(utf8_block.0);
    }
    // for function in module.functions() {
    //     println!("{:?}", function);
    //     break;
    // }

    #[cfg(not(target_arch = "wasm32"))]
    project.zip_file("scratch/out.sb3")?;

    Ok(())
}
