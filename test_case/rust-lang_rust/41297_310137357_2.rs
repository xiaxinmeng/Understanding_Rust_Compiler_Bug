console
$ cargo run # in bar/
error: internal compiler error: /checkout/src/librustc_trans/collector.rs:657: Cannot create local trans-item for DefId { krate: CrateNum(11), node: DefIndex(9) => foo/c66423d::{{impl}}[0]::id[0] }

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.20.0-nightly (445077963 2017-06-20) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:478
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:355
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:365
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:549
   5: std::panicking::begin_panic
   6: rustc_errors::Handler::bug
   7: rustc::session::opt_span_bug_fmt::{{closure}}
   8: rustc::session::opt_span_bug_fmt
   9: rustc::session::bug_fmt
  10: rustc_trans::collector::should_trans_locally
  11: rustc_trans::collector::visit_instance_use
  12: <rustc_trans::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_terminator_kind
  13: rustc::mir::visit::Visitor::visit_mir
  14: rustc_trans::collector::collect_items_rec
  15: rustc_trans::collector::collect_items_rec
  16: rustc_trans::collector::collect_items_rec
  17: rustc_trans::collector::collect_items_rec
  18: rustc_trans::collector::collect_items_rec
  19: rustc_trans::collector::collect_items_rec
  20: rustc_trans::collector::collect_items_rec
  21: rustc_trans::collector::collect_items_rec
  22: rustc_trans::collector::collect_items_rec
  23: rustc_trans::collector::collect_items_rec
  24: rustc_trans::collector::collect_items_rec
  25: rustc_trans::collector::collect_items_rec
  26: rustc_trans::collector::collect_items_rec
  27: rustc_trans::base::collect_and_partition_translation_items::{{closure}}
  28: rustc_trans::base::trans_crate
  29: rustc_driver::driver::phase_4_translate_to_llvm
  30: rustc_driver::driver::compile_input::{{closure}}
  31: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}
  32: rustc_driver::driver::phase_3_run_analysis_passes
  33: rustc_driver::driver::compile_input
  34: rustc_driver::run_compiler

error: Could not compile `bar`.
