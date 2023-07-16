rust
pub fn nest<F: FnMut() -> R, R>(mut f: F) -> R {
    f()
}

macro_rules! nest {
    ($inner:expr) => (nest(|| {
        $inner
    }))
}

macro_rules! nests {
    () => (());
    ($_first:tt $($rest:tt)*) => (nest!(nests!($($rest)*)))
}
