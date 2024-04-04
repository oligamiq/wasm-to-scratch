use sb_itchy::data::ListBuilder;
use sb_sbity::{list::List as SbList, string_hashmap::StringHashMap};

use crate::scratch::generate_id::generate_id;

use super::{block::to_utf8::generator::to_utf8_generator_list, sb3::ProjectZip};

pub fn rewrite_list(ctx: &mut ProjectZip) {
    let lists = vec!["__wasm_global_stack".into(), "__wasm_local_stack".into(), "__wasm_function_stack".into()];
    for list in lists {
        ctx.add_list_builder(list, ListBuilder::new(Vec::new()));
    }
}
