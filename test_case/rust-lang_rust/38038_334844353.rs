rust
#![feature(test)]
extern crate test;

/// this is a simplified version of std::iter::Chain
pub struct Chain2<I> { a: I, b: Option<I> }

fn chain2<I>(a: I, b: I) -> Chain2<I> where I: Iterator {
    Chain2 { a: a, b: Some(b) }
}

impl<I> Iterator for Chain2<I> where I: Iterator {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.a.next() {
            return Some(item);
        }
        if let Some(b) = self.b.take() {
            self.a = b;
        }
        self.a.next()
    }
}

fn init() -> [u8; 1024] {
    let mut x: [u8; 1024] = unsafe { std::mem::zeroed() };
    for i in 0..x.len() {
        x[i] = i as u8;
    }
    x
}

#[bench]
fn unrolled(b: &mut test::Bencher) {
    let x = init();
    let y = init();
    b.iter(|| {
        for &v in x.iter() {
            test::black_box(v);
        }
        for &v in y.iter() {
            test::black_box(v);
        }
    });
}

#[bench]
fn chained(b: &mut test::Bencher) {
    let x = init();
    let y = init();
    b.iter(|| {
        for &v in x.iter().chain(y.iter()) {
            test::black_box(v);
        }
    });
}

#[bench]
fn chained2(b: &mut test::Bencher) {
    let x = init();
    let y = init();
    b.iter(|| {
        for &v in chain2(x.iter(), y.iter()) {
            test::black_box(v);
        }
    });
}
