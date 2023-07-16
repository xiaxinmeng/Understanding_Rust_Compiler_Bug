 rust
#![feature(test)]
#![crate_type="lib"]

extern crate test;
struct Big {
  large: [u64; 100000],
}


#[inline(never)]
fn foo() -> Big {
    Big {
        large: [123; 100000],
    }
}

pub fn test_func() {
  let x = foo();
  test::black_box(x);
}
