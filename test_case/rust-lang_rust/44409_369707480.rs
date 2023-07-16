
error[E0623]: lifetime mismatch
  --> src/test/compile-fail/regions-free-region-ordering-caller.rs:18:12
   |
17 | fn call2<'a, 'b>(a: &'a usize, b: &'b usize) {
   |                     ---------     ---------
   |                     |
   |                     these two types are declared with different lifetimes...
18 |     let z: Option<&'b &'a usize> = None;//~ ERROR E0623
   |            ^^^^^^^^^^^^^^^^^^^^^ ...but it is required for `a` to outlive `b` here
   |
help: you can mark `a` as outliving `b` by adding a lifetime bound
   |
17 | fn call2<'a: 'b, 'b>(a: &'a usize, b: &'b usize) {
   |          ^^^^^^
