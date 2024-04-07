use sb_itchy::{
    block::BlockInputBuilder, blocks::define_custom_block, custom_block::CustomBlockInputType,
    stack::StackBuilder,
};
use sb_itchy_support::{block_generator_into::BlockGeneratorInto, blocks_wrapper::*, stack};

use crate::scratch::{block::to_utf8::PRE_UNICODE, sb3::ProjectZip};

use super::{tmp_list_name, upper_case_data_list_name};

pub fn check_unicode_func_generator(ctx: &mut ProjectZip) {
    let check_unicode_func_name = format!("{PRE_UNICODE}check_unicode");

    ctx.define_custom_block(
        vec![
            CustomBlockInputType::Text(check_unicode_func_name.clone()),
            CustomBlockInputType::StringOrNumber("unicode".into()),
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

    let check_surrogate_pair = stack![set_dichotomous_search_min(add("0xD800", 0))];

    let check_surrogate_no_pair = stack![
        set_dichotomous_search_mid(add("0x8000", 0)),
        set_dichotomous_search_min(1.to()),
        set_dichotomous_search_max(add("0xFFFF", 0)),
        forever(stack![if_else(
            less_than(unicode(), to_unicode(dichotomous_search_mid())),
            stack![set_dichotomous_search_max(sub(dichotomous_search_mid(), 1))],
            stack![set_dichotomous_search_min(add(dichotomous_search_mid(), 1))],
        )])
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
}
