use std::collections::HashMap;

use crate::{scratch::sb3::{ProjectZip, TargetContextWrapper}, util::wrap_by_len, GenCtx};

use sb_itchy::{
    block::{BlockFieldBuilder, BlockInputBuilder},
    blocks::*,
    build_context::TargetContext, func::CustomFuncInputType, stack::StackBuilder, uid::Uid,
};

use sb_sbity::{
    block::{Block, BlockInputValue},
    string_hashmap::StringHashMap,
};
use walrus::{Function, Type, ValType};
use crate::pre_name::PRE_FUNC_NAME;

// https://developer.mozilla.org/ja/docs/WebAssembly/Understanding_the_text_format

impl ProjectZip {
    pub fn generate_func_block(
        &mut self,
        _function: &Function,
        func_type: &Type,
        ctx: &mut GenCtx,
    ) -> StringHashMap<Block> {
        let params_len = func_type.params().len();
        let name = format!("{PRE_FUNC_NAME}{}", ctx.gen_pre_name());
        let mut func_type = func_type
            .params()
            .iter()
            .enumerate()
            .flat_map(|(k, f)| {
                let mut types = vec![];
                types.push(match f {
                    ValType::I32 => CustomFuncInputType::StringOrNumber(format!(
                        "{}_i32",
                        wrap_by_len(k, params_len)
                    )),
                    ValType::I64 => CustomFuncInputType::StringOrNumber(format!(
                        "{}_i64",
                        wrap_by_len(k, params_len)
                    )),
                    ValType::F32 => CustomFuncInputType::StringOrNumber(format!(
                        "{}_f32",
                        wrap_by_len(k, params_len)
                    )),
                    ValType::F64 => CustomFuncInputType::StringOrNumber(format!(
                        "{}_f64",
                        wrap_by_len(k, params_len)
                    )),
                    ValType::Externref => unimplemented!("Externref"),
                    ValType::Funcref => unimplemented!("FuncRef"),
                    ValType::V128 => unimplemented!("V128"),
                });
                types
            })
            .collect::<Vec<CustomFuncInputType>>();

        func_type.insert(0, CustomFuncInputType::Text(name.clone()));
        let mut inner_builder = define_custom_block(func_type, true);
        inner_builder.set_top_block_position(self.get_x() as f64, self.get_y() as f64);
        ctx.update_func_block();
        self.update_y(200);

        inner_builder = inner_builder.next(replace_in_list(
            BlockFieldBuilder::new("__wasm_function_stack".into()),
            BlockInputBuilder::value(BlockInputValue::Integer { value: 1.into() }),
            BlockInputBuilder::value(BlockInputValue::String {
                value: "Hello world".to_owned().into(),
            }),
        ));

        let blocks = inner_builder.build(
            &Uid::generate(),
            &mut HashMap::default(),
            &*self.get_target_context(),
        );

        let blocks = blocks
            .into_iter()
            .map(|(k, v)| (k.into_inner(), v))
            .collect();

        StringHashMap(blocks)
    }
}
