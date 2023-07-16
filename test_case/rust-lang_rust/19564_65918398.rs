 rust
#![feature(unboxed_closures)]

fn main() {
    let mut arr: Vec<&Fn(i32) -> i32> = Vec::new();
    let f = |&: x: i32| x * 4;
    arr.push(&f);
    arr[0].call((1, ));
}
