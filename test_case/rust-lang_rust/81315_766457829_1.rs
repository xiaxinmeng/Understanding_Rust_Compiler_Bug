
   Compiling playground v0.0.1 (/playground)
thread 'rustc' panicked at 'assertion failed: promoted.is_none()', compiler/rustc_mir/src/transform/check_consts/qualifs.rs:250:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (4d0dd02ee 2021-01-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] borrow-checking `aa_storage`
#1 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `playground`

To learn more, run the command again with --verbose.
