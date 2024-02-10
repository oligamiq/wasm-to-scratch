use sb_sbity::{block::BlockField, string_hashmap::StringHashMap};
use wasm_ast::Expression;

pub fn generate_func_block_code(
    expression: &Expression,
    parent: String,
) -> StringHashMap<BlockField> {
    let mut fields = StringHashMap::default();
    for (i, item) in expression.instructions().iter().enumerate() {
        match item {
            wasm_ast::Instruction::Numeric(_) => todo!(),
            // wasm_ast::Instruction::Numeric(numeric_instruction) => {
            //     fields.0.insert(
            //         "NUM".into(),
            //         BlockField::Number(numeric_instruction.value().to_string()),
            //     );
            // }
            wasm_ast::Instruction::Reference(_) => todo!(),
            wasm_ast::Instruction::Parametric(_) => todo!(),
            wasm_ast::Instruction::Variable(variable_instruction) => {}
            wasm_ast::Instruction::Table(_) => todo!(),
            wasm_ast::Instruction::Memory(_) => todo!(),
            wasm_ast::Instruction::Control(control_instruction) => {}
        }
    }

    fields
}
