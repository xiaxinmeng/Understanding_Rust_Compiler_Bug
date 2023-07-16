rust
macro_rules! used_macro {
    ($($e:expr),*$(,)?) => {}; // Only one used arm.
}

fn main() {
    used_macro!(7, 13); // Without the comma.
    used_macro!(
        7,
        13, // With the comma.
        );
}
