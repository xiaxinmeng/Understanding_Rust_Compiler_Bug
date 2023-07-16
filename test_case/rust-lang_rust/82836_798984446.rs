
git clone https://gitlab.com/ootoo1/objrs-min.git
cd objrs-min/frameworks/foundation/
cargo build
    Updating crates.io index
  Downloaded syn v1.0.64
  Downloaded 1 crate (231.5 KB) in 0.48s
   Compiling proc-macro2 v1.0.24
   Compiling unicode-xid v0.2.1
   Compiling syn v1.0.64
   Compiling libc v0.2.88
   Compiling quote v1.0.9
   Compiling objrs_macros v0.0.3-dev (/Users/ootoo/tmp/objrs-min/macros)
   Compiling objrs v0.0.3-dev (/Users/ootoo/tmp/objrs-min)
   Compiling objrs_frameworks_foundation v0.0.3-dev (/Users/ootoo/tmp/objrs-min/frameworks/foundation)
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: Unknown(T)', compiler/rustc_lint/src/types.rs:802:52
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (caca2121f 2021-03-05) running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [lint_mod] linting top-level module
#1 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `objrs_frameworks_foundation`

To learn more, run the command again with --verbose.
