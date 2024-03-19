use std::{
    mem,
    ops::{Deref, DerefMut},
};

use crate::__wasm_sb_bindgen_copy_to_typed_array;

use super::{
    cast::SbObject,
    describe::{inform, WasmDescribeVector, *},
    impls::{sb_value_vector_from_abi, sb_value_vector_into_abi},
    FromWasmAbi, IntoWasmAbi, LongRefFromWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi,
    RefFromWasmAbi, RefMutFromWasmAbi, VectorFromWasmAbi, VectorIntoWasmAbi, WasmAbi,
};
use crate::SbValue;

#[repr(C)]
pub struct WasmSlice {
    pub ptr: u32,
    pub len: u32,
}

impl WasmAbi for WasmSlice {
    /// `self.ptr`
    type Prim1 = u32;
    /// `self.len`
    type Prim2 = u32;
    type Prim3 = ();
    type Prim4 = ();

    #[inline]
    fn split(self) -> (u32, u32, (), ()) {
        (self.ptr, self.len, (), ())
    }

    #[inline]
    fn join(ptr: u32, len: u32, _: (), _: ()) -> Self {
        Self { ptr, len }
    }
}

#[inline]
fn null_slice() -> WasmSlice {
    WasmSlice { ptr: 0, len: 0 }
}

// if_std! {
pub struct WasmMutSlice {
    pub slice: WasmSlice,
    pub idx: u32,
}

impl WasmAbi for WasmMutSlice {
    /// `self.slice.ptr`
    type Prim1 = u32;
    /// `self.slice.len`
    type Prim2 = u32;
    /// `self.idx`
    type Prim3 = u32;
    type Prim4 = ();

    #[inline]
    fn split(self) -> (u32, u32, u32, ()) {
        (self.slice.ptr, self.slice.len, self.idx, ())
    }

    #[inline]
    fn join(ptr: u32, len: u32, idx: u32, _: ()) -> Self {
        Self {
            slice: WasmSlice { ptr, len },
            idx,
        }
    }
}

/// The representation of a mutable slice passed from Sb to Rust.
pub struct MutSlice<T> {
    /// A copy of the data in the Sb typed array.
    contents: Box<[T]>,
    /// A reference to the original Sb typed array.
    sb: SbValue,
}

impl<T> Drop for MutSlice<T> {
    fn drop(&mut self) {
        unsafe {
            __wasm_sb_bindgen_copy_to_typed_array(
                self.contents.as_ptr() as *const u8,
                self.contents.len() * mem::size_of::<T>(),
                self.sb.idx,
            );
        }
    }
}

impl<T> Deref for MutSlice<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.contents
    }
}

impl<T> DerefMut for MutSlice<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.contents
    }
}
// }

macro_rules! vectors {
    ($($t:ident)*) => ($(
        // if_std! {
            impl WasmDescribeVector for $t {
                fn describe_vector() {
                    inform(VECTOR);
                    $t::describe();
                }
            }

            impl VectorIntoWasmAbi for $t {
                type Abi = WasmSlice;

                #[inline]
                fn vector_into_abi(vector: Box<[$t]>) -> WasmSlice {
                    let ptr = vector.as_ptr();
                    let len = vector.len();
                    mem::forget(vector);
                    WasmSlice {
                        ptr: ptr.into_abi(),
                        len: len as u32,
                    }
                }
            }

            impl VectorFromWasmAbi for $t {
                type Abi = WasmSlice;

                #[inline]
                unsafe fn vector_from_abi(sb: WasmSlice) -> Box<[$t]> {
                    let ptr = <*mut $t>::from_abi(sb.ptr);
                    let len = sb.len as usize;
                    Vec::from_raw_parts(ptr, len, len).into_boxed_slice()
                }
            }
        // }

        impl<'a> IntoWasmAbi for &'a [$t] {
            type Abi = WasmSlice;

            #[inline]
            fn into_abi(self) -> WasmSlice {
                WasmSlice {
                    ptr: self.as_ptr().into_abi(),
                    len: self.len() as u32,
                }
            }
        }

        impl<'a> OptionIntoWasmAbi for &'a [$t] {
            #[inline]
            fn none() -> WasmSlice { null_slice() }
        }

        impl<'a> IntoWasmAbi for &'a mut [$t] {
            type Abi = WasmSlice;

            #[inline]
            fn into_abi(self) -> WasmSlice {
                (&*self).into_abi()
            }
        }

        impl<'a> OptionIntoWasmAbi for &'a mut [$t] {
            #[inline]
            fn none() -> WasmSlice { null_slice() }
        }

        impl RefFromWasmAbi for [$t] {
            type Abi = WasmSlice;
            type Anchor = Box<[$t]>;

            #[inline]
            unsafe fn ref_from_abi(sb: WasmSlice) -> Box<[$t]> {
                <Box<[$t]>>::from_abi(sb)
            }
        }

        impl RefMutFromWasmAbi for [$t] {
            type Abi = WasmMutSlice;
            type Anchor = MutSlice<$t>;

            #[inline]
            unsafe fn ref_mut_from_abi(sb: WasmMutSlice) -> MutSlice<$t> {
                let contents = <Box<[$t]>>::from_abi(sb.slice);
                let sb = SbValue::from_abi(sb.idx);
                MutSlice { contents, sb }
            }
        }

        impl LongRefFromWasmAbi for [$t] {
            type Abi = WasmSlice;
            type Anchor = Box<[$t]>;

            #[inline]
            unsafe fn long_ref_from_abi(sb: WasmSlice) -> Box<[$t]> {
                Self::ref_from_abi(sb)
            }
        }
    )*)
}

