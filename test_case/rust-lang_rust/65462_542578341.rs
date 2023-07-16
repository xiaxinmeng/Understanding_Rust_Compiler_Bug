
thread 'rustc' panicked at 'Tried to access field 0 of union with 0 fields', src/librustc_target/abi/mod.rs:742:17
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
             at src/libstd/panicking.rs:189
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:206
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:473
  12: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:376
  13: std::panicking::begin_panic_fmt
             at src/libstd/panicking.rs:331
  14: rustc_target::abi::FieldPlacement::offset
  15: rustc_codegen_ssa::mir::place::PlaceRef<V>::project_field
  16: rustc_codegen_ssa::mir::place::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_place
  17: rustc_codegen_ssa::mir::codegen_mir
  18: rustc_codegen_ssa::base::codegen_instance
  19: <rustc::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::define
  20: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  21: rustc::dep_graph::graph::DepGraph::with_task
  22: rustc_codegen_llvm::base::compile_codegen_unit
  23: rustc_codegen_ssa::base::codegen_crate
  24: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  25: rustc_interface::passes::start_codegen::{{closure}}
  26: rustc::util::common::time
  27: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  28: rustc_interface::passes::create_global_ctxt::{{closure}}
  29: rustc_interface::queries::Query<T>::compute
  30: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  31: rustc_interface::interface::run_compiler_in_existing_thread_pool
  32: std::thread::local::LocalKey<T>::with
  33: scoped_tls::ScopedKey<T>::set
  34: syntax::with_globals
