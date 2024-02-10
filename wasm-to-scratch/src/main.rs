use sb_sbity::target::SpriteOrStage;
use scratch::rewrite_dependency::rewrite_list;
use scratch::test_data::test_project;

use util::get_preview_rect_from_block;

use crate::scratch::block::procedures_definition::generate_func_block;
use crate::scratch::test_data::test_wasm_binary;

pub mod scratch;
pub mod util;

fn main() {
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

        let data = test_wasm_binary().unwrap();
        let wain = match wain_syntax_binary::parse(&data) {
            Ok(wasm) => wasm,
            Err(_) => {
                println!("error");
                return;
            }
        };

        let module = wain.module;

        let functions_count = module.funcs.len() * 2;

        let function_types = &module.types;

        let mut blocks_y = top_y;
        let mut i = 0;
        for (func_index, function) in module.funcs.iter().enumerate() {
            // len文字の長さで0埋め
            let _name = util::wrap_by_len(i, functions_count);

            // println!("{:?}", function.idx);
            // println!("{:?}", function.start);

            let func_type = util::get_type_from_func(function, function_types);
            match &function.kind {
                wain_ast::FuncKind::Import(import) => {
                    println!("import {func_index}");
                    println!("{:?}", import);
                }
                wain_ast::FuncKind::Body { locals, expr: _ } => {
                    println!("local {func_index}");
                    println!("{:?}", locals);
                    println!("{:?}", func_type);
                    println!("");
                }
            }

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
    }
    // for function in module.functions() {
    //     println!("{:?}", function);
    //     break;
    // }

    project.zip("scratch/out.sb3").unwrap();
}
