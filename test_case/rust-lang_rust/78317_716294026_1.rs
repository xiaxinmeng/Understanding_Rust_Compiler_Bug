Rust
pub struct Foo<T>(PhantomData<T>);
pub struct Bar<T>(PhantomData<T>);

mod foo_1 {
    pub struct A;
    impl Foo<Bar<A>> { fn foo() {} }
}
/* ... */
mod foo_n {
    pub struct A;
    impl Foo<Bar<A>> { fn foo() {} }
}
