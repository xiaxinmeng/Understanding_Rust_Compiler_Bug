 rust
#![crate_name="repr"]
#![crate_type="rlib"]

pub use link::Link;

mod link {
    use error::Error;
    pub struct Link;
    impl Link {
        pub fn error(&self) -> Error { Error }
    }
}

mod error {
    use std::default::Default;
    pub struct Error;
    impl Error {
        pub fn produce<T : Default>(self) -> (T, uint) {
            static LINK_ERROR : uint = 0u;
            (Default::default(), LINK_ERROR)
        }
    }
}
