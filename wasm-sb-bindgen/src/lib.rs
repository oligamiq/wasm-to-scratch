pub mod closure;
pub mod convert;
pub mod externref;
use crate::convert::{slices::WasmSlice, WasmRet};
pub use convert::describe;
pub use convert::WasmDescribe;
use convert::{cast::SbCast, FromWasmAbi, TryFromSbValue};
use std::{
    fmt, marker, mem,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub},
};
pub use wasm_sb_bindgen_macro::wasm_sb_bindgen;

#[macro_export]
macro_rules! if_std {
    ($($i:item)*) => ($(
        #[cfg(feature = "std")] $i
    )*)
}

#[macro_export]
macro_rules! externs {
    ($(#[$attr:meta])* extern "C" { $(fn $name:ident($($args:tt)*) -> $ret:ty;)* }) => (
        #[cfg(all(target_arch = "wasm32", not(any(target_os = "emscripten", target_os = "wasi"))))]
        $(#[$attr])*
        extern "C" {
            $(fn $name($($args)*) -> $ret;)*
        }

        $(
            #[cfg(not(all(target_arch = "wasm32", not(any(target_os = "emscripten", target_os = "wasi")))))]
            #[allow(unused_variables)]
            unsafe extern fn $name($($args)*) -> $ret {
                panic!("function not implemented on non-wasm32 targets")
            }
        )*
    )
}

pub struct SbValue {
    pub(crate) idx: u32,
    _marker: marker::PhantomData<*mut u8>, // not at all threadsafe
}

const SBIDX_OFFSET: u32 = 128; // keep in sync with sb/mod.rs
const SBIDX_UNDEFINED: u32 = SBIDX_OFFSET;
const SBIDX_NULL: u32 = SBIDX_OFFSET + 1;
const SBIDX_TRUE: u32 = SBIDX_OFFSET + 2;
const SBIDX_FALSE: u32 = SBIDX_OFFSET + 3;
const SBIDX_RESERVED: u32 = SBIDX_OFFSET + 4;

impl SbValue {
    /// The `null` SB value constant.
    pub const NULL: SbValue = SbValue::_new(SBIDX_NULL);

    /// The `undefined` SB value constant.
    pub const UNDEFINED: SbValue = SbValue::_new(SBIDX_UNDEFINED);

    /// The `true` SB value constant.
    pub const TRUE: SbValue = SbValue::_new(SBIDX_TRUE);

    /// The `false` SB value constant.
    pub const FALSE: SbValue = SbValue::_new(SBIDX_FALSE);

    #[inline]
    const fn _new(idx: u32) -> SbValue {
        SbValue {
            idx,
            _marker: marker::PhantomData,
        }
    }

    /// Creates a new SB value which is a string.
    ///
    /// The utf-8 string provided is copied to the SB heap and the string will
    /// be owned by the SB garbage collector.
    #[allow(clippy::should_implement_trait)] // cannot fix without breaking change
    #[inline]
    pub fn from_str(s: &str) -> SbValue {
        unsafe { SbValue::_new(__wasm_sb_bindgen_string_new(s.as_ptr(), s.len())) }
    }

    /// Creates a new SB value which is a number.
    ///
    /// This function creates a SB value representing a number (a heap
    /// allocated number) and returns a handle to the SB version of it.
    #[inline]
    pub fn from_f64(n: f64) -> SbValue {
        unsafe { SbValue::_new(__wasm_sb_bindgen_number_new(n)) }
    }

    /// Creates a new SB value which is a bigint from a string representing a number.
    ///
    /// This function creates a SB value representing a bigint (a heap
    /// allocated large integer) and returns a handle to the SB version of it.
    #[inline]
    pub fn bigint_from_str(s: &str) -> SbValue {
        unsafe { SbValue::_new(__wasm_sb_bindgen_bigint_from_str(s.as_ptr(), s.len())) }
    }

    /// Creates a new SB value which is a boolean.
    ///
    /// This function creates a SB object representing a boolean (a heap
    /// allocated boolean) and returns a handle to the SB version of it.
    #[inline]
    pub const fn from_bool(b: bool) -> SbValue {
        if b {
            SbValue::TRUE
        } else {
            SbValue::FALSE
        }
    }

    /// Creates a new SB value representing `undefined`.
    #[inline]
    pub const fn undefined() -> SbValue {
        SbValue::UNDEFINED
    }

    /// Creates a new SB value representing `null`.
    #[inline]
    pub const fn null() -> SbValue {
        SbValue::NULL
    }

    /// Creates a new SB symbol with the optional description specified.
    ///
    /// This function will invoke the `Symbol` constructor in SB and return the
    /// SB object corresponding to the symbol created.
    pub fn symbol(description: Option<&str>) -> SbValue {
        unsafe {
            match description {
                Some(description) => SbValue::_new(__wasm_sb_bindgen_symbol_named_new(
                    description.as_ptr(),
                    description.len(),
                )),
                None => SbValue::_new(__wasm_sb_bindgen_symbol_anonymous_new()),
            }
        }
    }

    /// Creates a new `SbValue` from the SBON serialization of the object `t`
    /// provided.
    ///
    /// **This function is deprecated**, due to [creating a dependency cycle in
    /// some circumstances][dep-cycle-issue]. Use [`serde-wasm-bindgen`] or
    /// [`gloo_utils::format::SbValueSerdeExt`] instead.
    ///
    /// [dep-cycle-issue]: https://github.com/rustwasm/wasm-bindgen/issues/2770
    /// [`serde-wasm-bindgen`]: https://docs.rs/serde-wasm-bindgen
    /// [`gloo_utils::format::SbValueSerdeExt`]: https://docs.rs/gloo-utils/latest/gloo_utils/format/trait.SbValueSerdeExt.html
    ///
    /// This function will serialize the provided value `t` to a SBON string,
    /// send the SBON string to SB, parse it into a SB object, and then return
    /// a handle to the SB object. This is unlikely to be super speedy so it's
    /// not recommended for large payloads, but it's a nice to have in some
    /// situations!
    ///
    /// Usage of this API requires activating the `serde-serialize` feature of
    /// the `wasm-bindgen` crate.
    ///
    /// # Errors
    ///
    /// Returns any error encountered when serializing `T` into SBON.
    #[cfg(feature = "serde-serialize")]
    #[deprecated = "causes dependency cycles, use `serde-wasm-bindgen` or `gloo_utils::format::SbValueSerdeExt` instead"]
    pub fn from_serde<T>(t: &T) -> serde_json::Result<SbValue>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        let s = serde_json::to_string(t)?;
        unsafe {
            Ok(SbValue::_new(__wasm_sb_bindgen_json_parse(
                s.as_ptr(),
                s.len(),
            )))
        }
    }

    /// Invokes `SBON.stringify` on this value and then parses the resulting
    /// SBON into an arbitrary Rust value.
    ///
    /// **This function is deprecated**, due to [creating a dependency cycle in
    /// some circumstances][dep-cycle-issue]. Use [`serde-wasm-bindgen`] or
    /// [`gloo_utils::format::SbValueSerdeExt`] instead.
    ///
    /// [dep-cycle-issue]: https://github.com/rustwasm/wasm-bindgen/issues/2770
    /// [`serde-wasm-bindgen`]: https://docs.rs/serde-wasm-bindgen
    /// [`gloo_utils::format::SbValueSerdeExt`]: https://docs.rs/gloo-utils/latest/gloo_utils/format/trait.SbValueSerdeExt.html
    ///
    /// This function will first call `SBON.stringify` on the `SbValue` itself.
    /// The resulting string is then passed into Rust which then parses it as
    /// SBON into the resulting value.
    ///
    /// Usage of this API requires activating the `serde-serialize` feature of
    /// the `wasm-bindgen` crate.
    ///
    /// # Errors
    ///
    /// Returns any error encountered when parsing the SBON into a `T`.
    #[cfg(feature = "serde-serialize")]
    #[deprecated = "causes dependency cycles, use `serde-wasm-bindgen` or `gloo_utils::format::SbValueSerdeExt` instead"]
    pub fn into_serde<T>(&self) -> serde_json::Result<T>
    where
        T: for<'a> serde::de::Deserialize<'a>,
    {
        unsafe {
            let ret = __wasm_sb_bindgen_json_serialize(self.idx);
            let s = String::from_abi(ret);
            serde_json::from_str(&s)
        }
    }

    /// Returns the `f64` value of this SB value if it's an instance of a
    /// number.
    ///
    /// If this SB value is not an instance of a number then this returns
    /// `None`.
    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        unsafe { __wasm_sb_bindgen_number_get(self.idx).join() }
    }

    /// Tests whether this SB value is a SB string.
    #[inline]
    pub fn is_string(&self) -> bool {
        unsafe { __wasm_sb_bindgen_is_string(self.idx) == 1 }
    }

    /// If this SB value is a string value, this function copies the SB string
    /// value into wasm linear memory, encoded as UTF-8, and returns it as a
    /// Rust `String`.
    ///
    /// To avoid the copying and re-encoding, consider the
    /// `SbString::try_from()` function from [js-sys](https://docs.rs/js-sys)
    /// instead.
    ///
    /// If this SB value is not an instance of a string or if it's not valid
    /// utf-8 then this returns `None`.
    ///
    /// # UTF-16 vs UTF-8
    ///
    /// JavaScript strings in general are encoded as UTF-16, but Rust strings
    /// are encoded as UTF-8. This can cause the Rust string to look a bit
    /// different than the SB string sometimes. For more details see the
    /// [documentation about the `str` type][caveats] which contains a few
    /// caveats about the encodings.
    ///
    /// [caveats]: https://rustwasm.github.io/docs/wasm-bindgen/reference/types/str.html
    // #[cfg(feature = "std")]
    #[inline]
    pub fn as_string(&self) -> Option<String> {
        unsafe { FromWasmAbi::from_abi(__wasm_sb_bindgen_string_get(self.idx)) }
    }

    /// Returns the `bool` value of this SB value if it's an instance of a
    /// boolean.
    ///
    /// If this SB value is not an instance of a boolean then this returns
    /// `None`.
    #[inline]
    pub fn as_bool(&self) -> Option<bool> {
        unsafe {
            match __wasm_sb_bindgen_boolean_get(self.idx) {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            }
        }
    }

    /// Tests whether this SB value is `null`
    #[inline]
    pub fn is_null(&self) -> bool {
        unsafe { __wasm_sb_bindgen_is_null(self.idx) == 1 }
    }

    /// Tests whether this SB value is `undefined`
    #[inline]
    pub fn is_undefined(&self) -> bool {
        unsafe { __wasm_sb_bindgen_is_undefined(self.idx) == 1 }
    }

    /// Tests whether the type of this SB value is `symbol`
    #[inline]
    pub fn is_symbol(&self) -> bool {
        unsafe { __wasm_sb_bindgen_is_symbol(self.idx) == 1 }
    }

    /// Tests whether `typeof self == "object" && self !== null`.
    #[inline]
    pub fn is_object(&self) -> bool {
        unsafe { __wasm_sb_bindgen_is_object(self.idx) == 1 }
    }

    /// Tests whether this SB value is an instance of Array.
    #[inline]
    pub fn is_array(&self) -> bool {
        unsafe { __wasm_sb_bindgen_is_array(self.idx) == 1 }
    }

    /// Tests whether the type of this SB value is `function`.
    #[inline]
    pub fn is_function(&self) -> bool {
        unsafe { __wasm_sb_bindgen_is_function(self.idx) == 1 }
    }

    /// Tests whether the type of this SB value is `bigint`.
    #[inline]
    pub fn is_bigint(&self) -> bool {
        unsafe { __wasm_sb_bindgen_is_bigint(self.idx) == 1 }
    }

    /// Applies the unary `typeof` SB operator on a `SbValue`.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/typeof)
    #[inline]
    pub fn sb_typeof(&self) -> SbValue {
        unsafe { SbValue::_new(__wasm_sb_bindgen_typeof(self.idx)) }
    }

    /// Applies the binary `in` SB operator on the two `SbValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/in)
    #[inline]
    pub fn sb_in(&self, obj: &SbValue) -> bool {
        unsafe { __wasm_sb_bindgen_in(self.idx, obj.idx) == 1 }
    }

    /// Tests whether the value is ["truthy"].
    ///
    /// ["truthy"]: https://developer.mozilla.org/en-US/docs/Glossary/Truthy
    #[inline]
    pub fn is_truthy(&self) -> bool {
        !self.is_falsy()
    }

    /// Tests whether the value is ["falsy"].
    ///
    /// ["falsy"]: https://developer.mozilla.org/en-US/docs/Glossary/Falsy
    #[inline]
    pub fn is_falsy(&self) -> bool {
        unsafe { __wasm_sb_bindgen_is_falsy(self.idx) == 1 }
    }

    /// Get a string representation of the JavaScript object for debugging.
    #[cfg(feature = "std")]
    fn as_debug_string(&self) -> String {
        unsafe {
            let mut ret = [0; 2];
            __wasm_sb_bindgen_debug_string(&mut ret, self.idx);
            let data = Vec::from_raw_parts(ret[0] as *mut u8, ret[1], ret[1]);
            String::from_utf8_unchecked(data)
        }
    }

    /// Compare two `SbValue`s for equality, using the `==` operator in SB.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Equality)
    #[inline]
    pub fn loose_eq(&self, other: &Self) -> bool {
        unsafe { __wasm_sb_bindgen_sbval_loose_eq(self.idx, other.idx) != 0 }
    }

    /// Applies the unary `~` SB operator on a `SbValue`.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Bitwise_NOT)
    #[inline]
    pub fn bit_not(&self) -> SbValue {
        unsafe { SbValue::_new(__wasm_sb_bindgen_bit_not(self.idx)) }
    }

    /// Applies the binary `>>>` SB operator on the two `SbValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Unsigned_right_shift)
    #[inline]
    pub fn unsigned_shr(&self, rhs: &Self) -> u32 {
        unsafe { __wasm_sb_bindgen_unsigned_shr(self.idx, rhs.idx) }
    }

    /// Applies the binary `/` SB operator on two `SbValue`s, catching and returning any `RangeError` thrown.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Division)
    #[inline]
    pub fn checked_div(&self, rhs: &Self) -> Self {
        unsafe { SbValue::_new(__wasm_sb_bindgen_checked_div(self.idx, rhs.idx)) }
    }

    /// Applies the binary `**` SB operator on the two `SbValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Exponentiation)
    #[inline]
    pub fn pow(&self, rhs: &Self) -> Self {
        unsafe { SbValue::_new(__wasm_sb_bindgen_pow(self.idx, rhs.idx)) }
    }

    /// Applies the binary `<` SB operator on the two `SbValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Less_than)
    #[inline]
    pub fn lt(&self, other: &Self) -> bool {
        unsafe { __wasm_sb_bindgen_lt(self.idx, other.idx) == 1 }
    }

    /// Applies the binary `<=` SB operator on the two `SbValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Less_than_or_equal)
    #[inline]
    pub fn le(&self, other: &Self) -> bool {
        unsafe { __wasm_sb_bindgen_le(self.idx, other.idx) == 1 }
    }

    /// Applies the binary `>=` SB operator on the two `SbValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Greater_than_or_equal)
    #[inline]
    pub fn ge(&self, other: &Self) -> bool {
        unsafe { __wasm_sb_bindgen_ge(self.idx, other.idx) == 1 }
    }

    /// Applies the binary `>` SB operator on the two `SbValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Greater_than)
    #[inline]
    pub fn gt(&self, other: &Self) -> bool {
        unsafe { __wasm_sb_bindgen_gt(self.idx, other.idx) == 1 }
    }

    /// Applies the unary `+` SB operator on a `SbValue`. Can throw.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Unary_plus)
    #[inline]
    pub fn unchecked_into_f64(&self) -> f64 {
        unsafe { __wasm_sb_bindgen_as_number(self.idx) }
    }
}

