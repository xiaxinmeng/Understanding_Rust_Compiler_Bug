 rust
#[crate_id="10284"];

pub mod bar {
    impl ::foo::Zar {
        pub fn baz(&self) {}
    }
}

pub mod foo {
    pub struct Zar;
}
