use dlopen::wrapper::{Container, WrapperApi};
use dlopen_derive::WrapperApi;
use paste::paste;
use std::lazy::SyncLazy;
use std::os::raw::c_char;

use super::{Il2CppAssembly, Il2CppClass, Il2CppDomain, Il2CppImage};

macro_rules! define_functions {
    ( $( pub fn $name:ident ( $( $arg_name:ident : $arg_type:ty ),* ) $( -> $return:ty )* ; )+ ) => {
        paste! {
            #[derive(WrapperApi)]
            struct LibIl2Cpp {
                $(
                    [<il2cpp_ $name>]: extern "C" fn ( $( $arg_name : $arg_type ),* ) $( -> $return )*
                ),*
            }
        }

        static LIBIL2CPP: SyncLazy<Container<LibIl2Cpp>> =
            SyncLazy::new(|| unsafe { Container::load("libil2cpp.so") }.unwrap());

        paste! {
            $(
                pub fn $name ( $( $arg_name : $arg_type ),* ) $( -> $return )* {
                    LIBIL2CPP.[<il2cpp_ $name>]( $( $arg_name ),* )
                }
            )+
        }
    }
}

define_functions! {
    pub fn domain_get() -> &'static Il2CppDomain;
    pub fn domain_get_assemblies(domain: &Il2CppDomain, size: &mut usize) -> &'static [&'static Il2CppAssembly];
    pub fn assembly_get_image(assembly: &Il2CppAssembly) -> Option<&'static Il2CppImage>;
    pub fn class_from_name(image: &Il2CppImage, namespace: *const c_char, name: *const c_char) -> Option<&'static Il2CppClass>;
}