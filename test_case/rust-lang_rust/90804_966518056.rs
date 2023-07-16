
   Compiling unsizer v0.1.0 (/Users/username/unsizer)
error[E0554]: `#![feature]` may not be used on the stable release channel
 --> src/main.rs:1:1
  |
1 | #![feature(unsize)]
  | ^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/compiler/rustc_middle/src/ty/subst.rs:303:43
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.1 (59eed8a2a 2021-11-01) running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `&'static (): std::marker::Unsize`
#1 [typeck] type-checking `main`
end of query stack
For more information about this error, try `rustc --explain E0554`.
error: could not compile `unsizer` due to previous error
