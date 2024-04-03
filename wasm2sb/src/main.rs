use std::collections::HashMap;

use colored::Colorize;
use config::CommandLineArgs;
use sb_itchy::{
    block::{BlockFieldBuilder, BlockInputBuilder},
    blocks::{self, *},
    uid::Uid,
};
use sb_sbity::{block::BlockInputValue, target::SpriteOrStage};
use scratch::rewrite_dependency::rewrite_list;
use scratch::test_data::test_project;

use crate::{
    scratch::block::buddy_block::generate_buddy_block,
    util::get_type_from_func,
    wasm::adjust::{check_rm_import_fn, rm_export_fn, wasm_opt_module},
};
use eyre::{Result, WrapErr};

pub mod config;
pub mod pre_name;
pub mod scratch;
pub mod test_exec;
pub mod util;
pub mod wasm;
#[allow(unused_imports)]
use crate::test_exec::test_exec;
pub use util::GenCtx;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // 🌠

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

    let mut sprite = sprite.unwrap();

    // println!("{:#?}", blocks);

    let data = std::fs::read(&path).wrap_err(format!("failed to read file: {:?}", path))?;

    let ty = wasm::get_ty(&data).wrap_err(format!("failed to get type from wasm"))?;
    println!(
        "{}",
        "original func type loaded successfully!".green().bold()
    );

    // println!("ty: {:?}", ty);

    let mut module = walrus::Module::from_buffer(&data).unwrap();
    println!("{}", "module loaded successfully!".green().bold());
    rm_export_fn(&mut module, ty.keys().map(|k| k.to_string()).collect())?;
    println!(
        "{}",
        "describe function removed successfully!".green().bold()
    );
    let module = wasm_opt_module(module)?;
    println!("{}", "module optimized successfully!".green().bold());
    check_rm_import_fn(&module)?;

    log::info!("module: {:#?}", module.imports);
    log::info!("module: {:#?}", module.exports);

    let function_types = &module.types;

    let mut ctx = GenCtx::new();
    ctx.functions_count = module.funcs.iter().count() + module.exports.iter().count();

    let utf8_block =
        scratch::block::to_utf8::generator::to_utf8_generator(project.target_context_mut());

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

    let blocks = generate_buddy_block(&mut project, 16, 4)?;
    sprite.target.blocks.0.extend(blocks.0);

    // let translater = translate_to(
    //     // BlockInputBuilder::value(sb_sbity::block::BlockInputValue::String {
    //     //     value: sb_sbity::value::Value::Text(String::from("hello world")),
    //     // }),
    //     BlockInputBuilder::stack(get_viewer_language()),
    //     "ja",
    // );
    // let blocks = translater.build(
    //     &Uid::generate(),
    //     &mut std::collections::HashMap::default(),
    //     &*project.get_target_context(),
    // );
    // sprite.target.blocks.0.extend(
    //     blocks
    //         .iter()
    //         .map(|(i, b)| (i.clone().into_inner(), b.clone()))
    //         .collect::<Vec<_>>(),
    // );

    // for function in module.functions() {
    //     println!("{:?}", function);
    //     break;
    // }

    {
        let blocks = when_flag_clicked().next(call_custom_block::<_, String>(
            "__wasm_internal_func_20",
            HashMap::default(),
        ));
        let blocks = blocks.next(set_x(BlockInputBuilder::stack(custom_block_var_boolean(
            "c",
        ))));
        let blocks = blocks.next(set_var_to(
            BlockFieldBuilder::new("t".into()),
            BlockInputBuilder::stack(custom_block_var_string_number("b")),
        ));
        let blocks = blocks.next(set_var_to(
            BlockFieldBuilder::new("k".into()),
            BlockInputBuilder::stack(global_var("t")),
        ));
        let blocks = blocks.build(
            &Uid::generate(),
            &mut HashMap::default(),
            &*project.get_target_context(),
        );
        sprite.target.blocks.0.extend(
            blocks
                .iter()
                .map(|(i, b)| (i.clone().into_inner(), b.clone()))
                .collect::<Vec<_>>(),
        );
    }

    std::mem::drop(internal_project);
    println!("{}", "project generated successfully!".green().bold());
    println!("{}", "zipping project...".green().bold());

    #[cfg(not(target_arch = "wasm32"))]
    project.zip_file("scratch/out.sb3")?;

    println!("{}", "project zipped successfully!".green().bold());

    // test_exec()?;

    Ok(())
}
