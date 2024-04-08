use sb_itchy::{custom_block::CustomBlockInputType, data::ListBuilder};
use sb_sbity::value::{Number, ValueWithBool};

use crate::scratch::sb3::ProjectZip;

use super::{
    check_unicode::check_unicode_func_generator, check_uppercase::check_uppercase_func_generator,
    upper_case_data_list_name,
};

pub fn to_utf8_generator(target_ctx: &mut ProjectZip) {
    let upper_case_data_list_name = upper_case_data_list_name();

    let mut list_init_data = vec![
        ValueWithBool::Number(Number::Int(0)), // check_uppercase_func return value
        ValueWithBool::Bool(false),            // check_uppercase_func_impl end flag
        ValueWithBool::Text("".to_string()),   // tmp
        ValueWithBool::Text({
            // check_unicode_func ascii data
            let mut s = String::new();
            for i in 0x20..0x7E + 1 {
                s.push(char::from(i));
            }
            s
        }),
        ValueWithBool::Text("".to_string()), // check_unicode_func return
        ValueWithBool::Number(Number::Int(0)), // check_unicode_func 二分探索法用 mid
        ValueWithBool::Number(Number::Int(0)), // check_unicode_func 二分探索法用 min
        ValueWithBool::Number(Number::Int(0)), // check_unicode_func 二分探索法用 max
    ];

    check_unicode_func_generator(target_ctx, &mut list_init_data);

    let offset = list_init_data.len() as i32;

    check_uppercase_func_generator(target_ctx, offset, &mut list_init_data);

    target_ctx.add_list_builder(
        upper_case_data_list_name.clone(),
        ListBuilder::new(list_init_data),
    );

    target_ctx.define_custom_block(
        vec![
            CustomBlockInputType::Text("to_utf8".into()),
            CustomBlockInputType::StringOrNumber("str".into()),
        ],
        true,
    );
    // let stack_builder = define_custom_block("to_utf8");
    // let block_input_builder = BiB::value(BiV::String {
    //     value: String::from("t").into(),
    // });
    // let stack_builder = stack_builder.next(set_var_to(
    //     BlockFieldBuilder::new("a".into()),
    //     block_input_builder,
    // ));

    // stack_builder
}
