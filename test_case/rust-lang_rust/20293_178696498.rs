
#![feature(test)]

extern crate test;
fn main() {
    let mut sum = 0;
    for i in 0..10 { // `s<CR>` lands on this line twice
        sum += i;
    }
    test::black_box(sum);
}
