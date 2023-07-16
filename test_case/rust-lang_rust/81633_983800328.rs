rust
pub mod foo {
    pub struct Bar(Baz);
    impl std::ops::Deref for Bar {
        type Target = Baz;
        fn deref(&self) -> &Baz { &self.0 }
    }

    pub struct Baz {}
    impl Baz {
        /// [str]
        pub fn foo(&self) {
            str!()
        }
    }
}

mod bar {
    macro_rules! str {() => {}}
}
