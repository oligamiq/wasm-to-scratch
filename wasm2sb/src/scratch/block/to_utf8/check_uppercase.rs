use crate::scratch::block::to_utf8::unicode::all_unicode_upper_letter_case_range;
use crate::scratch::sb3::ProjectZip;

use super::{unicode::all_unicode_upper_letter_case, PRE_UNICODE};
use sb_itchy::prelude::*;
use sb_sbity::value::{Number, ValueWithBool};

type Bib = BlockInputBuilder;

use sb_itchy_support::block_generator_into::*;
use sb_itchy_support::blocks_wrapper::*;

pub fn check_uppercase_func_generator(
    ctx: &mut ProjectZip,
    offset: i32,
    list_init_data: &mut Vec<ValueWithBool>,
) -> String {
    let unicodes = all_unicode_upper_letter_case();

    for unicodes in &unicodes {
        println!("{:?}", unicodes);
    }
    println!("{:?}", unicodes.len());

    list_init_data.extend(
        unicodes
            .iter()
            .flat_map(|((first, last), diff, _)| {
                vec![
                    ValueWithBool::Number(Number::Int(*first as i64 - 1)),
                    ValueWithBool::Number(Number::Int(*last as i64 + 1)),
                    ValueWithBool::Number(Number::Int(*diff as i64)),
                ]
            })
            .collect::<Vec<ValueWithBool>>(),
    );

    let check_uppercase_func_name = format!("{PRE_UNICODE}check_uppercase");
    let check_uppercase_impl_func_name = format!("{PRE_UNICODE}check_uppercase_impl");

    let tmp_list_name = format!("{PRE_UNICODE}tmp");
    ctx.add_list_builder(tmp_list_name.clone(), ListBuilder::new(Vec::new()));
    let upper_case_data_list_name = format!("{PRE_UNICODE}uppercase_data");

    let default_resource = Resource::new(
        "svg".into(),
        r#"<?xml version="1.0"?><svg xmlns="http://www.w3.org/2000/svg"></svg>"#.into(),
    )
    .unwrap();
    ctx.add_costume_builder(CostumeBuilder::new(AssetBuilder::new(
        "default",
        default_resource.clone(),
    )));
    for (_, _, name) in &unicodes {
        ctx.add_costume_builder(CostumeBuilder::new(AssetBuilder::new(
            name,
            default_resource.clone(),
        )));
    }

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
            CustomBlockInputType::StringOrNumber("unicode".to_string()),
            CustomBlockInputType::StringOrNumber("n".to_string()),
        ],
        true,
    );

    let upper_case_data_list = || global_list_menu(&upper_case_data_list_name);
    let tmp_list = || global_list_menu(&tmp_list_name);
    let set_return_var = |value: BlockInputBuilder| -> StackBuilder {
        replace_in_list(&upper_case_data_list(), 1, value)
    };
    let item_in_upper_case_data = |index| item_in_list(&upper_case_data_list(), index);
    let return_var = || item_in_list(&upper_case_data_list(), 1);
    let unicode = || custom_block_var_string_number("unicode");
    let str = || custom_block_var_string_number("str");
    let stop_this_script = || stop("this script", false);
    let switch_costume_to_default = || switch_costume_to(costume_menu("default"));

    let check_uppercase = define_custom_block(&check_uppercase_func_name)
        .next(set_return_var((offset + 1).to()))
        .next(repeat(
            unicodes.len(),
            if_else(
                less_than(item_in_upper_case_data(return_var()), unicode()),
                if_(
                    less_than(unicode(), item_in_upper_case_data(add(return_var(), 1))),
                    switch_costume_to_default()
                        .next(switch_costume_to(add(
                            costume("number"),
                            div(sub(return_var(), offset + 1 - 3), 3),
                        )))
                        .next(call_custom_block(
                            &check_uppercase_impl_func_name,
                            vec![
                                ("target", costume("name")),
                                ("str", str()),
                                ("unicode", unicode()),
                                ("n", return_var()),
                            ]
                            .into_iter()
                            .collect(),
                        ))
                        .next(if_(item_in_upper_case_data(2.to()), stop_this_script())),
                ),
                set_return_var(unicode()).next(stop_this_script()),
            )
            .next(set_return_var(add(return_var(), 3))),
        ))
        .next(set_return_var(unicode()));

    ctx.add_stack_builder(check_uppercase);

    let target = || custom_block_var_string_number("target");
    let set_flag_var =
        |flag: bool| -> StackBuilder { replace_in_list(upper_case_data_list(), 2, flag) };
    let n = || custom_block_var_string_number("n");
    let letter_of_target = |index: BlockInputBuilder| -> Bib { letter_of(index, &target()) };
    let set_tmp_var = |value: BlockInputBuilder| -> StackBuilder {
        replace_in_list(upper_case_data_list(), 3, value)
    };
    let tmp_var = || item_in_list(upper_case_data_list(), 3);

    let feature_surrogate_pair = false;

    let feature_surrogate_1 = repeat(
        length_of(target()),
        add_to_list(
            tmp_list(),
            letter_of_target(add(length_of_list(tmp_list()), 1)),
        ),
    );

    let feature_surrogate_2 =
        switch_costume_to(global_list(&tmp_list_name)).next(delete_all_in_list(tmp_list()));

    let check_uppercase_impl = define_custom_block(&check_uppercase_impl_func_name).next(if_else(
        contains(target(), str()),
        if feature_surrogate_pair {
            if_else(
                less_than("0xFFFF", unicode()),
                repeat(
                    div(length_of(target()), 2),
                    add_to_list(
                        tmp_list(),
                        join(
                            letter_of_target(add(mul(length_of_list(tmp_list()), 2), 1)),
                            letter_of_target(add(mul(length_of_list(tmp_list()), 2), 2)),
                        ),
                    ),
                ),
                feature_surrogate_1,
            )
        } else {
            feature_surrogate_1
        }
        .next(replace_in_list(
            tmp_list(),
            count_of_item_in_list(tmp_list(), str()),
            str(),
        ))
        .next(switch_costume_to_default())
        .next(if feature_surrogate_pair {
            if_else(
                less_than("0xFFFF", unicode()),
                set_tmp_var("".to())
                    .next(repeat(
                        length_of_list(tmp_list()),
                        set_tmp_var(join(item_in_list(tmp_list(), "last"), tmp_var()))
                            .next(delete_in_list(tmp_list(), "last")),
                    ))
                    .next(switch_costume_to(tmp_var()))
                    .next(set_tmp_var("".to())),
                feature_surrogate_2,
            )
        } else {
            feature_surrogate_2
        })
        .next(if_else(
            equals(costume("name"), "default"),
            set_flag_var(false),
            set_flag_var(true)
                .next(set_return_var(sub(
                    unicode(),
                    item_in_upper_case_data(add(n(), 2)),
                )))
                .next(switch_costume_to_default()),
        )),
        set_flag_var(true).next(set_return_var(unicode())),
    ));

    ctx.add_stack_builder(check_uppercase_impl);

    let ranges = all_unicode_upper_letter_case_range();
    for range in &ranges {
        println!("{:?}", range);
    }
    println!("len: {:?}", ranges.len());

    upper_case_data_list_name
}
