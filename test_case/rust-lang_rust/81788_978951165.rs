
thread 'rustc' panicked at 'assertion failed: matches!(ty.kind(), ty :: Param(_))', compiler/rustc_const_eval/src/interpret/util.rs:55:37
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-dev running on aarch64-apple-darwin

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z polymorphize=on -Z mir-opt-level=3 -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0

query stack during panic:
#0 [eval_to_allocation_raw] const-evaluating + checking `foo::promoted[0]`
#1 [eval_to_const_value_raw] simplifying constant for the type system `foo::promoted[0]`
end of query stack
