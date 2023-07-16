
error: internal compiler error: src/librustc_traits/normalize_erasing_regions.rs:34: could not fully normalize `&mut <usize as std::slice::SliceIndex<[std::string::String]>>::Output`

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:873:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1052
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1426
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:204
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:224
  10: rustc_driver::report_ice
  11: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
             at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447/src/liballoc/boxed.rs:1029
  12: proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}
             at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447/src/libproc_macro/bridge/client.rs:305
  13: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:476
  14: std::panicking::begin_panic
  15: rustc_errors::HandlerInner::bug
  16: rustc_errors::Handler::bug
  17: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  18: rustc::ty::context::tls::with_opt::{{closure}}
  19: rustc::ty::context::tls::with_opt
  20: rustc::util::bug::opt_span_bug_fmt
  21: rustc::util::bug::bug_fmt
  22: rustc::ty::context::GlobalCtxt::enter_local
  23: rustc_traits::normalize_erasing_regions::normalize_ty_after_erasing_regions
  24: rustc::ty::query::__query_compute::normalize_ty_after_erasing_regions
  25: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::normalize_ty_after_erasing_regions>::compute
  26: rustc::dep_graph::graph::DepGraph::with_task_impl
  27: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  28: <rustc::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc::ty::fold::TypeFolder>::fold_ty
  29: rustc::ty::structural_impls::fold_list
  30: rustc::ty::normalize_erasing_regions::<impl rustc::ty::context::TyCtxt>::normalize_erasing_late_bound_regions
  31: <rustc_target::abi::call::FnAbi<&rustc::ty::TyS> as rustc::ty::layout::FnAbiExt<C>>::of_instance
  32: rustc_codegen_ssa::mir::block::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_terminator
  33: rustc_codegen_ssa::mir::codegen_mir
  34: <rustc::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::define
  35: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  36: rustc::dep_graph::graph::DepGraph::with_task
  37: rustc_codegen_llvm::base::compile_codegen_unit
  38: rustc_codegen_ssa::base::codegen_crate
  39: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  40: rustc_session::utils::<impl rustc_session::session::Session>::time
  41: rustc_interface::passes::QueryContext::enter
  42: rustc_interface::queries::Queries::ongoing_codegen
  43: rustc_interface::interface::run_compiler_in_existing_thread_pool
  44: scoped_tls::ScopedKey<T>::set
  45: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0 (b8cedc004 2020-03-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [normalize_ty_after_erasing_regions] normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: &mut <usize as std::slice::SliceIndex<[std::string::String]>>::Output }`
end of query stack
