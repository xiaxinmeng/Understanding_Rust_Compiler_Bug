
error: internal compiler error: compiler/rustc_mir_build/src/build/matches/test.rs:69:35: or-patterns should have already been handled

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:945:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.48.0-nightly (7f7a1cbfd 2020-09-27) running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental -C target-cpu=haswell --crate-type lib

note: some of the compiler flags provided by cargo are hidden
