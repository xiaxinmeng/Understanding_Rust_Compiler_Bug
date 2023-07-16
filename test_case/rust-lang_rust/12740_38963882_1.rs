 rust
pub type MyEnum = MyEnum::MyEnum; // expose the enum at this scope
pub mod MyEnum {
    // unfortunately the real enum has to be public for the parent module to see it
    // which means MyEnum::MyEnum is valid, despite being weird.
    pub enum MyEnum {
        VariantOne,
        VariantTwo
    }
    // add any impls of MyEnum here
}
