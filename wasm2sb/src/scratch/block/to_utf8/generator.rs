use std::collections::HashMap;

use sb_itchy::{
    block::{BlockFieldBuilder, BlockInputBuilder},
    blocks::set_var_to,
    build_context::TargetContext,
    data::ListBuilder,
};
use sb_sbity::{
    block::{Block, BlockInputValue},
    string_hashmap::StringHashMap,
    value::Value,
};

use crate::scratch::block::custom_block_stack_builder::CustomStackBuilder;

use super::unicode::all_unicode;

pub fn to_utf8_generator() -> (StringHashMap<Block>, ListBuilder) {
    let stack_builder = CustomStackBuilder::new(vec![], true);
    // let block_input_builder = BlockInputBuilder::value(BlockInputValue::String { value: Value::Text(all_unicode()) });
    let block_input_builder = BlockInputBuilder::value(BlockInputValue::String {
        value: Value::Text(String::from("t")),
    });
    // println!("all_unicode: {}", all_unicode());
    let stack_builder = stack_builder.next(set_var_to(
        BlockFieldBuilder::new("a".into()),
        block_input_builder,
    ));

    let list_builder_values = all_unicode()
        .chars()
        .map(|c| Value::Text(c.to_string()))
        .collect();
    let list_builder = ListBuilder::new(list_builder_values);

    let blocks = stack_builder.build(
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

    (StringHashMap(blocks), list_builder)
}
