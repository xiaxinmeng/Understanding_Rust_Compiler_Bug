rust
error[E0308]: mismatched types
  --> /Users/nlroscoemeow/rust/src/test/ui/macros/assert-macro-owned.rs:6:20
   |
LL |     assert!(false, "test-assert-owned".to_string());
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                    |
   |                    expected `&str`, found struct `String`
   |                    help: consider borrowing here: `&"test-assert-owned".to_string()`
