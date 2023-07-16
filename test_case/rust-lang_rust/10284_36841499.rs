 rust
#[crate_id="10284"];

pub mod foo {
    pub struct Bar;
}

pub mod bar {
    impl ::foo::Bar {
        pub fn baz(&self) {}
    }
}
