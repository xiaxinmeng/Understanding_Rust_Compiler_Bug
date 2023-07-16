
// clone a &'static fn
macro_rules! closure_clone(
    ($($A:ident),*) => (
        impl<$($A,)* Ret_Type> Clone for &'static fn($($A),*) -> Ret_Type {
            /// Return a copy of a captureless closure
            #[inline]
            fn clone(&self) -> &'static fn($($A),*) -> Ret_Type {
                unsafe {
                    let closure: raw::Closure = cast::transmute_copy(self);
                    assert!(ptr::is_null(closure.env));
                    cast::transmute(closure)
                }
            }
        }
    )
)
