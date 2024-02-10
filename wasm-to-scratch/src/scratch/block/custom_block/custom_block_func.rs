use std::collections::HashMap;

use sb_itchy::uid::Uid;
use sb_sbity::{
    block::BlockField::WithId,
    block::{
        Block, BlockInput, BlockMutation, BlockMutationEnum::ProceduresPrototype, BlockNormal,
        ShadowInputType, UidOrValue,
    },
    string_hashmap::StringHashMap,
    value::{Value, ValueWithBool},
};
pub enum CustomBlockInputType {
    Text(String),
    StringOrNumber(String),
    Boolean(String),
}

pub fn generate_custom_block(
    input: Vec<CustomBlockInputType>,
) -> ((Uid, BlockNormal), HashMap<Uid, Block>) {
    let this_block_id = Uid::generate();
    let mut stack = HashMap::default();

    let params_id = generate_func_input_block(this_block_id.clone(), &input, &mut stack);

    let mut inputs = StringHashMap::default();
    inputs.0.insert(
        "custom_block".into(),
        BlockInput {
            shadow: ShadowInputType::Shadow,
            inputs: vec![Some(UidOrValue::Uid(params_id.into_inner()))],
        },
    );

    let block = BlockNormal {
        opcode: "procedures_definition".into(),
        next: None,
        parent: None,
        shadow: false,
        top_level: true,
        x: None,
        y: None,
        inputs,
        fields: StringHashMap::default(),
        mutation: None,
        comment: None,
    };

    ((this_block_id, block), stack)
}

pub fn generate_func_input_block(
    parent: Uid,
    params: &Vec<CustomBlockInputType>,
    stack: &mut HashMap<Uid, Block>,
) -> Uid {
    let this_block_id = Uid::generate();

    let mut inputs = StringHashMap::default();

    let mut argumentids = Vec::new();
    let mut argumentnames = Vec::new();
    let mut argumentdefaults = Vec::new();

    let mut proccode: Option<String> = None;

    for ty in params {
        let wrapper_id = Uid::generate();
        match ty {
            CustomBlockInputType::Text(name) => match &mut proccode {
                Some(proccode) => {
                    proccode.push_str(&format!(" {name}"));
                }
                None => {
                    proccode = Some(name.clone());
                }
            },
            CustomBlockInputType::StringOrNumber(name) => {
                let (id, block, ty, default) = generate_func_input_block_var_string_number(
                    this_block_id.clone(),
                    name.clone(),
                );
                inputs.0.insert(
                    wrapper_id.clone().into_inner(),
                    BlockInput {
                        shadow: ShadowInputType::Shadow,
                        inputs: vec![Some(UidOrValue::Uid(id.clone().into_inner()))],
                    },
                );
                argumentids.push(wrapper_id.into_inner());
                argumentnames.push(name.clone());
                argumentdefaults.push(default);
                stack.insert(id, block);
                match &mut proccode {
                    Some(proccode) => {
                        proccode.push_str(&format!(" {ty}"));
                    }
                    None => {
                        proccode = Some(ty);
                    }
                }
            }
            CustomBlockInputType::Boolean(name) => {
                let (id, block, ty, default) = generate_func_input_block_var_boolean(
                    this_block_id.clone().into_inner(),
                    name.clone(),
                );
                inputs.0.insert(
                    wrapper_id.clone().into_inner(),
                    BlockInput {
                        shadow: ShadowInputType::Shadow,
                        inputs: vec![Some(UidOrValue::Uid(id.clone().into_inner()))],
                    },
                );
                argumentids.push(wrapper_id.into_inner());
                argumentnames.push(name.clone());
                argumentdefaults.push(default);
                stack.insert(id, block);
                match &mut proccode {
                    Some(proccode) => {
                        proccode.push_str(&format!(" {ty}"));
                    }
                    None => {
                        proccode = Some(ty);
                    }
                }
            }
        }
    }

    let block = BlockNormal {
        opcode: "procedures_prototype".into(),
        next: None,
        parent: Some(parent.into_inner()),
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
                proccode: proccode.unwrap(),
                argumentids,
                argumentnames,
                argumentdefaults,
                warp: Some(true),
            },
        }),
        comment: None,
    };
    stack.insert(this_block_id.clone(), Block::Normal(block));
    this_block_id
}

pub fn generate_func_input_block_var_boolean(
    parent: String,
    name: String,
) -> (Uid, Block, String, ValueWithBool) {
    let this_block_id = Uid::generate();

    let mut fields = StringHashMap::default();

    fields.0.insert(
        "VALUE".into(),
        WithId {
            id: None,
            value: Value::Text(name.clone()),
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
        ValueWithBool::Bool(false),
    )
}

pub fn generate_func_input_block_var_string_number(
    parent: Uid,
    name: String,
) -> (Uid, Block, String, ValueWithBool) {
    let this_block_id = Uid::generate();

    let mut fields = StringHashMap::default();

    fields.0.insert(
        "VALUE".into(),
        WithId {
            id: None,
            value: Value::Text(name.clone()),
        },
    );

    let block = BlockNormal {
        opcode: "argument_reporter_string_number".into(),
        next: None,
        parent: Some(parent.into_inner()),
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
        ValueWithBool::Text("".into()),
    )
}
