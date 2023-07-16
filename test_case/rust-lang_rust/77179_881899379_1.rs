
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: UnresolvedIntTy(_#0i)', compiler\rustc_typeck\src\check\writeback.rs:499:75
stack backtrace:
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (74ef0c3e4 2021-07-16) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `main`
#1 [type_of] computing type of `Pointer::{opaque#0}`
#2 [check_mod_item_types] checking item types in top-level module
#3 [analysis] running analysis passes on this crate
end of query stack
For more information about this error, try `rustc --explain E0658`.
