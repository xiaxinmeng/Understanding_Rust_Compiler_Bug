
error: internal compiler error: src/librustc_trans/mir/operand.rs:111: not immediate: OperandRef(Pair((i8*:i8* getelementptr inbounds ([0 x i8], [0 x i8]* @str.0, i32 0, i32 0)), (i64:i64 0)) @ &str)

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:473
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
...
  13: rustc_trans::mir::operand::OperandRef::immediate
             at /rust/src/librustc_trans/mir/operand.rs:111
  14: core::ops::impls::<impl core::ops::FnOnce<A> for &'a mut F>::call_once
             at /rust/src/librustc_trans/mir/statement.rs:83
             at /rust/src/libcore/ops.rs:2710
  15: <collections::vec::Vec<T> as collections::vec::SpecExtend<T, I>>::spec_extend
             at /rust/src/libcore/option.rs:392
             at /rust/src/libcore/iter/mod.rs:1011
             at /rust/src/libcollections/vec.rs:1760
  16: <collections::vec::Vec<T> as collections::vec::SpecExtend<T, I>>::from_iter
             at /rust/src/libcollections/vec.rs:1743
  17: rustc_trans::mir::statement::<impl rustc_trans::mir::MirContext<'a, 'tcx>>::trans_statement
             at /rust/src/libcollections/vec.rs:1630
             at /rust/src/libcore/iter/iterator.rs:1222
             at /rust/src/librustc_trans/mir/statement.rs:82
  18: rustc_trans::mir::block::<impl rustc_trans::mir::MirContext<'a, 'tcx>>::trans_block
             at /rust/src/librustc_trans/mir/block.rs:109
  19: rustc_trans::mir::trans_mir
             at /rust/src/librustc_trans/mir/mod.rs:330
  20: rustc_trans::base::trans_instance
             at /rust/src/librustc_trans/base.rs:606
  21: rustc_trans::trans_item::TransItem::define
             at /rust/src/librustc_trans/trans_item.rs:104
  22: rustc_trans::base::trans_crate::module_translation
             at /rust/src/librustc_trans/base.rs:1182
  23: rustc::dep_graph::graph::DepGraph::with_task
             at /rust/src/librustc/dep_graph/graph.rs:111
  24: core::ops::impls::<impl core::ops::FnOnce<A> for &'a mut F>::call_once
             at /rust/src/librustc_trans/base.rs:1116
             at /rust/src/libcore/ops.rs:2710
  25: <collections::vec::Vec<T> as collections::vec::SpecExtend<T, I>>::spec_extend
             at /rust/src/libcore/option.rs:392
             at /rust/src/libcore/iter/mod.rs:1011
             at /rust/src/libcollections/vec.rs:1760
  26: <collections::vec::Vec<T> as collections::vec::SpecExtend<T, I>>::from_iter
             at /rust/src/libcollections/vec.rs:1743
  27: rustc_trans::base::trans_crate
             at /rust/src/libcollections/vec.rs:1630
             at /rust/src/libcore/iter/iterator.rs:1222
             at /rust/src/librustc_trans/base.rs:1111
  28: rustc::util::common::time
             at /rust/src/librustc_driver/driver.rs:1051
             at /rust/src/librustc/util/common.rs:48
  29: rustc_driver::driver::phase_4_translate_to_llvm
             at /rust/src/librustc_driver/driver.rs:1049
  30: rustc_driver::driver::compile_input::{{closure}}
             at /rust/src/librustc_driver/driver.rs:206
  31: <std::thread::local::LocalKey<T>>::with
             at /rust/src/librustc_driver/driver.rs:1032
             at /rust/src/librustc/ty/context.rs:915
             at /rust/src/libstd/thread/local.rs:253
  32: rustc::ty::context::tls::enter
             at /rust/src/librustc/ty/context.rs:912
  33: <std::thread::local::LocalKey<T>>::with
             at /rust/src/librustc/ty/context.rs:899
             at /rust/src/libstd/thread/local.rs:253
  34: rustc::ty::context::tls::enter_global
             at /rust/src/librustc/ty/context.rs:896
  35: rustc::ty::context::TyCtxt::create_and_enter
             at /rust/src/librustc/ty/context.rs:706
  36: rustc_driver::driver::phase_3_run_analysis_passes
             at /rust/src/librustc_driver/driver.rs:940
  37: rustc_driver::driver::compile_input
             at /rust/src/librustc_driver/driver.rs:172
  38: rustc_driver::run_compiler
             at /rust/src/librustc_driver/lib.rs:224
