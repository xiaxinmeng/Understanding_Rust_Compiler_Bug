
$ cargo +nightly build
   Compiling autocfg v1.1.0
   Compiling matrixmultiply v0.1.15
   Compiling ndarray v0.12.1
   Compiling rawpointer v0.1.0
   Compiling either v1.6.1
   Compiling itertools v0.7.11
   Compiling num-traits v0.2.15
   Compiling num-complex v0.2.4
error[E0716]: temporary value dropped while borrowed
  --> /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/ndarray-0.12.1/src/layout/layoutfmt.rs:29:24
   |
25 |                        .format_with(" | ", |i, f| {
   |                                                - has type `&mut dyn FnMut(&'1 (dyn std::fmt::Display + '1)) -> Result<(), std::fmt::Error>`
...
29 |                     f(&format_args!("0x{:x}", i))
   |                     ---^^^^^^^^^^^^^^^^^^^^^^^^^-
   |                     |  |
   |                     |  creates a temporary which is freed while still in use
   |                     argument requires that borrow lasts for `'1`
30 |                 }
   |                 - temporary value is freed at the end of this statement
   |
   = note: this error originates in the macro `format_args` (in Nightly builds, run with -Z macro-backtrace for more info)
