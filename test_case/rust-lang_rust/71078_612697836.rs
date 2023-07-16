rust
pub static FOO: refl::Id<usize, Box<u8>> = FOO;

fn main() {
    let x: Box<_> = FOO.cast(42usize);
    drop(x);
}

mod refl {
    pub struct Id<S: ?Sized, T: ?Sized>(core::marker::PhantomData<(fn(S) -> S, fn(T) -> T)>);

    impl<S: ?Sized, T: ?Sized> Copy for Id<S, T> {}
    impl<S: ?Sized, T: ?Sized> Clone for Id<S, T> { fn clone(&self) -> Self { *self } }

    impl<S: ?Sized, T: ?Sized> Id<S, T> {
        /// Casts a value of type `S` to `T`.
        ///
        /// This is safe because the `Id` type is always guaranteed to
        /// only be inhabited by `Id<T, T>` types by construction.
        pub fn cast(self, value: S) -> T where S: Sized, T: Sized {
            unsafe {
                // Transmute the value;
                // This is safe since we know by construction that
                // S == T (including lifetime invariance) always holds.
                let cast_value = core::mem::transmute_copy(&value);
    
                // Forget the value;
                // otherwise the destructor of S would be run.
                core::mem::forget(value);
    
                cast_value
            }
        }
    }
}
