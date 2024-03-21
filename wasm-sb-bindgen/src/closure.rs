use std::{
    fmt,
    mem::{self, ManuallyDrop},
};

use crate::{
    convert::{
        FromWasmAbi, IntoWasmAbi, OptionIntoWasmAbi, RefFromWasmAbi, ReturnWasmAbi, WasmAbi,
        WasmRet,
    },
    describe::{inform, CLOSURE, EXTERNREF},
    throw_str, SbValue, UnwrapThrowExt as _, WasmDescribe,
};

pub struct Closure<T: ?Sized> {
    sb: ManuallyDrop<SbValue>,
    data: ManuallyDrop<Box<T>>,
}

union FatPtr<T: ?Sized> {
    ptr: *mut T,
    fields: (f64, f64),
}

impl<T> Closure<T>
where
    T: ?Sized + WasmClosure,
{
    /// Creates a new instance of `Closure` from the provided Rust function.
    ///
    /// Note that the closure provided here, `F`, has a few requirements
    /// associated with it:
    ///
    /// * It must implement `Fn` or `FnMut` (for `FnOnce` functions see
    ///   `Closure::once` and `Closure::once_into_sb`).
    ///
    /// * It must be `'static`, aka no stack references (use the `move`
    ///   keyword).
    ///
    /// * It can have at most 7 arguments.
    ///
    /// * Its arguments and return values are all types that can be shared with
    ///   JS (i.e. have `#[wasm_bindgen]` annotations or are simple numbers,
    ///   etc.)
    pub fn new<F>(t: F) -> Closure<T>
    where
        F: IntoWasmClosure<T> + 'static,
    {
        Closure::wrap(Box::new(t).unsize())
    }

    /// A more direct version of `Closure::new` which creates a `Closure` from
    /// a `Box<dyn Fn>`/`Box<dyn FnMut>`, which is how it's kept internally.
    pub fn wrap(mut data: Box<T>) -> Closure<T> {
        assert_eq!(mem::size_of::<*const T>(), mem::size_of::<FatPtr<T>>());
        let (a, b) = unsafe {
            FatPtr {
                ptr: &mut *data as *mut T,
            }
            .fields
        };

        // Here we need to create a `SbValue` with the data and `T::invoke()`
        // function pointer. To do that we... take a few unconventional turns.
        // In essence what happens here is this:
        //
        // 1. First up, below we call a function, `breaks_if_inlined`. This
        //    function, as the name implies, does not work if it's inlined.
        //    More on that in a moment.
        // 2. This function internally calls a special import recognized by the
        //    `wasm-bindgen` CLI tool, `__wasm_sb_bindgen_describe_closure`. This
        //    imported symbol is similar to `__wasm_sb_bindgen_describe` in that it's
        //    not intended to show up in the final binary but it's an
        //    intermediate state for a `wasm-bindgen` binary.
        // 3. The `__wasm_sb_bindgen_describe_closure` import is namely passed a
        //    descriptor function, monomorphized for each invocation.
        //
        // Most of this doesn't actually make sense to happen at runtime! The
        // real magic happens when `wasm-bindgen` comes along and updates our
        // generated code. When `wasm-bindgen` runs it performs a few tasks:
        //
        // * First, it finds all functions that call
        //   `__wasm_sb_bindgen_describe_closure`. These are all `breaks_if_inlined`
        //   defined below as the symbol isn't called anywhere else.
        // * Next, `wasm-bindgen` executes the `breaks_if_inlined`
        //   monomorphized functions, passing it dummy arguments. This will
        //   execute the function just enough to invoke the special import,
        //   namely telling us about the function pointer that is the describe
        //   shim.
        // * This knowledge is then used to actually find the descriptor in the
        //   function table which is then executed to figure out the signature
        //   of the closure.
        // * Finally, and probably most heinously, the call to
        //   `breaks_if_inlined` is rewritten to call an otherwise globally
        //   imported function. This globally imported function will generate
        //   the `SbValue` for this closure specialized for the signature in
        //   question.
        //
        // Later on `wasm-gc` will clean up all the dead code and ensure that
        // we don't actually call `__wasm_sb_bindgen_describe_closure` at runtime. This
        // means we will end up not actually calling `breaks_if_inlined` in the
        // final binary, all calls to that function should be pruned.
        //
        // See crates/cli-support/src/sb/closures.rs for a more information
        // about what's going on here.

        extern "C" fn describe<T: WasmClosure + ?Sized>() {
            inform(CLOSURE);
            T::describe()
        }

        #[inline(never)]
        unsafe fn breaks_if_inlined<T: WasmClosure + ?Sized>(a: f64, b: f64) -> f64 {
            super::__wasm_sb_bindgen_describe_closure(
                a as f64,
                b as f64,
                describe::<T> as u32 as f64,
            )
        }

        let idx = unsafe { breaks_if_inlined::<T>(a, b) };

        Closure {
            sb: ManuallyDrop::new(SbValue::_new(idx)),
            data: ManuallyDrop::new(data),
        }
    }

    /// Release memory management of this closure from Rust to the JS GC.
    ///
    /// When a `Closure` is dropped it will release the Rust memory and
    /// invalidate the associated JS closure, but this isn't always desired.
    /// Some callbacks are alive for the entire duration of the program or for a
    /// lifetime dynamically managed by the JS GC. This function can be used
    /// to drop this `Closure` while keeping the associated JS function still
    /// valid.
    ///
    /// If the platform supports weak references, the Rust memory will be
    /// reclaimed when the JS closure is GC'd. If weak references is not
    /// supported, this can be dangerous if this function is called many times
    /// in an application because the memory leak will overwhelm the page
    /// quickly and crash the wasm.
    pub fn into_sb_value(self) -> SbValue {
        let idx = self.sb.idx;
        mem::forget(self);
        SbValue::_new(idx)
    }

    /// Same as `into_sb_value`, but doesn't return a value.
    pub fn forget(self) {
        drop(self.into_sb_value());
    }
}
impl Closure<dyn FnOnce()> {
    /// Create a `Closure` from a function that can only be called once.
    ///
    /// Since we have no way of enforcing that JS cannot attempt to call this
    /// `FnOne(A...) -> R` more than once, this produces a `Closure<dyn FnMut(A...)
    /// -> R>` that will dynamically throw a JavaScript error if called more
    /// than once.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use wasm_bindgen::prelude::*;
    ///
    /// // Create an non-`Copy`, owned `String`.
    /// let mut s = String::from("Hello");
    ///
    /// // Close over `s`. Since `f` returns `s`, it is `FnOnce` and can only be
    /// // called once. If it was called a second time, it wouldn't have any `s`
    /// // to work with anymore!
    /// let f = move || {
    ///     s += ", World!";
    ///     s
    /// };
    ///
    /// // Create a `Closure` from `f`. Note that the `Closure`'s type parameter
    /// // is `FnMut`, even though `f` is `FnOnce`.
    /// let closure: Closure<dyn FnMut() -> String> = Closure::once(f);
    /// ```
    pub fn once<F, A, R>(fn_once: F) -> Closure<F::FnMut>
    where
        F: 'static + WasmClosureFnOnce<A, R>,
    {
        Closure::wrap(fn_once.into_fn_mut())
    }

