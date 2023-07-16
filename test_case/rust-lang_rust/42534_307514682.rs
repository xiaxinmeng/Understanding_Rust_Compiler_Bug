rust
#![feature(iterator_step_by)]
pub fn foo(x: ::std::ops::Range<i32>) -> i32 {
    let mut sum = 0;
    for a in x.step_by(3) {
        sum += a;
    }
    return sum;
}
