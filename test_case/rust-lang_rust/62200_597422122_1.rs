
   Compiling playground v0.0.1 (/playground)
error: internal compiler error: src/librustc_infer/traits/codegen/mod.rs:61: Encountered error `OutputTypeParameterMismatch(Binder(<[closure@src/main.rs:23:9: 23:14] as std::ops::Fn<(<S as T<'_>>::A,)>>), Binder(<[closure@src/main.rs:23:9: 23:14] as std::ops::Fn<(i32,)>>), Sorts(ExpectedFound { expected: i32, found: <S as T<'_>>::A }))` selecting `Binder(<[closure@src/main.rs:23:9: 23:14] as std::ops::Fn<(i32,)>>)` during codegen

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:875:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1063
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1428
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:204
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:224
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:474
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc::ty::context::tls::with_opt::{{closure}}
  17: rustc::ty::context::tls::with_opt
  18: rustc::util::bug::opt_span_bug_fmt
  19: rustc::util::bug::bug_fmt
  20: rustc::ty::context::GlobalCtxt::enter_local
  21: rustc_infer::traits::codegen::codegen_fulfill_obligation
  22: rustc::ty::query::__query_compute::codegen_fulfill_obligation
  23: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::codegen_fulfill_obligation>::compute
  24: rustc::dep_graph::graph::DepGraph::with_task_impl
  25: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  26: rustc_ty::instance::resolve_instance
  27: rustc::ty::instance::Instance::resolve
  28: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc::mir::visit::Visitor>::visit_terminator_kind
  29: rustc_mir::monomorphize::collector::collect_items_rec
  30: rustc_mir::monomorphize::collector::collect_items_rec
  31: rustc_mir::monomorphize::collector::collect_items_rec
  32: rustc_session::utils::<impl rustc_session::session::Session>::time
  33: rustc_mir::monomorphize::collector::collect_crate_mono_items
  34: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  35: rustc::ty::query::__query_compute::collect_and_partition_mono_items
  36: rustc::dep_graph::graph::DepGraph::with_task_impl
  37: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  38: rustc_codegen_ssa::base::codegen_crate
  39: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  40: rustc_interface::passes::start_codegen
  41: rustc::ty::context::tls::enter_global
  42: rustc_interface::queries::Queries::ongoing_codegen
  43: rustc_interface::interface::run_compiler_in_existing_thread_pool
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.43.0-nightly (3dbade652 2020-03-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [codegen_fulfill_obligation] checking if `std::ops::Fn` fulfills its obligations
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error

error: could not compile `playground`.

To learn more, run the command again with --verbose.
