rust
impl<T> Foo<T> { // <- generic parameter is here
    #[no_mangle] fn foo() {} // <- no generic parameter here, but the function may be instantiated multiple times
}
