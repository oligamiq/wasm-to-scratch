#![no_std]

// mod utils;

use wasm_bindgen::prelude::*;

// // #[wasm_bindgen]
// // extern "C" {
// //     #[wasm_bindgen(js_namespace=console)]
// //     fn log(s: &str);
// // }

// // #[wasm_bindgen]
// // pub fn greet() {
// //     log("Hello, world!");
// // }

// #[wasm_bindgen]
// pub fn greet() -> String {
//     "Hello, world!".into()
// }

// #[wasm_bindgen]
// pub fn nya(t: String) -> String {
//     format!("nya(=^・・^=) {}", t)
// }

// // https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/convert/trait.IntoWasmAbi.html

// #[wasm_bindgen]
// pub fn bowwow(t: String) -> Option<usize> {
//     if t.contains("bowwow") {
//         Some(t.len())
//     } else {
//         None
//     }
// }

// #[wasm_bindgen]
// pub fn meow(t: String) -> Option<String> {
//     if t.contains("meow") {
//         Some(format!("meow(=^・・^=) {}", t))
//     } else {
//         None
//     }
// }

// #[wasm_bindgen]
// pub fn woof(t: String, n: usize) -> String {
//     format!("woof(=^・・^=) {} {}", t, n)
// }

// #[wasm_bindgen]
// pub fn bark(t: String, n: String) -> String {
//     format!("bark(=^・・^=) {} {}", t, n)
// }

// #[wasm_bindgen]
// pub fn mew(t: String, n: Option<String>) -> String {
//     match n {
//         Some(n) => format!("mew(=^・・^=) {} {}", t, n),
//         None => format!("mew(=^・・^=) {}", t),
//     }
// }

// #[wasm_bindgen]
// pub fn nya_nya(t: String, n: Option<usize>) -> String {
//     match n {
//         Some(n) => format!("nya_nya(=^・・^=) {} {}", t, n),
//         None => format!("nya_nya(=^・・^=) {}", t),
//     }
// }

// #[wasm_bindgen]
// pub fn woof_woof(t: Option<String>, n: String) -> String {
//     match t {
//         Some(t) => format!("woof_woof(=^・・^=) {} {}", t, n),
//         None => format!("woof_woof(=^・・^=) {}", n),
//     }
// }

#[wasm_bindgen]
pub fn non() {
    // log("non");
}

// #[wasm_bindgen]
// pub fn vec() -> Vec<u64> {
//     let mut v = Vec::new();
//     v.push(1);
//     v.push(2);
//     v.push(3);
//     v
// }

// #[wasm_bindgen]
// pub fn vec2() -> Vec<String> {
//     let mut v = Vec::new();
//     v.push("a".to_string());
//     v.push("b".to_string());
//     v.push("c".to_string());
//     v
// }

// static mut V: Vec<u64> = Vec::new();

// #[wasm_bindgen]
// pub fn vec_push() {
//     unsafe {
//         V.push(1);
//     }
// }

static mut n: u64 = 0;

#[wasm_bindgen]
pub fn inc() {
    unsafe {
        n += 1;
    }
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
