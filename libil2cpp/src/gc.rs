use std::fmt::{self, Debug, Formatter};
use std::ops::{Deref, DerefMut, Not};

use crate::{Argument, Returned, ThisArgument, Type};

pub struct Gc<T>(*mut T)
where
    *mut T: GcType, // assert that *mut T is a GcType
    T: for<'a> Type<Held<'a> = Option<&'a mut T>>;

pub trait GcType = Type + Returned + ThisArgument + Argument;

impl<T> Gc<T>
where
    *mut T: GcType,
    T: for<'a> Type<Held<'a> = Option<&'a mut T>>,
{
    /// Creates a new `Gc` instance with the given pointer.
    pub fn new(ptr: *mut T) -> Self {
        Self(ptr)
    }

    /// Creates a new `Gc` instance with a null pointer.
    pub fn null() -> Self {
        Self::default()
    }

    /// Checks if the pointer is null.
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn as_opt(&self) -> Option<&T> {
        self.is_null().not().then(|| unsafe { &*self.0 })
    }
    pub fn as_opt_mut(&mut self) -> Option<&mut T> {
        self.is_null().not().then(|| unsafe { &mut *self.0 })
    }
}

unsafe impl<T> Type for Gc<T>
where
    *mut T: GcType,
    T: for<'a> Type<Held<'a> = Option<&'a mut T>>,
{
    type Held<'a> = Option<&'a mut T>;

    type HeldRaw = *mut T;

    const NAMESPACE: &'static str = T::NAMESPACE;

    const CLASS_NAME: &'static str = T::CLASS_NAME;

    fn matches_reference_argument(ty: &crate::Il2CppType) -> bool {
        T::matches_reference_argument(ty)
    }

    fn matches_value_argument(ty: &crate::Il2CppType) -> bool {
        T::matches_value_argument(ty)
    }

    fn matches_reference_parameter(ty: &crate::Il2CppType) -> bool {
        T::matches_reference_parameter(ty)
    }

    fn matches_value_parameter(ty: &crate::Il2CppType) -> bool {
        T::matches_value_parameter(ty)
    }
}

impl<T> From<Gc<T>> for Option<&T>
where
    *mut T: GcType,
    T: for<'a> Type<Held<'a> = Option<&'a mut T>>,
{
    fn from(value: Gc<T>) -> Self {
        value.is_null().not().then(|| unsafe { &*value.0 })
    }
}
impl<T> From<Gc<T>> for Option<&mut T>
where
    *mut T: GcType,
    T: for<'a> Type<Held<'a> = Option<&'a mut T>>,
{
    fn from(value: Gc<T>) -> Self {
        value.is_null().not().then(|| unsafe { &mut *value.0 })
    }
}

impl<T> PartialEq for Gc<T>
where
    *mut T: GcType,
    T: for<'a> Type<Held<'a> = Option<&'a mut T>>,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for Gc<T>
where
    *mut T: GcType,
    T: for<'a> Type<Held<'a> = Option<&'a mut T>>,
{
}

impl<T> Clone for Gc<T>
where
    *mut T: GcType,
    T: for<'a> Type<Held<'a> = Option<&'a mut T>>,
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Gc<T>
where
    *mut T: GcType,
    T: for<'a> Type<Held<'a> = Option<&'a mut T>>,
{
}

impl<T> Default for Gc<T>
where
    *mut T: GcType,
    T: for<'a> Type<Held<'a> = Option<&'a mut T>>,
{
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}

impl<T> Deref for Gc<T>
where
    *mut T: GcType,
    T: for<'a> Type<Held<'a> = Option<&'a mut T>>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        if self.is_null() {
            panic!(
                "Attempted to dereference a null type {}::{}",
                T::NAMESPACE,
                T::CLASS_NAME
            );
        }
        unsafe { &*self.0 }
    }
}
impl<T> DerefMut for Gc<T>
where
    *mut T: GcType,
    T: for<'a> Type<Held<'a> = Option<&'a mut T>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.is_null() {
            panic!(
                "Attempted to dereference a null type {}::{}",
                T::NAMESPACE,
                T::CLASS_NAME
            );
        }
        unsafe { &mut *self.0 }
    }
}

impl<T> From<*mut T> for Gc<T>
where
    *mut T: GcType,
    T: for<'a> Type<Held<'a> = Option<&'a mut T>>,
{
    fn from(ptr: *mut T) -> Self {
        Self(ptr)
    }
}
impl<T> From<&mut T> for Gc<T>
where
    *mut T: GcType,
    T: for<'a> Type<Held<'a> = Option<&'a mut T>>,
{
    fn from(ptr: &mut T) -> Self {
        Self(ptr)
    }
}
impl<T> From<Option<&mut T>> for Gc<T>
where
    *mut T: GcType,
    T: for<'a> Type<Held<'a> = Option<&'a mut T>>,
{
    fn from(ptr: Option<&mut T>) -> Self {
        match ptr {
            Some(ptr) => Self(ptr),
            None => Self::null(),
        }
    }
}

impl<T> Debug for Gc<T>
where
    *mut T: GcType,
    T: for<'a> Type<Held<'a> = Option<&'a mut T>>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_null() {
            write!(f, "Gc<{}>::null()", T::CLASS_NAME)
        } else {
            write!(f, "Gc<{}>({:p})", T::CLASS_NAME, self.0)
        }
    }
}
