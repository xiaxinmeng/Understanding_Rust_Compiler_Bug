
error[E0658]: the target feature `neon` is currently unstable
 --> neon.rs:3:18
  |
3 | #[target_feature(enable = "neon")]
  |                  ^^^^^^^^^^^^^^^
  |
  = note: see issue #44839 <https://github.com/rust-lang/rust/issues/44839> for more information

error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
