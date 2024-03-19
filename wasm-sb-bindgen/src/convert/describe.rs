// ref wasm-bindgen-describe

#![doc(hidden)]

use crate::{if_std, SbError, SbValue, __wasm_sb_bindgen_describe};

use super::cast::SbObject;

#[inline(always)] // see the wasm-interpreter crate
pub fn inform(a: u32) {
    unsafe { __wasm_sb_bindgen_describe(a) }
}

macro_rules! tys {
    ($($a:ident)*) => (tys! { @ ($($a)*) 0 });
    (@ () $v:expr) => {};
    (@ ($a:ident $($b:ident)*) $v:expr) => {
        pub const $a: u32 = $v;
        tys!(@ ($($b)*) $v+1);
    }
}

tys! {
    I8
    U8
    I16
    U16
    I32
    U32
    I64
    U64
    F32
    F64
    BOOLEAN
    FUNCTION
    CLOSURE
    CACHED_STRING
    STRING
    REF
    REFMUT
    LONGREF
    SLICE
    VECTOR
    EXTERNREF
    NAMED_EXTERNREF
    ENUM
    RUST_STRUCT
    CHAR
    OPTIONAL
    RESULT
    UNIT
}
pub trait WasmDescribe {
    fn describe();
}

/// Trait for element types to implement WasmDescribe for vectors of
/// themselves.
pub trait WasmDescribeVector {
    fn describe_vector();
}

macro_rules! simple {
    ($($t:ident => $d:ident)*) => ($(
        impl WasmDescribe for $t {
            fn describe() { inform($d) }
        }
    )*)
}

simple! {
    i8 => I8
    u8 => U8
    i16 => I16
    u16 => U16
    i32 => I32
    u32 => U32
    i64 => I64
    u64 => U64
    isize => I32
    usize => U32
    f32 => F32
    f64 => F64
    bool => BOOLEAN
    char => CHAR
    str => STRING
    SbValue => EXTERNREF
}

impl<T> WasmDescribe for *const T {
    fn describe() {
        inform(U32)
    }
}

impl<T> WasmDescribe for *mut T {
    fn describe() {
        inform(U32)
    }
}

impl<T: WasmDescribe> WasmDescribe for [T] {
    fn describe() {
        inform(SLICE);
        T::describe();
    }
}

impl<'a, T: WasmDescribe + ?Sized> WasmDescribe for &'a T {
    fn describe() {
        inform(REF);
        T::describe();
    }
}

impl<'a, T: WasmDescribe + ?Sized> WasmDescribe for &'a mut T {
    fn describe() {
        inform(REFMUT);
        T::describe();
    }
}

// if_std! {
simple! {
    String => STRING
}

impl WasmDescribeVector for SbValue {
    fn describe_vector() {
        inform(VECTOR);
        SbValue::describe();
    }
}

impl<T: SbObject> WasmDescribeVector for T {
    fn describe_vector() {
        inform(VECTOR);
        T::describe();
    }
}

impl<T: WasmDescribeVector> WasmDescribe for Box<[T]> {
    fn describe() {
        T::describe_vector();
    }
}

impl<T> WasmDescribe for Vec<T>
where
    Box<[T]>: WasmDescribe,
{
    fn describe() {
        <Box<[T]>>::describe();
    }
}
// }

impl<T: WasmDescribe> WasmDescribe for Option<T> {
    fn describe() {
        inform(OPTIONAL);
        T::describe();
    }
}

impl WasmDescribe for () {
    fn describe() {
        inform(UNIT)
    }
}

impl<T: WasmDescribe, E: Into<SbValue>> WasmDescribe for Result<T, E> {
    fn describe() {
        inform(RESULT);
        T::describe();
    }
}

impl WasmDescribe for SbError {
    fn describe() {
        SbValue::describe();
    }
}
