use crate::{Arguments, Il2CppException, Returned, ThisArgument, Type};

pub trait ValueTypeExt: for<'a> Type<Held<'a> = Self> + Sized + ThisArgument {
    /// Invokes the method with the given name on `self` using the given
    /// arguments, with type checking
    ///
    /// # Panics
    ///
    /// This method will panic if a matching method can't be found.
    fn invoke<A, R, const N: usize>(
        &mut self,
        name: &str,
        args: A,
    ) -> Result<R, &mut Il2CppException>
    where
        A: Arguments<N>,
        R: Returned,
    {
        let method = Self::class().find_method::<A, R, N>(name).unwrap();
        unsafe { method.invoke_unchecked(self, args) }
    }

    /// Invokes the `void` method with the given name on `self` using the
    /// given arguments, with type checking
    ///
    /// # Panics
    ///
    /// This method will panic if a matching method can't be found.
    fn invoke_void<A, const N: usize>(
        &mut self,
        name: &str,
        args: A,
    ) -> Result<(), &mut Il2CppException>
    where
        A: Arguments<N>,
    {
        let method = Self::class().find_method::<A, (), N>(name).unwrap();
        unsafe { method.invoke_unchecked(self, args) }
    }
}
impl<T> ValueTypeExt for T
where
    T: for<'a> Type<Held<'a> = T>,
    T: ThisArgument,
{
}
