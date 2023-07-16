rust
#[cfg(feature = "nightly")]
macro_rules! default_fn {
	($($x:tt)*) => {
        default $($x)*
    }
}
#[cfg(not(feature = "nightly"))]
macro_rules! default_fn {
	($($x:tt)*) => {
        $($x)*
    }
}
