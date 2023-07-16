rust
#![feature(generic_const_exprs)]

fn foo<const N: usize>(_arr: [u64; N + 1]) where [u64; N + 1]: {}

fn main() {
  let arr = [5; 5];       
  foo(arr);
}
