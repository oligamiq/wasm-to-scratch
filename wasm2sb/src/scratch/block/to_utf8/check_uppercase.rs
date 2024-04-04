use std::collections::HashMap;

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

    let check_uppercase_func_name = format!("{PRE_UNICODE}check_uppercase");
    let check_uppercase_impl_func_name = format!("{PRE_UNICODE}check_uppercase_impl");

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
            CustomBlockInputType::Text(check_uppercase_func_name.clone()),
            CustomBlockInputType::StringOrNumber("str".to_string()),
            CustomBlockInputType::StringOrNumber("unicode".to_string()),
        ],
        true,
    );

    ctx.define_custom_block(
        vec![
            CustomBlockInputType::Text(check_uppercase_impl_func_name.clone()),
            CustomBlockInputType::StringOrNumber("target".to_string()),
            CustomBlockInputType::StringOrNumber("str".to_string()),
            CustomBlockInputType::StringOrNumber("num".to_string()),
            CustomBlockInputType::StringOrNumber("n".to_string()),
        ],
        true,
    );

    let upper_case_data_list = Bfb::new_with_kind(upper_case_data_list_name, FieldKind::GlobalList);
    let tmp_list = Bfb::new_with_kind(tmp_list_name, FieldKind::GlobalList);
    let set_return_var = |value: BlockInputBuilder| -> StackBuilder {
        replace_in_list(upper_case_data_list.clone(), 1.to(), value)
    };
    let item_in_upper_case_data = |index: BlockInputBuilder| -> Bib {
        Bib::stack(item_in_list(upper_case_data_list.clone(), index))
    };
    let return_var = || Bib::stack(item_in_list(upper_case_data_list.clone(), 1.to()));
    let custom_block_var_string_number =
        |name: &str| -> Bib { Bib::stack(custom_block_var_string_number(name)) };
    let num = || custom_block_var_string_number("num");
    let str = || custom_block_var_string_number("str");
    let less_than = |a: Bib, b: Bib| -> Bib { Bib::stack(less_than(a, b)) };
    let repeat = |times: Bib, stack: StackBuilder| -> StackBuilder {
        repeat(times, Some(Bib::stack(stack)))
    };
    let if_ = |condition: Bib, if__: StackBuilder| -> StackBuilder {
        if_(condition, Some(Bib::stack(if__)))
    };
    let if_else = |condition: Bib, if_: StackBuilder, else_: StackBuilder| -> StackBuilder {
        if_else(condition, Some(Bib::stack(if_)), Some(Bib::stack(else_)))
    };
    let costume = |name: &str| -> Bib { Bib::stack(costume(name.to())) };
    let add = |a: Bib, b: Bib| -> Bib { Bib::stack(add(a, b)) };
    let div = |lhs: Bib, rhs: Bib| -> Bib { Bib::stack(div(lhs, rhs)) };
    let stop_this_script = || stop("this script".to(), false);

    let check_uppercase = define_custom_block(&check_uppercase_impl_func_name)
        .next(set_return_var(3.to()))
        .next(repeat(
            unicode.len().to(),
            if_else(
                less_than(item_in_upper_case_data(return_var()), num()),
                if_(
                    less_than(num(), item_in_upper_case_data(add(return_var(), 1.to()))),
                    switch_costume_to("default".to())
                        .next(switch_costume_to(add(
                            costume("number"),
                            div(return_var(), 3.to()),
                        )))
                        .next(call_custom_block(
                            &check_uppercase_impl_func_name,
                            vec![
                                ("target", costume("name")),
                                ("str", str()),
                                ("num", num()),
                                ("n", return_var()),
                            ]
                            .into_iter()
                            .collect(),
                        ))
                        .next(if_(item_in_upper_case_data(2.to()), stop_this_script())),
                ),
                set_return_var(num()).next(stop_this_script()),
            )
            .next(set_return_var(add(return_var(), 3.to()))),
        ));

    ctx.add_stack_builder(check_uppercase);

    let target = || custom_block_var_string_number("target");
    let length_of = |target: Bib| -> Bib { Bib::stack(length_of(target)) };
    let contains = |target: Bib, str: Bib| -> Bib { Bib::stack(contains(target, str.to())) };
    let count_of_item_in_list =
        |target: Bfb, str: Bib| -> Bib { Bib::stack(count_of_item_in_list(target, str)) };
    let letter_of = |target: Bib, index: Bib| -> Bib { Bib::stack(letter_of(target, index)) };
    let length_of_list = |target: Bfb| -> Bib { Bib::stack(length_of_list(target)) };
    let set_flag_var = |flag: bool| -> StackBuilder {
        replace_in_list(upper_case_data_list.clone(), 2.to(), flag.to())
    };

    let check_uppercase_impl = define_custom_block(&check_uppercase_impl_func_name).next(if_else(
        contains(target(), str()),
        repeat(length_of(target()), add_to_list(tmp_list.clone(), letter_of(add(length_of_list(tmp_list.clone()), 1.to()), target()))).next(replace_in_list(
            tmp_list.clone(),
            count_of_item_in_list(tmp_list.clone(), str()),
            str(),
        )),
        set_flag_var(true).next(set_return_var(num())),
    ));

    ctx.add_stack_builder(check_uppercase_impl);
}