    /// Convert a `FnOnce(A...) -> R` into a JavaScript `Function` object.
    ///
    /// If the JavaScript function is invoked more than once, it will throw an
    /// exception.
    ///
    /// Unlike `Closure::once`, this does *not* return a `Closure` that can be
    /// dropped before the function is invoked to deallocate the closure. The
    /// only way the `FnOnce` is deallocated is by calling the JavaScript
    /// function. If the JavaScript function is never called then the `FnOnce`
    /// and everything it closes over will leak.
    ///
    /// ```rust,ignore
    /// use wasm_bindgen::{prelude::*, SbCast};
    ///
    /// let f = Closure::once_into_sb(move || {
    ///     // ...
    /// });
    ///
    /// assert!(f.is_instance_of::<sb_sys::Function>());
    /// ```
    pub fn once_into_sb<F, A, R>(fn_once: F) -> SbValue
    where
        F: 'static + WasmClosureFnOnce<A, R>,
    {
        fn_once.into_sb_function()
    }
}

#[doc(hidden)]
pub trait WasmClosureFnOnce<A, R>: 'static {
    type FnMut: ?Sized + 'static + WasmClosure;

    fn into_fn_mut(self) -> Box<Self::FnMut>;

    fn into_sb_function(self) -> SbValue;
}

impl<T: ?Sized> AsRef<SbValue> for Closure<T> {
    fn as_ref(&self) -> &SbValue {
        &self.sb
    }
}

