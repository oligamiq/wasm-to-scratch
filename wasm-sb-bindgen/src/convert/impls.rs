use std::mem::{self, ManuallyDrop};

use crate::{SbError, SbValue, UnwrapThrowExt as _};

use super::{
    FromWasmAbi, IntoWasmAbi, LongRefFromWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi,
    RefFromWasmAbi, ReturnWasmAbi, TryFromSbValue, WasmAbi, WasmPrimitive,
};
// if_std! {
//     use std::boxed::Box;
//     use std::fmt::Debug;
//     use std::vec::Vec;
// }

// Primitive types can always be passed over the ABI.
impl<T: WasmPrimitive> WasmAbi for T {
    type Prim1 = Self;
    type Prim2 = ();
    type Prim3 = ();
    type Prim4 = ();

    #[inline]
    fn split(self) -> (Self, (), (), ()) {
        (self, (), (), ())
    }

    #[inline]
    fn join(prim: Self, _: (), _: (), _: ()) -> Self {
        prim
    }
}

impl<T: WasmAbi<Prim4 = ()>> WasmAbi for Option<T> {
    /// Whether this `Option` is a `Some` value.
    type Prim1 = f64;
    type Prim2 = <T as WasmAbi>::Prim1;
    type Prim3 = <T as WasmAbi>::Prim2;
    type Prim4 = <T as WasmAbi>::Prim3;

    #[inline]
    fn split(
        self,
    ) -> (
        f64,
        <T as WasmAbi>::Prim1,
        <T as WasmAbi>::Prim2,
        <T as WasmAbi>::Prim3,
    ) {
        match self {
            None => (
                0f64,
                Default::default(),
                Default::default(),
                Default::default(),
            ),
            Some(value) => {
                let (prim1, prim2, prim3, ()) = value.split();
                (1f64, prim1, prim2, prim3)
            }
        }
    }

    #[inline]
    fn join(
        is_some: f64,
        prim1: <T as WasmAbi>::Prim1,
        prim2: <T as WasmAbi>::Prim2,
        prim3: <T as WasmAbi>::Prim3,
    ) -> Self {
        if is_some == 0f64 {
            None
        } else {
            Some(T::join(prim1, prim2, prim3, ()))
        }
    }
}

macro_rules! type_wasm_native {
    ($($t:tt as $c:tt)*) => ($(
        impl IntoWasmAbi for $t {
            type Abi = $c;

            #[inline]
            fn into_abi(self) -> $c { self as $c }
        }

        impl FromWasmAbi for $t {
            type Abi = $c;

            #[inline]
            unsafe fn from_abi(sb: $c) -> Self { sb as $t }
        }

        impl IntoWasmAbi for Option<$t> {
            type Abi = Option<$c>;

            #[inline]
            fn into_abi(self) -> Self::Abi {
                self.map(|v| v as $c)
            }
        }

        impl FromWasmAbi for Option<$t> {
            type Abi = Option<$c>;

            #[inline]
            unsafe fn from_abi(sb: Self::Abi) -> Self {
                sb.map(|v: $c| v as $t)
            }
        }
    )*)
}

type_wasm_native!(f64 as f64);

macro_rules! type_abi_as_f64 {
    ($($t:tt)*) => ($(
        impl IntoWasmAbi for $t {
            type Abi = f64;

            #[inline]
            fn into_abi(self) -> f64 { self as f64 }
        }

        impl FromWasmAbi for $t {
            type Abi = f64;

            #[inline]
            unsafe fn from_abi(sb: f64) -> Self { sb as $t }
        }

        impl OptionIntoWasmAbi for $t {
            #[inline]
            fn none() -> f64 { std::f64::MAX }
        }

        impl OptionFromWasmAbi for $t {
            #[inline]
            fn is_none(sb: &f64) -> bool { *sb == std::f64::MAX }
        }
    )*)
}

type_abi_as_f64!(i8 u8 i16 u16 i32 isize u32 usize f32);

impl IntoWasmAbi for bool {
    type Abi = f64;

    #[inline]
    fn into_abi(self) -> f64 {
        self as u32 as f64
    }
}

impl FromWasmAbi for bool {
    type Abi = f64;

    #[inline]
    unsafe fn from_abi(sb: f64) -> bool {
        sb != 0f64
    }
}

impl OptionIntoWasmAbi for bool {
    #[inline]
    fn none() -> f64 {
        std::f64::MAX
    }
}

impl OptionFromWasmAbi for bool {
    #[inline]
    fn is_none(sb: &f64) -> bool {
        *sb == std::f64::MAX
    }
}

impl IntoWasmAbi for char {
    type Abi = f64;

    #[inline]
    fn into_abi(self) -> f64 {
        self as u32 as f64
    }
}

impl FromWasmAbi for char {
    type Abi = f64;

    #[inline]
    unsafe fn from_abi(sb: f64) -> char {
        char::from_u32_unchecked(sb as u32)
    }
}

impl OptionIntoWasmAbi for char {
    #[inline]
    fn none() -> f64 {
        std::f64::MAX
    }
}

impl OptionFromWasmAbi for char {
    #[inline]
    fn is_none(sb: &f64) -> bool {
        *sb == std::f64::MAX
    }
}

impl<T> IntoWasmAbi for *const T {
    type Abi = f64;

    #[inline]
    fn into_abi(self) -> f64 {
        self as u32 as f64
    }
}

impl<T> FromWasmAbi for *const T {
    type Abi = f64;

    #[inline]
    unsafe fn from_abi(sb: f64) -> *const T {
        sb as u32 as *const T
    }
}

