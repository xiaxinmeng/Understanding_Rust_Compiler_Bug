 rust
#![feature(test)]
#![crate_type="lib"]

extern crate test;

struct Big {
  large: [u64; 100000],
}

pub fn test_func() {
  let x = Big {
    large: [0; 100000],
  };
  test::black_box(x);
}
