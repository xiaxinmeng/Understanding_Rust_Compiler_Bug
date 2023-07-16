
thread 'rustc' panicked at 'assertion failed: i < this.fields.count()', /rustc/04caa632dd10c2bf64b69524c7f9c4c30a436877/compiler/rustc_middle/src/ty/layout.rs:2172:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (04caa632d 2021-01-30) running on x86_64-apple-darwin

note: compiler flags: -Z unstable-options -C opt-level=3 -C panic=abort -C embed-bitcode=no --crate-type staticlib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
