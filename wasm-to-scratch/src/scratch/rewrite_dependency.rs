use sb_sbity::{list::List as SbList, string_hashmap::StringHashMap};

use crate::scratch::generate_id::generate_id;

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
    for item in blocks {
        if !names.contains(&item.name) {
            list.0.insert(generate_id(), item);
        }
    }
}
