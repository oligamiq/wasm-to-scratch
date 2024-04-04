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
    const ALL_CAPACITY: u32 = 0x60;
    const OVER_CAPACITY: u32 = 0x30;
    let mut s: Vec<((u32, u32), i64, String)> = Vec::new();
    let mut is_succession = false;
    let mut succession = Vec::new();
    let mut first = 0;
    let mut over = 0;
    let mut diff = 0;
    for i in 0x0000..0x1ffff + 1 {
        if let Some(c) = std::char::from_u32(i) {
            let diff_c = c as u32 as i64 - c.to_uppercase().to_string().chars().next().unwrap() as u32 as i64;
            if c.is_lowercase() && diff_c != 0 {
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
                    // if first + ALL_CAPACITY < i && over > OVER_CAPACITY {
                    //     // first <= chars <= i - over
                    //     s.push(((first, i - over), diff, succession.iter().collect()));
                    //     succession = Vec::new();
                    //     is_succession = false;
                    //     over = 0;
                    // }
                }
            }
        }
    }
    s
}
