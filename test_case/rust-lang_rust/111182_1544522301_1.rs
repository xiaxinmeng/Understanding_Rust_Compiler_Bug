 rust
// Fake impl that's only really used for docs.
#[cfg(doc)]
#[unstable(feature = "fn_ptr_trait")]
#[doc(fake_variadic)]
/// This trait is implemented on function pointers with any number of arguments. 
impl<Ret, T> FnPtr for fn(T) -> Ret {
    fn addr(&self) -> *const () {
        // empty
    }
}
