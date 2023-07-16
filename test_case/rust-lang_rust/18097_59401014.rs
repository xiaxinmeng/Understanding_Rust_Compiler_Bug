
extern crate test;
use test::Bencher;

#[deriving(Clone)]
struct Foo { x: u8  }

impl Drop for Foo {
    fn drop(&mut self) { }
}

#[bench]
fn bench_no_drop(b: &mut Bencher) {
    let v = Vec::from_elem(10000, 0u8);
    b.iter(|| { let mut x = v.clone(); x.truncate(0); });
}

#[bench]
fn bench_drop(b: &mut Bencher) {
    let v = Vec::from_elem(10000, Foo { x: 0 });
    b.iter(|| { let mut x = v.clone(); x.truncate(0); });
}
