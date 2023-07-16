
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_middle/src/hir/map/mod.rs:178:30
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.48.0-nightly (9b4154193 2020-09-14) running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental -C linker=clang --crate-type lib

note: some of the compiler flags provided by cargo are hidden
