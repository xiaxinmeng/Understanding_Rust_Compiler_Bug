 rust
// ORIGINAL CODE:
// test simple_format ... bench:        77 ns/iter (+/- 2)

// MODIFIED CODE:
// test simple_format ... bench:        54 ns/iter (+/- 6)

#![feature(test)]
extern crate test;

#[bench]
fn simple_format(b: &mut test::Bencher) {
    b.iter(|| {
        test::black_box(format!("Hello World"));
    });
}