impl PartialEq for SbValue {
    /// Compares two `SbValue`s for equality, using the `===` operator in SB.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Strict_equality)
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { __wasm_sb_bindgen_sbval_eq(self.idx, other.idx) != 0 }
    }
}

impl PartialEq<bool> for SbValue {
    #[inline]
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == Some(*other)
    }
}

impl PartialEq<str> for SbValue {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        *self == SbValue::from_str(other)
    }
}

impl<'a> PartialEq<&'a str> for SbValue {
    #[inline]
    fn eq(&self, other: &&'a str) -> bool {
        <SbValue as PartialEq<str>>::eq(self, other)
    }
}

if_std! {
    impl PartialEq<String> for SbValue {
        #[inline]
        fn eq(&self, other: &String) -> bool {
            <SbValue as PartialEq<str>>::eq(self, other)
        }
    }
    impl<'a> PartialEq<&'a String> for SbValue {
        #[inline]
        fn eq(&self, other: &&'a String) -> bool {
            <SbValue as PartialEq<str>>::eq(self, other)
        }
    }
}

macro_rules! forward_deref_unop {
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl $imp for $t {
            type Output = <&'static $t as $imp>::Output;

            #[inline]
            fn $method(self) -> <&'static $t as $imp>::Output {
                $imp::$method(&self)
            }
        }
    };
}

