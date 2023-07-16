
% rustc +nightly f33.rs -C incremental=true
error: internal compiler error: compiler/rustc_middle/src/ich/impls_ty.rs:94:17: StableHasher: unexpected region '_#0r

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1034:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (e3b1c12be 2021-08-02) running on x86_64-apple-darwin

note: compiler flags: -C incremental

query stack during panic:
#0 [check_impl_item_well_formed] checking that `<impl at f33.rs:10:1: 13:2>::M` is well-formed
#1 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
