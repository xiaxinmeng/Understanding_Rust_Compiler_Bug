 rust
#![feature(const_fn)]

const X : usize = {
    let mut sum = 0;
    for i in 0..5 {
        sum += i;
    }
    sum
};

#[allow(unused_variables)]
fn main() {
    let a : [i32; X];
}
