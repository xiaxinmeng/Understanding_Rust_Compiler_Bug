
thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', /rustc/58b834344fc7b9185e7a50db1ff24e5eb07dae5e/src/libcore/slice/mod.rs:2791:10
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt               at src/libstd/sys_common/backtrace.rs:59
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
  11: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
             at /rustc/58b834344fc7b9185e7a50db1ff24e5eb07dae5e/src/liballoc/boxed.rs:1030
  12: proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}
             at /rustc/58b834344fc7b9185e7a50db1ff24e5eb07dae5e/src/libproc_macro/bridge/client.rs:305
  13: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:474
  14: rust_begin_unwind
             at src/libstd/panicking.rs:378
  15: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  16: core::panicking::panic_bounds_check
             at src/libcore/panicking.rs:63
  17: rustc_mir::borrow_check::type_check::type_check
  18: rustc_mir::borrow_check::nll::compute_regions
  19: rustc_mir::borrow_check::do_mir_borrowck
  20: rustc::ty::context::GlobalCtxt::enter_local
  21: rustc_mir::borrow_check::mir_borrowck
  22: rustc::ty::query::__query_compute::mir_borrowck
  23: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute
  24: rustc::dep_graph::graph::DepGraph::with_task_impl
  25: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  26: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  27: rustc_session::utils::<impl rustc_session::session::Session>::time
  28: rustc_interface::passes::analysis
  29: rustc::ty::query::__query_compute::analysis
  30: rustc::dep_graph::graph::DepGraph::with_task_impl
  31: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  32: rustc::ty::context::tls::enter_global
  33: rustc_interface::interface::run_compiler_in_existing_thread_pool
  34: scoped_tls::ScopedKey<T>::set
  35: syntax::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.  
error: internal compiler error: unexpected panic