vectors! {
    u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64
}

// if_std! {
impl WasmDescribeVector for String {
    fn describe_vector() {
        inform(VECTOR);
        inform(NAMED_EXTERNREF);
        // Trying to use an actual loop for this breaks the wasm interpreter.
        inform(6);
        inform('s' as u32);
        inform('t' as u32);
        inform('r' as u32);
        inform('i' as u32);
        inform('n' as u32);
        inform('g' as u32);
    }
}

impl VectorIntoWasmAbi for String {
    type Abi = <Box<[SbValue]> as IntoWasmAbi>::Abi;

    fn vector_into_abi(vector: Box<[Self]>) -> Self::Abi {
        sb_value_vector_into_abi(vector)
    }
}

impl VectorFromWasmAbi for String {
    type Abi = <Box<[SbValue]> as FromWasmAbi>::Abi;

    unsafe fn vector_from_abi(sb: Self::Abi) -> Box<[Self]> {
        sb_value_vector_from_abi(sb)
    }
}
// }

#[inline]
fn unsafe_get_cached_str(_x: &str) -> Option<WasmSlice> {
    None
}

// if_std! {
impl<T> IntoWasmAbi for Vec<T>
where
    Box<[T]>: IntoWasmAbi<Abi = WasmSlice>,
{
    type Abi = <Box<[T]> as IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self.into_boxed_slice().into_abi()
    }
}

impl<T> OptionIntoWasmAbi for Vec<T>
where
    Box<[T]>: IntoWasmAbi<Abi = WasmSlice>,
{
    #[inline]
    fn none() -> WasmSlice {
        null_slice()
    }
}

impl<T> FromWasmAbi for Vec<T>
where
    Box<[T]>: FromWasmAbi<Abi = WasmSlice>,
{
    type Abi = <Box<[T]> as FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(sb: Self::Abi) -> Self {
        <Box<[T]>>::from_abi(sb).into()
    }
}

impl<T> OptionFromWasmAbi for Vec<T>
where
    Box<[T]>: FromWasmAbi<Abi = WasmSlice>,
{
    #[inline]
    fn is_none(abi: &WasmSlice) -> bool {
        abi.ptr == 0
    }
}

impl IntoWasmAbi for String {
    type Abi = <Vec<u8> as IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        // This is safe because the SbValue is immediately looked up in the heap and
        // then returned, so use-after-free cannot occur.
        unsafe_get_cached_str(&self).unwrap_or_else(|| self.into_bytes().into_abi())
    }
}

impl OptionIntoWasmAbi for String {
    #[inline]
    fn none() -> Self::Abi {
        null_slice()
    }
}

