
% cargo --version
cargo 1.35.0-nightly (95b45eca1 2019-03-06)
% rustc --version
rustc 1.35.0-nightly (9d71ec135 2019-03-10)
% cargo new demo --edition 2015
     Created binary (application) `demo` package
% cat >> demo/src/main.rs
macro_rules! m {
    () => {
        Box<dyn Iterator<Item = u32>>
    }
}

pub fn foo() -> m!() {
    Box::new(None.into_iter())
}
^D
% cd demo
% cargo build
   Compiling demo v0.1.0 (/home/pnkfelix/Dev/Mozilla/issue59065/demo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
% cargo fix --edition
    Checking demo v0.1.0 (/home/pnkfelix/Dev/Mozilla/issue59065/demo)
warning: failed to automatically apply fixes suggested by rustc to crate `demo`

after fixes were automatically applied the compiler reported errors within these files:

  * src/main.rs

This likely indicates a bug in either rustc or cargo itself,
and we would appreciate a bug report! You're likely to see
a number of compiler warnings after this message which cargo
attempted to fix but failed. If you could open an issue at
https://github.com/rust-lang/rust/issues
quoting the full output of this command we'd be very appreciative!
Note that you may be able to make some more progress in the near-term
fixing code with the `--broken-code` flag

The following errors were reported:
error: expected one of `!`, `(`, `+`, `,`, `::`, `<`, or `>`, found `Iterator`
  --> src/main.rs:6:19
   |
6  |         Box<r#dyn Iterator<Item = u32>>
   |                   ^^^^^^^^ expected one of 7 possible tokens here
...
10 | pub fn foo() -> m!() {
   |                 ---- in this macro invocation

error: aborting due to previous error

Original diagnostics will follow.

warning: `dyn` is a keyword in the 2018 edition
 --> src/main.rs:6:13
  |
6 |         Box<dyn Iterator<Item = u32>>
  |             ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
  |
  = note: `-W keyword-idents` implied by `-W rust-2018-compatibility`
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
  = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>

warning: `dyn` is a keyword in the 2018 edition
 --> src/main.rs:6:13
  |
6 |         Box<dyn Iterator<Item = u32>>
  |             ^^^ help: you can use a raw identifier to stay compatible: `r#dyn`
  |
  = note: `-W keyword-idents` implied by `-W rust-2018-compatibility`
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
  = note: for more information, see issue #49716 <https://github.com/rust-lang/rust/issues/49716>

    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
