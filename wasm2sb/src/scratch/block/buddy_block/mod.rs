// management memory system for buddy block

use std::collections::HashMap;

use eyre::Result;
use sb_itchy::uid::Uid;
use sb_sbity::{block::Block, string_hashmap::StringHashMap};

use crate::scratch::sb3::ProjectZip;
use crate::pre_name::PRE_FUNC_NAME;
use super::{
    custom_block_func::CustomBlockInputType, custom_block_stack_builder::CustomStackBuilder,
};

pub fn generate_buddy_block(
    ctx: &ProjectZip,
    n: usize,
    block_size: usize,
) -> Result<StringHashMap<Block>> {
    let pre_name = format!("{PRE_FUNC_NAME}buddy_block_{n}{block_size}");
    let mut blocks: HashMap<Uid, Block> = HashMap::new();

    blocks.extend(generate_buddy_block_init(&ctx, n, block_size, &pre_name));

    Ok(StringHashMap(
        blocks
            .into_iter()
            .map(|(k, v)| (k.inner().to_owned(), v))
            .collect::<HashMap<_, _>>(),
    ))
}

fn generate_buddy_block_init(
    ctx: &ProjectZip,
    n: usize,
    block_size: usize,
    pre_name: &str,
) -> HashMap<Uid, Block> {
    let custom_stack_builder = CustomStackBuilder::new(
        vec![CustomBlockInputType::Text(format!(
            "{pre_name}init",
        ))],
        true,
    );

    custom_stack_builder.build(&mut HashMap::default(), &*ctx.get_target_context())
}
