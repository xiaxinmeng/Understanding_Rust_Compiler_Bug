
error: `Self` cannot be a raw identifier
 --> src/lib.rs:5:5
  |
5 |     r#Self
  |     ^^^^^^

thread 'rustc' panicked at '`Self` cannot be a raw identifier', compiler/rustc_expand/src/proc_macro_server.rs:336:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: custom attribute panicked
 --> src/lib.rs:3:1
  |
3 | #[macros::this_panics]
  | ^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: message: `Self` cannot be a raw identifier

error: aborting due to 2 previous errors

error: could not compile `crate1`
