plain
........................................................................................ 2728/3995
........................................................................................ 2816/3995
........................................................................................ 2904/3995
........................................................................................ 2992/3995
.................................F.F.................................................... 3080/3995
........................................................................................ 3256/3995
........................................................................................ 3344/3995
........................................................................................ 3432/3995
........................................................................................ 3520/3995
---
---- src/option.rs - option::Option<T>::as_mut_slice (line 836) stdout ----
error[E0308]: mismatched types
 --> src/option.rs:839:1
  |
6 | assert_eq!(Some(123).as_mut_slice().first_mut(), &mut Some(123))
  | |
  | expected enum `Option`, found mutable reference
  | expected enum `Option`, found mutable reference
  | help: you can convert from `&Option<T>` to `Option<&T>` using `.as_ref()`: `*right_val.as_ref()`
  |
  = note:           expected enum `Option<&mut {integer}>`
          found mutable reference `&mut Option<{integer}>`
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
Couldn't compile the test.
---- src/option.rs - option::Option<T>::as_slice (line 777) stdout ----
error[E0308]: mismatched types
 --> src/option.rs:781:5
  |
7 |     assert_eq!(i, i.as_slice().first());
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `u16`, found `&u16`
  = note: expected enum `Option<u16>`
             found enum `Option<&u16>`
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
help: use `Option::copied` to copy the value inside the `Option`
  |
  |
40|                 if !(*left_val == *right_val.copied()) {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
    src/option.rs - option::Option<T>::as_slice (line 777)

test result: FAILED. 3956 passed; 2 failed; 37 ignored; 0 measured; 0 filtered out; finished in 50.72s

error: doctest failed, to rerun pass `-p core --doc`
