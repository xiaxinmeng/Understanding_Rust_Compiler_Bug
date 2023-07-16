rust
pub trait FooLt<'env> {
    type Bar;
    
    fn get(&self) -> &Self::Bar;
}

// doesn't compile, because 
// the associated type `<T as FooLt<'_>>::Bar` may not live long enough
// in `get` even though it can!
impl<'env, T: ?Sized + FooLt<'env>> FooLt<'env> for &T {
    type Bar = T::Bar;

    fn get(&self) -> &Self::Bar { T::get(self) }
}