macro_rules! forward_deref_binop {
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl<'a> $imp<$t> for &'a $t {
            type Output = <&'static $t as $imp<&'static $t>>::Output;

            #[inline]
            fn $method(self, other: $t) -> <&'static $t as $imp<&'static $t>>::Output {
                $imp::$method(self, &other)
            }
        }

        impl $imp<&$t> for $t {
            type Output = <&'static $t as $imp<&'static $t>>::Output;

            #[inline]
            fn $method(self, other: &$t) -> <&'static $t as $imp<&'static $t>>::Output {
                $imp::$method(&self, other)
            }
        }

        impl $imp<$t> for $t {
            type Output = <&'static $t as $imp<&'static $t>>::Output;

            #[inline]
            fn $method(self, other: $t) -> <&'static $t as $imp<&'static $t>>::Output {
                $imp::$method(&self, &other)
            }
        }
    };
}

impl Not for &SbValue {
    type Output = bool;

    /// Applies the `!` SB operator on a `SbValue`.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Logical_NOT)
    #[inline]
    fn not(self) -> Self::Output {
        SbValue::is_falsy(self)
    }
}

forward_deref_unop!(impl Not, not for SbValue);

impl TryFrom<SbValue> for f64 {
    type Error = SbValue;

    /// Applies the unary `+` SB operator on a `SbValue`.
    /// Returns the numeric result on success, or the SB error value on error.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Unary_plus)
    #[inline]
    fn try_from(val: SbValue) -> Result<Self, Self::Error> {
        f64::try_from(&val)
    }
}

impl TryFrom<&SbValue> for f64 {
    type Error = SbValue;

    /// Applies the unary `+` SB operator on a `SbValue`.
    /// Returns the numeric result on success, or the SB error value on error.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Unary_plus)
    #[inline]
    fn try_from(val: &SbValue) -> Result<Self, Self::Error> {
        let sbval = unsafe { SbValue::_new(__wasm_sb_bindgen_try_into_number(val.idx)) };
        match sbval.as_f64() {
            Some(num) => Ok(num),
            None => Err(sbval),
        }
    }
}

impl Neg for &SbValue {
    type Output = SbValue;

