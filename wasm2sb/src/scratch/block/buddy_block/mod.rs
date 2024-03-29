// management memory system for buddy block

use std::collections::HashMap;

use eyre::Result;
use sb_sbity::{block::Block, string_hashmap::StringHashMap};

use crate::scratch::sb3::TargetContextWrapper;

pub fn generate_buddy_block(ctx: &TargetContextWrapper, n: usize, block_size: usize) -> Result<StringHashMap<Block>> {

    Ok(StringHashMap(HashMap::new()))
}
