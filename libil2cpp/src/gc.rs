use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use crate::{Argument, Returned, ThisArgument, Type};

pub struct Gc<T>(*mut T)
where
    T: GcType;

pub trait GcType = Type + Returned + ThisArgument + Argument;

impl<T: GcType> Gc<T> {
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
}

unsafe impl<T: GcType> Type for Gc<T>
where
    T: Type,
{
    type Held<'a> = Option<&'a mut Self>;

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

impl<T: GcType> PartialEq for Gc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: GcType> Eq for Gc<T> {}

impl<T: GcType> Clone for Gc<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: GcType> Copy for Gc<T> {}

impl<T: GcType> Default for Gc<T> {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}

impl<T: GcType> Deref for Gc<T> {
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
impl<T: GcType> DerefMut for Gc<T> {
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

impl<T: GcType> From<*mut T> for Gc<T> {
    fn from(ptr: *mut T) -> Self {
        Self(ptr)
    }
}
impl<T: GcType> From<&mut T> for Gc<T> {
    fn from(ptr: &mut T) -> Self {
        Self(ptr)
    }
}
impl<T: GcType> From<Option<&mut T>> for Gc<T> {
    fn from(ptr: Option<&mut T>) -> Self {
        match ptr {
            Some(ptr) => Self(ptr),
            None => Self::null(),
        }
    }
}