    /// Applies the unary `-` SB operator on a `SbValue`.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Unary_negation)
    #[inline]
    fn neg(self) -> Self::Output {
        unsafe { SbValue::_new(__wasm_sb_bindgen_neg(self.idx)) }
    }
}

forward_deref_unop!(impl Neg, neg for SbValue);

impl BitAnd for &SbValue {
    type Output = SbValue;

    /// Applies the binary `&` SB operator on two `SbValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Bitwise_AND)
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { SbValue::_new(__wasm_sb_bindgen_bit_and(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl BitAnd, bitand for SbValue);

impl BitOr for &SbValue {
    type Output = SbValue;

    /// Applies the binary `|` SB operator on two `SbValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Bitwise_OR)
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { SbValue::_new(__wasm_sb_bindgen_bit_or(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl BitOr, bitor for SbValue);

impl BitXor for &SbValue {
    type Output = SbValue;

    /// Applies the binary `^` SB operator on two `SbValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Bitwise_XOR)
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe { SbValue::_new(__wasm_sb_bindgen_bit_xor(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl BitXor, bitxor for SbValue);

impl Shl for &SbValue {
    type Output = SbValue;

    /// Applies the binary `<<` SB operator on two `SbValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Left_shift)
    #[inline]
    fn shl(self, rhs: Self) -> Self::Output {
        unsafe { SbValue::_new(__wasm_sb_bindgen_shl(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl Shl, shl for SbValue);

impl Shr for &SbValue {
    type Output = SbValue;

    /// Applies the binary `>>` SB operator on two `SbValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Right_shift)
    #[inline]
    fn shr(self, rhs: Self) -> Self::Output {
        unsafe { SbValue::_new(__wasm_sb_bindgen_shr(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl Shr, shr for SbValue);

impl Add for &SbValue {
    type Output = SbValue;

    /// Applies the binary `+` SB operator on two `SbValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Addition)
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        unsafe { SbValue::_new(__wasm_sb_bindgen_add(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl Add, add for SbValue);

impl Sub for &SbValue {
    type Output = SbValue;

    /// Applies the binary `-` SB operator on two `SbValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Subtraction)
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { SbValue::_new(__wasm_sb_bindgen_sub(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl Sub, sub for SbValue);

impl Div for &SbValue {
    type Output = SbValue;

    /// Applies the binary `/` SB operator on two `SbValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Division)
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        unsafe { SbValue::_new(__wasm_sb_bindgen_div(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl Div, div for SbValue);

impl Mul for &SbValue {
    type Output = SbValue;

    /// Applies the binary `*` SB operator on two `SbValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Multiplication)
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        unsafe { SbValue::_new(__wasm_sb_bindgen_mul(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl Mul, mul for SbValue);

impl Rem for &SbValue {
    type Output = SbValue;

    /// Applies the binary `%` SB operator on two `SbValue`s.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Remainder)
    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        unsafe { SbValue::_new(__wasm_sb_bindgen_rem(self.idx, rhs.idx)) }
    }
}

forward_deref_binop!(impl Rem, rem for SbValue);

impl<'a> From<&'a str> for SbValue {
    #[inline]
    fn from(s: &'a str) -> SbValue {
        SbValue::from_str(s)
    }
}

impl<T> From<*mut T> for SbValue {
    #[inline]
    fn from(s: *mut T) -> SbValue {
        SbValue::from(s as usize)
    }
}

impl<T> From<*const T> for SbValue {
    #[inline]
    fn from(s: *const T) -> SbValue {
        SbValue::from(s as usize)
    }
}

// if_std! {
impl<'a> From<&'a String> for SbValue {
    #[inline]
    fn from(s: &'a String) -> SbValue {
        SbValue::from_str(s)
    }
}

impl From<String> for SbValue {
    #[inline]
    fn from(s: String) -> SbValue {
        SbValue::from_str(&s)
    }
}

impl TryFrom<SbValue> for String {
    type Error = SbValue;

    fn try_from(value: SbValue) -> Result<Self, Self::Error> {
        match value.as_string() {
            Some(s) => Ok(s),
            None => Err(value),
        }
    }
}

impl TryFromSbValue for String {
    type Error = SbValue;

    fn try_from_sb_value(value: SbValue) -> Result<Self, Self::Error> {
        match value.as_string() {
            Some(s) => Ok(s),
            None => Err(value),
        }
    }
}
// }

impl From<bool> for SbValue {
    #[inline]
    fn from(s: bool) -> SbValue {
        SbValue::from_bool(s)
    }
}

impl<'a, T> From<&'a T> for SbValue
where
    T: SbCast,
{
    #[inline]
    fn from(s: &'a T) -> SbValue {
        s.as_ref().clone()
    }
}

impl<T> From<Option<T>> for SbValue
where
    SbValue: From<T>,
{
    #[inline]
    fn from(s: Option<T>) -> SbValue {
        match s {
            Some(s) => s.into(),
            None => SbValue::undefined(),
        }
    }
}

impl SbCast for SbValue {
    // everything is a `SbValue`!
    #[inline]
    fn instanceof(_val: &SbValue) -> bool {
        true
    }
    #[inline]
    fn unchecked_from_sb(val: SbValue) -> Self {
        val
    }
    #[inline]
    fn unchecked_from_sb_ref(val: &SbValue) -> &Self {
        val
    }
}

impl AsRef<SbValue> for SbValue {
    #[inline]
    fn as_ref(&self) -> &SbValue {
        self
    }
}

macro_rules! numbers {
    ($($n:ident)*) => ($(
        impl PartialEq<$n> for SbValue {
            #[inline]
            fn eq(&self, other: &$n) -> bool {
                self.as_f64() == Some(f64::from(*other))
            }
        }

        impl From<$n> for SbValue {
            #[inline]
            fn from(n: $n) -> SbValue {
                SbValue::from_f64(n.into())
            }
        }
    )*)
}

numbers! { i8 u8 i16 u16 i32 u32 f32 f64 }

macro_rules! big_numbers {
    (|$arg:ident|, $($n:ident = $handle:expr,)*) => ($(
        impl PartialEq<$n> for SbValue {
            #[inline]
            fn eq(&self, other: &$n) -> bool {
                self == &SbValue::from(*other)
            }
        }

        impl From<$n> for SbValue {
            #[inline]
            fn from($arg: $n) -> SbValue {
                unsafe { SbValue::_new($handle) }
            }
        }
    )*)
}

fn bigint_get_as_i64(v: &SbValue) -> Option<i64> {
    unsafe { __wasm_sb_bindgen_bigint_get_as_i64(v.idx).join() }
}

macro_rules! try_from_for_num64 {
    ($ty:ty) => {
        impl TryFrom<SbValue> for $ty {
            type Error = SbValue;

            #[inline]
            fn try_from(v: SbValue) -> Result<Self, SbValue> {
                bigint_get_as_i64(&v)
                    // Reinterpret bits; ABI-wise this is safe to do and allows us to avoid
                    // having separate intrinsics per signed/unsigned types.
                    .map(|as_i64| as_i64 as Self)
                    // Double-check that we didn't truncate the bigint to 64 bits.
                    .filter(|as_self| v == *as_self)
                    // Not a bigint or not in range.
                    .ok_or(v)
            }
        }
    };
}

try_from_for_num64!(i64);
try_from_for_num64!(u64);

macro_rules! try_from_for_num128 {
    ($ty:ty, $hi_ty:ty) => {
        impl TryFrom<SbValue> for $ty {
            type Error = SbValue;

            #[inline]
            fn try_from(v: SbValue) -> Result<Self, SbValue> {
                // Truncate the bigint to 64 bits, this will give us the lower part.
                let lo = match bigint_get_as_i64(&v) {
                    // The lower part must be interpreted as unsigned in both i128 and u128.
                    Some(lo) => lo as u64,
                    // Not a bigint.
                    None => return Err(v),
                };
                // Now we know it's a bigint, so we can safely use `>> 64n` without
                // worrying about a SB exception on type mismatch.
                let hi = v >> SbValue::from(64_u64);
                // The high part is the one we want checked against a 64-bit range.
                // If it fits, then our original number is in the 128-bit range.
                let hi = <$hi_ty>::try_from(hi)?;
                Ok(Self::from(hi) << 64 | Self::from(lo))
            }
        }
    };
}

try_from_for_num128!(i128, i64);
try_from_for_num128!(u128, u64);

big_numbers! {
    |n|,
    i64 = __wasm_sb_bindgen_bigint_from_i64(n),
    u64 = __wasm_sb_bindgen_bigint_from_u64(n),
    i128 = __wasm_sb_bindgen_bigint_from_i128((n >> 64) as i64, n as u64),
    u128 = __wasm_sb_bindgen_bigint_from_u128((n >> 64) as u64, n as u64),
}

// `usize` and `isize` have to be treated a bit specially, because we know that
// they're 32-bit but the compiler conservatively assumes they might be bigger.
// So, we have to manually forward to the `u32`/`i32` versions.
impl PartialEq<usize> for SbValue {
    #[inline]
    fn eq(&self, other: &usize) -> bool {
        *self == (*other as u32)
    }
}

impl From<usize> for SbValue {
    #[inline]
    fn from(n: usize) -> Self {
        Self::from(n as u32)
    }
}

impl PartialEq<isize> for SbValue {
    #[inline]
    fn eq(&self, other: &isize) -> bool {
        *self == (*other as i32)
    }
}

impl From<isize> for SbValue {
    #[inline]
    fn from(n: isize) -> Self {
        Self::from(n as i32)
    }
}

externs! {
    #[link(wasm_import_module = "__wasm_sb_bindgen_placeholder__")]
    extern "C" {
        fn __wasm_sb_bindgen_object_clone_ref(idx: u32) -> u32;
        fn __wasm_sb_bindgen_object_drop_ref(idx: u32) -> ();

        fn __wasm_sb_bindgen_string_new(ptr: *const u8, len: usize) -> u32;
        fn __wasm_sb_bindgen_number_new(f: f64) -> u32;
        fn __wasm_sb_bindgen_bigint_from_str(ptr: *const u8, len: usize) -> u32;
        fn __wasm_sb_bindgen_bigint_from_i64(n: i64) -> u32;
        fn __wasm_sb_bindgen_bigint_from_u64(n: u64) -> u32;
        fn __wasm_sb_bindgen_bigint_from_i128(hi: i64, lo: u64) -> u32;
        fn __wasm_sb_bindgen_bigint_from_u128(hi: u64, lo: u64) -> u32;
        fn __wasm_sb_bindgen_symbol_named_new(ptr: *const u8, len: usize) -> u32;
        fn __wasm_sb_bindgen_symbol_anonymous_new() -> u32;

        fn __wasm_sb_bindgen_externref_heap_live_count() -> u32;

        fn __wasm_sb_bindgen_is_null(idx: u32) -> u32;
        fn __wasm_sb_bindgen_is_undefined(idx: u32) -> u32;
        fn __wasm_sb_bindgen_is_symbol(idx: u32) -> u32;
        fn __wasm_sb_bindgen_is_object(idx: u32) -> u32;
        fn __wasm_sb_bindgen_is_array(idx: u32) -> u32;
        fn __wasm_sb_bindgen_is_function(idx: u32) -> u32;
        fn __wasm_sb_bindgen_is_string(idx: u32) -> u32;
        fn __wasm_sb_bindgen_is_bigint(idx: u32) -> u32;
        fn __wasm_sb_bindgen_typeof(idx: u32) -> u32;

        fn __wasm_sb_bindgen_in(prop: u32, obj: u32) -> u32;

        fn __wasm_sb_bindgen_is_falsy(idx: u32) -> u32;
        fn __wasm_sb_bindgen_as_number(idx: u32) -> f64;
        fn __wasm_sb_bindgen_try_into_number(idx: u32) -> u32;
        fn __wasm_sb_bindgen_neg(idx: u32) -> u32;
        fn __wasm_sb_bindgen_bit_and(a: u32, b: u32) -> u32;
        fn __wasm_sb_bindgen_bit_or(a: u32, b: u32) -> u32;
        fn __wasm_sb_bindgen_bit_xor(a: u32, b: u32) -> u32;
        fn __wasm_sb_bindgen_bit_not(idx: u32) -> u32;
        fn __wasm_sb_bindgen_shl(a: u32, b: u32) -> u32;
        fn __wasm_sb_bindgen_shr(a: u32, b: u32) -> u32;
        fn __wasm_sb_bindgen_unsigned_shr(a: u32, b: u32) -> u32;
        fn __wasm_sb_bindgen_add(a: u32, b: u32) -> u32;
        fn __wasm_sb_bindgen_sub(a: u32, b: u32) -> u32;
        fn __wasm_sb_bindgen_div(a: u32, b: u32) -> u32;
        fn __wasm_sb_bindgen_checked_div(a: u32, b: u32) -> u32;
        fn __wasm_sb_bindgen_mul(a: u32, b: u32) -> u32;
        fn __wasm_sb_bindgen_rem(a: u32, b: u32) -> u32;
        fn __wasm_sb_bindgen_pow(a: u32, b: u32) -> u32;
        fn __wasm_sb_bindgen_lt(a: u32, b: u32) -> u32;
        fn __wasm_sb_bindgen_le(a: u32, b: u32) -> u32;
        fn __wasm_sb_bindgen_ge(a: u32, b: u32) -> u32;
        fn __wasm_sb_bindgen_gt(a: u32, b: u32) -> u32;

        fn __wasm_sb_bindgen_number_get(idx: u32) -> WasmRet<Option<f64>>;
        fn __wasm_sb_bindgen_boolean_get(idx: u32) -> u32;
        fn __wasm_sb_bindgen_string_get(idx: u32) -> WasmSlice;
        fn __wasm_sb_bindgen_bigint_get_as_i64(idx: u32) -> WasmRet<Option<i64>>;

        fn __wasm_sb_bindgen_debug_string(ret: *mut [usize; 2], idx: u32) -> ();

        fn __wasm_sb_bindgen_throw(a: *const u8, b: usize) -> !;
        fn __wasm_sb_bindgen_rethrow(a: u32) -> !;
        fn __wasm_sb_bindgen_error_new(a: *const u8, b: usize) -> u32;

        fn __wasm_sb_bindgen_cb_drop(idx: u32) -> u32;

        fn __wasm_sb_bindgen_describe(v: u32) -> ();
        fn __wasm_sb_bindgen_describe_closure(a: u32, b: u32, c: u32) -> u32;

        fn __wasm_sb_bindgen_json_parse(ptr: *const u8, len: usize) -> u32;
        fn __wasm_sb_bindgen_json_serialize(idx: u32) -> WasmSlice;
        fn __wasm_sb_bindgen_sbval_eq(a: u32, b: u32) -> u32;
        fn __wasm_sb_bindgen_sbval_loose_eq(a: u32, b: u32) -> u32;

        fn __wasm_sb_bindgen_copy_to_typed_array(ptr: *const u8, len: usize, idx: u32) -> ();

        fn __wasm_sb_bindgen_not(idx: u32) -> u32;

        fn __wasm_sb_bindgen_exports() -> u32;
        fn __wasm_sb_bindgen_memory() -> u32;
        fn __wasm_sb_bindgen_module() -> u32;
        fn __wasm_sb_bindgen_function_table() -> u32;
    }
}

impl Clone for SbValue {
    #[inline]
    fn clone(&self) -> SbValue {
        unsafe {
            let idx = __wasm_sb_bindgen_object_clone_ref(self.idx);
            SbValue::_new(idx)
        }
    }
}

#[cfg(feature = "std")]
impl fmt::Debug for SbValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SbValue({})", self.as_debug_string())
    }
}

#[cfg(not(feature = "std"))]
impl fmt::Debug for SbValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SbValue")
    }
}

impl Drop for SbValue {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            // We definitely should never drop anything in the stack area
            debug_assert!(self.idx >= SBIDX_OFFSET, "free of stack slot {}", self.idx);

            // Otherwise if we're not dropping one of our reserved values,
            // actually call the intrinsic. See #1054 for eventually removing
            // this branch.
            if self.idx >= SBIDX_RESERVED {
                __wasm_sb_bindgen_object_drop_ref(self.idx);
            }
        }
    }
}

impl Default for SbValue {
    fn default() -> Self {
        Self::UNDEFINED
    }
}

#[cfg(feature = "std")]
pub struct SbStatic<T: 'static> {
    #[doc(hidden)]
    pub __inner: &'static std::thread::LocalKey<T>,
}

#[cfg(feature = "std")]
impl<T: FromWasmAbi + 'static> Deref for SbStatic<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // We know that our tls key is never overwritten after initialization,
        // so it should be safe (on that axis at least) to hand out a reference
        // that lives longer than the closure below.
        //
        // FIXME: this is not sound if we ever implement thread exit hooks on
        // wasm, as the pointer will eventually be invalidated but you can get
        // `&'static T` from this interface. We... probably need to deprecate
        // and/or remove this interface nowadays.
        unsafe { self.__inner.with(|ptr| &*(ptr as *const T)) }
    }
}

#[cold]
#[inline(never)]
#[deprecated(note = "renamed to `throw_str`")]
#[doc(hidden)]
pub fn throw(s: &str) -> ! {
    throw_str(s)
}

#[cold]
#[inline(never)]
pub fn throw_str(s: &str) -> ! {
    unsafe {
        __wasm_sb_bindgen_throw(s.as_ptr(), s.len());
    }
}

#[cold]
#[inline(never)]
pub fn throw_val(s: SbValue) -> ! {
    unsafe {
        let idx = s.idx;
        mem::forget(s);
        __wasm_sb_bindgen_rethrow(idx);
    }
}

pub fn externref_heap_live_count() -> u32 {
    unsafe { __wasm_sb_bindgen_externref_heap_live_count() }
}

#[doc(hidden)]
pub fn anyref_heap_live_count() -> u32 {
    externref_heap_live_count()
}

pub trait UnwrapThrowExt<T>: Sized {
    /// Unwrap this `Option` or `Result`, but instead of panicking on failure,
    /// throw an exception to JavaScript.
    #[cfg_attr(debug_assertions, track_caller)]
    fn unwrap_throw(self) -> T {
        if cfg!(all(debug_assertions, feature = "std")) {
            let loc = core::panic::Location::caller();
            let msg = std::format!(
                "`unwrap_throw` failed ({}:{}:{})",
                loc.file(),
                loc.line(),
                loc.column()
            );
            self.expect_throw(&msg)
        } else {
            self.expect_throw("`unwrap_throw` failed")
        }
    }

    /// Unwrap this container's `T` value, or throw an error to SB with the
    /// given message if the `T` value is unavailable (e.g. an `Option<T>` is
    /// `None`).
    #[cfg_attr(debug_assertions, track_caller)]
    fn expect_throw(self, message: &str) -> T;
}

impl<T> UnwrapThrowExt<T> for Option<T> {
    #[cfg_attr(debug_assertions, track_caller)]
    fn expect_throw(self, message: &str) -> T {
        if cfg!(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        )) {
            match self {
                Some(val) => val,
                None => throw_str(message),
            }
        } else {
            self.expect(message)
        }
    }
}

impl<T, E> UnwrapThrowExt<T> for Result<T, E>
where
    E: core::fmt::Debug,
{
    #[cfg_attr(debug_assertions, track_caller)]
    fn expect_throw(self, message: &str) -> T {
        if cfg!(all(
            target_arch = "wasm32",
            not(any(target_os = "emscripten", target_os = "wasi"))
        )) {
            match self {
                Ok(val) => val,
                Err(_) => throw_str(message),
            }
        } else {
            self.expect(message)
        }
    }
}

pub fn module() -> SbValue {
    unsafe { SbValue::_new(__wasm_sb_bindgen_module()) }
}

pub fn exports() -> SbValue {
    unsafe { SbValue::_new(__wasm_sb_bindgen_exports()) }
}

pub fn memory() -> SbValue {
    unsafe { SbValue::_new(__wasm_sb_bindgen_memory()) }
}

pub fn function_table() -> SbValue {
    unsafe { SbValue::_new(__wasm_sb_bindgen_function_table()) }
}

#[doc(hidden)]
pub mod __rt {
    use crate::SbValue;
    use core::borrow::{Borrow, BorrowMut};
    use core::cell::{Cell, UnsafeCell};
    use core::convert::Infallible;
    use core::ops::{Deref, DerefMut};

    pub extern crate core;
    #[cfg(feature = "std")]
    pub extern crate std;

    #[macro_export]
    #[doc(hidden)]
    #[cfg(feature = "std")]
    macro_rules! __wasm_sb_bindgen_if_not_std {
        ($($i:item)*) => {};
    }

    #[macro_export]
    #[doc(hidden)]
    #[cfg(not(feature = "std"))]
    macro_rules! __wasm_sb_bindgen_if_not_std {
        ($($i:item)*) => ($($i)*)
    }

    #[inline]
    pub fn assert_not_null<T>(s: *mut T) {
        if s.is_null() {
            throw_null();
        }
    }

    #[cold]
    #[inline(never)]
    fn throw_null() -> ! {
        super::throw_str("null pointer passed to rust");
    }

    /// A vendored version of `RefCell` from the standard library.
    ///
    /// Now why, you may ask, would we do that? Surely `RefCell` in libstd is
    /// quite good. And you're right, it is indeed quite good! Functionally
    /// nothing more is needed from `RefCell` in the standard library but for
    /// now this crate is also sort of optimizing for compiled code size.
    ///
    /// One major factor to larger binaries in Rust is when a panic happens.
    /// Panicking in the standard library involves a fair bit of machinery
    /// (formatting, panic hooks, synchronization, etc). It's all worthwhile if
    /// you need it but for something like `WasmRefCell` here we don't actually
    /// need all that!
    ///
    /// This is just a wrapper around all Rust objects passed to SB intended to
    /// guard accidental reentrancy, so this vendored version is intended solely
    /// to not panic in libstd. Instead when it "panics" it calls our `throw`
    /// function in this crate which raises an error in SB.
    pub struct WasmRefCell<T: ?Sized> {
        borrow: Cell<usize>,
        value: UnsafeCell<T>,
    }

    impl<T: ?Sized> WasmRefCell<T> {
        pub fn new(value: T) -> WasmRefCell<T>
        where
            T: Sized,
        {
            WasmRefCell {
                value: UnsafeCell::new(value),
                borrow: Cell::new(0),
            }
        }

        pub fn get_mut(&mut self) -> &mut T {
            unsafe { &mut *self.value.get() }
        }

        pub fn borrow(&self) -> Ref<T> {
            unsafe {
                if self.borrow.get() == usize::max_value() {
                    borrow_fail();
                }
                self.borrow.set(self.borrow.get() + 1);
                Ref {
                    value: &*self.value.get(),
                    borrow: &self.borrow,
                }
            }
        }

        pub fn borrow_mut(&self) -> RefMut<T> {
            unsafe {
                if self.borrow.get() != 0 {
                    borrow_fail();
                }
                self.borrow.set(usize::max_value());
                RefMut {
                    value: &mut *self.value.get(),
                    borrow: &self.borrow,
                }
            }
        }

        pub fn into_inner(self) -> T
        where
            T: Sized,
        {
            self.value.into_inner()
        }
    }

    pub struct Ref<'b, T: ?Sized + 'b> {
        value: &'b T,
        borrow: &'b Cell<usize>,
    }

    impl<'b, T: ?Sized> Deref for Ref<'b, T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &T {
            self.value
        }
    }

    impl<'b, T: ?Sized> Borrow<T> for Ref<'b, T> {
        #[inline]
        fn borrow(&self) -> &T {
            self.value
        }
    }

    impl<'b, T: ?Sized> Drop for Ref<'b, T> {
        fn drop(&mut self) {
            self.borrow.set(self.borrow.get() - 1);
        }
    }

    pub struct RefMut<'b, T: ?Sized + 'b> {
        value: &'b mut T,
        borrow: &'b Cell<usize>,
    }

    impl<'b, T: ?Sized> Deref for RefMut<'b, T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &T {
            self.value
        }
    }

    impl<'b, T: ?Sized> DerefMut for RefMut<'b, T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut T {
            self.value
        }
    }

    impl<'b, T: ?Sized> Borrow<T> for RefMut<'b, T> {
        #[inline]
        fn borrow(&self) -> &T {
            self.value
        }
    }

    impl<'b, T: ?Sized> BorrowMut<T> for RefMut<'b, T> {
        #[inline]
        fn borrow_mut(&mut self) -> &mut T {
            self.value
        }
    }

    impl<'b, T: ?Sized> Drop for RefMut<'b, T> {
        fn drop(&mut self) {
            self.borrow.set(0);
        }
    }

    fn borrow_fail() -> ! {
        super::throw_str(
            "recursive use of an object detected which would lead to \
             unsafe aliasing in rust",
        );
    }

    if_std! {
        use std::alloc::{alloc, dealloc, realloc, Layout};

        #[no_mangle]
        pub extern "C" fn __wasm_sb_bindgen_malloc(size: usize, align: usize) -> *mut u8 {
            if let Ok(layout) = Layout::from_size_align(size, align) {
                unsafe {
                    if layout.size() > 0 {
                        let ptr = alloc(layout);
                        if !ptr.is_null() {
                            return ptr
                        }
                    } else {
                        return align as *mut u8
                    }
                }
            }

            malloc_failure();
        }

        #[no_mangle]
        pub unsafe extern "C" fn __wasm_sb_bindgen_realloc(ptr: *mut u8, old_size: usize, new_size: usize, align: usize) -> *mut u8 {
            debug_assert!(old_size > 0);
            debug_assert!(new_size > 0);
            if let Ok(layout) = Layout::from_size_align(old_size, align) {
                let ptr = realloc(ptr, layout, new_size);
                if !ptr.is_null() {
                    return ptr
                }
            }
            malloc_failure();
        }

        #[cold]
        fn malloc_failure() -> ! {
            if cfg!(debug_assertions) {
                super::throw_str("invalid malloc request")
            } else {
                std::process::abort();
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn __wasm_sb_bindgen_free(ptr: *mut u8, size: usize, align: usize) {
            // This happens for zero-length slices, and in that case `ptr` is
            // likely bogus so don't actually send this to the system allocator
            if size == 0 {
                return
            }
            let layout = Layout::from_size_align_unchecked(size, align);
            dealloc(ptr, layout);
        }
    }

    /// This is a curious function necessary to get wasm-bindgen working today,
    /// and it's a bit of an unfortunate hack.
    ///
    /// The general problem is that somehow we need the above two symbols to
    /// exist in the final output binary (__wasm_sb_bindgen_malloc and
    /// __wasm_sb_bindgen_free). These symbols may be called by SB for various
    /// bindings, so we for sure need to make sure they're exported.
    ///
    /// The problem arises, though, when what if no Rust code uses the symbols?
    /// For all intents and purposes it looks to LLVM and the linker like the
    /// above two symbols are dead code, so they're completely discarded!
    ///
    /// Specifically what happens is this:
    ///
    /// * The above two symbols are generated into some object file inside of
    ///   libwasm_bindgen.rlib
    /// * The linker, LLD, will not load this object file unless *some* symbol
    ///   is loaded from the object. In this case, if the Rust code never calls
    ///   __wasm_sb_bindgen_malloc or __wasm_sb_bindgen_free then the symbols never get linked
    ///   in.
    /// * Later when `wasm-bindgen` attempts to use the symbols they don't
    ///   exist, causing an error.
    ///
    /// This function is a weird hack for this problem. We inject a call to this
    /// function in all generated code. Usage of this function should then
    /// ensure that the above two intrinsics are translated.
    ///
    /// Due to how rustc creates object files this function (and anything inside
    /// it) will be placed into the same object file as the two intrinsics
    /// above. That means if this function is called and referenced we'll pull
    /// in the object file and link the intrinsics.
    ///
    /// Ideas for how to improve this are most welcome!
    pub fn link_mem_intrinsics() {
        crate::externref::link_intrinsics();
    }

    static mut GLOBAL_EXNDATA: [u32; 2] = [0; 2];

    #[no_mangle]
    pub unsafe extern "C" fn __wasm_sb_bindgen_exn_store(idx: u32) {
        debug_assert_eq!(GLOBAL_EXNDATA[0], 0);
        GLOBAL_EXNDATA[0] = 1;
        GLOBAL_EXNDATA[1] = idx;
    }

    pub fn take_last_exception() -> Result<(), super::SbValue> {
        unsafe {
            let ret = if GLOBAL_EXNDATA[0] == 1 {
                Err(super::SbValue::_new(GLOBAL_EXNDATA[1]))
            } else {
                Ok(())
            };
            GLOBAL_EXNDATA[0] = 0;
            GLOBAL_EXNDATA[1] = 0;
            ret
        }
    }

    /// An internal helper trait for usage in `#[wasm_bindgen]` on `async`
    /// functions to convert the return value of the function to
    /// `Result<SbValue, SbValue>` which is what we'll return to SB (where an
    /// error is a failed future).
    pub trait IntoSbResult {
        fn into_js_result(self) -> Result<SbValue, SbValue>;
    }

    impl IntoSbResult for () {
        fn into_js_result(self) -> Result<SbValue, SbValue> {
            Ok(SbValue::undefined())
        }
    }

    impl<T: Into<SbValue>> IntoSbResult for T {
        fn into_js_result(self) -> Result<SbValue, SbValue> {
            Ok(self.into())
        }
    }

    impl<T: Into<SbValue>, E: Into<SbValue>> IntoSbResult for Result<T, E> {
        fn into_js_result(self) -> Result<SbValue, SbValue> {
            match self {
                Ok(e) => Ok(e.into()),
                Err(e) => Err(e.into()),
            }
        }
    }

    impl<E: Into<SbValue>> IntoSbResult for Result<(), E> {
        fn into_js_result(self) -> Result<SbValue, SbValue> {
            match self {
                Ok(()) => Ok(SbValue::undefined()),
                Err(e) => Err(e.into()),
            }
        }
    }

    /// An internal helper trait for usage in `#[wasm_bindgen(start)]`
    /// functions to throw the error (if it is `Err`).
    pub trait Start {
        fn start(self);
    }

    impl Start for () {
        #[inline]
        fn start(self) {}
    }

    impl<E: Into<SbValue>> Start for Result<(), E> {
        #[inline]
        fn start(self) {
            if let Err(e) = self {
                crate::throw_val(e.into());
            }
        }
    }

    /// An internal helper struct for usage in `#[wasm_bindgen(main)]`
    /// functions to throw the error (if it is `Err`).
    pub struct MainWrapper<T>(pub Option<T>);

    pub trait Main {
        fn __wasm_bindgen_main(&mut self);
    }

    impl Main for &mut &mut MainWrapper<()> {
        #[inline]
        fn __wasm_bindgen_main(&mut self) {}
    }

    impl Main for &mut &mut MainWrapper<Infallible> {
        #[inline]
        fn __wasm_bindgen_main(&mut self) {}
    }

    impl<E: Into<SbValue>> Main for &mut &mut MainWrapper<Result<(), E>> {
        #[inline]
        fn __wasm_bindgen_main(&mut self) {
            if let Err(e) = self.0.take().unwrap() {
                crate::throw_val(e.into());
            }
        }
    }

    impl<E: std::fmt::Debug> Main for &mut MainWrapper<Result<(), E>> {
        #[inline]
        fn __wasm_bindgen_main(&mut self) {
            if let Err(e) = self.0.take().unwrap() {
                crate::throw_str(&std::format!("{:?}", e));
            }
        }
    }
}

#[derive(Clone)]
pub struct SbError {
    value: SbValue,
}

impl SbError {
    /// Construct a JavaScript `Error` object with a string message
    #[inline]
    pub fn new(s: &str) -> SbError {
        Self {
            value: unsafe {
                SbValue::_new(crate::__wasm_sb_bindgen_error_new(s.as_ptr(), s.len()))
            },
        }
    }
}

if_std! {
    impl<E> From<E> for SbError
    where
        E: std::error::Error,
    {
        fn from(error: E) -> Self {
            SbError::new(&error.to_string())
        }
    }
}

impl From<SbError> for SbValue {
    fn from(error: SbError) -> Self {
        error.value
    }
}
