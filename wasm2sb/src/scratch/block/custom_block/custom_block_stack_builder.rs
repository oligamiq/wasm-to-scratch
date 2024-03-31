use std::collections::HashMap;

use sb_itchy::{block::BlockBuilder, build_context::TargetContext, stack::StackBuilder, uid::Uid};
use sb_sbity::{
    block::{Block, BlockNormal},
    comment::Comment,
};

use super::custom_block_func::{generate_custom_block, CustomBlockInputType};

#[derive(Debug, Clone, PartialEq)]
pub struct CustomStackBuilder {
    pub first_block: (String, BlockNormal),
    pub stack: Vec<BlockBuilder>,
    pub stack_b: HashMap<Uid, Block>,
}

impl CustomStackBuilder {
    pub fn new(input: Vec<CustomBlockInputType>, warp: bool) -> CustomStackBuilder {
        let ((id, first_block), stack) = generate_custom_block(input, warp);
        CustomStackBuilder {
            first_block: (id.into_inner(), first_block),
            stack: vec![],
            stack_b: stack,
        }
    }

    pub fn next(mut self, mut next_stack: StackBuilder) -> CustomStackBuilder {
        self.stack.append(&mut next_stack.stack);
        self
    }

    pub fn set_top_block_position(&mut self, x: f64, y: f64) -> &mut Self {
        self.first_block.1.x = Some(sb_sbity::value::Number::Float(x));
        self.first_block.1.y = Some(sb_sbity::value::Number::Float(y));
        self
    }

    pub fn build(
        self,
        comment_buff: &mut HashMap<Uid, Comment>,
        target_context: &TargetContext,
    ) -> HashMap<Uid, Block> {
        let mut stack_b = self.stack_b.clone();
        let mut previous_block = (self.first_block.1, Uid::new(self.first_block.0));
        for block_builder2 in self.stack {
            let (mut block1, block1_uid) = previous_block;
            let block2_uid = Uid::generate();
            let Block::Normal(mut block2) =
                block_builder2.build(&block2_uid, comment_buff, &mut stack_b, target_context)
            else {
                unreachable!("BlockVarList shouldn't exist here")
            };

            block1.next = Some(block2_uid.clone().into_inner());
            block2.parent = Some(block1_uid.clone().into_inner());

            previous_block = (block2, block2_uid);

            stack_b.insert(block1_uid, Block::Normal(block1));
        }
        stack_b.insert(previous_block.1, Block::Normal(previous_block.0));
        stack_b
    }
}
