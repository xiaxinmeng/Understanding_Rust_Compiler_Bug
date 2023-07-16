Rust
pub struct Foo<T>(PhantomData<T>);

mod foo_1 {
    pub struct A;
    impl Foo<A> { fn foo() {} }
}
/* ... */
mod foo_n {
    pub struct A;
    impl Foo<A> { fn foo() {} }
}
