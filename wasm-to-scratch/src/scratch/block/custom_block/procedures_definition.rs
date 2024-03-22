use std::collections::HashMap;

use crate::util::wrap_by_len;

use sb_itchy::{
    block::{BlockFieldBuilder, BlockInputBuilder},
    blocks::*,
    build_context::TargetContext,
};

use sb_sbity::{
    block::{Block, BlockInputValue},
    string_hashmap::StringHashMap,
};
use walrus::{Function, Type, ValType};

use super::{
    custom_block_func::CustomBlockInputType, custom_block_stack_builder::CustomStackBuilder,
};

// https://developer.mozilla.org/ja/docs/WebAssembly/Understanding_the_text_format

pub fn generate_func_block(
    _function: &Function,
    func_type: &Type,
    left_x: i64,
    blocks_y: &mut i64,
    (i, len): (&mut usize, usize),
) -> StringHashMap<Block> {
    let pre_name = "__wasm_internal_func_";
    let params_len = func_type.params().len();
    let name = format!("{}{}", pre_name, wrap_by_len(*i, len));
    let mut func_type = func_type
        .params()
        .iter()
        .enumerate()
        .flat_map(|(k, f)| {
            let mut types = vec![];
            types.push(match f {
                ValType::I32 => CustomBlockInputType::StringOrNumber(format!(
                    "{}_i32",
                    wrap_by_len(k, params_len)
                )),
                ValType::I64 => CustomBlockInputType::StringOrNumber(format!(
                    "{}_i64",
                    wrap_by_len(k, params_len)
                )),
                ValType::F32 => CustomBlockInputType::StringOrNumber(format!(
                    "{}_f32",
                    wrap_by_len(k, params_len)
                )),
                ValType::F64 => CustomBlockInputType::StringOrNumber(format!(
                    "{}_f64",
                    wrap_by_len(k, params_len)
                )),
                ValType::Externref => unimplemented!("Externref"),
                ValType::Funcref => unimplemented!("FuncRef"),
                ValType::V128 => unimplemented!("V128"),
            });
            types
        })
        .collect::<Vec<CustomBlockInputType>>();

    func_type.insert(0, CustomBlockInputType::Text(name.clone()));
    let mut inner_builder = CustomStackBuilder::new(func_type, true);
    inner_builder.set_top_block_position((left_x - 2000) as f64, (*blocks_y) as f64);
    *i += 1;
    *blocks_y += 200;

    inner_builder = inner_builder.next(replace_in_list(
        BlockFieldBuilder::new("__wasm_function_stack".into()),
        BlockInputBuilder::value(BlockInputValue::Integer { value: 1.into() }),
        BlockInputBuilder::value(BlockInputValue::String {
            value: "Hello world".to_owned().into(),
        }),
    ));

    let blocks = inner_builder.build(
        &mut HashMap::default(),
        &mut TargetContext {
            global_vars: &HashMap::default(),
            global_lists: &HashMap::default(),
            this_sprite_vars: &HashMap::default(),
            this_sprite_lists: &HashMap::default(),
            all_broadcasts: &HashMap::default(),
        },
    );

    let blocks = blocks
        .into_iter()
        .map(|(k, v)| (k.into_inner(), v))
        .collect();

    StringHashMap(blocks)
}