impl<T> WasmDescribe for Closure<T>
where
    T: WasmClosure + ?Sized,
{
    fn describe() {
        inform(EXTERNREF);
    }
}

// `Closure` can only be passed by reference to imports.
impl<'a, T> IntoWasmAbi for &'a Closure<T>
where
    T: WasmClosure + ?Sized,
{
    type Abi = f64;

    fn into_abi(self) -> f64 {
        (&*self.sb).into_abi()
    }
}

impl<'a, T> OptionIntoWasmAbi for &'a Closure<T>
where
    T: WasmClosure + ?Sized,
{
    fn none() -> Self::Abi {
        0f64
    }
}

// fn _check() {
//     fn _assert<T: IntoWasmAbi>() {}
//     _assert::<&Closure<dyn Fn()>>();
//     _assert::<&Closure<dyn Fn(String)>>();
//     _assert::<&Closure<dyn Fn() -> String>>();
//     _assert::<&Closure<dyn FnMut()>>();
//     _assert::<&Closure<dyn FnMut(String)>>();
//     _assert::<&Closure<dyn FnMut() -> String>>();
// }

impl<T> fmt::Debug for Closure<T>
where
    T: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Closure {{ ... }}")
    }
}

impl<T> Drop for Closure<T>
where
    T: ?Sized,
{
    fn drop(&mut self) {
        unsafe {
            // this will implicitly drop our strong reference in addition to
            // invalidating all future invocations of the closure
            if super::__wasm_sb_bindgen_cb_drop(self.sb.idx) != 0f64 {
                ManuallyDrop::drop(&mut self.data);
            }
        }
    }
}

/// An internal trait for the `Closure` type.
///
/// This trait is not stable and it's not recommended to use this in bounds or
/// implement yourself.
#[doc(hidden)]
pub unsafe trait WasmClosure {
    fn describe();
}

#[doc(hidden)]
pub trait IntoWasmClosure<T: ?Sized> {
    fn unsize(self: Box<Self>) -> Box<T>;
}

