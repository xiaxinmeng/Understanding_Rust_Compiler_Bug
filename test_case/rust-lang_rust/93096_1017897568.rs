
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/02072b482a8b5357f7fb5e5637444ae30e423c40/compiler/rustc_hir/src/definitions.rs:452:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0 (02072b482 2022-01-11) running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `impl core::future::future::Future<Output = core::result::Result<alloc::boxed::Box<(dyn warp::reply::Reply + 'static)>, core::convert::Infallible>>: core::marker::Send`  |  = note: this failure-note originates in the macro `mk_route` (in Nightly builds, run with -Z macro-backtrace for more info)

#1 [typeck] type-checking `api::run`
end of query stack
