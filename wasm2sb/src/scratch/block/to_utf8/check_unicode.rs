use sb_itchy::{
    block::BlockInputBuilder, blocks::define_custom_block, custom_block::CustomBlockInputType,
    stack::StackBuilder,
};
use sb_itchy_support::{block_generator_into::BlockGeneratorInto, blocks_wrapper::*, stack};
use sb_sbity::value::{Number, ValueWithBool};

use crate::scratch::{block::to_utf8::PRE_UNICODE, sb3::ProjectZip};

use super::{
    tmp_list_name, unicode::all_unicode_upper_letter_case_range, upper_case_data_list_name,
};

pub fn check_unicode_func_generator(ctx: &mut ProjectZip, list_init_data: &mut Vec<ValueWithBool>) {
    let offset = list_init_data.len() as i32;

    println!("offset: {}", offset);

    let ranges = all_unicode_upper_letter_case_range();
    list_init_data.extend(
        ranges
            .into_iter()
            .flat_map(|((first, last), diff)| {
                vec![
                    ValueWithBool::Number(Number::Int(first as i64)),
                    ValueWithBool::Number(Number::Int(last as i64)),
                    ValueWithBool::Number(Number::Int(diff as i64)),
                ]
            })
            .collect::<Vec<ValueWithBool>>(),
    );

    let check_unicode_func_name = format!("{PRE_UNICODE}check_unicode");
    let check_unicode_func_impl_name = format!("{PRE_UNICODE}check_unicode_impl");

    ctx.define_custom_block(
        vec![
            CustomBlockInputType::Text(check_unicode_func_name.clone()),
            CustomBlockInputType::StringOrNumber("unicode".into()),
        ],
        true,
    );

    ctx.define_custom_block(
        vec![
            CustomBlockInputType::Text(check_unicode_func_impl_name.clone()),
            CustomBlockInputType::StringOrNumber("n".into()),
            CustomBlockInputType::Boolean("min?".into()),
        ],
        true,
    );

    let tmp_list_name = tmp_list_name();
    let upper_case_data_list_name = upper_case_data_list_name();

    let upper_case_data_list = || global_list_menu(&upper_case_data_list_name);
    let tmp_list = || global_list_menu(&tmp_list_name);
    let ascii_var = || item_in_list(&upper_case_data_list(), 4);
    let set_unicode = |value: BlockInputBuilder| -> StackBuilder {
        replace_in_list(&upper_case_data_list(), 5, value)
    };
    let set_dichotomous_search_mid = |value: BlockInputBuilder| -> StackBuilder {
        replace_in_list(&upper_case_data_list(), 6, value)
    };
    let set_dichotomous_search_min = |value: BlockInputBuilder| -> StackBuilder {
        replace_in_list(&upper_case_data_list(), 7, value)
    };
    let set_dichotomous_search_max = |value: BlockInputBuilder| -> StackBuilder {
        replace_in_list(&upper_case_data_list(), 8, value)
    };
    let dichotomous_search_mid = || item_in_list(&upper_case_data_list(), 6);
    let dichotomous_search_min = || item_in_list(&upper_case_data_list(), 7);
    let dichotomous_search_max = || item_in_list(&upper_case_data_list(), 8);
    let unicode = || custom_block_var_string_number("unicode");
    let to_unicode = |value: BlockInputBuilder| -> BlockInputBuilder {
        translate_to(join(join("&#", value), ";"), "ja")
    };
    let stop_this_script = || stop("this script", false);

    let check_unicode_func_generator_inner_ascii = stack![
        repeat(
            length_of(ascii_var()),
            stack![add_to_list(
                &tmp_list(),
                letter_of(add(length_of_list(tmp_list()), 1), ascii_var()),
            )],
        ),
        set_unicode(add(count_of_item_in_list(tmp_list(), unicode()), "0x1F")),
        delete_all_in_list(&tmp_list())
    ];
    let hexadecimal = |str: &str| -> BlockInputBuilder { add(str, 0) };

    let check_surrogate_pair = stack![set_dichotomous_search_min(hexadecimal("0xD800"))];

    let check_surrogate_no_pair = stack![
        set_dichotomous_search_mid(hexadecimal("0x8000")),
        set_dichotomous_search_min(1.to()),
        set_dichotomous_search_max(hexadecimal("0xFFFF")),
        forever(stack![
            if_else(
                less_than(unicode(), to_unicode(dichotomous_search_mid())),
                stack![
                    set_dichotomous_search_max(sub(dichotomous_search_mid(), 1)),
                    call_custom_block(
                        &check_unicode_func_impl_name,
                        vec![("n", 8.to())].into_iter().collect()
                    )
                ],
                if_else(
                    less_than(to_unicode(dichotomous_search_mid()), unicode()),
                    stack![
                        set_dichotomous_search_min(add(dichotomous_search_mid(), 1)),
                        call_custom_block(
                            &check_unicode_func_impl_name,
                            vec![("n", 7.to()), ("min?", always_true())]
                                .into_iter()
                                .collect()
                        )
                    ],
                    stack![set_unicode(dichotomous_search_mid()), stop_this_script()],
                ),
            ),
            set_dichotomous_search_mid(math_op(
                "floor",
                div(add(dichotomous_search_min(), dichotomous_search_max()), 2)
            ))
        ])
    ];

    let check_unicode_func = stack![
        define_custom_block(&check_unicode_func_name),
        if_else(
            contains(ascii_var(), unicode()),
            stack![check_unicode_func_generator_inner_ascii],
            stack![if_else(
                equals(length_of(unicode()), 2),
                check_surrogate_pair,
                check_surrogate_no_pair,
            )]
        )
    ];

    ctx.add_stack_builder(check_unicode_func);

    let tmp_var = || item_in_list(upper_case_data_list(), 3);
    let set_tmp_var = |value: BlockInputBuilder| -> StackBuilder {
        replace_in_list(upper_case_data_list(), 3, value)
    };
    let first_var = || item_in_list(upper_case_data_list(), tmp_var());
    let last_var = || item_in_list(upper_case_data_list(), add(tmp_var(), 1));
    let diff_var = || item_in_list(upper_case_data_list(), add(tmp_var(), 2));
    let if_min = || custom_block_var_boolean("min?");
    let n = || custom_block_var_string_number("n");
    let n_var = || item_in_list(upper_case_data_list(), n());
    let set_n_var = |value: BlockInputBuilder| -> StackBuilder {
        replace_in_list(upper_case_data_list(), n(), value)
    };

    let check_unicode_func_impl = stack![
        define_custom_block(&check_unicode_func_impl_name),
        set_tmp_var((offset + 1).to()),
        forever(stack![
            if_else(
                greater_than(first_var(), n_var()),
                stop_this_script(),
                stack![if_(
                    not(greater_than(n_var(), last_var())),
                    stack![
                        if_(
                            equals(modulo(sub(n_var(), first_var()), diff_var()), 0),
                            stack![if_else(
                                if_min(),
                                stack![set_n_var(last_var())],
                                stack![set_n_var(first_var())]
                            )]
                        ),
                        stop_this_script()
                    ],
                )]
            ),
            set_tmp_var(add(tmp_var(), 3))
        ])
    ];

    ctx.add_stack_builder(check_unicode_func_impl);
}
