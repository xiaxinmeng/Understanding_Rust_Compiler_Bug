
pub mod foo {
    mod bar { ... }
    pub type Foo = bar::Foo<i32>;
    impl Foo {
        pub fn baz() { ... }
    }
}
