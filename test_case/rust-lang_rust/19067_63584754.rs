 rust
extern crate test;

pub struct X {
    x: [u8, ..1 << 10],
}

pub fn f() -> X {
    let mut x: X = unsafe { std::mem::uninitialized() };
    for i in range(0, x.x.len()) {
        x.x[i] = i as u8;
    }
    x
}

fn main() {
    let x = box f();
    test::black_box(&x);
}
