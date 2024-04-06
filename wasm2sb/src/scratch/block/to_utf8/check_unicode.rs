use sb_itchy::{
    blocks::define_custom_block, custom_block::CustomBlockInputType, stack::StackBuilder,
};
use sb_itchy_support::{blocks_wrapper::*, stacks};

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

    let check_unicode_func_generator_inner_ascii = check_unicode_func_generator_inner_ascii();

    let check_unicode_func = define_custom_block(&check_unicode_func_name);
}

pub fn check_unicode_func_generator_inner_ascii() -> StackBuilder {
    let tmp_list_name = tmp_list_name();
    let upper_case_data_list_name = upper_case_data_list_name();

    let upper_case_data_list = || global_list_menu(&upper_case_data_list_name);
    let tmp_list = || global_list_menu(&tmp_list_name);
    let ascii_var = || item_in_list(&upper_case_data_list(), 4);

    let check_unicode_func_generator_inner_ascii = repeat(
        length_of(ascii_var()),
        add_to_list(
            &tmp_list(),
            letter_of(add(length_of_list(tmp_list()), 1), ascii_var()),
        ),
    );

    check_unicode_func_generator_inner_ascii
}
