 rust
macro_rules! foo {
    ($($x:tt)*) => (println!($($x)*));
}

fn main() {
    foo!("Hello, {}!", "world");
}
