
   Compiling playground v0.0.1 (/playground)
warning: the feature `const_generics` is incomplete and may cause the compiler to crash
 --> src/main.rs:1:12
  |
1 | #![feature(const_generics)]
  |            ^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default

error: internal compiler error: src/librustc_codegen_llvm/context.rs:905: failed to get layout for `[type error]`: the type `[type error]` has an unknown layout

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:883:9
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
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc::ty::context::tls::with_opt::{{closure}}
  17: rustc::ty::context::tls::with_opt
  18: rustc::util::bug::opt_span_bug_fmt
  19: rustc::util::bug::bug_fmt
  20: <rustc_codegen_llvm::context::CodegenCx as rustc_target::abi::LayoutOf>::spanned_layout_of::{{closure}}
  21: <rustc_codegen_llvm::context::CodegenCx as rustc_target::abi::LayoutOf>::spanned_layout_of
  22: rustc_codegen_ssa::mir::analyze::non_ssa_locals
  23: rustc_codegen_ssa::mir::codegen_mir
  24: <rustc::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::define
  25: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  26: rustc::dep_graph::graph::DepGraph::with_task
  27: rustc_codegen_llvm::base::compile_codegen_unit
  28: rustc_codegen_ssa::base::codegen_crate
  29: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  30: rustc_session::utils::<impl rustc_session::session::Session>::time
  31: rustc_interface::passes::QueryContext::enter
  32: rustc_interface::queries::Queries::ongoing_codegen
  33: rustc_interface::interface::run_compiler_in_existing_thread_pool
  34: scoped_tls::ScopedKey<T>::set
  35: syntax::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.43.0-nightly (07a34df18 2020-02-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: aborting due to previous error

error: could not compile `playground`.

To learn more, run the command again with --verbose.
