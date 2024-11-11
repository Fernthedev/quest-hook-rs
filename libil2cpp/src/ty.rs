use paste::paste;
use std::borrow::Cow;
use std::ffi::{c_void, CStr};
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::ptr::null_mut;

use crate::raw::{
    Il2CppTypeEnum_IL2CPP_TYPE_BOOLEAN, Il2CppTypeEnum_IL2CPP_TYPE_CHAR,
    Il2CppTypeEnum_IL2CPP_TYPE_I1, Il2CppTypeEnum_IL2CPP_TYPE_I2, Il2CppTypeEnum_IL2CPP_TYPE_I4,
    Il2CppTypeEnum_IL2CPP_TYPE_I8, Il2CppTypeEnum_IL2CPP_TYPE_OBJECT,
    Il2CppTypeEnum_IL2CPP_TYPE_R4, Il2CppTypeEnum_IL2CPP_TYPE_R8,
    Il2CppTypeEnum_IL2CPP_TYPE_STRING, Il2CppTypeEnum_IL2CPP_TYPE_U1,
    Il2CppTypeEnum_IL2CPP_TYPE_U2, Il2CppTypeEnum_IL2CPP_TYPE_U4, Il2CppTypeEnum_IL2CPP_TYPE_U8,
    Il2CppTypeEnum_IL2CPP_TYPE_VOID,
};
use crate::{raw, Generics, Il2CppClass, Il2CppException, Il2CppObject, WrapRaw};

/// An il2cpp type
#[repr(transparent)]
pub struct Il2CppType(raw::Il2CppType);

unsafe impl Send for Il2CppType {}
unsafe impl Sync for Il2CppType {}

impl Il2CppType {
    /// Class of the type
    pub fn class(&self) -> &Il2CppClass {
        unsafe { Il2CppClass::wrap(raw::class_from_il2cpp_type(self.raw())) }
    }

    /// Name of the type
    pub fn name(&self) -> Cow<'_, str> {
        if let Some(name) = self.as_builtin().map(Builtin::name) {
            return name.into();
        }

        let name = unsafe { raw::type_get_name(self.raw()) };
        assert!(!name.is_null());
        unsafe { CStr::from_ptr(name) }.to_string_lossy()
    }

    /// Whether the type is a ref type
    pub fn is_ref(&self) -> bool {
        self.raw().byref() != 0
    }

    /// [`Il2CppReflectionType`] which represents the type
    pub fn reflection_object(&self) -> &Il2CppReflectionType {
        unsafe { Il2CppReflectionType::wrap_mut(raw::type_get_object(self.raw())) }
    }
}

unsafe impl WrapRaw for Il2CppType {
    type Raw = raw::Il2CppType;
}

impl PartialEq for Il2CppType {
    #[cfg(any(feature = "unity2019", feature = "unity2018"))]
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.raw().data.klassIndex == other.raw().data.klassIndex }
    }

    #[cfg(feature = "unity2022")]
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.raw().data.__klassIndex == other.raw().data.__klassIndex }
    }
}
impl Eq for Il2CppType {}

impl fmt::Debug for Il2CppType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Il2CppType")
            .field("name", &self.name())
            .finish()
    }
}

impl fmt::Display for Il2CppType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&*self.name())
    }
}

macro_rules! builtins {
    ($($const:ident => ($variant:ident, $id:ident, $name:literal),)*) => {

        // essentially Windows clang will use i32
        // https://github.com/rust-lang/rust-bindgen/issues/1966


        #[doc = "Builtin C# types"]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "unity2022", repr(i32))]
        #[cfg_attr(feature = "unity2019", repr(u32))]
        #[cfg_attr(feature = "unity2018", repr(i32))]
        pub enum Builtin {
            $(
                #[doc = concat!("`", $name, "`")]
                $variant = $const,
            )*
        }

        impl Il2CppType {
            #[doc = "Whether the type represents the given [`Builtin`]"]
            #[inline]
            pub fn is_builtin(&self, builtin: Builtin) -> bool {
                #[cfg(feature = "unity2022")]
                { self.raw().type_() == builtin as i32 }

                #[cfg(feature = "unity2019")]
                { self.raw().type_() == builtin as u32 }

                #[cfg(feature = "unity2018")]
                { self.raw().type_() == builtin as i32 }
            }

            paste! {
                $(
                    #[doc = concat!("Whether the type represents a `", $name , "`")]
                    pub fn [<is_ $id>](&self) -> bool {
                        self.is_builtin(Builtin::$variant)
                    }
                )*
            }

            #[doc = "[`Builtin`] the type represents, if any"]
            pub fn as_builtin(&self) -> Option<Builtin> {
                #[allow(non_upper_case_globals)]
                match self.raw().type_() {
                    $($const => Some(Builtin::$variant),)*
                    _ => None
                }
            }
        }

        impl Builtin {
            #[doc = "Name of the builtin"]
            pub fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $name,)*
                }
            }
        }
    }
}

