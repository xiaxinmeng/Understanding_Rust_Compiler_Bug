rust
macro_rules! print_token {
    ($x:tt) => { println!("{}", stringify!($x)) }
}

fn main() {
    print_token!("hello"_);
}
