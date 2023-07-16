 rust
#![feature(macro_rules)]

macro_rules! expr {
    ($e: expr) => { $e }
}
macro_rules! vec {
    ($($contents: tt)*) => {
       {
            use std::slice::BoxedSlice;
            use std::boxed::HEAP;
            let xs = expr!(box (HEAP) [$($contents)*]);
            xs.into_vec()
        }
    }
}

fn main() {
    println!("{}", vec![1u, 2, 3]);
    println!("{}", vec![1u, .. 10]);
}
