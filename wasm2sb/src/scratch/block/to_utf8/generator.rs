use std::collections::HashMap;

use sb_itchy::{
    block::{BlockFieldBuilder, BlockInputBuilder},
    blocks::{define_custom_block, set_var_to},
    build_context::TargetContext,
    custom_block::{CustomBlockInputType, CustomBlockTy},
    data::ListBuilder,
    uid::Uid,
};
use sb_sbity::{
    block::{Block, BlockInputValue},
    string_hashmap::StringHashMap,
    value::Value,
};

use crate::scratch::sb3::TargetContextWrapper;

use super::unicode::all_unicode;

pub fn to_utf8_generator_list() -> ListBuilder {
    let list_builder_values = all_unicode()
        .chars()
        .map(|c| Value::Text(c.to_string()))
        .collect();
    ListBuilder::new(list_builder_values)
}

pub fn to_utf8_generator(target_ctx: &mut TargetContextWrapper) -> StringHashMap<Block> {
    target_ctx.define_custom_block(vec![CustomBlockInputType::Text("to_utf8".into())], true);
    let stack_builder = define_custom_block("to_utf8");
    // let block_input_builder = BlockInputBuilder::value(BlockInputValue::String { value: Value::Text(all_unicode()) });
    let block_input_builder = BlockInputBuilder::value(BlockInputValue::String {
        value: Value::Text(String::from("t")),
    });
    // println!("all_unicode: {}", all_unicode());
    let stack_builder = stack_builder.next(set_var_to(
        BlockFieldBuilder::new("a".into()),
        block_input_builder,
    ));

    let blocks = stack_builder.build(
        &Uid::generate(),
        &mut HashMap::default(),
        &*target_ctx.get_target_context(),
    );

    let blocks = blocks
        .into_iter()
        .map(|(k, v)| (k.into_inner(), v))
        .collect();

    StringHashMap(blocks)
}
