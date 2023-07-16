Rust
macro_rules! len {
    () => { 0 };
    ($item:literal) => { 1 };
    ($item:literal, $($extras:literal),*) => { 1 + len!($($extras),*); }
}

macro_rules! slice {
    ($($item:literal),*) => {
        const SIZED: [&str; len!($($item),*)] = [$($item),*];
    }
}

slice!["first", "second", "third", "last"];

fn main() {
    println!("{:?}", SIZED);
}
