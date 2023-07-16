rust
#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![deny(unused_mut)]

fn main() {
   ...
   // issue 43526
   let mut x43526 = &mut 5; //~ ERROR: variable does not need to be mutable
   *x43526 = 4;

   // include 30280 here as well
}

// issue 25049, reduced
fn foo(mut buf: &mut [u8]) { //~ ERROR: variable does not need to be mutable
    &mut buf[..];
}

...
