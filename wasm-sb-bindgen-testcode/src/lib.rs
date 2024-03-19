extern crate wee_alloc;

use wasm_sb_bindgen::wasm_sb_bindgen;

// #[wasm_sb_bindgen]
// pub fn greet() -> String {
//     "Hello, world!".into()
// }

#[wasm_sb_bindgen]
pub fn nya(t: String) -> String {
    // <wasm_bindgen::convert::ReturnWasmAbi>::return_abi(Ok(1));

    ("nya(=^・・^=) {}".to_string() + &t).clone()
}

#[wasm_sb_bindgen]
pub fn nya_2(t: String) -> String {
    // <wasm_bindgen::convert::ReturnWasmAbi>::return_abi(Ok(1));

    ("nya(=^・・^=) {}".to_string() + &t).clone()
}

#[wasm_sb_bindgen]
pub fn woof(t: String, n: usize) -> String {
    format!("woof(=^・・^=) {} {}", t, n)
}

// #[wasm_sb_bindgen]
// pub fn kow<T>(t: T) -> String where
//     T: WasmAbi + Debug + Clone + FromWasmAbi
// {
//     format!("woof(=^・・^=) {:?}", t)
// }

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
