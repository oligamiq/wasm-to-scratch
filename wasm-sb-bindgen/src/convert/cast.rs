use crate::{SbValue, WasmDescribe};

pub trait SbCast
where
    Self: AsRef<SbValue> + Into<SbValue>,
{
    /// Test whether this SB value has a type `T`.
    ///
    /// This method will dynamically check to see if this SB object can be
    /// casted to the SB object of type `T`. Usually this uses the `instanceof`
    /// operator. This also works with primitive types like
    /// booleans/strings/numbers as well as cross-realm object like `Array`
    /// which can originate from other iframes.
    ///
    /// In general this is intended to be a more robust version of
    /// `is_instance_of`, but if you want strictly the `instanceof` operator
    /// it's recommended to use that instead.
    fn has_type<T>(&self) -> bool
    where
        T: SbCast,
    {
        T::is_type_of(self.as_ref())
    }

    /// Performs a dynamic cast (checked at runtime) of this value into the
    /// target type `T`.
    ///
    /// This method will return `Err(self)` if `self.has_type::<T>()`
    /// returns `false`, and otherwise it will return `Ok(T)` manufactured with
    /// an unchecked cast (verified correct via the `has_type` operation).
    fn dyn_into<T>(self) -> Result<T, Self>
    where
        T: SbCast,
    {
        if self.has_type::<T>() {
            Ok(self.unchecked_into())
        } else {
            Err(self)
        }
    }

    /// Performs a dynamic cast (checked at runtime) of this value into the
    /// target type `T`.
    ///
    /// This method will return `None` if `self.has_type::<T>()`
    /// returns `false`, and otherwise it will return `Some(&T)` manufactured
    /// with an unchecked cast (verified correct via the `has_type` operation).
    fn dyn_ref<T>(&self) -> Option<&T>
    where
        T: SbCast,
    {
        if self.has_type::<T>() {
            Some(self.unchecked_ref())
        } else {
            None
        }
    }

    /// Performs a zero-cost unchecked cast into the specified type.
    ///
    /// This method will convert the `self` value to the type `T`, where both
    /// `self` and `T` are simple wrappers around `SbValue`. This method **does
    /// not check whether `self` is an instance of `T`**. If used incorrectly
    /// then this method may cause runtime exceptions in both Rust and SB, this
    /// should be used with caution.
    fn unchecked_into<T>(self) -> T
    where
        T: SbCast,
    {
        T::unchecked_from_sb(self.into())
    }

    /// Performs a zero-cost unchecked cast into a reference to the specified
    /// type.
    ///
    /// This method will convert the `self` value to the type `T`, where both
    /// `self` and `T` are simple wrappers around `SbValue`. This method **does
    /// not check whether `self` is an instance of `T`**. If used incorrectly
    /// then this method may cause runtime exceptions in both Rust and SB, this
    /// should be used with caution.
    ///
    /// This method, unlike `unchecked_into`, does not consume ownership of
    /// `self` and instead works over a shared reference.
    fn unchecked_ref<T>(&self) -> &T
    where
        T: SbCast,
    {
        T::unchecked_from_sb_ref(self.as_ref())
    }

    /// Test whether this SB value is an instance of the type `T`.
    ///
    /// This method performs a dynamic check (at runtime) using the SB
    /// `instanceof` operator. This method returns `self instanceof T`.
    ///
    /// Note that `instanceof` does not always work with primitive values or
    /// across different realms (e.g. iframes). If you're not sure whether you
    /// specifically need only `instanceof` it's recommended to use `has_type`
    /// instead.
    fn is_instance_of<T>(&self) -> bool
    where
        T: SbCast,
    {
        T::instanceof(self.as_ref())
    }

    /// Performs a dynamic `instanceof` check to see whether the `SbValue`
    /// provided is an instance of this type.
    ///
    /// This is intended to be an internal implementation detail, you likely
    /// won't need to call this. It's generally called through the
    /// `is_instance_of` method instead.
    fn instanceof(val: &SbValue) -> bool;

    /// Performs a dynamic check to see whether the `SbValue` provided
    /// is a value of this type.
    ///
    /// Unlike `instanceof`, this can be specialised to use a custom check by
    /// adding a `#[wasm_bindgen(is_type_of = callback)]` attribute to the
    /// type import declaration.
    ///
    /// Other than that, this is intended to be an internal implementation
    /// detail of `has_type` and you likely won't need to call this.
    fn is_type_of(val: &SbValue) -> bool {
        Self::instanceof(val)
    }

    /// Performs a zero-cost unchecked conversion from a `SbValue` into an
    /// instance of `Self`
    ///
    /// This is intended to be an internal implementation detail, you likely
    /// won't need to call this.
    fn unchecked_from_sb(val: SbValue) -> Self;

    /// Performs a zero-cost unchecked conversion from a `&SbValue` into an
    /// instance of `&Self`.
    ///
    /// Note the safety of this method, which basically means that `Self` must
    /// be a newtype wrapper around `SbValue`.
    ///
    /// This is intended to be an internal implementation detail, you likely
    /// won't need to call this.
    fn unchecked_from_sb_ref(val: &SbValue) -> &Self;
}

/// Trait implemented for wrappers around `SbValue`s generated by `#[wasm_bindgen]`.
#[doc(hidden)]
pub trait SbObject: SbCast + WasmDescribe {}
