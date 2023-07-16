
 stderr:
------------------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:354:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  left: `Binder(<[u8; _] as std::default::Default>)`,
 right: `Binder(<[u8; 1] as std::default::Default>)`', compiler/rustc_trait_selection/src/traits/codegen.rs:30:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.49.0-nightly (726888479 2020-10-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

query stack during panic:
#0 [codegen_fulfill_obligation] checking if `std::default::Default` fulfills its obligations
#1 [resolve_instance] resolving instance `<[u8; _] as std::default::Default>::default`
end of query stack

------------------------------------------



failures:
    [ui] ui/const-generics/issue-73298.rs#full
