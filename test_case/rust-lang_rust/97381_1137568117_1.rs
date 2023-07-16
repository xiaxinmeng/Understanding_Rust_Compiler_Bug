`
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Type`,
 right: `Fn`', /rustc/285fc7d704fcdd7b2a37d475d04d5d955490e000/compiler/rustc_trait_selection/src/traits/util.rs:314:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.48.0-nightly (285fc7d70 2020-09-16) running on x86_64-unknown-linux-gnu
