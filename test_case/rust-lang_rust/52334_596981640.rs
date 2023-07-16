rust

Standard Error

   Compiling playground v0.0.1 (/playground)
warning: `extern` block uses type `std::ffi::CStr`, which is not FFI-safe
 --> src/main.rs:3:18
  |
3 |     fn meh(blah: Foo);
  |                  ^^^ not FFI-safe
  |
  = note: `#[warn(improper_ctypes)]` on by default
  = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
  = note: this struct has unspecified layout

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc_target/abi/call/x86_64.rs:158:14
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
  12: rust_begin_unwind
             at src/libstd/panicking.rs:378
  13: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  14: core::panicking::panic
             at src/libcore/panicking.rs:52
  15: rustc_target::abi::call::x86_64::cast_target
  16: rustc_target::abi::call::x86_64::compute_abi_info::{{closure}}
  17: <rustc_target::abi::call::FnAbi<&rustc::ty::TyS> as rustc::ty::layout::FnAbiExt<C>>::adjust_for_abi
  18: <rustc_target::abi::call::FnAbi<&rustc::ty::TyS> as rustc::ty::layout::FnAbiExt<C>>::of_fn_ptr
  19: <rustc_target::abi::TyLayout<&rustc::ty::TyS> as rustc_codegen_llvm::type_of::LayoutLlvmExt>::llvm_type
  20: <rustc_target::abi::call::FnAbi<&rustc::ty::TyS> as rustc_codegen_llvm::abi::FnAbiLlvmExt>::llvm_type
  21: rustc_codegen_llvm::callee::get_fn
  22: rustc_codegen_ssa::mir::rvalue::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_rvalue_operand
  23: rustc_codegen_ssa::mir::block::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_block
  24: rustc_codegen_ssa::mir::codegen_mir
  25: <rustc::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::define
  26: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  27: rustc::dep_graph::graph::DepGraph::with_task
  28: rustc_codegen_llvm::base::compile_codegen_unit
  29: rustc_codegen_ssa::base::codegen_crate
  30: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  31: rustc_interface::passes::start_codegen
  32: rustc::ty::context::tls::enter_global
  33: rustc_interface::queries::Queries::ongoing_codegen
  34: rustc_interface::interface::run_compiler_in_existing_thread_pool
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.43.0-nightly (3dbade652 2020-03-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `playground`.

To learn more, run the command again with --verbose.
