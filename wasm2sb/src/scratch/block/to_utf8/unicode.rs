pub fn all_unicode() -> String {
    let mut s = String::new();
    for i in 0x0000..0x007f + 1 {
        s.push(std::char::from_u32(i).unwrap_or_default());
    }
    for i in 0x0080..0x07ff + 1 {
        s.push(std::char::from_u32(i).unwrap_or_default());
    }
    for i in 0x0800..0xffff + 1 {
        s.push(std::char::from_u32(i).unwrap_or_default());
    }
    s
}

pub fn all_unicode_upper_letter_case() -> Vec<((u32, u32), i64, String)> {
    let mut s: Vec<((u32, u32), i64, String)> = Vec::new();
    let mut succession = Vec::new();
    let mut first = 0;
    let mut diff = None;
    let mut chars = (0x0000..0x3ffff + 1)
        .filter_map(|i| std::char::from_u32(i))
        .filter_map(|c| {
            // if !c.is_uppercase() {
            //     return None;
            // }
            let lower = c.to_lowercase().to_string();
            let len = lower.chars().count();
            if len != 1 {
                return None;
            }
            let lower = lower.chars().next().unwrap();
            let diff_c = c as u32 as i64 - lower as u32 as i64;
            if diff_c != 0 {
                // let upper = lower.to_uppercase().to_string();
                // if upper.chars().count() == 1 && upper.chars().next().unwrap() != c {
                //     println!("alert: {} -> {} -> {}", c, lower, upper.chars().next().unwrap());
                // }
                if !c.is_uppercase() {
                    println!("is_uppercase: alert: {}, lower: {}", c, lower);
                }
                return Some((lower as u32, lower, c));
            } else {
                return None;
            }
        })
        .collect::<Vec<_>>();
    chars.sort_by(|a, b| a.0.cmp(&b.0));
    let mut kept_i = 0;
    for (i, c, upper) in chars {
        let diff_c = c as u32 as i64 - upper as u32 as i64;
        if None == diff {
            diff = Some(diff_c);
            first = i;
        }
        if diff.unwrap() != diff_c {
            s.push(((first, kept_i), diff.unwrap(), succession.iter().collect()));
            succession = Vec::new();
            diff = Some(diff_c);
            first = i;
        }
        succession.push(upper);
        kept_i = i;
    }
    s
}

pub fn all_unicode_upper_letter_case_range() -> Vec<((u32, u32), u32)> {
    let mut chars = (0x0000..0x3ffff + 1)
        .filter_map(|i| std::char::from_u32(i))
        .filter_map(|c| {
            let lower = c.to_lowercase().to_string();
            // let len = lower.chars().count();
            // if len != 1 {
            //     println!("alert: {}, lower: {}", c, lower);
            // }
            let lower = lower.chars().next().unwrap();
            let diff_c = c as u32 as i64 - lower as u32 as i64;
            if diff_c != 0 {
                return Some((c as u32, lower, c));
            } else {
                return None;
            }
        })
        .collect::<Vec<_>>();

    chars.sort_by(|a, b| a.0.cmp(&b.0));

    println!("len: {:?}", chars.len());

    let mut s: Vec<((u32, u32), u32)> = Vec::new();
    let mut first = 0;
    let mut before = None;
    let mut before_before = None;

    for (i, _, _) in &chars {
        let i = *i;
        if None == before_before {
            before_before = Some(i);
            first = i;
            continue;
        }
        if None == before {
            before = Some(i);
            continue;
        }
        let old_diff = before.unwrap() - before_before.unwrap();
        let diff = i - before.unwrap();
        if old_diff != diff {
            s.push(((first, before.unwrap()), old_diff));
            first = i;
            before_before = Some(i);
            before = None;
        } else {
            before_before = before;
            before = Some(i);
        }
    }

    s
}

#[test]
fn test_all_unicode_upper_letter_case_range() {
    let s = all_unicode_upper_letter_case_range();

    let mut chars = (0x0000..0x3ffff + 1)
        .filter_map(|i| std::char::from_u32(i))
        .filter_map(|c| {
            let lower = c.to_lowercase().to_string();
            let len = lower.chars().count();
            if len != 1 {
                println!("alert: {}, lower: {}", c, lower);
            }
            let lower = lower.chars().next().unwrap();
            let diff_c = c as u32 as i64 - lower as u32 as i64;
            if diff_c != 0 {
                return Some((c as u32, lower, c));
            } else {
                return None;
            }
        })
        .collect::<Vec<_>>();

    chars.sort_by(|a, b| a.0.cmp(&b.0));

    let chars = chars.iter().map(|(c, _, _)| *c).collect::<Vec<_>>();
    for c in 0x0000..0x3ffff + 1 {
        for ((first, last), diff) in s.clone() {
            if first <= c && c <= last {
                if (c - first) % diff == 0 {
                    assert!(chars.contains(&c));
                } else {
                    assert!(!chars.contains(&c));
                }
            }
        }
    }
}
