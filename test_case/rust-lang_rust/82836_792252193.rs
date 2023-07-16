
warning: the feature `min_const_generics` has been stable since 1.51.0 and no longer requires an attribute to enable
 --> macros/src/lib.rs:6:73
  |
6 | #![feature(proc_macro_diagnostic, proc_macro_span, proc_macro_def_site, min_const_generics)]
  |                                                                         ^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(stable_features)]` on by default

warning: 1 warning emitted

   Compiling objrs_frameworks_foundation_macros v0.0.3-dev (/Users/ootoo/tmp/objrs/frameworks/foundation/macros)
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> src/lib.rs:13:3
   |
13 |   specialization,
   |   ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete

warning: 1 warning emitted

   Compiling objrs_frameworks_foundation v0.0.3-dev (/Users/ootoo/tmp/objrs/frameworks/foundation)
warning: field is never read: `isa`
  --> frameworks/foundation/src/nsobject.rs:22:3
   |
22 |   isa: *mut objrs::Class,
   |   ^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: Unknown(T)', compiler/rustc_lint/src/types.rs:802:52
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (caca2121f 2021-03-05) running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [lint_mod] linting module `nsarray`
#1 [analysis] running analysis passes on this crate
end of query stack
warning: 1 warning emitted

error: could not compile `objrs_frameworks_foundation`

To learn more, run the command again with --verbose.
