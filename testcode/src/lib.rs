#![no_std]

extern crate alloc;
extern crate wee_alloc;

use core::sync::atomic::{AtomicBool, Ordering};

use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
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

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello, world!".into()
}

static STR: AtomicBool = AtomicBool::new(false);

#[wasm_bindgen]
pub fn nya_sama_2() -> String {
    STR.store(true, Ordering::SeqCst);
    format!("nya_sama2: {}", STR.load(Ordering::SeqCst))
}

// #[doc(hidden)]
// pub extern "C" fn __wbindgen_describe_greet() {
//     use wasm_bindgen::describe::*;
//     wasm_bindgen::__rt::link_mem_intrinsics();
//     inform(FUNCTION);
//     inform(0);
//     inform(0u32);
//     <String as WasmDescribe>::describe();
//     <String as WasmDescribe>::describe();
// }

#[wasm_bindgen]
pub fn nya(t: String) -> String {
    // <wasm_bindgen::convert::ReturnWasmAbi>::return_abi(Ok(1));
    format!("nya(=^・・^=) {}", t)
}

#[wasm_bindgen]
pub fn nya_sama(t: String) -> JsValue {
    let cb = Closure::wrap(
        Box::new(move |u| nya(format!("{t}: {u}"))) as Box<dyn FnMut(String) -> String>
    );

    // Extract the `JsValue` from this `Closure`, the handle
    // on a JS function representing the closure
    let ret = cb.as_ref().clone();

    // Once `cb` is dropped it'll "neuter" the closure and
    // cause invocations to throw a JS exception. Memory
    // management here will come later, so just leak it
    // for now.
    cb.forget();

    ret
}

// #[doc(hidden)]
// pub extern "C" fn __wbindgen_describe_non() {
//     use wasm_bindgen::describe::*;
//     wasm_bindgen::__rt::link_mem_intrinsics();
//     inform(FUNCTION);
//     inform(0);
//     inform(0u32);
//     <() as WasmDescribe>::describe();
//     <() as WasmDescribe>::describe();
// }

// // https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/convert/trait.IntoWasmAbi.html

#[wasm_bindgen]
pub fn bowwow(t: String) -> Option<usize> {
    if t.contains("bowwow") {
        Some(t.len())
    } else {
        None
    }
}
// #[doc(hidden)]
// pub extern "C" fn __wbindgen_describe_inc() {
//     use wasm_bindgen::describe::*;
//     wasm_bindgen::__rt::link_mem_intrinsics();
//     inform(FUNCTION);
//     inform(0);
//     inform(0u32);
//     <() as WasmDescribe>::describe();
//     <() as WasmDescribe>::describe();
// }

// #[wasm_bindgen]
// pub fn meow(t: String) -> Option<String> {
//     if t.contains("meow") {
//         Some(format!("meow(=^・・^=) {}", t))
//     } else {
//         None
//     }
// }

#[wasm_bindgen]
pub fn woof(t: String, z: usize) -> String {
    format!("woof(=^・・^=) {} {}", t, z)
}
// #[doc(hidden)]
// pub extern "C" fn __wbindgen_describe_woof() {
//     use wasm_bindgen::describe::*;
//     wasm_bindgen::__rt::link_mem_intrinsics();
//     inform(FUNCTION);
//     inform(0);
//     inform(2u32);
//     <String as WasmDescribe>::describe();
//     <usize as WasmDescribe>::describe();
//     <String as WasmDescribe>::describe();
//     <String as WasmDescribe>::describe();
// }

#[wasm_bindgen]
pub fn woof2(t: usize) -> String {
    format!("woof(=^・・^=) {}", t)
}
// pub extern "C" fn __wbindgen_describe_woof2() {
//     use wasm_bindgen::describe::*;
//     wasm_bindgen::__rt::link_mem_intrinsics();
//     inform(FUNCTION);
//     inform(0);
//     inform(1u32);
//     <usize as WasmDescribe>::describe();
//     <String as WasmDescribe>::describe();
//     <String as WasmDescribe>::describe();
// }

#[wasm_bindgen]
pub fn woof3(t: i64) -> String {
    format!("woof(=^・・^=) {}", t)
}
// pub extern "C" fn __wbindgen_describe_woof3() {
//     use wasm_bindgen::describe::*;
//     wasm_bindgen::__rt::link_mem_intrinsics();
//     inform(FUNCTION);
//     inform(0);
//     inform(1u32);
//     <i64 as WasmDescribe>::describe();
//     <String as WasmDescribe>::describe();
//     <String as WasmDescribe>::describe();
// }

#[wasm_bindgen]
pub fn woof4(t: i64) {
    format!("woof(=^・・^=) {}", t);
}
// pub extern "C" fn __wbindgen_describe_woof4() {
//     use wasm_bindgen::describe::*;
//     wasm_bindgen::__rt::link_mem_intrinsics();
//     inform(FUNCTION);
//     inform(0);
//     inform(1u32);
//     <i64 as WasmDescribe>::describe();
//     <() as WasmDescribe>::describe();
//     <() as WasmDescribe>::describe();
// }

#[wasm_bindgen]
pub fn woof5(t: i64) -> i64 {
    t
}
// pub extern "C" fn __wbindgen_describe_woof5() {
//     use wasm_bindgen::describe::*;
//     wasm_bindgen::__rt::link_mem_intrinsics();
//     inform(FUNCTION);
//     inform(0);
//     inform(1u32);
//     <i64 as WasmDescribe>::describe();
//     <i64 as WasmDescribe>::describe();
//     <i64 as WasmDescribe>::describe();
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
// #[doc(hidden)]
// pub extern "C" fn __wbindgen_describe_non() {
//     use wasm_bindgen::describe::*;
//     wasm_bindgen::__rt::link_mem_intrinsics();
//     inform(FUNCTION);
//     inform(0);
//     inform(0u32);
//     <() as WasmDescribe>::describe();
//     <() as WasmDescribe>::describe();
// }

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
// #[doc(hidden)]
// pub extern "C" fn __wbindgen_describe_inc() {
//     use wasm_bindgen::describe::*;
//     wasm_bindgen::__rt::link_mem_intrinsics();
//     inform(FUNCTION);
//     inform(0);
//     inform(0u32);
//     <() as WasmDescribe>::describe();
//     <() as WasmDescribe>::describe();
// }

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
