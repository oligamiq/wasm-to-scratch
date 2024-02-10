use sb_sbity::{block::Block, string_hashmap::StringHashMap, target::SpriteOrStage};
use scratch::rewrite_dependency::rewrite_list;
use scratch::test_data::test_project;

use util::get_preview_rect_from_block;
use wasm_ast::{parser, Function, FunctionType};

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
            _ => {}
        }
    }
    if let Some(sprite) = sprite {
        let blocks = &mut sprite.target.blocks;
        let (left_x, right_x, top_y, bottom_y) = get_preview_rect_from_block(blocks);
        // println!("{:#?}", blocks);

        let data = test_wasm_binary().unwrap();
        let mut win = match wain_syntax_binary::parse(&data) {
            Ok(wasm) => wasm,
            Err(_) => {
                println!("error");
                return;
            }
        };
        // println!("{:?}", win.module);
        for item in &win.module.funcs {
            println!("{:?}", item.idx);
            println!("{:?}", item.start);
            println!("{:?}", item.kind);
            match &item.kind {
                wain_ast::FuncKind::Import(import) => {
                    // println!("import");
                    // println!("{:?}", import);
                }
                wain_ast::FuncKind::Body { locals, expr } => {
                    // println!("body");
                    // println!("{:?}", locals);
                    // println!("{:?}", expr);
                }
            }
        }

        let module = parser::parse_binary(data.as_slice()).unwrap();

        // println!("{:#?}", module.function_types());

        // println!("{:?}", module);

        let functions_count = module.functions().unwrap().len() * 2;

        let mut blocks_y = top_y;
        let mut i = 0;
        if let Some(function) = module.functions() {
            for function in function {
                // len文字の長さで0埋め
                let name = util::wrap_by_len(i, functions_count);

                // let function_type = get_type_from_func(&function, module.function_types().unwrap());

                // let name = function.kind();
                let body = function.body();
                // let locals = function.locals();
                // println!("{:?}", name);
                // println!("{:?}", locals);
                // let instructions = body.instructions();
                // println!("{:?}", body);
                // for instruction in instructions {
                //     println!("{:?}", instruction);
                // }
                let block =
                    generate_func_block(function, left_x, &mut blocks_y, (&mut i, functions_count));
                blocks.0.extend(block.0);

                // println!("{}", serde_json::to_string(&blocks).unwrap());

                // println!("func: {:#?}", function);

                if i == 1 {
                    break;
                }
            }
        }
    }
    // for function in module.functions() {
    //     println!("{:?}", function);
    //     break;
    // }

    project.zip("scratch/out.sb3").unwrap();
}
