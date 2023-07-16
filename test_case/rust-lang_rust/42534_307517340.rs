
#![feature(iterator_step_by, step_by)]

#[inline(never)]
pub fn first_foo(mut a: i32, b: i32) -> i32 {
    let mut tot = 0;

    while a < b {
        tot += a;
        a += 3;
    }
    tot
}

#[inline(never)]
pub fn second_foo(x: std::ops::Range<i32>) -> i32 {
    let mut tot = 0;

    #[allow(deprecated)]
    for a in x.step_by(3) {
        tot += a;
    }
    tot
}

#[inline(never)]
pub fn third_foo(mut x: std::ops::Range<i32>) -> i32 {
    let mut tot = 0;

    for a in Iterator::step_by(&mut x, 3) {
        tot += a;
    }
    tot
}

fn main() {
    println!("{}", first_foo(0, 1000));
    println!("{}", first_foo(0, 100000));
    println!("{}", second_foo(0 .. 1000));
    println!("{}", second_foo(0 .. 100000));
    println!("{}", third_foo(0 .. 1000));
    println!("{}", third_foo(0 .. 100000));
}
