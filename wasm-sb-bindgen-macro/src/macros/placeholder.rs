use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

use crate::version::{schema_version, wasm_sb_bindgen_version};

static mut ONCE: bool = false;

/// 最初の一回だけ関数を一個追加する
pub fn placeholder(input: TokenStream) -> Result<TokenStream> {
    if unsafe { ONCE } {
        return Ok(input);
    } else {
        unsafe { ONCE = true };
    }

    let schema_version =
        String::from("schema_version_") + schema_version().replace(".", "_").as_str();
    let wasm_sb_bindgen_version = String::from("wasm_sb_bindgen_version_")
        + wasm_sb_bindgen_version().replace(".", "_").as_str();

    let mut input = input;
    let new = quote! {
        #[automatically_derived]
        const _: () = {
            #[link(wasm_import_module = "__wasm_sb_bindgen_placeholder__")]
            extern {
                #[link_name = #schema_version]
                fn __wasm_sb_bindgen_placeholder_schema_version__();
                #[link_name = #wasm_sb_bindgen_version]
                fn __wasm_sb_bindgen_placeholder_wasm_sb_bindgen_version__();
            }

            #[export_name = "__wasm_sb_bindgen_placeholder_anchor__"]
            pub unsafe extern "C" fn __wasm_sb_bindgen_placeholder_anchor__() {
                unsafe {
                    __wasm_sb_bindgen_placeholder_schema_version__();
                    __wasm_sb_bindgen_placeholder_wasm_sb_bindgen_version__();
                }
            }
        };
    };
    input.extend(new);
    Ok(input)
}
