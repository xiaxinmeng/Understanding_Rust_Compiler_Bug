rust
; cat src/main.rs
#![deny(warnings)]
const C: usize = 1;

fn main() {
    core::mem::discriminant(&1);
}
; rustc src/main.rs
; rustc src/main.rs -Wwarnings                
error: constant `C` is never used
 --> src/main.rs:3:7
  |
3 | const C: usize = 1;
  |       ^
  |
note: the lint level is defined here
 --> src/main.rs:1:9
  |
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(dead_code)]` implied by `#[deny(warnings)]`

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
 --> src/main.rs:6:5
  |
6 |     core::mem::discriminant(&1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `i32`, which is not an enum.
 --> src/main.rs:6:29
  |
6 |     core::mem::discriminant(&1);
  |                             ^^
  = note: `#[deny(enum_intrinsics_non_enums)]` on by default
