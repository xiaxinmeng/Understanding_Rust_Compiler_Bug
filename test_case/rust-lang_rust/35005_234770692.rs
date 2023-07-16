
mod outer {
    mod inner {
        pub(outer) struct Private; // or `pub(super)`
        pub(outer) fn interface() -> Private { // or `pub(super)`
            Private
        }
    }
    pub use self::inner::{interface}; // <- becomes an error
}