impl FromWasmAbi for String {
    type Abi = <Vec<u8> as FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(sb: Self::Abi) -> Self {
        String::from_utf8_unchecked(<Vec<u8>>::from_abi(sb))
    }
}

impl OptionFromWasmAbi for String {
    #[inline]
    fn is_none(slice: &WasmSlice) -> bool {
        slice.ptr == 0
    }
}
// }

impl<'a> IntoWasmAbi for &'a str {
    type Abi = <&'a [u8] as IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        // This is safe because the SbValue is immediately looked up in the heap and
        // then returned, so use-after-free cannot occur.
        unsafe_get_cached_str(self).unwrap_or_else(|| self.as_bytes().into_abi())
    }
}

impl<'a> OptionIntoWasmAbi for &'a str {
    #[inline]
    fn none() -> Self::Abi {
        null_slice()
    }
}

impl RefFromWasmAbi for str {
    type Abi = <[u8] as RefFromWasmAbi>::Abi;
    type Anchor = Box<str>;

    #[inline]
    unsafe fn ref_from_abi(sb: Self::Abi) -> Self::Anchor {
        mem::transmute::<Box<[u8]>, Box<str>>(<Box<[u8]>>::from_abi(sb))
    }
}

impl LongRefFromWasmAbi for str {
    type Abi = <[u8] as RefFromWasmAbi>::Abi;
    type Anchor = Box<str>;

    #[inline]
    unsafe fn long_ref_from_abi(sb: Self::Abi) -> Self::Anchor {
        Self::ref_from_abi(sb)
    }
}

// if_std! {
// use crate::SbValue;

impl<T: VectorIntoWasmAbi> IntoWasmAbi for Box<[T]> {
    type Abi = <T as VectorIntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        T::vector_into_abi(self)
    }
}

impl<T> OptionIntoWasmAbi for Box<[T]>
where
    Self: IntoWasmAbi<Abi = WasmSlice>,
{
    fn none() -> WasmSlice {
        null_slice()
    }
}

impl<T: VectorFromWasmAbi> FromWasmAbi for Box<[T]> {
    type Abi = <T as VectorFromWasmAbi>::Abi;

    unsafe fn from_abi(sb: Self::Abi) -> Self {
        T::vector_from_abi(sb)
    }
}

impl<T> OptionFromWasmAbi for Box<[T]>
where
    Self: FromWasmAbi<Abi = WasmSlice>,
{
    fn is_none(slice: &WasmSlice) -> bool {
        slice.ptr == 0
    }
}

impl VectorIntoWasmAbi for SbValue {
    type Abi = WasmSlice;

    #[inline]
    fn vector_into_abi(vector: Box<[Self]>) -> WasmSlice {
        let ptr = vector.as_ptr();
        let len = vector.len();
        mem::forget(vector);
        WasmSlice {
            ptr: ptr.into_abi(),
            len: len as u32,
        }
    }
}

impl VectorFromWasmAbi for SbValue {
    type Abi = WasmSlice;

    #[inline]
    unsafe fn vector_from_abi(sb: WasmSlice) -> Box<[Self]> {
        let ptr = <*mut SbValue>::from_abi(sb.ptr);
        let len = sb.len as usize;
        Vec::from_raw_parts(ptr, len, len).into_boxed_slice()
    }
}

impl<T> VectorIntoWasmAbi for T
where
    T: SbObject,
{
    type Abi = WasmSlice;

    #[inline]
    fn vector_into_abi(vector: Box<[T]>) -> WasmSlice {
        let ptr = vector.as_ptr();
        let len = vector.len();
        mem::forget(vector);
        WasmSlice {
            ptr: ptr.into_abi(),
            len: len as u32,
        }
    }
}

impl<T> VectorFromWasmAbi for T
where
    T: SbObject,
{
    type Abi = WasmSlice;

    #[inline]
    unsafe fn vector_from_abi(sb: WasmSlice) -> Box<[T]> {
        let ptr = <*mut SbValue>::from_abi(sb.ptr);
        let len = sb.len as usize;
        let vec: Vec<T> = Vec::from_raw_parts(ptr, len, len)
            .drain(..)
            .map(|sb_value| T::unchecked_from_sb(sb_value))
            .collect();
        vec.into_boxed_slice()
    }
}
// }
