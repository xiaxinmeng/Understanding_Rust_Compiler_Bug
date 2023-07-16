
error[E0601]: `main` function not found in crate `fuzz_input`
  --> fuzz_input.rs:1:1
   |
1  | / #![feature(const_trait_impl)]
2  | | #![feature(fn_traits)]
3  | | #![feature(unboxed_closures)]
4  | |
...  |
16 | |     V(T),
17 | | }
   | |_^ consider adding a `main` function to `fuzz_input.rs`

error: internal compiler error: rust/compiler/rustc_middle/src/ty/layout.rs:3020:21: argument to function with "rust-call" ABI is not a tuple

thread 'rustc' panicked at 'Box<dyn Any>', rust/compiler/rustc_errors/src/lib.rs:1160:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
