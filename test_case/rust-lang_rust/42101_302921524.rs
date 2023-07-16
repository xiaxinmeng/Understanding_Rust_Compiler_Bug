
<proc-macro source code>")', src/librustc_incremental/persist/hash.rs:254
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
   1: std::panicking::default_hook::{{closure}}
   2: std::panicking::default_hook
   3: std::panicking::rust_panic_with_hook
   4: std::panicking::begin_panic
   5: std::panicking::begin_panic_fmt
   6: rustc_incremental::persist::hash::HashContext::load_data
   7: rustc_incremental::persist::hash::HashContext::hash
   8: rustc_incremental::persist::save::save_dep_graph
   9: rustc_driver::driver::phase_4_translate_to_llvm
  10: rustc_driver::driver::compile_input::{{closure}}
  11: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}
  12: rustc_driver::driver::phase_3_run_analysis_passes
  13: rustc_driver::driver::compile_input
  14: rustc_driver::run_compiler
