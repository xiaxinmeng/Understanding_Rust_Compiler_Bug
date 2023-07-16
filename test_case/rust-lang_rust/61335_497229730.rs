
   Compiling playground v0.0.1 (/playground)
error: internal compiler error: src/librustc/ty/layout.rs:328: univariant: field #3 of `[static generator@src/main.rs:11:35: 15:2 x:std::boxed::Box<dyn std::fmt::Display> for<'r, 's> {std::boxed::Box<(dyn std::fmt::Display + 'r)>, (dyn std::fmt::Display + 's), std::future::GenFuture<[static generator@src/main.rs:9:17: 9:19 {}]>, ()}]` comes after unsized field

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:637:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.25/src/backtrace/libunwind.rs:97
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.25/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:197
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:211
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:478
   8: std::panicking::begin_panic
   9: rustc_errors::Handler::bug
  10: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  11: rustc::ty::context::tls::with_opt::{{closure}}
  12: rustc::ty::context::tls::with_context_opt
  13: rustc::ty::context::tls::with_opt
  14: rustc::util::bug::opt_span_bug_fmt
  15: rustc::util::bug::bug_fmt
  16: rustc::ty::layout::LayoutCx<rustc::ty::context::TyCtxt>::layout_raw_uncached::{{closure}}
  17: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once
  18: <&mut I as core::iter::traits::iterator::Iterator>::next
  19: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  20: rustc::ty::layout::LayoutCx<rustc::ty::context::TyCtxt>::layout_raw_uncached
  21: rustc::ty::layout::layout_raw
  22: rustc::ty::query::__query_compute::layout_raw
  23: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::layout_raw>::compute
  24: rustc::dep_graph::graph::DepGraph::with_task_impl
  25: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  26: <rustc::ty::layout::LayoutCx<rustc::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of
  27: <rustc_mir::transform::const_prop::ConstPropagator as rustc::mir::visit::MutVisitor>::visit_statement
  28: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
  29: rustc_mir::transform::run_passes::{{closure}}
  30: rustc_mir::transform::run_passes
  31: rustc_mir::transform::optimized_mir
  32: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute
  33: rustc::dep_graph::graph::DepGraph::with_task_impl
  34: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  35: rustc_mir::monomorphize::collector::collect_items_rec
  36: rustc_mir::monomorphize::collector::collect_items_rec
  37: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
  38: rustc::util::common::time
  39: rustc_mir::monomorphize::collector::collect_crate_mono_items
  40: rustc::util::common::time
  41: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  42: rustc::ty::query::__query_compute::collect_and_partition_mono_items
  43: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_and_partition_mono_items>::compute
  44: rustc::dep_graph::graph::DepGraph::with_task_impl
  45: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  46: rustc_codegen_ssa::base::codegen_crate
  47: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  48: rustc::util::common::time
  49: rustc_interface::passes::start_codegen
  50: rustc::ty::context::tls::enter_global
  51: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  52: rustc_interface::passes::create_global_ctxt::{{closure}}
  53: rustc_interface::passes::BoxedGlobalCtxt::enter
  54: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  55: rustc_interface::interface::run_compiler_in_existing_thread_pool
  56: std::thread::local::LocalKey<T>::with
  57: scoped_tls::ScopedKey<T>::set
  58: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [layout_raw] computing layout of `[static generator@src/main.rs:11:35: 15:2 x:std::boxed::Box<dyn std::fmt::Display> for<'r, 's> {std::boxed::Box<(dyn std::fmt::Display + 'r)>, (dyn std::fmt::Display + 's), std::future::GenFuture<[static generator@src/main.rs:9:17: 9:19 {}]>, ()}]`
#1 [optimized_mir] processing `foo`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.37.0-nightly (37d001e4d 2019-05-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin
