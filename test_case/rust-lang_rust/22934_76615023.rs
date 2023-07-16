 rust
macro_rules! foo {
    (<-) => ("yes");
    ($($x:tt)*) => ("no");
}

fn main() {
    println!("{}", foo!(< -));
}
