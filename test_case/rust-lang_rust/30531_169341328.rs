 rust
macro_rules! cell {
    ($style:ident : $value:expr) => { 1 };
    ($value:expr) => { 0 };
}

fn main() {
    println!("{}", cell!(t : T));
    println!("{}", cell!(42));
}
