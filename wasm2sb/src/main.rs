use colored::Colorize;
use config::CommandLineArgs;

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

    let (_config, path) =
        CommandLineArgs::parse_and_check().wrap_err("failed to parse command line arguments")?;

    let mut project = test_project().unwrap();
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

    let stack_builder = scratch::block::to_utf8::generator::to_utf8_generator(&mut project);
    project.add_stack_builder(stack_builder);

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
            walrus::FunctionKind::Local(_locals) => {
                println!("local {:?}", function.id());
                // println!("{:?}", locals);
                println!("{:?}", func_type);
                println!("");
            }
            walrus::FunctionKind::Uninitialized(_) => todo!(),
        };

        let stack_builders = project.generate_func_block(function, func_type, &mut ctx);
        project.add_stack_builders(stack_builders);

        // println!("{}", serde_json::to_string(&blocks).unwrap());

        // println!("func: {:#?}", function);

        // if i == 1 {
        //     break;
        // }
    }

    let stack_builders = generate_buddy_block(&mut project, 16, 4)?;
    project.add_stack_builders(stack_builders);

    println!("{}", "project building!".green().bold());

    project.build();

    println!("{}", "project generated successfully!".green().bold());
    println!("{}", "zipping project...".green().bold());

    #[cfg(not(target_arch = "wasm32"))]
    project.zip_file("scratch/out.sb3")?;

    println!("{}", "project zipped successfully!".green().bold());

    // test_exec()?;

    Ok(())
}
