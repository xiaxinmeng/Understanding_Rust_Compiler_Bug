
   Compiling playground v0.0.1 (/playground)
warning: unused variable: `assign`
 --> src/main.rs:5:9
  |
5 |     let assign: unsafe extern "rust-intrinsic" fn(*const i32) -> *mut i32 = transmute;
  |         ^^^^^^ help: consider prefixing with an underscore: `_assign`
  |
  = note: `#[warn(unused_variables)]` on by default

error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:722: intrinsic DefId(2:905 ~ core[463c]::intrinsics[0]::[1]::transmute[0]) being reified

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:646:9
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
             at src/libstd/panicking.rs:200
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:214
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:481
   8: std::panicking::begin_panic
   9: rustc_errors::Handler::bug
  10: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  11: rustc::ty::context::tls::with_opt::{{closure}}
  12: rustc::ty::context::tls::with_context_opt
  13: rustc::ty::context::tls::with_opt
  14: rustc::util::bug::opt_span_bug_fmt
  15: rustc::util::bug::bug_fmt
  16: rustc_mir::monomorphize::collector::visit_instance_use
  17: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc::mir::visit::Visitor>::visit_rvalue
  18: rustc_mir::monomorphize::collector::collect_items_rec
  19: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
  20: rustc::util::common::time
  21: rustc_mir::monomorphize::collector::collect_crate_mono_items
  22: rustc::util::common::time
  23: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  24: rustc::ty::query::__query_compute::collect_and_partition_mono_items
  25: rustc::dep_graph::graph::DepGraph::with_task_impl
  26: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  27: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  28: rustc::util::common::time
  29: rustc_interface::passes::start_codegen
  30: rustc::ty::context::tls::enter_global
  31: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  32: rustc_interface::passes::create_global_ctxt::{{closure}}
  33: rustc_interface::passes::BoxedGlobalCtxt::enter
  34: rustc_interface::queries::Query<T>::compute
  35: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  36: rustc_interface::interface::run_compiler_in_existing_thread_pool
  37: std::thread::local::LocalKey<T>::with
  38: scoped_tls::ScopedKey<T>::set
  39: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.38.0-nightly (890881f8f 2019-07-25) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
