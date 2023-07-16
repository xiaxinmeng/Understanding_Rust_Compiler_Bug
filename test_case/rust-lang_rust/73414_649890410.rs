
test time.rs - time::Duration::zero (line 145) ... ok
test unit.rs - unit::() (line 8) ... ok

failures:

---- slice/mod.rs - slice::[T]::strip_prefix (line 1468) stdout ----
error[E0308]: mismatched types
---- slice/mod.rs - slice::[T]::strip_suffix (line 1519) stdout ----
error[E0308]: mismatched types
 --> slice/mod.rs:1522:1
  |
6 | assert_eq!(v.strip_suffix(&[]), Some(v));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected slice `[{integer}]`, found array `[{integer}; 3]`
  |
  = note: expected enum `std::option::Option<&[{integer}]>`
             found enum `std::option::Option<&[{integer}; 3]>`
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.

failures:
    slice/mod.rs - slice::[T]::strip_prefix (line 1468)
    slice/mod.rs - slice::[T]::strip_prefix (line 1479)
    slice/mod.rs - slice::[T]::strip_suffix (line 1508)
    slice/mod.rs - slice::[T]::strip_suffix (line 1519)

test result: FAILED. 2600 passed; 4 failed; 28 ignored; 0 measured; 0 filtered out

