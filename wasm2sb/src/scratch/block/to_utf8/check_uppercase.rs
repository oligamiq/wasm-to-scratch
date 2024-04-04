use crate::scratch::sb3::ProjectZip;

use super::{unicode::all_unicode_upper_letter_case, PRE_UNICODE};
use sb_itchy::{blocks::*, prelude::*};
use sb_sbity::value::{self, Number, Value};

type Bfb = BlockFieldBuilder;
type Bib = BlockInputBuilder;

use crate::scratch::block::block_generator_into::*;

pub fn check_uppercase_func_generator(ctx: &mut ProjectZip) {
    let unicode = all_unicode_upper_letter_case();

    for unicode in &unicode {
        println!("{:?}", unicode);
    }
    println!("{:?}", unicode.len());

    let name = format!("{PRE_UNICODE}check_uppercase");

    let tmp_list_name = format!("{PRE_UNICODE}tmp");
    ctx.add_list_builder(
        tmp_list_name.clone(),
        ListBuilder::new(vec![Value::Number(Number::Int(0))]),
    );
    let upper_case_data_list_name = format!("{PRE_UNICODE}uppercase_data");
    ctx.add_list_builder(
        upper_case_data_list_name.clone(),
        ListBuilder::new({
            unicode
                .iter()
                .flat_map(|((first, last), diff, _)| {
                    vec![
                        Value::Number(Number::Int(*first as i64 - 1)),
                        Value::Number(Number::Int(*last as i64 + 1)),
                        Value::Number(Number::Int(*diff as i64)),
                    ]
                })
                .collect::<Vec<Value>>()
        }),
    );

    ctx.define_custom_block(
        vec![
            CustomBlockInputType::Text(name.clone()),
            CustomBlockInputType::StringOrNumber("str".to_string()),
            CustomBlockInputType::StringOrNumber("unicode".to_string()),
        ],
        true,
    );

    let upper_case_data_list = Bfb::new_with_kind(upper_case_data_list_name, FieldKind::GlobalList);
    let set_return_var = |value: BlockInputBuilder| -> StackBuilder {
        replace_in_list(upper_case_data_list.clone(), 1.to(), value)
    };
    let item_in_upper_case_data = |index: BlockInputBuilder| -> Bib {
        Bib::stack(item_in_list(upper_case_data_list.clone(), index))
    };
    let return_var = item_in_list(upper_case_data_list.clone(), 1.to());

    let check_uppercase = define_custom_block(name)
        .next(set_return_var(3.to()))
        .next(repeat(
            unicode.len().to(),
            Some(Bib::stack(if_else(
                Bib::stack(less_than(
                    item_in_upper_case_data(Bib::stack(return_var.clone())),
                    Bib::stack(custom_block_var_string_number("num")),
                )),
                None,
                None,
            ))),
        ));

    ctx.add_stack_builder(check_uppercase);
}
