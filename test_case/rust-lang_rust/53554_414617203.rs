
thread 'main' panicked at 'assertion failed: !are_upstream_rust_objects_already_included(sess)', librustc_codegen_llvm/back/link.rs:1533:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: rustc_codegen_llvm::back::link::link_natively
   8: rustc_codegen_llvm::back::link::link_binary
   9: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::join_codegen_and_link::{{closure}}
  10: rustc::util::common::time
  11: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::join_codegen_and_link
  12: rustc_driver::driver::compile_input
  13: rustc_driver::run_compiler_with_pool
  14: syntax::with_globals
  15: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  16: __rust_maybe_catch_panic
  17: rustc_driver::run
  18: rustc_driver::main
  19: std::rt::lang_start::{{closure}}
  20: std::panicking::try::do_call
  21: __rust_maybe_catch_panic
  22: std::rt::lang_start_internal
  23: main
query stack during panic:
end of query stack
