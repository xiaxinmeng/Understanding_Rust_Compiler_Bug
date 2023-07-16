
Compiling syn v0.11.11 (/tmp/.tmpnHQAja) ...

thread 'rustc' panicked at 'found unstable fingerprints for evaluate_obligation(e3352ed64d6e2ccd-c82ee1c6b3ce2c20): Ok(EvaluatedToOk)', /rustc/b8719c51e0e44483cff9b6975a830f6e51812a48/compiler/rustc_query_system/src/query/plumbing.rs:593:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (b8719c51e 2021-03-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z self-profile=/tmp/.tmpnHQAja/self-profile-output -C opt-level=3 -C embed-bitcode=no -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `quote::Tokens: std::marker::Unpin`
#1 [is_unpin_raw] computing whether `quote::Tokens` is `Unpin`
end of query stack
warning: 49 warnings emitted

thread 'main' panicked at 'assertion failed: status.success()', collector/src/rustc-fake.rs:76:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: could not compile `syn`
