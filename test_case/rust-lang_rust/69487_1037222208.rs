
error[E0601]: `main` function not found in crate `fuzz_input`
 --> fuzz_input.rs:1:1
  |
1 | / struct Bug {
2 | |     A: [(); {
3 | |         let x: usize;
4 | |         x
5 | |     }],
6 | | }
  | |_^ consider adding a `main` function to `fuzz_input.rs`

error[E0381]: use of possibly-uninitialized variable: `x`
 --> fuzz_input.rs:4:9
  |
4 |         x
  |         ^ use of possibly-uninitialized `x`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0381, E0601.
For more information about an error, try `rustc --explain E0381`.

