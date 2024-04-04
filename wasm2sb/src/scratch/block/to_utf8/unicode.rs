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
    let mut is_succession = false;
    let mut succession = Vec::new();
    let mut first = 0;
    let mut over = 0;
    let mut diff = 0;
    let chars = (0x0000..0x1ffff + 1).filter_map(|i| std::char::from_u32(i)).filter_map(|c| {
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
            return Some((lower as u32, lower))
        } else {
            return None
        }
    }).collect::<Vec<_>>();
    for i in 0x0000..0x1ffff + 1 {
        if let Some(c) = std::char::from_u32(i) {
            let diff_c = c as u32 as i64 - c.to_lowercase().to_string().chars().next().unwrap() as u32 as i64;
            let len = c.to_lowercase().to_string().chars().count();
            if c.is_uppercase() && diff_c != 0 && len == 1 {
                if is_succession {
                    if diff != diff_c {
                        s.push(((first, i - over - 1), diff, succession.iter().collect()));
                        succession = Vec::new();
                        diff = diff_c;
                        first = i;
                    }
                    succession.push(c);
                } else {
                    diff = diff_c;
                    is_succession = true;
                    first = i;
                    succession.push(c);
                }
                over = 0;
            } else {
                if is_succession {
                    over += 1;
                }
            }
        }
    }
    s
}
