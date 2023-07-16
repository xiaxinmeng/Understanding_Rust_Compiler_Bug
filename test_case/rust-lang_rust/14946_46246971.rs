 rust
macro_rules! rec_works(
    () => (0);
    ($a:expr) => ($a);
    ($a:expr, $($rest:tt),*) => (
        $a + rec_works!($($rest),*)
    );
)
