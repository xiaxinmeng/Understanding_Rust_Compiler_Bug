
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/ff0e14829e1806ca0d4226595f7fdf3e8658758f\compiler\rustc_hir\src\definitions.rs:452:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (ff0e14829 2021-10-31) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `ph`
