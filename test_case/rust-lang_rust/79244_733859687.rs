
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/tools/clippy/clippy_lints/src/methods/mod.rs:3904:68
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.0.212 (21dea46 2020-11-18)

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
warning: 280 warnings emitted
