use sb_sbity::{block::Block, string_hashmap::StringHashMap};
use wain_ast::{Func, FuncType};

pub fn wrap_by_len(i: usize, len: usize) -> String {
    let len = format!("{:x}", len).len();
    let mut name = format!("{:x}", i);
    while name.len() < len {
        name = format!("0{}", name);
    }
    name
}

pub fn get_type_from_func<'a>(func: &'a Func, types: &'a Vec<FuncType>) -> &'a FuncType {
    let index = func.idx;
    &types[index as usize]
}

pub fn get_preview_rect_from_block<'a>(func: &StringHashMap<Block>) -> (i64, i64, i64, i64) {
    let mut left_x = None;
    let mut right_x = None;
    let mut top_y = None;
    let mut bottom_y = None;
    for block in func.0.values() {
        match block {
            Block::Normal(block) => {
                if let Some(x) = block.x {
                    let x = match x {
                        sb_sbity::value::Number::Int(x) => x,
                        sb_sbity::value::Number::Float(x) => x as i64,
                    };
                    if let Some(ref mut left_x) = left_x {
                        if x < *left_x {
                            *left_x = x;
                        }
                    } else {
                        left_x = Some(x);
                    }
                    if let Some(ref mut right_x) = right_x {
                        if x > *right_x {
                            *right_x = x;
                        }
                    } else {
                        right_x = Some(x);
                    }
                }
                if let Some(y) = block.y {
                    let y = match y {
                        sb_sbity::value::Number::Int(y) => y,
                        sb_sbity::value::Number::Float(y) => y as i64,
                    };
                    if let Some(ref mut top_y) = top_y {
                        if y < *top_y {
                            *top_y = y;
                        }
                    } else {
                        top_y = Some(y);
                    }
                    if let Some(ref mut bottom_y) = bottom_y {
                        if y > *bottom_y {
                            *bottom_y = y;
                        }
                    } else {
                        bottom_y = Some(y);
                    }
                }
            }
            _ => {}
        }
    }
    (
        left_x.unwrap_or_default(),
        right_x.unwrap_or_default(),
        top_y.unwrap_or_default(),
        bottom_y.unwrap_or_default(),
    )
}