macro_rules! doit {
    ($(
        ($($var:ident $arg1:ident $arg2:ident $arg3:ident $arg4:ident)*)
    )*) => ($(
        unsafe impl<$($var,)* R> WasmClosure for dyn Fn($($var),*) -> R + 'static
            where $($var: FromWasmAbi + 'static,)*
                  R: ReturnWasmAbi + 'static,
        {
            fn describe() {
                #[allow(non_snake_case)]
                unsafe extern "C" fn invoke<$($var: FromWasmAbi,)* R: ReturnWasmAbi>(
                    a: f64,
                    b: f64,
                    $(
                    $arg1: <$var::Abi as WasmAbi>::Prim1,
                    $arg2: <$var::Abi as WasmAbi>::Prim2,
                    $arg3: <$var::Abi as WasmAbi>::Prim3,
                    $arg4: <$var::Abi as WasmAbi>::Prim4,
                    )*
                ) -> WasmRet<R::Abi> {
                    if a == 0f64 {
                        throw_str("closure invoked after being dropped");
                    }
                    // Make sure all stack variables are converted before we
                    // convert `ret` as it may throw (for `Result`, for
                    // example)
                    let ret = {
                        let f: *const dyn Fn($($var),*) -> R =
                            FatPtr { fields: (a, b) }.ptr;
                        $(
                            let $var = <$var as FromWasmAbi>::from_abi($var::Abi::join($arg1, $arg2, $arg3, $arg4));
                        )*
                        (*f)($($var),*)
                    };
                    ret.return_abi().into()
                }

                inform(invoke::<$($var,)* R> as u32 as f64);

                unsafe extern fn destroy<$($var: FromWasmAbi,)* R: ReturnWasmAbi>(
                    a: f64,
                    b: f64,
                ) {
                    // This can be called by the JS glue in erroneous situations
                    // such as when the closure has already been destroyed. If
                    // that's the case let's not make things worse by
                    // segfaulting and/or asserting, so just ignore null
                    // pointers.
                    if a == 0f64 {
                        return;
                    }
                    drop(Box::from_raw(FatPtr::<dyn Fn($($var,)*) -> R> {
                        fields: (a, b)
                    }.ptr));
                }
                inform(destroy::<$($var,)* R> as u32 as f64);

                <&Self>::describe();
            }
        }

        unsafe impl<$($var,)* R> WasmClosure for dyn FnMut($($var),*) -> R + 'static
            where $($var: FromWasmAbi + 'static,)*
                  R: ReturnWasmAbi + 'static,
        {
            fn describe() {
                #[allow(non_snake_case)]
                unsafe extern "C" fn invoke<$($var: FromWasmAbi,)* R: ReturnWasmAbi>(
                    a: f64,
                    b: f64,
                    $(
                    $arg1: <$var::Abi as WasmAbi>::Prim1,
                    $arg2: <$var::Abi as WasmAbi>::Prim2,
                    $arg3: <$var::Abi as WasmAbi>::Prim3,
                    $arg4: <$var::Abi as WasmAbi>::Prim4,
                    )*
                ) -> WasmRet<R::Abi> {
                    if a == 0f64 {
                        throw_str("closure invoked recursively or after being dropped");
                    }
                    // Make sure all stack variables are converted before we
                    // convert `ret` as it may throw (for `Result`, for
                    // example)
                    let ret = {
                        let f: *const dyn FnMut($($var),*) -> R =
                            FatPtr { fields: (a, b) }.ptr;
                        let f = f as *mut dyn FnMut($($var),*) -> R;
                        $(
                            let $var = <$var as FromWasmAbi>::from_abi($var::Abi::join($arg1, $arg2, $arg3, $arg4));
                        )*
                        (*f)($($var),*)
                    };
                    ret.return_abi().into()
                }

                inform(invoke::<$($var,)* R> as u32 as f64);

                unsafe extern fn destroy<$($var: FromWasmAbi,)* R: ReturnWasmAbi>(
                    a: f64,
                    b: f64,
                ) {
                    // See `Fn()` above for why we simply return
                    if a == 0f64 {
                        return;
                    }
                    drop(Box::from_raw(FatPtr::<dyn FnMut($($var,)*) -> R> {
                        fields: (a, b)
                    }.ptr));
                }
                inform(destroy::<$($var,)* R> as u32 as f64);

                <&mut Self>::describe();
            }
        }

        #[allow(non_snake_case, unused_parens)]
        impl<T, $($var,)* R> WasmClosureFnOnce<($($var),*), R> for T
            where T: 'static + FnOnce($($var),*) -> R,
                  $($var: FromWasmAbi + 'static,)*
                  R: ReturnWasmAbi + 'static
        {
            type FnMut = dyn FnMut($($var),*) -> R;

            fn into_fn_mut(self) -> Box<Self::FnMut> {
                let mut me = Some(self);
                Box::new(move |$($var),*| {
                    let me = me.take().expect_throw("FnOnce called more than once");
                    me($($var),*)
                })
            }

            fn into_sb_function(self) -> SbValue {
                use std::rc::Rc;
                use crate::__rt::WasmRefCell;

                let mut me = Some(self);

                let rc1 = Rc::new(WasmRefCell::new(None));
                let rc2 = rc1.clone();

                let closure = Closure::wrap(Box::new(move |$($var),*| {
                    // Invoke ourself and get the result.
                    let me = me.take().expect_throw("FnOnce called more than once");
                    let result = me($($var),*);

                    // And then drop the `Rc` holding this function's `Closure`
                    // alive.
                    debug_assert_eq!(Rc::strong_count(&rc2), 1);
                    let option_closure = rc2.borrow_mut().take();
                    debug_assert!(option_closure.is_some());
                    drop(option_closure);

                    result
                }) as Box<dyn FnMut($($var),*) -> R>);

                let sb_val = closure.as_ref().clone();

                *rc1.borrow_mut() = Some(closure);
                debug_assert_eq!(Rc::strong_count(&rc1), 2);
                drop(rc1);

                sb_val
            }
        }

        impl<T, $($var,)* R> IntoWasmClosure<dyn FnMut($($var),*) -> R> for T
            where T: 'static + FnMut($($var),*) -> R,
                  $($var: FromWasmAbi + 'static,)*
                  R: ReturnWasmAbi + 'static,
        {
            fn unsize(self: Box<Self>) -> Box<dyn FnMut($($var),*) -> R> { self }
        }

        impl<T, $($var,)* R> IntoWasmClosure<dyn Fn($($var),*) -> R> for T
            where T: 'static + Fn($($var),*) -> R,
                  $($var: FromWasmAbi + 'static,)*
                  R: ReturnWasmAbi + 'static,
        {
            fn unsize(self: Box<Self>) -> Box<dyn Fn($($var),*) -> R> { self }
        }
    )*)
}

// unsafe impl<A, R> WasmClosure for dyn Fn(A) -> R + 'static
// where
//     A: FromWasmAbi + 'static,
//     R: ReturnWasmAbi + 'static,

// unsafe impl<A, B, R> WasmClosure for dyn Fn(A, B) -> R + 'static
// where
//     A: FromWasmAbi + 'static,
//     B: FromWasmAbi + 'static,
//     R: ReturnWasmAbi + 'static,

doit! {
    ()
    (A a1 a2 a3 a4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4 E e1 e2 e3 e4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4 E e1 e2 e3 e4 F f1 f2 f3 f4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4 E e1 e2 e3 e4 F f1 f2 f3 f4 G g1 g2 g3 g4)
    (A a1 a2 a3 a4 B b1 b2 b3 b4 C c1 c2 c3 c4 D d1 d2 d3 d4 E e1 e2 e3 e4 F f1 f2 f3 f4 G g1 g2 g3 g4 H h1 h2 h3 h4)
}

// unsafe impl<A, R> WasmClosure for dyn Fn(&A) -> R
// where
//     A: RefFromWasmAbi,
//     R: ReturnWasmAbi + 'static,
// {
//     fn describe() {
//         #[allow(non_snake_case)]
//         unsafe extern "C" fn invoke<A: RefFromWasmAbi, R: ReturnWasmAbi>(
//             a: f64,
//             b: f64,
//             arg1: <A::Abi as WasmAbi>::Prim1,
//             arg2: <A::Abi as WasmAbi>::Prim2,
//             arg3: <A::Abi as WasmAbi>::Prim3,
//             arg4: <A::Abi as WasmAbi>::Prim4,
//         ) -> WasmRet<R::Abi> {
//             if a == 0 {
//                 throw_str("closure invoked after being dropped");
//             }
//             // Make sure all stack variables are converted before we
//             // convert `ret` as it may throw (for `Result`, for
//             // example)
//             let ret = {
//                 let f: *const dyn Fn(&A) -> R = FatPtr { fields: (a, b) }.ptr;
//                 let arg = <A as RefFromWasmAbi>::ref_from_abi(A::Abi::join(arg1, arg2, arg3, arg4));
//                 (*f)(&*arg)
//             };
//             ret.return_abi().into()
//         }

//         inform(invoke::<A, R> as u32);

//         unsafe extern "C" fn destroy<A: RefFromWasmAbi, R: ReturnWasmAbi>(a: f64, b: f64) {
//             // See `Fn()` above for why we simply return
//             if a == 0 {
//                 return;
//             }
//             drop(Box::from_raw(
//                 FatPtr::<dyn Fn(&A) -> R> { fields: (a, b) }.ptr,
//             ));
//         }
//         inform(destroy::<A, R> as u32);

//         <&Self>::describe();
//     }
// }

trait WasmClosureFirst {}

// impl<T> WasmClosureCommon for T where T: !RefFromWasmAbi {}

// pub trait WasmClosureCommon: !WasmClosureFirst {}

trait WasmClosureCommon {}

impl<A, R> WasmClosureCommon for dyn Fn(A) -> R + 'static
where
    A: FromWasmAbi + 'static,
    R: ReturnWasmAbi + 'static,
{
}

impl<A, R> WasmClosureCommon for dyn Fn(&A) -> R
where
    A: RefFromWasmAbi,
    R: ReturnWasmAbi + 'static,
{
}

unsafe impl<A, R> WasmClosure for dyn Fn(&A) -> R
where
    A: RefFromWasmAbi,
    R: ReturnWasmAbi + 'static,
{
    fn describe() {
        #[allow(non_snake_case)]
        unsafe extern "C" fn invoke<A: RefFromWasmAbi, R: ReturnWasmAbi>(
            a: f64,
            b: f64,
            arg1: <A::Abi as WasmAbi>::Prim1,
            arg2: <A::Abi as WasmAbi>::Prim2,
            arg3: <A::Abi as WasmAbi>::Prim3,
            arg4: <A::Abi as WasmAbi>::Prim4,
        ) -> WasmRet<R::Abi> {
            if a == 0f64 {
                throw_str("closure invoked after being dropped");
            }
            // Make sure all stack variables are converted before we
            // convert `ret` as it may throw (for `Result`, for
            // example)
            let ret = {
                let f: *const dyn Fn(&A) -> R = FatPtr { fields: (a, b) }.ptr;
                let arg = <A as RefFromWasmAbi>::ref_from_abi(A::Abi::join(arg1, arg2, arg3, arg4));
                (*f)(&*arg)
            };
            ret.return_abi().into()
        }

        inform(invoke::<A, R> as u32 as f64);

        unsafe extern "C" fn destroy<A: RefFromWasmAbi, R: ReturnWasmAbi>(a: f64, b: f64) {
            // See `Fn()` above for why we simply return
            if a == 0f64 {
                return;
            }
            drop(Box::from_raw(
                FatPtr::<dyn Fn(&A) -> R> { fields: (a, b) }.ptr,
            ));
        }
        inform(destroy::<A, R> as u32 as f64);

        <&Self>::describe();
    }
}

unsafe impl<A, R> WasmClosure for dyn FnMut(&A) -> R
where
    A: RefFromWasmAbi,
    R: ReturnWasmAbi + 'static,
{
    fn describe() {
        #[allow(non_snake_case)]
        unsafe extern "C" fn invoke<A: RefFromWasmAbi, R: ReturnWasmAbi>(
            a: f64,
            b: f64,
            arg1: <A::Abi as WasmAbi>::Prim1,
            arg2: <A::Abi as WasmAbi>::Prim2,
            arg3: <A::Abi as WasmAbi>::Prim3,
            arg4: <A::Abi as WasmAbi>::Prim4,
        ) -> WasmRet<R::Abi> {
            if a == 0f64 {
                throw_str("closure invoked recursively or after being dropped");
            }
            // Make sure all stack variables are converted before we
            // convert `ret` as it may throw (for `Result`, for
            // example)
            let ret = {
                let f: *const dyn FnMut(&A) -> R = FatPtr { fields: (a, b) }.ptr;
                let f = f as *mut dyn FnMut(&A) -> R;
                let arg = <A as RefFromWasmAbi>::ref_from_abi(A::Abi::join(arg1, arg2, arg3, arg4));
                (*f)(&*arg)
            };
            ret.return_abi().into()
        }

        inform(invoke::<A, R> as u32 as f64);

        unsafe extern "C" fn destroy<A: RefFromWasmAbi, R: ReturnWasmAbi>(a: f64, b: f64) {
            // See `Fn()` above for why we simply return
            if a == 0f64 {
                return;
            }
            drop(Box::from_raw(
                FatPtr::<dyn FnMut(&A) -> R> { fields: (a, b) }.ptr,
            ));
        }
        inform(destroy::<A, R> as u32 as f64);

        <&mut Self>::describe();
    }
}

#[allow(non_snake_case)]
impl<T, A, R> WasmClosureFnOnce<(&A,), R> for T
where
    T: 'static + FnOnce(&A) -> R,
    A: RefFromWasmAbi + 'static,
    R: ReturnWasmAbi + 'static,
{
    type FnMut = dyn FnMut(&A) -> R;

    fn into_fn_mut(self) -> Box<Self::FnMut> {
        let mut me = Some(self);
        Box::new(move |arg| {
            let me = me.take().expect_throw("FnOnce called more than once");
            me(arg)
        })
    }

    fn into_sb_function(self) -> SbValue {
        use crate::__rt::WasmRefCell;
        use std::rc::Rc;

        let mut me = Some(self);

        let rc1 = Rc::new(WasmRefCell::new(None));
        let rc2 = rc1.clone();

        let closure = Closure::wrap(Box::new(move |arg: &A| {
            // Invoke ourself and get the result.
            let me = me.take().expect_throw("FnOnce called more than once");
            let result = me(arg);

            // And then drop the `Rc` holding this function's `Closure`
            // alive.
            debug_assert_eq!(Rc::strong_count(&rc2), 1);
            let option_closure = rc2.borrow_mut().take();
            debug_assert!(option_closure.is_some());
            drop(option_closure);

            result
        }) as Box<dyn FnMut(&A) -> R>);

        let sb_val = closure.as_ref().clone();

        *rc1.borrow_mut() = Some(closure);
        debug_assert_eq!(Rc::strong_count(&rc1), 2);
        drop(rc1);

        sb_val
    }
}
