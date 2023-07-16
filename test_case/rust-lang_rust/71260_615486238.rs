
thread 'rustc' panicked at 'index out of bounds: the len is 2283 but the index is 32648', /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447/src/libcore/slice/mod.rs:2791:10
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
             at src/libstd/io/mod.rs:1426
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:204
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:224
  10: rustc_driver::report_ice
  11: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
             at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447/src/liballoc/boxed.rs:1029
  12: proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}
             at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447/src/libproc_macro/bridge/client.rs:305
  13: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:476
  14: rust_begin_unwind
             at src/libstd/panicking.rs:380
  15: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  16: core::panicking::panic_bounds_check
             at src/libcore/panicking.rs:63
  17: serialize::serialize::Encoder::emit_map
  18: serialize::serialize::Encoder::emit_struct
  19: <&T as serialize::serialize::Encodable>::encode
  20: rustc::ty::query::on_disk_cache::encode_query_results
  21: rustc_session::utils::<impl rustc_session::session::Session>::time
  22: rustc::dep_graph::graph::DepGraph::with_ignore
  23: rustc_data_structures::sync::join
  24: rustc::dep_graph::graph::DepGraph::with_ignore
  25: rustc_incremental::persist::save::save_dep_graph
  26: rustc_session::utils::<impl rustc_session::session::Session>::time
  27: rustc_codegen_ssa::base::finalize_tcx
  28: rustc_codegen_ssa::base::codegen_crate
  29: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  30: rustc_session::utils::<impl rustc_session::session::Session>::time
  31: rustc_interface::passes::QueryContext::enter
  32: rustc_interface::queries::Queries::ongoing_codegen
  33: rustc_interface::interface::run_compiler_in_existing_thread_pool
  34: scoped_tls::ScopedKey<T>::set
  35: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0 (b8cedc004 2020-03-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
