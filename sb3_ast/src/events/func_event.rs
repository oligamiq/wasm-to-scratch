use std::collections::HashMap;

use sb_sbity::{block::Block as BlockData, string_hashmap::StringHashMap};

use crate::Block;

pub struct FuncEvent {
    blocks: Vec<(String, Block)>,
    this_block_id: String,
    define_blocks: StringHashMap<BlockData>,
}

impl FuncEvent {
    pub fn define() -> Self {
        Self {}
    }

    pub fn flush(&mut self) -> HashMap<String, BlockData> {
        let mut define_blocks = HashMap::new();
        for (block_id, block) in &mut self.blocks {
            define_blocks.extend(block.flush());
        }
        define_blocks
    }
}
