rust
mod inner {
    pub struct SomeStruct;
    impl SomeStruct {
        pub fn new() -> SomeStruct { SomeStruct }
    }
    impl From<SomeStruct> for Vec<u8> {
        fn from(x: SomeStruct) -> Vec<u8> {
            Vec::new()
        }
    }
}
pub use inner::SomeStruct as MyStruct;
