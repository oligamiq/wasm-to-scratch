use std::collections::HashMap;

use sb_sbity::block::Block as BlockData;

pub mod func_event;

pub enum Events {
    FuncEvent(func_event::FuncEvent),
}

impl Events {
    pub fn flush(&mut self) -> HashMap<String, BlockData> {
        match self {
            Self::FuncEvent(event) => event.flush(),
        }
    }
}
