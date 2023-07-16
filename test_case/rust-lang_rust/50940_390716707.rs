
$ RUST_BACKTRACE=1 cargo +nightly build
   Compiling rust50940 v0.1.0 (file:///tmp/rust50940)
thread 'main' panicked at 'unsupported integer: Reg { kind: Integer, size: Size { raw: 0 } }', librustc_target/abi/call/mod.rs:147:26
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:467
   6: std::panicking::begin_panic_fmt
             at libstd/panicking.rs:350
   7: rustc_target::abi::call::Reg::align
   8: <rustc_target::abi::call::ArgType<'tcx, &'tcx rustc::ty::TyS<'tcx>> as rustc_codegen_llvm::abi::ArgTypeExt<'a, 'tcx>>::store
   9: <rustc_target::abi::call::ArgType<'tcx, &'tcx rustc::ty::TyS<'tcx>> as rustc_codegen_llvm::abi::ArgTypeExt<'a, 'tcx>>::store_fn_arg
  10: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &'a mut F>::call_once
  11: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  12: rustc_codegen_llvm::mir::codegen_mir
  13: rustc_codegen_llvm::base::codegen_instance
  14: rustc_codegen_llvm::mono_item::MonoItemExt::define
  15: rustc_codegen_llvm::base::compile_codegen_unit
  16: rustc::ty::maps::<impl rustc::ty::maps::config::QueryConfig<'tcx> for rustc::ty::maps::queries::compile_codegen_unit<'tcx>>::compute
  17: rustc::ty::context::tls::with_context
  18: rustc::dep_graph::graph::DepGraph::with_task_impl
  19: rustc::ty::context::tls::with_related_context
  20: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  21: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  22: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::compile_codegen_unit
  23: rustc_codegen_llvm::base::codegen_crate
  24: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  25: rustc::util::common::time
  26: rustc_driver::driver::phase_4_codegen
  27: rustc_driver::driver::compile_input::{{closure}}
  28: rustc::ty::context::tls::enter_context
  29: <std::thread::local::LocalKey<T>>::with
  30: rustc::ty::context::TyCtxt::create_and_enter
  31: rustc_driver::driver::compile_input
  32: rustc_driver::run_compiler_with_pool
  33: syntax::with_globals
  34: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  35: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  36: rustc_driver::run
  37: rustc_driver::main
  38: std::rt::lang_start::{{closure}}
  39: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  40: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  41: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:374
             at libstd/rt.rs:58
  42: main
  43: __libc_start_main
  44: <unknown>
query stack during panic:
#0 [compile_codegen_unit] compile_codegen_unit
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0-nightly (a1d4a9503 2018-05-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `rust50940`.

To learn more, run the command again with --verbose.
