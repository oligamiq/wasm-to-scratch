extern crate wee_alloc;

use wasm_sb_bindgen::{convert::{FromWasmAbi, WasmAbi}, wasm_sb_bindgen};
use std::fmt::Debug;

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
pub fn nya_3(t: String) -> u64 {
    2
}

#[wasm_sb_bindgen]
pub fn nya_4(t: String) {
}

#[wasm_sb_bindgen]
pub fn woof(t: String, n: usize) -> String {
    format!("woof(=^・・^=) {} {}", t, n)
}

#[wasm_sb_bindgen]
pub fn kow(t: char) -> String
{
    format!("woof(=^・・^=) {:?}", t)
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// const _: () = {
//     #[export_name = "kow"]
//     pub unsafe extern "C" fn __wasm_sb_bindgen_generated_kow(
//         arg0_1: <<char as wasm_sb_bindgen::convert::FromWasmAbi>::Abi as wasm_sb_bindgen::convert::WasmAbi>::Prim1,
//         arg0_2: <<char as wasm_sb_bindgen::convert::FromWasmAbi>::Abi as wasm_sb_bindgen::convert::WasmAbi>::Prim2,
//         arg0_3: <<char as wasm_sb_bindgen::convert::FromWasmAbi>::Abi as wasm_sb_bindgen::convert::WasmAbi>::Prim3,
//         arg0_4: <<char as wasm_sb_bindgen::convert::FromWasmAbi>::Abi as wasm_sb_bindgen::convert::WasmAbi>::Prim4,
//     ) -> wasm_sb_bindgen::convert::WasmRet<
//         <String as wasm_sb_bindgen::convert::ReturnWasmAbi>::Abi,
//     > {
//         let _ret = {
//             let arg0 = unsafe {
//                 <char as wasm_sb_bindgen::convert::FromWasmAbi>::from_abi(
//                     <<char as wasm_sb_bindgen::convert::FromWasmAbi>::Abi as wasm_sb_bindgen::convert::WasmAbi>::join(
//                         arg0_1,
//                         arg0_2,
//                         arg0_3,
//                         arg0_4,
//                     ),
//                 )
//             };
//             let _ret = kow(arg0);
//             _ret
//         };
//         let t = <String as wasm_sb_bindgen::convert::ReturnWasmAbi>::return_abi(_ret).into();
//         t
//     }
// };
