plain
---- src/ptr/mut_ptr.rs - ptr::mut_ptr::*mut[T]::split_at_mut (line 1485) stdout ----
error[E0308]: mismatched types
 --> src/ptr/mut_ptr.rs:1491:1
  |
9 | assert_eq!(left, [1, 0]);
  | ^^^^^^^^^^^^^^^^^^^^^^^^ expected *-ptr, found array `[{integer}; 2]`
  |
  = note: expected raw pointer `*mut [{integer}]`
                   found array `[{integer}; 2]`
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
  --> src/ptr/mut_ptr.rs:1492:1
   |
   |
10 | assert_eq!(right, [3, 0, 5, 6]);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected *-ptr, found array `[{integer}; 4]`
   |
   = note: expected raw pointer `*mut [{integer}]`
                    found array `[{integer}; 4]`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
