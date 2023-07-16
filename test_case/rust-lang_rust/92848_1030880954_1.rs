
error: internal compiler error: compiler/rustc_typeck/src/collect.rs:1089:14: impossible case reached

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.1 (db9d1b20b 2022-01-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [adt_def] computing ADT definition for `auth::AuthorizationError`
#1 [unsafety_check_result] unsafety-checking `http::encoding::error::handle_reject::{closure#0}`
end of query stack
error: could not compile `tomiko`