impl<T> IntoWasmAbi for *mut T {
    type Abi = f64;

    #[inline]
    fn into_abi(self) -> f64 {
        self as u32 as f64
    }
}

impl<T> FromWasmAbi for *mut T {
    type Abi = f64;

    #[inline]
    unsafe fn from_abi(sb: f64) -> *mut T {
        sb as u32 as *mut T
    }
}

impl IntoWasmAbi for SbValue {
    type Abi = f64;

    #[inline]
    fn into_abi(self) -> f64 {
        let ret = self.idx;
        mem::forget(self);
        ret
    }
}

impl FromWasmAbi for SbValue {
    type Abi = f64;

    #[inline]
    unsafe fn from_abi(sb: f64) -> SbValue {
        SbValue::_new(sb)
    }
}

impl<'a> IntoWasmAbi for &'a SbValue {
    type Abi = f64;

    #[inline]
    fn into_abi(self) -> f64 {
        self.idx
    }
}

impl RefFromWasmAbi for SbValue {
    type Abi = f64;
    type Anchor = ManuallyDrop<SbValue>;

    #[inline]
    unsafe fn ref_from_abi(sb: f64) -> Self::Anchor {
        ManuallyDrop::new(SbValue::_new(sb))
    }
}

impl LongRefFromWasmAbi for SbValue {
    type Abi = f64;
    type Anchor = SbValue;

    #[inline]
    unsafe fn long_ref_from_abi(sb: f64) -> Self::Anchor {
        Self::from_abi(sb)
    }
}

impl<T: OptionIntoWasmAbi> IntoWasmAbi for Option<T> {
    type Abi = T::Abi;

    #[inline]
    fn into_abi(self) -> T::Abi {
        match self {
            None => T::none(),
            Some(me) => me.into_abi(),
        }
    }
}

impl<T: OptionFromWasmAbi> FromWasmAbi for Option<T> {
    type Abi = T::Abi;

    #[inline]
    unsafe fn from_abi(sb: T::Abi) -> Self {
        if T::is_none(&sb) {
            None
        } else {
            Some(T::from_abi(sb))
        }
    }
}

impl IntoWasmAbi for () {
    type Abi = ();

    #[inline]
    fn into_abi(self) {
        self
    }
}

impl<T: WasmAbi<Prim3 = (), Prim4 = ()>> WasmAbi for Result<T, f64> {
    type Prim1 = T::Prim1;
    type Prim2 = T::Prim2;
    // The order of primitives here is such that we can pop() the possible error
    // first, deal with it and move on. Later primitives are popped off the
    // stack first.
    /// If this `Result` is an `Err`, the error value.
    type Prim3 = f64;
    /// Whether this `Result` is an `Err`.
    type Prim4 = f64;

    #[inline]
    fn split(self) -> (T::Prim1, T::Prim2, f64, f64) {
        match self {
            Ok(value) => {
                let (prim1, prim2, (), ()) = value.split();
                (prim1, prim2, 0f64, 0f64)
            }
            Err(err) => (Default::default(), Default::default(), err, 1f64),
        }
    }

    #[inline]
    fn join(prim1: T::Prim1, prim2: T::Prim2, err: f64, is_err: f64) -> Self {
        if is_err == 0f64 {
            Ok(T::join(prim1, prim2, (), ()))
        } else {
            Err(err)
        }
    }
}

impl<T, E> ReturnWasmAbi for Result<T, E>
where
    T: IntoWasmAbi,
    E: Into<SbValue>,
    T::Abi: WasmAbi<Prim3 = (), Prim4 = ()>,
{
    type Abi = Result<T::Abi, f64>;

    #[inline]
    fn return_abi(self) -> Self::Abi {
        match self {
            Ok(v) => Ok(v.into_abi()),
            Err(e) => {
                let sbval = e.into();
                Err(sbval.into_abi())
            }
        }
    }
}

impl IntoWasmAbi for SbError {
    type Abi = <SbValue as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        self.value.into_abi()
    }
}

use std::fmt::Debug;
// if_std! {
// Note: this can't take `&[T]` because the `Into<SbValue>` impl needs
// ownership of `T`.
pub fn sb_value_vector_into_abi<T: Into<SbValue>>(
    vector: Box<[T]>,
) -> <Box<[SbValue]> as IntoWasmAbi>::Abi {
    let sb_vals: Box<[SbValue]> = vector.into_vec().into_iter().map(|x| x.into()).collect();

    sb_vals.into_abi()
}

pub unsafe fn sb_value_vector_from_abi<T: TryFromSbValue>(
    sb: <Box<[SbValue]> as FromWasmAbi>::Abi,
) -> Box<[T]>
where
    T::Error: Debug,
{
    let sb_vals = <Vec<SbValue> as FromWasmAbi>::from_abi(sb);

    let mut result = Vec::with_capacity(sb_vals.len());
    for value in sb_vals {
        // We push elements one-by-one instead of using `collect` in order to improve
        // error messages. When using `collect`, this `expect_throw` is buried in a
        // giant chain of internal iterator functions, which results in the actual
        // function that takes this `Vec` falling off the end of the call stack.
        // So instead, make sure to call it directly within this function.
        //
        // This is only a problem in debug mode. Since this is the browser's error stack
        // we're talking about, it can only see functions that actually make it to the
        // final wasm binary (i.e., not inlined functions). All of those internal
        // iterator functions get inlined in release mode, and so they don't show up.
        result.push(
            T::try_from_sb_value(value).expect_throw("array contains a value of the wrong type"),
        );
    }
    result.into_boxed_slice()
}
// }
