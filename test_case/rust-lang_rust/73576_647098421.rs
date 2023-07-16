rust
macro_rules! used_macro {
    ($($e:expr),*) => {};
    ($($e:expr),*,) => {}; // <-- This is unused.
}

fn main() {
    used_macro!(7, 13);
} 
