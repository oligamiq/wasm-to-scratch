pub fn all_unicode() -> String {
    let mut s = String::new();
    for i in 0x0000..0x007f {
        s.push(std::char::from_u32(i).unwrap_or_default());
    }
    for i in 0x0080..0x07ff {
        s.push(std::char::from_u32(i).unwrap_or_default());
    }
    for i in 0x0800..0xffff {
        s.push(std::char::from_u32(i).unwrap_or_default());
    }
    s
}
