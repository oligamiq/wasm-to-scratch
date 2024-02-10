use std::collections::HashMap;

use sb_sbity::{block::Block as BlockData, string_hashmap::StringHashMap};

use crate::InputBlock;

pub struct Block {
    block: BlockData,
    inner_blocks: StringHashMap<BlockData>,
    input_blocks: StringHashMap<InputBlock>,
    before_block_id: String,
}

impl Block {
    pub fn flush(&mut self) -> HashMap<String, BlockData> {
        let mut define_blocks = HashMap::new();
        define_blocks.extend(self.inner_blocks.0.clone());
        define_blocks
    }
}
