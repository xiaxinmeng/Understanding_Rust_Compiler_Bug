
cargo fix --edition --allow-dirty
    Checking rustfix_error v0.1.0 (C:\Users\Markus\Dropbox\Programming\rustfix_error)
warning: failed to automatically apply fixes suggested by rustc to crate `rustfix_error`

after fixes were automatically applied the compiler reported errors within these files:

  * src\main.rs

This likely indicates a bug in either rustc or cargo itself,
and we would appreciate a bug report! You're likely to see
a number of compiler warnings after this message which cargo
attempted to fix but failed. If you could open an issue at
https://github.com/rust-lang/cargo/issues
quoting the full output of this command we'd be very appreciative!

warning: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
 --> src\main.rs:6:27
  |
6 | #[serde(serialize_state = "::Test")]
  |                           ^^^^^^^^ help: use `crate`: `crate::"::Test"`
  |
  = note: `-W absolute-paths-not-starting-with-crate` implied by `-W rust-2018-compatibility`
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
  = note: for more information, see issue #53130 <https://github.com/rust-lang/rust/issues/53130>

warning: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
 --> src\main.rs:6:27
  |
6 | #[serde(serialize_state = "::Test")]
  |                           ^^^^^^^^ help: use `crate`: `crate::"::Test"`
  |
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
  = note: for more information, see issue #53130 <https://github.com/rust-lang/rust/issues/53130>

warning: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
 --> src\main.rs:6:27
  |
6 | #[serde(serialize_state = "::Test")]
  |                           ^^^^^^^^ help: use `crate`: `crate::"::Test"`
  |
  = note: `-W absolute-paths-not-starting-with-crate` implied by `-W rust-2018-compatibility`
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
  = note: for more information, see issue #53130 <https://github.com/rust-lang/rust/issues/53130>

warning: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
 --> src\main.rs:6:27
  |
6 | #[serde(serialize_state = "::Test")]
  |                           ^^^^^^^^ help: use `crate`: `crate::"::Test"`
  |
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
  = note: for more information, see issue #53130 <https://github.com/rust-lang/rust/issues/53130>

