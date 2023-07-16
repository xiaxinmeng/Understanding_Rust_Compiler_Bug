
% cat -n /tmp/foo.rs
     1  use std::num::Zero;
     2  use std::num::One;
     3  
     4  #[inline(always)]
     5  pub fn add2<I:Integer>(x: I) -> I {
     6      assert!(x >= Zero::zero());
     7      (x + One::one()) + One::one()
     8  }
% cat -n /tmp/bar.rs
     1  extern mod foo;
     2  use foo::*;
     3  
     4  fn main() {
     5      let four = add2(2);
     6      println(fmt!("2+2 == %d", four));
     7      let zero = add2(-2);
     8      println(fmt!("-2+2 == %d", zero));
     9  }
% rustc --lib /tmp/foo.rs && rustc -L/tmp /tmp/bar.rs && /tmp/bar
warning: missing crate link meta `name`, using `foo` as default
warning: missing crate link meta `vers`, using `0.0` as default
warning: no debug symbols in executable (-arch x86_64)
warning: no debug symbols in executable (-arch x86_64)
2+2 == 4
rust: task failed at 'assertion failed: x >= Zero::zero()', /tmp/foo.rs:6
rust: domain main @0x7f965c014c10 root task failed
