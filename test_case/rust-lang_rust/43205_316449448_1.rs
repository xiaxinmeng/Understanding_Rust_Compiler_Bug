
error: internal compiler error: src/librustc_trans/mir/lvalue.rs:301: using operand local _2 as lvalue

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.20.0-dev (cdd8dc67b 2017-07-18) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:489:8
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:71
             at src/libstd/sys_common/backtrace.rs:60
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:380
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:390
   4: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:611
   5: std::panicking::begin_panic_new
             at /home/mark/BuildDisk/rust/src/libstd/panicking.rs:553
   6: rustc_errors::Handler::bug
             at src/librustc_errors/lib.rs:489
   7: <std::thread::local::LocalKey<T>>::try_with
             at src/librustc/session/mod.rs:859
             at src/librustc/ty/context.rs:985
             at src/librustc/ty/context.rs:974
             at /home/mark/BuildDisk/rust/src/libstd/thread/local.rs:429
   8: <std::thread::local::LocalKey<T>>::with
             at /home/mark/BuildDisk/rust/src/libstd/thread/local.rs:343
   9: rustc::ty::context::tls::with
             at src/librustc/ty/context.rs:970
  10: rustc::ty::context::tls::with_opt
             at src/librustc/ty/context.rs:985
  11: rustc::session::opt_span_bug_fmt
             at src/librustc/session/mod.rs:855
  12: rustc::session::bug_fmt
             at src/librustc/session/mod.rs:839
  13: rustc_trans::mir::lvalue::<impl rustc_trans::mir::MirContext<'a, 'tcx>>::trans_lvalue
             at src/librustc_trans/mir/lvalue.rs:301
  14: rustc_trans::mir::rvalue::<impl rustc_trans::mir::MirContext<'a, 'tcx>>::trans_rvalue_operand
             at src/librustc_trans/mir/rvalue.rs:354
  15: rustc_trans::mir::statement::<impl rustc_trans::mir::MirContext<'a, 'tcx>>::trans_statement
             at src/librustc_trans/mir/statement.rs:38
  16: rustc_trans::mir::block::<impl rustc_trans::mir::MirContext<'a, 'tcx>>::trans_block
             at src/librustc_trans/mir/block.rs:49
  17: rustc_trans::mir::trans_mir
             at src/librustc_trans/mir/mod.rs:324
  18: rustc_trans::base::trans_instance
             at src/librustc_trans/base.rs:607
  19: rustc_trans::trans_item::TransItem::define
             at src/librustc_trans/trans_item.rs:90
  20: rustc_trans::base::trans_crate::module_translation
             at src/librustc_trans/base.rs:1190
  21: rustc::dep_graph::graph::DepGraph::with_task
             at /home/mark/BuildDisk/rust/src/librustc/dep_graph/graph.rs:125
  22: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &'a mut F>::call_once
             at src/librustc_trans/base.rs:1124
             at /home/mark/BuildDisk/rust/src/libcore/ops/function.rs:191
  23: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::spec_extend
             at /home/mark/BuildDisk/rust/src/libcore/option.rs:398
             at /home/mark/BuildDisk/rust/src/libcore/iter/mod.rs:1073
             at /home/mark/BuildDisk/rust/src/liballoc/vec.rs:1825
  24: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
             at /home/mark/BuildDisk/rust/src/liballoc/vec.rs:1808
  25: rustc_trans::base::trans_crate
             at /home/mark/BuildDisk/rust/src/liballoc/vec.rs:1695
             at /home/mark/BuildDisk/rust/src/libcore/iter/iterator.rs:1303
             at src/librustc_trans/base.rs:1119
  26: rustc::util::common::time
             at src/librustc_driver/driver.rs:1069
             at /home/mark/BuildDisk/rust/src/librustc/util/common.rs:48
  27: rustc_driver::driver::phase_4_translate_to_llvm
             at src/librustc_driver/driver.rs:1067
  28: rustc_driver::driver::compile_input::{{closure}}
             at src/librustc_driver/driver.rs:211
  29: <std::thread::local::LocalKey<T>>::try_with
             at src/librustc_driver/driver.rs:1049
             at /home/mark/BuildDisk/rust/src/librustc/ty/context.rs:958
             at /home/mark/BuildDisk/rust/src/libstd/thread/local.rs:429
  30: <std::thread::local::LocalKey<T>>::with
             at /home/mark/BuildDisk/rust/src/libstd/thread/local.rs:343
  31: rustc::ty::context::tls::enter
             at /home/mark/BuildDisk/rust/src/librustc/ty/context.rs:955
  32: <std::thread::local::LocalKey<T>>::try_with
             at /home/mark/BuildDisk/rust/src/librustc/ty/context.rs:942
             at /home/mark/BuildDisk/rust/src/libstd/thread/local.rs:429
  33: <std::thread::local::LocalKey<T>>::with
             at /home/mark/BuildDisk/rust/src/libstd/thread/local.rs:343
  34: rustc::ty::context::tls::enter_global
             at /home/mark/BuildDisk/rust/src/librustc/ty/context.rs:939
  35: rustc::ty::context::TyCtxt::create_and_enter
             at /home/mark/BuildDisk/rust/src/librustc/ty/context.rs:720
  36: rustc_driver::driver::phase_3_run_analysis_passes
             at src/librustc_driver/driver.rs:962
  37: rustc_driver::driver::compile_input
             at src/librustc_driver/driver.rs:177
  38: rustc_driver::run_compiler
             at src/librustc_driver/lib.rs:218
