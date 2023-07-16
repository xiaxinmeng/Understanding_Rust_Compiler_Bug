rust
#![feature(proc_macro)]
extern crate my_macro;

pub use my_macro::my_macro;

macro_rules! m {
    ($($arg:expr),*) => (
        $crate::my_macro!($crate $(, $arg)*)
    )
}

fn main() {
    let x = 1usize;
    let y = m!(&x);
    println!("{}", y);
}
