rust
macro_rules! test {
    ($($t: ident)**) => {
        println!("{}", stringify!($($t)**));
    }
}

fn main() {
    let (a, b, c) = (1, 2, 3);
    println!("{}", test!(a * b * c));
}
