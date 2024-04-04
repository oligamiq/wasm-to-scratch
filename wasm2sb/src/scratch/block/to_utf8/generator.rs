use std::collections::HashMap;

use sb_itchy::{
    block::{BlockFieldBuilder, BlockInputBuilder},
    blocks::{define_custom_block, set_var_to},
    build_context::TargetContext,
    custom_block::{CustomBlockInputType, CustomBlockTy},
    data::ListBuilder,
    stack::StackBuilder,
    uid::Uid,
};
use sb_sbity::{
    block::{Block, BlockInputValue},
    string_hashmap::StringHashMap,
    value::Value,
};

use crate::scratch::sb3::{ProjectZip, TargetContextWrapper};

use super::{
    check_uppercase::check_uppercase_func_generator,
    unicode::{all_unicode, all_unicode_upper_letter_case},
};

type BiB = BlockInputBuilder;
type BiV = BlockInputValue;

pub fn to_utf8_generator(target_ctx: &mut ProjectZip) -> StackBuilder {
    check_uppercase_func_generator(target_ctx);

    target_ctx.define_custom_block(
        vec![
            CustomBlockInputType::Text("to_utf8".into()),
            CustomBlockInputType::StringOrNumber("str".into()),
        ],
        true,
    );
    let stack_builder = define_custom_block("to_utf8");
    let block_input_builder = BiB::value(BiV::String {
        value: String::from("t").into(),
    });
    let stack_builder = stack_builder.next(set_var_to(
        BlockFieldBuilder::new("a".into()),
        block_input_builder,
    ));

    stack_builder
}
