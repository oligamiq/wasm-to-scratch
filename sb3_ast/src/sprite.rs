use std::collections::HashMap;

use crate::Events;
use sb_sbity::{
    block::Block as BlockData, string_hashmap::StringHashMap, target::Sprite as SpriteData,
};

pub struct Sprite {
    sprite: SpriteData,
    event: Vec<Events>,
}

impl Sprite {
    pub fn new(sprite: SpriteData) -> Self {
        Self {
            sprite,
            event: Vec::new(),
        }
    }

    pub fn push_event(&mut self, event: Events) {
        self.event.push(event);
    }

    pub fn flush(&mut self) {
        let mut blocks = HashMap::new();
        for event in &mut self.event {
            blocks.extend(event.flush());
        }
        self.sprite.target.blocks.0.extend(blocks);
    }
}
