 rust
#![feature(macro_rules)]

macro_rules! expr {
    ($e:expr) => {
        $e
    }
}

macro_rules! foo(
    ($op:tt) => {
        expr!(10i $op 20i)
    }
)

fn main() {
    println!("{}", foo!(+));
    println!("{}", foo!(*));
}
