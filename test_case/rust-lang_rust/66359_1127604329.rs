rust
macro_rules! type_name_of {( $e:expr $(,)? ) => ({
    let it = [];
    #[allow(unreachable_code)] {
        if false {
            loop {} // disables borrowck and dropck
            (&mut { it })[0] = &$e; // nudges type inference
        }
    }
    $crate::__helper__(it)
})} pub(in crate) use type_name_of;

#[doc(hidden)] pub
fn __helper__<T> (_: [&T; 0])
  -> &'static str
{
    ::core::any::type_name::<T>()
}
