 rust
#![feature(macro_rules)]

macro_rules! my_vec {
    ($value:expr, ..$repeat:expr) => ({
        use std::slice::BoxedSlice;
        let xs: ::std::boxed::Box<[_]> = box() [$value, ..$repeat];
        xs.into_vec()
    });
    ($($x:expr),*) => ({
        use std::slice::BoxedSlice;
        let xs: ::std::boxed::Box<[_]> = box() [$($x),*];
        xs.into_vec()
    });
}

fn main() {
    println!("{}", my_vec![0u, ..16]);
    println!("{}", my_vec![0u, 1]);
}
