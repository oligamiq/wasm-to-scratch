use crate::util::wrap_by_len;

use crate::scratch::generate_id::generate_id;
use sb_sbity::block::BlockField::WithId;
use sb_sbity::block::BlockMutationEnum::ProceduresPrototype;
use sb_sbity::block::UidOrValue;
use sb_sbity::value::Value::Text;
use sb_sbity::value::ValueWithBool;
use sb_sbity::{
    block::{Block, BlockInput, BlockMutation, BlockNormal, ShadowInputType},
    string_hashmap::StringHashMap,
};
use wasm_ast::{Function, FunctionType, ResultType, ValueType};

use super::function_code::generate_func_block_code;

// https://developer.mozilla.org/ja/docs/WebAssembly/Understanding_the_text_format

pub fn generate_func_block(
    function: &Function,
    left_x: i64,
    blocks_y: &mut i64,
    (i, len): (&mut usize, usize),
) -> StringHashMap<Block> {
    let (wrapper_blocks, wrapper_id) =
        generate_func_block_impl(function, wrap_by_len(*i, len), left_x, blocks_y);
    *i += 1;

    // let

    wrapper_blocks
}

// pub fn generate_func_block_wrapper(
//     function: &Function,
//     left_x: i64,
//     blocks_y: &mut i64,
//     internal_id: String,
//     (i, len): (&mut usize, usize),
// ) -> (StringHashMap<Block>, String) {
//     let (blocks, this_block_id) = generate_func_block_impl(function, wrap_by_len(*i, len), left_x, blocks_y);

//     blocks.0.insert(

//     );
// }

pub fn generate_func_block_impl(
    function: &Function,
    name: String,
    left_x: i64,
    blocks_y: &mut i64,
) -> (StringHashMap<Block>, String) {
    let this_block_id = generate_id();

    let (func_input_blocks, param_id) =
        generate_func_input_block(this_block_id.clone(), function.locals(), name);

    let mut inputs = StringHashMap::default();
    inputs.0.insert(
        "custom_block".into(),
        BlockInput {
            shadow: ShadowInputType::Shadow,
            inputs: vec![Some(UidOrValue::Uid(param_id))],
        },
    );

    let block = BlockNormal {
        opcode: "procedures_definition".into(),
        next: None,
        parent: None,
        shadow: false,
        top_level: true,
        x: Some((left_x - 2000).into()),
        y: Some((*blocks_y).into()),
        inputs,
        fields: StringHashMap::default(),
        mutation: None,
        comment: None,
    };

    let mut blocks = StringHashMap::default();
    blocks.0.insert(this_block_id.clone(), Block::Normal(block));

    blocks.0.extend(func_input_blocks.0);

    let function_blocks = function.body();
    generate_func_block_code(function_blocks, this_block_id.clone());

    *blocks_y += 100;

    (blocks, this_block_id)
}

pub fn generate_func_input_block(
    parent: String,
    types: &ResultType,
    name: String,
) -> (StringHashMap<Block>, String) {
    let pre_name = "__wasm_";

    let this_block_id = generate_id();

    let mut blocks = StringHashMap::default();

    let mut inputs = StringHashMap::default();
    let mut proccode = format!("{pre_name}{name}");
    let mut argumentids = Vec::new();
    let mut argumentnames = Vec::new();
    let mut argumentdefaults = Vec::new();
    let len = types.kinds().len();
    for (i, ty) in types.kinds().iter().enumerate() {
        let wrapper_id = generate_id();
        let (id, block, ty, name, default) = generate_func_input_block_var(
            this_block_id.clone(),
            format!("{}_", wrap_by_len(i, len)),
            ty,
        );
        inputs.0.insert(
            wrapper_id.clone(),
            BlockInput {
                shadow: ShadowInputType::Shadow,
                inputs: vec![Some(UidOrValue::Uid(id.clone()))],
            },
        );
        proccode.push_str(&format!(" {ty}"));
        argumentids.push(wrapper_id);
        argumentnames.push(name);
        argumentdefaults.push(default);
        blocks.0.insert(id, block);
    }

    let block = BlockNormal {
        opcode: "procedures_prototype".into(),
        next: None,
        parent: Some(parent),
        shadow: true,
        top_level: false,
        x: None,
        y: None,
        inputs: inputs,
        fields: StringHashMap::default(),
        mutation: Some(BlockMutation {
            tag_name: "mutation".into(),
            children: vec![],
            mutation_enum: ProceduresPrototype {
                proccode,
                argumentids,
                argumentnames,
                argumentdefaults,
                warp: Some(true),
            },
        }),
        comment: None,
    };
    blocks.0.insert(this_block_id.clone(), Block::Normal(block));
    (blocks, this_block_id)
}

pub fn generate_func_input_block_var(
    parent: String,
    pre_name: String,
    ty: &ValueType,
) -> (String, Block, String, String, ValueWithBool) {
    match ty {
        ValueType::I32 => generate_func_input_block_var_string_number(parent, pre_name),
        ValueType::I64 => generate_func_input_block_var_string_number(parent, pre_name),
        ValueType::F32 => generate_func_input_block_var_string_number(parent, pre_name),
        ValueType::F64 => generate_func_input_block_var_string_number(parent, pre_name),
        ValueType::FunctionReference => todo!(),
        ValueType::ExternalReference => todo!(),
    }
}

pub fn generate_func_input_block_var_boolean(
    parent: String,
    pre_name: String,
) -> (String, Block, String, String, ValueWithBool) {
    let this_block_id = generate_id();

    let mut fields = StringHashMap::default();

    let name = format!("{pre_name}_bool");

    fields.0.insert(
        "VALUE".into(),
        WithId {
            id: None,
            value: Text(name.clone()),
        },
    );

    let block = BlockNormal {
        opcode: "argument_reporter_boolean".into(),
        next: None,
        parent: Some(parent),
        shadow: true,
        top_level: false,
        x: None,
        y: None,
        inputs: StringHashMap::default(),
        fields: fields,
        mutation: None,
        comment: None,
    };
    (
        this_block_id,
        Block::Normal(block),
        "%b".into(),
        name,
        ValueWithBool::Bool(false),
    )
}

pub fn generate_func_input_block_var_string_number(
    parent: String,
    pre_name: String,
) -> (String, Block, String, String, ValueWithBool) {
    let this_block_id = generate_id();

    let mut fields = StringHashMap::default();

    let name = format!("{pre_name}_str_num");

    fields.0.insert(
        "VALUE".into(),
        WithId {
            id: None,
            value: Text(name.clone()),
        },
    );

    let block = BlockNormal {
        opcode: "argument_reporter_string_number".into(),
        next: None,
        parent: Some(parent),
        shadow: true,
        top_level: false,
        x: None,
        y: None,
        inputs: StringHashMap::default(),
        fields: fields,
        mutation: None,
        comment: None,
    };
    (
        this_block_id,
        Block::Normal(block),
        "%s".into(),
        name,
        ValueWithBool::Text("".into()),
    )
}
