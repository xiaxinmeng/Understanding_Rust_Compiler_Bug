rust
#![no_std]
#![crate_type = "lib"]

pub mod xxx {
    pub enum Foo {
        TupleVrnt(u8),
        Vrnt { baz: i32 },
        Vrnt2 { quux: f32 },
    }

    pub struct SFoo {
        pub sbaz: (),
    }
}
