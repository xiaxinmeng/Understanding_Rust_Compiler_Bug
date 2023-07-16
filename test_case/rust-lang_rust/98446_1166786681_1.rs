
error: `#[derive]` can't be used on a `#[repr(packed)]` struct with type or const parameters (error E0133)
 --> f.rs:8:10
  |
8 | #[derive(Hash)]
  |          ^^^^
  |
  = note: `#[deny(unaligned_references)]` on by default
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
  = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)
