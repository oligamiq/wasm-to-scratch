use crate::scratch::sb3::ProjectZip;

use super::{unicode::all_unicode_upper_letter_case, PRE_UNICODE};
use sb_itchy::{blocks::*, prelude::*};
use sb_sbity::value::{Number, Value};

pub fn check_uppercase_func_generator(ctx: &mut ProjectZip) {
    let unicode = all_unicode_upper_letter_case();

    for unicode in &unicode {
        println!("{:?}", unicode);
    }

    let name = format!("{PRE_UNICODE}check_uppercase");

    ctx.add_list_builder(format!("{PRE_UNICODE}tmp"), ListBuilder::new(vec![
        Value::Number(Number::Int(0)),
    ]));
    ctx.add_list_builder(format!("{PRE_UNICODE}uppercase_data"), ListBuilder::new({
        unicode.iter().flat_map(|((first, last), diff, _)| {
            vec![
                Value::Number(Number::Int ( *first as i64 - 1 )),
                Value::Number(Number::Int ( *last as i64 + 1 )),
                Value::Number(Number::Int ( *diff as i64 )),
            ]
        }).collect::<Vec<Value>>()
    }));

    ctx.define_custom_block(
        vec![
            CustomBlockInputType::Text(name.clone()),
            CustomBlockInputType::StringOrNumber("str".to_string()),
            CustomBlockInputType::StringOrNumber("unicode".to_string()),
        ],
        true,
    );

    let mut check_uppercase = define_custom_block(name);


}
