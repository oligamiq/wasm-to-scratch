use std::collections::HashMap;

use crate::{
    scratch::sb3::{ProjectZip, TargetContextWrapper},
    util::wrap_by_len,
    GenCtx,
};

use sb_itchy::{
    block::{BlockFieldBuilder, BlockInputBuilder},
    blocks::*,
    build_context::TargetContext,
    custom_block::{CustomBlockInputType, CustomBlockTy},
    stack::StackBuilder,
    uid::Uid,
};

use crate::pre_name::PRE_FUNC_NAME;
use sb_sbity::{
    block::{Block, BlockInputValue},
    string_hashmap::StringHashMap,
};
use walrus::{Function, Type, ValType};

// https://developer.mozilla.org/ja/docs/WebAssembly/Understanding_the_text_format

impl ProjectZip {
    pub fn generate_func_block(
        &mut self,
        _function: &Function,
        func_type: &Type,
        ctx: &mut GenCtx,
    ) -> Vec<StackBuilder> {
        let params_len = func_type.params().len();
        let name = format!("{PRE_FUNC_NAME}{}", ctx.gen_pre_name());
        let mut func_type = func_type
            .params()
            .iter()
            .enumerate()
            .flat_map(|(k, f)| {
                let mut types = vec![];
                types.push(match f {
                    ValType::I32 => CustomBlockInputType::StringOrNumber(format!(
                        "{}_i32",
                        wrap_by_len(k, params_len)
                    )),
                    ValType::I64 => CustomBlockInputType::StringOrNumber(format!(
                        "{}_i64",
                        wrap_by_len(k, params_len)
                    )),
                    ValType::F32 => CustomBlockInputType::StringOrNumber(format!(
                        "{}_f32",
                        wrap_by_len(k, params_len)
                    )),
                    ValType::F64 => CustomBlockInputType::StringOrNumber(format!(
                        "{}_f64",
                        wrap_by_len(k, params_len)
                    )),
                    ValType::Externref => unimplemented!("Externref"),
                    ValType::Funcref => unimplemented!("FuncRef"),
                    ValType::V128 => unimplemented!("V128"),
                });
                types
            })
            .collect::<Vec<CustomBlockInputType>>();

        func_type.insert(0, CustomBlockInputType::Text(name.clone()));
        self.define_custom_block(func_type, true);

        let mut inner_builder = define_custom_block(name);
        ctx.update_func_block();

        inner_builder = inner_builder.next(replace_in_list(
            BlockFieldBuilder::new("__wasm_function_stack".into()),
            BlockInputBuilder::value(BlockInputValue::Integer { value: 1.into() }),
            BlockInputBuilder::value(BlockInputValue::String {
                value: "Hello world".to_owned().into(),
            }),
        ));

        vec![inner_builder]
    }
}
