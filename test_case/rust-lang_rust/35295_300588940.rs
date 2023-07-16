rust
pub struct FooA;
pub type FooB = FooA;

impl FooA {
        pub fn happy(&self) {}
}

pub struct Bar;
impl std::ops::Deref for Bar {
    type Target = FooB;
    fn deref(&self) -> &FooB { unimplemented!() }
}

