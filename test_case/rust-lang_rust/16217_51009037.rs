 Rust
pub mod Gsl {
    pub struct SomeStruct {
        ...
    }

    impl SomeStruct {
        pub fn however() {
            ::ffi::some_function();
        }
    }
}
