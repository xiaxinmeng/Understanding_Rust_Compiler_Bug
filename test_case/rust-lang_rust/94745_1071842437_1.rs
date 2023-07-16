rust
pub fn is_foo_enabled() -> bool {
    #[cfg(feature = "foo")] {
        true
    }
    #[cfg(not(feature = "foo"))] {
        false
    }
}
