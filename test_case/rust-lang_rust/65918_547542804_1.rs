text
error: internal compiler error: src/librustc_traits/normalize_erasing_regions.rs:42: could not fully normalize `fn(std::result::Result<T, <T as MyFrom<Phantom2<DummyT::<U>>>>::Error>) -> std::option::Option<T> {std::result::Result::<T, <T as MyFrom<Phantom2<DummyT::<U>>>>::Error>::ok}`

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:925:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:61
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1028
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:65
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:50
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:188
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:205
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:468
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc::ty::context::tls::with_opt::{{closure}}
  17: rustc::ty::context::tls::with_context_opt
  18: rustc::ty::context::tls::with_opt
  19: rustc::util::bug::opt_span_bug_fmt
  20: rustc::util::bug::bug_fmt
  21: rustc::ty::context::GlobalCtxt::enter_local
  22: rustc_traits::normalize_erasing_regions::normalize_ty_after_erasing_regions
  23: rustc::ty::query::__query_compute::normalize_ty_after_erasing_regions
  24: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::normalize_ty_after_erasing_regions>::compute
  25: rustc::dep_graph::graph::DepGraph::with_task_impl
  26: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  27: rustc::traits::query::normalize_erasing_regions::<impl rustc::ty::context::TyCtxt>::normalize_erasing_regions
  28: <rustc::ty::layout::LayoutCx<rustc::ty::query::TyCtxtAt> as rustc_target::abi::LayoutOf>::layout_of
  29: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_const_to_op
  30: rustc_mir::transform::const_prop::ConstPropagator::eval_constant
  31: <rustc_mir::transform::const_prop::ConstPropagator as rustc::mir::visit::MutVisitor>::visit_terminator
  32: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
  33: rustc_mir::transform::run_passes
  34: rustc_mir::transform::run_optimization_passes
  35: rustc_mir::transform::optimized_mir
  36: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute
  37: rustc::dep_graph::graph::DepGraph::with_task_impl
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  39: rustc_mir::monomorphize::collector::collect_items_rec
  40: rustc_mir::monomorphize::collector::collect_items_rec
  41: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
  42: rustc::util::common::time
  43: rustc_mir::monomorphize::collector::collect_crate_mono_items
  44: rustc::util::common::time
  45: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  46: rustc::ty::query::__query_compute::collect_and_partition_mono_items
  47: rustc::dep_graph::graph::DepGraph::with_task_impl
  48: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  49: rustc_codegen_ssa::base::codegen_crate
  50: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  51: rustc_interface::passes::start_codegen::{{closure}}
  52: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  53: rustc_interface::passes::create_global_ctxt::{{closure}}
  54: rustc_interface::queries::Query<T>::compute
  55: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  56: rustc_interface::interface::run_compiler_in_existing_thread_pool
  57: std::thread::local::LocalKey<T>::with
  58: scoped_tls::ScopedKey<T>::set
  59: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (b497e1899 2019-10-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [normalize_ty_after_erasing_regions] normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [Binder(TraitPredicate(<U as std::marker::Sized>)), Binder(TraitPredicate(<T as MyFrom<Phantom2<DummyT::<U>>>>)), Binder(TraitPredicate(<T as std::marker::Sized>))], reveal: All, def_id: None }, value: fn(std::result::Result<T, <T as MyFrom<Phantom2<DummyT::<U>>>>::Error>) -> std::option::Option<T> {std::result::Result::<T, <T as MyFrom<Phantom2<DummyT::<U>>>>::Error>::ok} }`
#1 [optimized_mir] processing `<Scope<U> as MyIndex<Phantom1<T>>>::my_index`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error

error: could not compile `playground`.
