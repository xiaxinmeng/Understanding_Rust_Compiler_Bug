
tcargo v0.1.0 (/home/pi/src/tcargo)
thread 'rustc' panicked at 'index out of bounds: the len is 18 but the index is 56', compiler/rustc_metadata/src/rmeta/decoder.rs:328:61
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0 (53cb7b09b 2021-06-17) running on armv7-unknown-linux-gnueabihf

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `tcargo`

To learn more, run the command again with --verbose.
