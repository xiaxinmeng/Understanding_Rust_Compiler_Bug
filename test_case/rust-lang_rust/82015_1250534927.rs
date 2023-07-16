
warning: trait objects without an explicit `dyn` are deprecated
 --> src/lib.rs:3:8
  |
3 |     x: Iterator<Item = i32>
  |        ^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(bare_trait_objects)]` on by default
  = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
  = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
  |
3 -     x: Iterator<Item = i32>
3 +     x: dyn Iterator<Item = i32>
  |

error[[E0204]](https://doc.rust-lang.org/stable/error-index.html#E0204): the trait `Copy` may not be implemented for this type
 --> src/lib.rs:1:17
  |
1 | #[derive(Clone, Copy)]
  |                 ^^^^
2 | struct Double {
3 |     x: Iterator<Item = i32>
  |     ----------------------- this field does not implement `Copy`
  |
  = note: this error originates in the derive macro `Copy` (in Nightly builds, run with -Z macro-backtrace for more info)
  