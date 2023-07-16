 rust
impl ::std::cmp::PartialEq for Foo {
    #[inline]
    fn eq(&self, __arg_0: &Foo) -> ::bool {
        fn eq<Sized? T: PartialEq>(a: &T, b: &T) { a.eq(b) } // add this function
        match *__arg_0 {
            Foo(ref __self_1_0) =>
            match *self {
                Foo(ref __self_0_0) =>
                true && eq(__self_0_0, __self_1_0), // modify function call here
            },
        }
    }
    // ne method omitted for brevity
}
