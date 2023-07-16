rust
#[doc(hidden)]
#[macro_export]
macro_rules! internal {
    () => { };
}

#[macro_export]
macro_rules! public {
    () => { $crate::internal!() };
}
