 rust
macro_rules! path(
    ($($t:tt),+) => (&Path::new(format!($($t),+)))
)
