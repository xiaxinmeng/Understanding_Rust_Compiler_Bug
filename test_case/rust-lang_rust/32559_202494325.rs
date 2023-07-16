 rust
fn main() {
    // may have any number of type parameters greater than zero
    struct WithParam<T>(std::marker::PhantomData<T>); 
    // Target can be anything
    impl<T> std::ops::Deref for WithParam<T> {
        type Target = ();
        fn deref(&self) -> &() {
            panic!()
        }
    }
    // can be literally anything
    struct OtherType;
    fn doop(_: &OtherType) { }
    // must not be able to infer the inner type
    doop(&WithParam(std::marker::PhantomData))
    // the references must be able to coerce,
    // so either &mut -> &, & -> &, or &mut -> &mut
    // passing an &WithParam to a function expecting an &mut OtherType
    // will not create ICE conditions
}
