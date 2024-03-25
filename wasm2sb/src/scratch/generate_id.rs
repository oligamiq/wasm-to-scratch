pub fn generate_id() -> String {
    let mut id = String::new();
    for _ in 0..20 {
        id.push(generate_id_impl());
    }
    id
}

fn generate_id_impl() -> char {
    // `&`  `< ` `\`  ` `  `"`  `>`  `'` とエスケープが必要な文字はscratchでは使われない
    // 85種類
    let ascii_chars =
        r"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^*()-_=+[{]};:,./?";
    let mut buffer: [u8; 1] = [0; 1];
    let s = loop {
        getrandom::getrandom(&mut buffer).unwrap();
        if buffer[0] < 85 * 3 {
            break buffer[0] % 85;
        }
    };
    ascii_chars.chars().nth(s as usize).unwrap()
}
