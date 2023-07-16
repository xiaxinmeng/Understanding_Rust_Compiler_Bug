rust
pub mod app {
    pub struct S;

    impl S {
        /**
          Doc comment hello! [`Foo::by_name`](`crate::enums::Foo::by_name`).
        */
        pub fn whatever(&self) {}
    }
}

pub mod enums {
    pub enum Foo {
        Bar,
    }

    impl Foo {
        pub fn by_name(&self) {}
    }
}
