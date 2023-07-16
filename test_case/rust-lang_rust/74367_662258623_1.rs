
#![feature(test)]
extern crate test;
#[inline]
pub fn pow_rust(x:i64, mut exp: u32) -> i64 {
    let mut base = x;
    let mut acc = 1;
    while exp > 1 {
        if (exp & 1) == 1 {
            acc = acc * base;
        }
        exp /= 2;
        base = base * base;
    }
    if exp == 1 {
        acc = acc * base;
    }
    acc
}
#[inline]
pub fn pow_new(x:i64, mut exp: u32) -> i64 {
    if exp==0{
        1
    }else{
        let mut base = x;
        let mut acc = 1;
        while exp > 1 {
            if (exp & 1) == 1 {
                acc = acc * base;
            }
            exp >>= 1;
            base = base * base;
        }
        acc * base
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn rust_xpow0(b: &mut Bencher) {
        b.iter(|| pow_rust(test::black_box(125233),test::black_box(0)));
    }
    #[bench]
    fn new_xpow0(b: &mut Bencher) {
        b.iter(|| pow_new(test::black_box(125233),test::black_box(0)));
    }

    #[bench]
    fn rust_xpow1(b: &mut Bencher) {
        b.iter(|| pow_rust(test::black_box(125233),test::black_box(1)));
    }
    #[bench]
    fn new_xpow1(b: &mut Bencher) {
        b.iter(|| pow_new(test::black_box(125233),test::black_box(1)));
    }

    #[bench]
    fn rust_xpow3(b: &mut Bencher) {
        b.iter(|| pow_rust(test::black_box(125233),test::black_box(3)));
    }
    #[bench]
    fn new_xpow3(b: &mut Bencher) {
        b.iter(|| pow_new(test::black_box(125233),test::black_box(3)));
    }
    #[bench]
    fn rust_xpow7(b: &mut Bencher) {
        b.iter(|| pow_rust(test::black_box(125233),test::black_box(7)));
    }
    #[bench]
    fn new_xpow7(b: &mut Bencher) {
        b.iter(|| pow_new(test::black_box(125233),test::black_box(7)));
    }
    #[bench]
    fn rust_xpow3123(b: &mut Bencher) {
        b.iter(|| pow_rust(test::black_box(125233),test::black_box(3123)));
    }
    #[bench]
    fn new_xpow3123(b: &mut Bencher) {
        b.iter(|| pow_new(test::black_box(125233),test::black_box(3123)));
    }
}
