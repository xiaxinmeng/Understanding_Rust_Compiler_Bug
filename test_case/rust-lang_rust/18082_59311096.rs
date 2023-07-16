 rust
// opaque.rs
mod inaccessible {
    pub struct Struct;

    impl Struct {
        pub fn kaboom(self) {
            unimplemented!();
        }
    }
}

pub fn new_thing() -> inaccessible::Struct {
    inaccessible::Struct
}
