
    Checking tokio-fs v0.1.7
    Checking rayon v1.3.0
    Checking tokio v0.1.22
thread 'rustc' panicked at 'assertion failed: data.is_empty()', src/librustc_infer/infer/mod.rs:1291:9
stack backtrace:
    Checking hyper v0.12.25
    Checking jsonrpc-server-utils v14.0.5
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
  11: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
             at /rustc/3dbade652ed8ebac70f903e01f51cd92c4e4302c/src/liballoc/boxed.rs:1031
  12: proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}
             at /rustc/3dbade652ed8ebac70f903e01f51cd92c4e4302c/src/libproc_macro/bridge/client.rs:305
  13: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:474
  14: std::panicking::begin_panic
  15: rustc_infer::infer::InferCtxt::take_region_var_origins
  16: rustc_mir::borrow_check::nll::compute_regions
  17: rustc_mir::borrow_check::do_mir_borrowck
  18: rustc::ty::context::GlobalCtxt::enter_local
  19: rustc_mir::borrow_check::mir_borrowck
  20: rustc::ty::query::__query_compute::mir_borrowck
  21: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute
  22: rustc::dep_graph::graph::DepGraph::with_task_impl
  23: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  24: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  25: rustc_interface::passes::analysis
  26: rustc::ty::query::__query_compute::analysis
  27: rustc::dep_graph::graph::DepGraph::with_task_impl
  28: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  29: rustc::ty::context::tls::enter_global
  30: rustc_interface::interface::run_compiler_in_existing_thread_pool
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.43.0-nightly (3dbade652 2020-03-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] borrow-checking `<tls::InStreamListenerTls<Sub> as r#trait::InStreamListener<&mut [u8], &[u8]>>::raw_bind`
#1 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `in_stream`.
