use sb_sbity::{block::BlockField, string_hashmap::StringHashMap};
use wain_ast::Instruction;

pub fn generate_func_block_code(
    instruction: &Vec<Instruction>,
    parent: String,
) -> StringHashMap<BlockField> {
    let mut fields = StringHashMap::default();
    for (i, item) in instruction.iter().enumerate() {
        match &item.kind {
            wain_ast::InsnKind::Block { ty, body } => {
                // let mut block = generate_func_block_code(&body, parent.clone());
                // fields.0.extend(block.0);
            }
            wain_ast::InsnKind::Loop { ty, body } => {
                // let mut block = generate_func_block_code(&body, parent.clone());
                // fields.0.extend(block.0);
            }
            wain_ast::InsnKind::I32Const(val) => {
                // let field = BlockField::Value(val.to_string());
                // fields.0.insert(format!("field_{}", i), field);
                println!("I32Const: {}", val);
            }
            wain_ast::InsnKind::I64Load(offset) => {
                // let field = BlockField::Value(offset.to_string());
                // fields.0.insert(format!("field_{}", i), field);
                println!("I64Load: {:?}", offset);
            }
            wain_ast::InsnKind::I64Const(val) => {
                // let field = BlockField::Value(val.to_string());
                // fields.0.insert(format!("field_{}", i), field);
                println!("I64Const: {}", val);
            }
            wain_ast::InsnKind::I64Store(offset) => {
                // let field = BlockField::Value(offset.to_string());
                // fields.0.insert(format!("field_{}", i), field);
                println!("I64Store: {:?}", offset);
            }
            wain_ast::InsnKind::I64Add => {
                // let field = BlockField::Value("".to_string());
                // fields.0.insert(format!("field_{}", i), field);
                println!("I64Add");
            }
            wain_ast::InsnKind::I32Store16(offset) => {
                // let field = BlockField::Value(offset.to_string());
                // fields.0.insert(format!("field_{}", i), field);
                println!("I32Store16: {:?}", offset);
            }
            wain_ast::InsnKind::LocalGet(index) => {
                // let field = BlockField::Value(index.to_string());
                // fields.0.insert(format!("field_{}", i), field);
                println!("LocalGet: {:?}", index);
            }
            _ => {
                todo!();
            }
        }
    }

    fields
}
