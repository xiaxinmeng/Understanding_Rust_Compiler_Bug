
error: internal compiler error: compiler/rustc_middle/src/ty/fold.rs:824:17: Trying to collect bound vars with a bound region: DebruijnIndex(0) BoundRegion { var: 0, kind: BrAnon(0) }

thread 'rustc' panicked at 'Box<Any>', /Users/user/rust4/library/std/src/panic.rs:59:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-dev running on x86_64-apple-darwin

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -Z symbol-mangling-version=v0 -C prefer-dynamic -C rpath -C debuginfo=0

query stack during panic:
#0 [symbol_name] computing the symbol for `test::<&dyn for<'r> std::ops::FnMut(&'r u8)>`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error
