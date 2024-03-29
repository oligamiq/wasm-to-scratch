use crate::{
    __wasm_sb_bindgen_i64_join, __wasm_sb_bindgen_i64_split, __wasm_sb_bindgen_u64_join,
    __wasm_sb_bindgen_u64_split,
};

use super::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi, WasmAbi};

#[repr(C)]
pub struct Wasm8Bytes {
    pub first: f64,
    pub last: f64,
}

impl WasmAbi for Wasm8Bytes {
    // 上位ビット
    type Prim1 = f64;
    // 下位ビット
    type Prim2 = f64;
    type Prim3 = ();
    type Prim4 = ();

    #[inline]
    fn split(self) -> (f64, f64, (), ()) {
        // let Wasm8Bytes { first, last } = unsafe { __wasm_sb_bindgen_i64_split(self) };
        // (first, last, (), ())
        (self.first, self.last, (), ())
    }

    #[inline]
    fn join(first: f64, last: f64, _: (), _: ()) -> Self {
        // let bytes = Wasm8Bytes {
        //     first: first,
        //     last: last,
        // };
        // unsafe { __wasm_sb_bindgen_i64_join(bytes) }
        Wasm8Bytes {
            first: first,
            last: last,
        }
    }
}

macro_rules! type_abi_big_bytes {
    ($($t:tt)*) => ($(

        impl OptionIntoWasmAbi for $t {
            #[inline]
            fn none() -> Wasm8Bytes {
                Wasm8Bytes {
                    first: std::f64::NAN,
                    last: std::f64::NAN,
                }
            }
        }

        impl OptionFromWasmAbi for $t {

            #[inline]
            fn is_none(sb: &Wasm8Bytes) -> bool {
                sb.first.is_nan() && sb.last.is_nan()
            }
        }
    )*)
}

impl IntoWasmAbi for i64 {
    type Abi = Wasm8Bytes;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        let Wasm8Bytes { first, last } = unsafe { __wasm_sb_bindgen_i64_split(self) };
        Wasm8Bytes {
            first: first,
            last: last,
        }
    }
}

impl FromWasmAbi for i64 {
    type Abi = Wasm8Bytes;

    #[inline]
    unsafe fn from_abi(sb: Wasm8Bytes) -> Self {
        unsafe { __wasm_sb_bindgen_i64_join(sb) }
    }
}

impl IntoWasmAbi for u64 {
    type Abi = Wasm8Bytes;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        let Wasm8Bytes { first, last } = unsafe { __wasm_sb_bindgen_u64_split(self) };
        Wasm8Bytes {
            first: first,
            last: last,
        }
    }
}

impl FromWasmAbi for u64 {
    type Abi = Wasm8Bytes;

    #[inline]
    unsafe fn from_abi(sb: Wasm8Bytes) -> Self {
        unsafe { __wasm_sb_bindgen_u64_join(sb) }
    }
}

type_abi_big_bytes!(i64 u64);
