console
warning: use of deprecated module `std::raw`: use pointer metadata APIs instead https://github.com/rust-lang/rust/issues/81513
 --> src/lib.rs:1:5
  |
1 | use core::raw;
  |     ^^^^^^^^^
  |
  = note: `#[warn(deprecated)]` on by default

error[E0658]: use of unstable library feature 'raw'
 --> src/lib.rs:1:5
  |
1 | use core::raw;
  |     ^^^^^^^^^
  |
  = note: see issue #27751 <https://github.com/rust-lang/rust/issues/27751> for more information

warning: use of deprecated module `std::raw`: use pointer metadata APIs instead https://github.com/rust-lang/rust/issues/81513
 --> src/lib.rs:2:5
  |
2 | use std::raw;
  |     ^^^^^^^^

error[E0658]: use of unstable library feature 'raw'
 --> src/lib.rs:2:5
  |
2 | use std::raw;
  |     ^^^^^^^^
  |
  = note: see issue #27751 <https://github.com/rust-lang/rust/issues/27751> for more information
