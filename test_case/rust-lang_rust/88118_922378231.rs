
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: PlaceBuilder { base: Upvar { var_hir_id: HirId { owner: DefId(0:1099 ~ rustc_errors[2427]::{impl#11}::print_error_count), local_id: 4 }, closure_def_id: DefId(0:1100 ~ rustc_errors[2427]::{impl#11}::print_error_count::{closure#0}), closure_kind: FnMut }, projection: [Deref] }', compiler/rustc_mir_build/src/build/expr/as_place.rs:306:69
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-beta.1 (e784c962e 2021-09-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z unstable-options -Z binary-dep-depinfo -Z unstable-options -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_built] building MIR for `<impl at compiler/rustc_errors/src/lib.rs:917:1: 1190:2>::print_error_count::{closure#0}`
#1 [unsafety_check_result] unsafety-checking `<impl at compiler/rustc_errors/src/lib.rs:917:1: 1190:2>::print_error_count::{closure#0}`
