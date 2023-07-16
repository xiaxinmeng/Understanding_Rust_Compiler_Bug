rust
#![feature(test)]
extern crate test;

fn foo(x: i32, y: i32) -> i64 {
    (x + y) as i64
}

#[inline(never)]
fn bar() {
    let f = Box::new(0);
    let y: fn(i32, i32) -> i64 = test::black_box(foo);
    test::black_box(y(1, 2));
}

fn main() {
    bar();
}
