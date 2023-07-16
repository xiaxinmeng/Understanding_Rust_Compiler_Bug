
rustc main.rs
error[E0425]: cannot find function `abs_diff` in this scope
 --> main.rs:5:20
  |
5 |     println!("{}", abs_diff(10_usize, 10_usize));        // error
  |                    ^^^^^^^^ not found in this scope

error[E0658]: use of unstable library feature 'int_abs_diff'
 --> main.rs:3:29
  |
3 |     println!("{}", 10_usize.abs_diff(10_usize));         // OK
  |                             ^^^^^^^^
  |
  = note: see issue #89492 <https://github.com/rust-lang/rust/issues/89492> for more information

error[E0658]: use of unstable library feature 'int_abs_diff'
 --> main.rs:4:20
  |
4 |     println!("{}", usize::abs_diff(10_usize, 10_usize)); // OK
  |                    ^^^^^^^^^^^^^^^
  |
  = note: see issue #89492 <https://github.com/rust-lang/rust/issues/89492> for more information

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0425, E0658.
 
