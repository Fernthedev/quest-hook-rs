use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};

use crate::{Argument, ObjectType, Returned, ThisArgument, Type};

/// Wrapper type which implies the type is ByRef managed lifetime
#[repr(transparent)]
pub struct ByRef<'a, T>(&'a mut T)
where
    T: ReffableType;

pub type ByRefMut<'a, T> = ByRef<'a, T>;

/// Trait alias for types that can be used with the `ByRef` wrapper.
pub trait ReffableType = Type + Returned + ThisArgument + Argument;

impl<'a, T> ByRef<'a, T>
where
    T: ReffableType,
{
    /// Creates a new `ByRef` instance with the given pointer.
    pub fn new(ptr: &'a mut T) -> Self {
        Self(ptr)
    }

    /// Returns a constant pointer to the value.
    pub fn get_pointer(&self) -> *const T {
        self.0
    }
    /// Returns a mutable pointer to the value.
    pub fn get_pointer_mut(&mut self) -> *mut T {
        self.0
    }

    pub fn into_actual(self) -> &'a T {
        self.0
    }

    pub fn from_actual(actual: &'a mut T) -> Self {
        Self::new(actual)
    }
}



unsafe impl<T> Type for ByRef<'_, T>
where
    T: ReffableType,
{
    type Held<'b> = &'b mut T::Held<'b>;

    type HeldRaw = *mut T::HeldRaw;

    const NAMESPACE: &'static str = T::NAMESPACE;

    const CLASS_NAME: &'static str = T::CLASS_NAME;

    fn class() -> &'static crate::Il2CppClass {
        T::class()
    }

    fn type_() -> &'static crate::Il2CppType {
        T::class().this_arg_ty()
    }

    fn matches_value_argument(_: &crate::Il2CppType) -> bool {
        false
    }
    fn matches_reference_argument(ty: &crate::Il2CppType) -> bool {
        T::class().this_arg_ty() == ty
            || ty.is_ref() && ty.class().is_assignable_from(<T as crate::Type>::class())
    }
    fn matches_value_parameter(_: &crate::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &crate::Il2CppType) -> bool {
        T::class().this_arg_ty() == ty
            || ty.is_ref() && <T as crate::Type>::class().is_assignable_from(ty.class())
    }
}

// // Should I do this or force to implement these on a wrapper?
unsafe impl<T> Send for ByRef<'_, T> where T: ReffableType {}
unsafe impl<T> Sync for ByRef<'_, T> where T: ReffableType {}

impl<'a, T> From<&'a mut T> for ByRef<'a, T>
where
    T: ReffableType,
{
    fn from(value: &'a mut T) -> Self {
        Self::new(value)
    }
}
impl<T> Deref for ByRef<'_, T>
where
    T: ReffableType,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T> DerefMut for ByRef<'a, T>
where
    T: ReffableType,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}
impl<'a, T> PartialEq for ByRef<'_, T>
where
    T: PartialEq,
    T: ReffableType,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<T> Eq for ByRef<'_, T>
where
    T: Eq,
    T: ReffableType,
{
}

impl<T> Debug for ByRef<'_, T>
where
    T: ReffableType,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ByRef<{}>({:p})", T::CLASS_NAME, self.0)
    }
}

impl<T> Display for ByRef<'_, T>
where
    T: Display,
    T: ReffableType,{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a, T> AsRef<T> for ByRef<'a, T>
where
    T: ReffableType,
{
    fn as_ref(&self) -> &T {
        self.0
    }
}

impl<'a, T> AsMut<T> for ByRef<'a, T>
where
    T: ReffableType,
{
    fn as_mut(&mut self) -> &mut T {
        self.0
    }
}