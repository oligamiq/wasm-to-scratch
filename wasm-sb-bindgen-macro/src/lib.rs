pub(crate) mod macros;
pub(crate) mod version;

use proc_macro::TokenStream;

use proc_macro_error::proc_macro_error;

#[proc_macro_attribute]
#[proc_macro_error(proc_macro_hack)]
pub fn wasm_sb_bindgen(attr: TokenStream, input: TokenStream) -> TokenStream {
    let first = macros::expand(attr.into(), input.into()).unwrap().into();
    let second = macros::placeholder(first).unwrap().into();
    second
}
