
warning: value assigned to `i` is never read
 --> src/main.rs:7:14
  |
7 |         3 => i = 1,
  |              ^
  |
  = note: `#[warn(unused_assignments)]` on by default
  = help: maybe it is overwritten before being read?

error: internal compiler error: src/librustc_codegen_llvm/context.rs:872: failed to get layout for `[type error]`: the type `[type error]` has an unknown layout

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:812:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:76
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:60
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1030
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:64
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:196
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:210
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:477
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc::ty::context::tls::with_opt::{{closure}}
  17: rustc::ty::context::tls::with_context_opt
  18: rustc::ty::context::tls::with_opt
  19: rustc::util::bug::opt_span_bug_fmt
  20: rustc::util::bug::bug_fmt
  21: <rustc_codegen_llvm::context::CodegenCx as rustc_target::abi::LayoutOf>::spanned_layout_of::{{closure}}
  22: <rustc_codegen_llvm::context::CodegenCx as rustc_target::abi::LayoutOf>::spanned_layout_of
  23: rustc_codegen_ssa::mir::analyze::non_ssa_locals
  24: rustc_codegen_ssa::mir::codegen_mir
  25: rustc_codegen_ssa::base::codegen_instance
  26: <rustc::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::define
  27: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  28: rustc::dep_graph::graph::DepGraph::with_task
  29: rustc_codegen_llvm::base::compile_codegen_unit
  30: rustc_codegen_ssa::base::codegen_crate
  31: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  32: rustc::util::common::time
  33: rustc_interface::passes::start_codegen
  34: rustc::ty::context::tls::enter_global
  35: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  36: rustc_interface::passes::create_global_ctxt::{{closure}}
  37: rustc_interface::passes::BoxedGlobalCtxt::enter
  38: rustc_interface::queries::Query<T>::compute
  39: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  40: rustc_interface::interface::run_compiler_in_existing_thread_pool
  41: std::thread::local::LocalKey<T>::with
  42: scoped_tls::ScopedKey<T>::set
  43: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0 (4560ea788 2019-11-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: aborting due to previous error

error: could not compile `playground`.

To learn more, run the command again with --verbose..
