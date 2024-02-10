use std::collections::HashMap;
use std::fmt::format;

use crate::scratch::block::function_code::generate_func_block_code;
use crate::util::wrap_by_len;

use crate::scratch::generate_id::generate_id;
use sb_itchy::block::BlockBuilder;
use sb_itchy::build_context::TargetContext;
use sb_itchy::stack::StackBuilder;
use sb_itchy::target::SpriteBuilder;
use sb_itchy::uid::Uid;
use sb_sbity::block::BlockField::WithId;
use sb_sbity::block::BlockMutationEnum::ProceduresPrototype;
use sb_sbity::block::UidOrValue;
use sb_sbity::value::Value::Text;
use sb_sbity::value::ValueWithBool;
use sb_sbity::{
    block::{Block, BlockInput, BlockMutation, BlockNormal, ShadowInputType},
    string_hashmap::StringHashMap,
};
use wain_ast::{Func, FuncType, ValType};

use super::custom_block_func::CustomBlockInputType;
use super::custom_block_stack_builder::CustomStackBuilder;

// https://developer.mozilla.org/ja/docs/WebAssembly/Understanding_the_text_format

pub fn generate_func_block(
    function: &Func,
    func_type: &FuncType,
    left_x: i64,
    blocks_y: &mut i64,
    (i, len): (&mut usize, usize),
) -> StringHashMap<Block> {
    let pre_name = "__wasm_internal_func_";
    let params_len = func_type.params.len();
    let name = format!("{}{}", pre_name, wrap_by_len(*i, len));
    let mut func_type = func_type
        .params
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
            });
            types
        })
        .collect::<Vec<CustomBlockInputType>>();

    func_type.insert(0, CustomBlockInputType::Text(name.clone()));
    let mut inner_builder = CustomStackBuilder::new(func_type);
    inner_builder.set_top_block_position((left_x - 2000) as f64, (*blocks_y) as f64);
    *i += 1;
    *blocks_y += 200;

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
