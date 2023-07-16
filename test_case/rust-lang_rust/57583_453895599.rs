
  thread 'rustc' panicked at 'assertion failed: position <= slice.len()', src/libserialize/leb128.rs:76:1
  note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
  stack backtrace:
     0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
               at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
     1: std::sys_common::backtrace::_print
               at src/libstd/sys_common/backtrace.rs:70
     2: std::panicking::default_hook::{{closure}}
               at src/libstd/sys_common/backtrace.rs:58
               at src/libstd/panicking.rs:200
     3: std::panicking::default_hook
               at src/libstd/panicking.rs:215
     4: rustc::util::common::panic_hook
     5: std::panicking::rust_panic_with_hook
               at src/libstd/panicking.rs:482
     6: std::panicking::begin_panic
     7: <rustc_codegen_llvm::back::wasm::WasmSections<'a> as core::iter::iterator::Iterator>::next
     8: rustc_codegen_llvm::back::wasm::rewrite_imports
     9: rustc_codegen_llvm::back::link::link_natively
    10: rustc_codegen_llvm::back::link::link_binary
    11: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::join_codegen_and_link::{{closure}}
    12: rustc::util::common::time
    13: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::join_codegen_and_link
    14: rustc_driver::driver::compile_input
    15: <scoped_tls::ScopedKey<T>>::set
    16: rustc_driver::run_compiler
    17: <scoped_tls::ScopedKey<T>>::set
  query stack during panic:
  end of query stack
  
  error: internal compiler error: unexpected panic
  
  note: the compiler unexpectedly panicked. this is a bug.
  
  note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
  
  note: rustc 1.33.0-nightly (2fadb0a16 2019-01-13) running on x86_64-unknown-linux-gnu
  
  note: compiler flags: -C opt-level=3 -C debuginfo=2 --crate-type cdylib
  
  note: some of the compiler flags provided by cargo are hidden
  
  error: Could not compile `client-wasm`.
  
| To learn more, run the command again with --verbose.
Error: Compiling your crate to WebAssembly failed
Caused by: failed to execute `cargo build`: exited with exit code: 101
