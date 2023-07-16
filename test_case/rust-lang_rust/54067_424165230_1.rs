
% rustc ice.rs
error: internal compiler error: librustc_codegen_llvm/mir/operand.rs:144: not immediate: OperandRef(Pair((i64:  %5 = load i64, i64* %4, align 8), (i64:  %7 = load i64, i64* %6, align 8)) @ TyLayout { ty: Foo, details: LayoutDetails { variants: Single { index: 0 }, fields: Arbitrary { offsets: [Size { raw: 0 }, Size { raw: 8 }], memory_index: [0, 1] }, abi: ScalarPair(Scalar { value: Int(I64, false), valid_range: 0..=18446744073709551615 }, Scalar { value: Int(I64, false), valid_range: 0..=18446744073709551615 }), align: Align { abi_pow2: 3, pref_pow2: 3 }, size: Size { raw: 16 } } })

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:587:9
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
             at libstd/panicking.rs:481
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  15: rustc_codegen_llvm::mir::codegen_mir
  16: rustc_codegen_llvm::base::codegen_instance
  17: rustc_codegen_llvm::mono_item::MonoItemExt::define
  18: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  19: rustc::dep_graph::graph::DepGraph::with_task
  20: rustc_codegen_llvm::base::codegen_crate
  21: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  22: rustc::util::common::time
  23: rustc_driver::driver::phase_4_codegen
  24: rustc_driver::driver::compile_input::{{closure}}
  25: rustc::ty::context::tls::enter_context
  26: <std::thread::local::LocalKey<T>>::with
  27: rustc::ty::context::TyCtxt::create_and_enter
  28: rustc_driver::driver::compile_input
  29: rustc_driver::run_compiler_with_pool
  30: <scoped_tls::ScopedKey<T>>::set
  31: rustc_driver::run_compiler
  32: syntax::with_globals
  33: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:103
  34: rustc_driver::run
  35: rustc_driver::main
  36: std::rt::lang_start::{{closure}}
  37: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  38: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:103
  39: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at libstd/rt.rs:58
  40: main
  41: __libc_start_main
  42: <unknown>
query stack during panic:
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.30.0-nightly (2f1547c0a 2018-09-11) running on x86_64-unknown-linux-gnu
