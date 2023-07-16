rust
#![allow(unreachable_code)]

macro_rules! cons {
    ($car:tt, $cdr:tt) => { ($car, $cdr) };
    ($($args:tt),*) => { compile_error!("Expected 2 arguments, got something else") };
}

macro_rules! count {
    () => { 0 };
    ($first:tt $($rest:tt)*) => { 1 + count!($($rest)*) };
}

fn main() {
    // works
    println!("{:?}", cons!(1, 2));
    
    // error
    println!("{:?}", cons!());
    println!("{:?}", cons!(1, 2, 3, 4));
}
