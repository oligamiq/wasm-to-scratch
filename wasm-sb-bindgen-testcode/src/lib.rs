extern crate wee_alloc;

use std::{
    cell::OnceCell,
    fmt::format,
    sync::{atomic::AtomicBool, OnceLock},
};

use wasm_sb_bindgen::{wasm_sb_bindgen, SbError};

// #[wasm_sb_bindgen]
// pub fn greet() -> String {
//     "Hello, world!".into()
// }

#[wasm_sb_bindgen]
pub fn nya(t: String) -> String {
    // <wasm_bindgen::convert::ReturnWasmAbi>::return_abi(Ok(1));

    ("nya(=^・・^=) {}".to_string() + &t).clone()
}

// https://stackoverflow.com/questions/53214434/how-to-return-a-rust-closure-to-javascript-via-webassembly
// #[wasm_sb_bindgen]
// pub fn nya_sama() -> SbValue {
//     // let _ = unsafe { alert(7) };

//     unsafe { __wasm_sb_bindgen_debug_num(-6) };

//     let box_fn = Box::new(move |u| {
//         nya(format!("ddd: {u}"))
//     });

//     unsafe { __wasm_sb_bindgen_debug_num(-5) };

//     let as_fn = box_fn as Box<dyn FnMut(String) -> String>;

//     unsafe { __wasm_sb_bindgen_debug_num(-4) };

//     let cb = Closure::wrap(as_fn);

//     unsafe { __wasm_sb_bindgen_debug_num(-3) };

//     // Extract the `JsValue` from this `Closure`, the handle
//     // on a JS function representing the closure
//     let ret = cb.as_ref().clone();

//     // Once `cb` is dropped it'll "neuter" the closure and
//     // cause invocations to throw a JS exception. Memory
//     // management here will come later, so just leak it
//     // for now.
//     cb.forget();

//     ret
// }

// #[wasm_sb_bindgen]
// pub fn nya_sama() -> String {
//     "nya_sama2".into()
// }

static STR: AtomicBool = AtomicBool::new(false);

#[wasm_sb_bindgen]
pub fn nya_sama() -> String {
    STR.store(true, std::sync::atomic::Ordering::SeqCst);
    format!(
        "nya_sama2: {}",
        STR.load(std::sync::atomic::Ordering::SeqCst)
    )
}

#[wasm_sb_bindgen]
pub fn nya_2(t: String) -> Result<Option<String>, SbError> {
    // <wasm_bindgen::convert::ReturnWasmAbi>::return_abi(Ok(1));

    Ok(Some(("nya(=^・・^=) {}".to_string() + &t).clone()))
}

#[wasm_sb_bindgen]
pub fn nya_3(_t: String) -> u64 {
    2
}

#[wasm_sb_bindgen]
pub fn nya_4(_t: String) {}

#[wasm_sb_bindgen]
pub fn woof(t: String, n: usize) -> String {
    format!("woof(=^・・^=) {} {}", t, n)
}

#[wasm_sb_bindgen]
pub fn kow(t: char) -> String {
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
// #[automatically_derived]
// const _: () = {
//     #[no_mangle]
//     #[doc(hidden)]
//     pub extern "C" fn __wasm_sb_bindgen_describe_kow() {
//         use wasm_sb_bindgen::describe::*;
//         wasm_sb_bindgen::__rt::link_mem_intrinsics();
//         <dyn Fn(char) -> String>::describe();
//     }
// };
