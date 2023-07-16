rust
#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
#[collapse_debuginfo] //                             <- this is new
macro_rules! assert {
    ($cond:expr) => ( /* ... */ );
    ($cond:expr, $($arg:tt)+) => ( /* ... */ );
}
