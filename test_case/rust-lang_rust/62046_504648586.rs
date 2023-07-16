rust
error: internal compiler error: src/librustc_codegen_ssa/mir/operand.rs:128: not immediate: OperandRef(Pair(([0 x i8]*:  %5 = load [0 x i8]*, [0 x i8]** %4, align 8, !dbg !20, !nonnull !4), (i64:  %7 = load i64, i64* %6, align 8, !dbg !20)) @ TyLayout { ty: &str, details: LayoutDetails { variants: Single { index: 0 }, fields: Arbitrary { offsets: [Size { raw: 0 }, Size { raw: 8 }], memory_index: [0, 1] }, abi: ScalarPair(Scalar { value: Pointer, valid_range: 1..=18446744073709551615 }, Scalar { value: Int(I64, false), valid_range: 0..=18446744073709551615 }), align: AbiAndPrefAlign { abi: Align { pow2: 3 }, pref: Align { pow2: 3 } }, size: Size { raw: 16 } } })

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:212
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:479
   8: std::panicking::begin_panic
   9: rustc_errors::Handler::bug
  10: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  11: rustc::ty::context::tls::with_opt::{{closure}}
  12: rustc::ty::context::tls::with_context_opt
  13: rustc::ty::context::tls::with_opt
  14: rustc::util::bug::opt_span_bug_fmt
  15: rustc::util::bug::bug_fmt
  16: rustc_codegen_llvm::asm::<impl rustc_codegen_ssa::traits::asm::AsmBuilderMethods for rustc_codegen_llvm::builder::Builder>::codegen_inline_asm
  17: rustc_codegen_ssa::mir::codegen_mir
  18: rustc_codegen_ssa::base::codegen_instance
  19: <rustc::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::define
  20: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  21: rustc::dep_graph::graph::DepGraph::with_task
  22: rustc_codegen_llvm::base::compile_codegen_unit
  23: rustc_codegen_ssa::base::codegen_crate
  24: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  25: rustc::util::common::time
  26: rustc_interface::passes::start_codegen
  27: rustc::ty::context::tls::enter_global
  28: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  29: rustc_interface::passes::create_global_ctxt::{{closure}}
  30: rustc_interface::passes::BoxedGlobalCtxt::enter
  31: rustc_interface::queries::Query<T>::compute
  32: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  33: rustc_interface::interface::run_compiler_in_existing_thread_pool
  34: std::thread::local::LocalKey<T>::with
  35: scoped_tls::ScopedKey<T>::set
  36: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.37.0-nightly (b25ee6449 2019-06-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
