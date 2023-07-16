
# rustup run beta rustc test.rs
error[E0554]: #[feature] may not be used on the beta release channel
 --> test.rs:1:1
  |
1 | #![feature(core_intrinsics)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

# set -x RUSTC_BOOTSTRAP '1'
# rustup run beta rustc test.rs
