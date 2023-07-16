 rust
macro_rules! twice_one(
    ($b:expr) => ( (twice!($b) + 1) );
)
