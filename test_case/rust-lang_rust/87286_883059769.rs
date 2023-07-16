
cargo +nightly b
   Compiling tcargo v0.1.0 (/home/pi/src/tcargo)
thread 'rustc' panicked at 'called `set` on feature `` which is not `active`', compiler/rustc_feature/src/active.rs:100:18
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (59216858a 2021-07-18) running on armv7-unknown-linux-gnueabihf

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `tcargo`
