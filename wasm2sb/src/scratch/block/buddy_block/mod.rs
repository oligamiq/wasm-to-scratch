// management memory system for buddy block

use eyre::Result;
use sb_itchy::blocks::*;
use sb_itchy::custom_block::CustomBlockInputType;
use sb_itchy::stack::StackBuilder;

use crate::pre_name::PRE_FUNC_NAME;
use crate::scratch::sb3::ProjectZip;

pub fn generate_buddy_block(
    ctx: &mut ProjectZip,
    n: usize,
    block_size: usize,
) -> Result<Vec<StackBuilder>> {
    let pre_name = format!("{PRE_FUNC_NAME}buddy_block_{n}{block_size}");
    let stack_builders = generate_buddy_block_init(ctx, n, block_size, &pre_name);

    Ok(stack_builders)
}

fn generate_buddy_block_init(
    ctx: &mut ProjectZip,
    _n: usize,
    _block_size: usize,
    pre_name: &str,
) -> Vec<StackBuilder> {
    let name = format!("{pre_name}init");
    ctx.define_custom_block(vec![CustomBlockInputType::Text(name.clone())], true);

    let stack_builder = define_custom_block(name);

    vec![stack_builder]
}
