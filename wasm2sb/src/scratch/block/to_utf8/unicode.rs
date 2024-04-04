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
    let mut chars = (0x0000..0x1ffff + 1)
        .filter_map(|i| std::char::from_u32(i))
        .filter_map(|c| {
            if !c.is_uppercase() {
                return None;
            }
            let lower = c.to_lowercase().to_string();
            let len = lower.chars().count();
            if len != 1 {
                return None;
            }
            let lower = lower.chars().next().unwrap();
            let diff_c = c as u32 as i64 - lower as u32 as i64;
            if diff_c != 0 {
                let upper = lower.to_uppercase().to_string();
                if upper.chars().count() == 1 && upper.chars().next().unwrap() != c {
                    println!("alert: {} -> {} -> {}", c, lower, upper.chars().next().unwrap());
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
        succession.push(c);
        kept_i = i;
    }
    s
}
