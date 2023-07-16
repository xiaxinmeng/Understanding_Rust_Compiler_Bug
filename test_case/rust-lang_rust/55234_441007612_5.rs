
$ cargo check
   Compiling temp v0.1.0 (/tmp/temp.fCUY)
thread 'main' panicked at 'index out of bounds: the len is 4792 but the index is 4792', /rustc/0b9f19dff1347e29bf4362ab5a8fab84b43023b5/src/libcore/slice/mod.rs:2447:10
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at libstd/sys_common/backtrace.rs:59
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:480
   6: std::panicking::continue_panic_fmt
             at libstd/panicking.rs:390
   7: rust_begin_unwind
             at libstd/panicking.rs:325
   8: core::panicking::panic_fmt
             at libcore/panicking.rs:77
   9: core::panicking::panic_bounds_check
             at libcore/panicking.rs:59
  10: rustc::dep_graph::graph::DepGraph::node_color
  11: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_mark_green_and_read
  12: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  13: <rustc::ty::layout::LayoutCx<'tcx, rustc::ty::context::TyCtxt<'a, 'tcx, 'tcx>> as rustc_target::abi::LayoutOf>::layout_of
  14: <rustc_codegen_llvm::context::CodegenCx<'ll, 'tcx> as rustc_target::abi::LayoutOf>::layout_of
  15: rustc_codegen_llvm::callee::get_fn
  16: rustc_codegen_ssa::base::maybe_create_entry_wrapper::create_entry_fn
  17: rustc_codegen_ssa::base::maybe_create_entry_wrapper
  18: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  19: rustc::dep_graph::graph::DepGraph::with_task
  20: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::ExtraBackendMethods>::compile_codegen_unit
  21: rustc_codegen_ssa::base::codegen_crate
  22: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  23: rustc::util::common::time
  24: rustc_driver::driver::phase_4_codegen
  25: rustc_driver::driver::compile_input::{{closure}}
  26: rustc::ty::context::tls::enter_context
  27: <std::thread::local::LocalKey<T>>::with
  28: rustc::ty::context::TyCtxt::create_and_enter
  29: rustc_driver::driver::compile_input
  30: rustc_driver::run_compiler_with_pool
  31: <scoped_tls::ScopedKey<T>>::set
  32: rustc_driver::run_compiler
  33: syntax::with_globals
  34: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  35: rustc_driver::run
  36: rustc_driver::main
  37: std::rt::lang_start::{{closure}}
  38: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  39: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  40: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:398
             at libstd/rt.rs:58
  41: main
  42: __libc_start_main
  43: <unknown>
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.32.0-nightly (0b9f19dff 2018-11-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `temp`.

To learn more, run the command again with --verbose.
