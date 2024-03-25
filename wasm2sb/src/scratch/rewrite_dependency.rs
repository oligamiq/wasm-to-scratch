use sb_sbity::{list::List as SbList, string_hashmap::StringHashMap};

use crate::scratch::generate_id::generate_id;

use super::block::to_utf8::{generator::to_utf8_generator, unicode::all_unicode};

pub fn rewrite_list(list: &mut StringHashMap<SbList>) {
    let names = list.0.keys().cloned().collect::<Vec<String>>();
    let blocks: Vec<SbList> = vec![
        SbList {
            name: "__wasm_global_stack".into(),
            values: Vec::new(),
        },
        SbList {
            name: "__wasm_local_stack".into(),
            values: Vec::new(),
        },
        SbList {
            name: "__wasm_function_stack".into(),
            values: Vec::new(),
        },
    ];
    let (_, utf8_list) = to_utf8_generator();
    let (utf8_list_list, utf8_list_id) = utf8_list.build("utf8".into());
    list.0.insert(utf8_list_id.into_inner(), utf8_list_list);

    for item in blocks {
        if !names.contains(&item.name) {
            list.0.insert(generate_id(), item);
        }
    }
}
