 rust
macro_rules! cast_aspersions {
    ($expr:expr, various stuff here, $ty:ty) => ($expr as $ty)
}

fn main() {
    cast_aspersions!(2, various stuff here, ());
}
