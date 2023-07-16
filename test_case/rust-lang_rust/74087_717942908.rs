rust
#[macro_export]
#[rustfmt::skip]
macro_rules! m {
    () => {};
}

#[macro_export]
macro_rules! other {
    () => {
        $crate::m!();
    }
}

other!();
