rust
macro_rules! foo {
    ($x:pat) => { "pattern" };
    ($x:expr) => { "expr" };
}

fn main() {
    println!("{}", foo!((4)));
}
