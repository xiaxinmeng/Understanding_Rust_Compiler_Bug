 rust
mod by_ref {
    pub trait ByRef<T> {
        fn get_ref<'a>(&'a self) -> &'a T;
    }

    // If we had a "Deref" kind, then rather than having to separately
    // impl for each pointer type, then we might have something like this:
    // impl<'b,T,U:Deref<T>> ByRef<T> for U {
    //     fn by_ref<'a>(&'a self) -> &'a T { &**self }
    // }
    impl<'b,T> ByRef<T> for &'b T {
        fn get_ref<'a>(&'a self) -> &'a T { &**self }
    }
    impl<T> ByRef<T> for ~T {
        fn get_ref<'a>(&'a self) -> &'a T { &**self }
    }

    // We can't make bare objects all implement this--so we have to wrap it:
    pub fn by_val<T>(t:T) -> ByRefBare<T> { ByRefBare(t) }
    struct ByRefBare<T>(T);
    impl<T> ByRef<T> for ByRefBare<T> {
        fn get_ref<'a>(&'a self) -> &'a T { match self { &ByRefBare(ref x) => x } }
    }
}