builtins! {
    Il2CppTypeEnum_IL2CPP_TYPE_VOID => (Void, void, "void"),
    Il2CppTypeEnum_IL2CPP_TYPE_OBJECT => (Object, object, "object"),
    Il2CppTypeEnum_IL2CPP_TYPE_BOOLEAN => (Bool, bool, "bool"),
    Il2CppTypeEnum_IL2CPP_TYPE_CHAR => (Char, char, "char"),
    Il2CppTypeEnum_IL2CPP_TYPE_U1 => (Byte, byte, "byte"),
    Il2CppTypeEnum_IL2CPP_TYPE_I1 => (SByte, sbyte, "sbyte"),
    Il2CppTypeEnum_IL2CPP_TYPE_I2 => (Short, short, "short"),
    Il2CppTypeEnum_IL2CPP_TYPE_U2 => (UShort, ushort, "ushort"),
    Il2CppTypeEnum_IL2CPP_TYPE_I4 => (Int, int, "int"),
    Il2CppTypeEnum_IL2CPP_TYPE_U4 => (UInt, uint, "uint"),
    Il2CppTypeEnum_IL2CPP_TYPE_I8 => (Long, long, "long"),
    Il2CppTypeEnum_IL2CPP_TYPE_U8 => (ULong, ulong, "ulong"),
    Il2CppTypeEnum_IL2CPP_TYPE_R4 => (Single, single, "single"),
    Il2CppTypeEnum_IL2CPP_TYPE_R8 => (Double, double, "double"),
    Il2CppTypeEnum_IL2CPP_TYPE_STRING => (String, string, "string"),
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Object used for reflection of types
pub struct Il2CppReflectionType(raw::Il2CppReflectionType);

unsafe impl WrapRaw for Il2CppReflectionType {
    type Raw = raw::Il2CppReflectionType;
}

impl Il2CppReflectionType {
    /// [`Il2CppType`] which this object represents
    pub fn ty(&self) -> &Il2CppType {
        unsafe { Il2CppType::wrap(&*self.raw().type_) }
    }

    /// Instanciates a generic type template with the provided generic
    /// arguments
    pub fn make_generic<G>(&self) -> Result<Option<&Self>, &mut Il2CppException>
    where
        G: Generics,
    {
        let generics = G::type_array();
        let make_generic = self
            .class()
            .find_method_unchecked("MakeGenericType", 2)
            .unwrap();
        let ret = unsafe {
            make_generic.invoke_raw(
                null_mut(),
                [
                    self as *const Self as *mut c_void,
                    (generics as *mut raw::Il2CppArray).cast(),
                ]
                .as_mut(),
            )
        };
        let obj = match ret {
            Ok(Some(obj)) => obj,
            Ok(None) => return Ok(None),
            Err(e) => return Err(unsafe { Il2CppException::wrap_mut(e) }),
        };
        let ty = unsafe { &mut *(obj as *mut raw::Il2CppObject).cast() };
        Ok(Some(ty))
    }
}

impl Deref for Il2CppReflectionType {
    type Target = Il2CppObject;

    fn deref(&self) -> &Self::Target {
        unsafe { Il2CppObject::wrap(&self.raw().object) }
    }
}

impl DerefMut for Il2CppReflectionType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { Il2CppObject::wrap_mut(&mut self.raw_mut().object) }
    }
}

impl fmt::Debug for Il2CppReflectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Il2CppReflectionType")
            .field("ty", self.ty())
            .finish()
    }
}
