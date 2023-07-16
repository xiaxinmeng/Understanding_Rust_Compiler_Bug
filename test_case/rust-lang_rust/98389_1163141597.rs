rust
#![feature(specialization)]

trait One {
    type Output;
}

impl<T> One for T {
    default type Output = i32;
}

trait Two {
    type Output;
}

impl Two for i32 {
    type Output = String;
}

type Whatever<T> = <<T as One>::Output as Two>::Output;

fn main() {
    let t: Whatever<Vec<i32>> = String::from("123");
}
