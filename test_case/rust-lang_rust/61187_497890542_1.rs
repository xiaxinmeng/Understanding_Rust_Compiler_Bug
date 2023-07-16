
   Compiling playground v0.0.1 (/playground)
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:347:21
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.25/src/backtrace/libunwind.rs:97
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.25/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:197
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:211
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:478
   8: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:381
   9: rust_begin_unwind
             at src/libstd/panicking.rs:308
  10: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  11: core::panicking::panic
             at src/libcore/panicking.rs:49
  12: rustc_mir::borrow_check::mutability_errors::<impl rustc_mir::borrow_check::MirBorrowckCtxt>::report_mutability_error
  13: rustc_mir::borrow_check::MirBorrowckCtxt::access_place
  14: <rustc_mir::borrow_check::MirBorrowckCtxt as rustc_mir::dataflow::DataflowResultsConsumer>::visit_statement_entry
  15: rustc_mir::borrow_check::do_mir_borrowck
  16: rustc::ty::context::GlobalCtxt::enter_local
  17: rustc_mir::borrow_check::mir_borrowck
  18: rustc::ty::query::__query_compute::mir_borrowck
  19: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute
  20: rustc::dep_graph::graph::DepGraph::with_task_impl
  21: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  22: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  23: rustc::util::common::time
  24: rustc_interface::passes::analysis
  25: rustc::ty::query::__query_compute::analysis
  26: rustc::dep_graph::graph::DepGraph::with_task_impl
  27: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  28: rustc::ty::context::tls::enter_global
  29: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  30: rustc_interface::passes::create_global_ctxt::{{closure}}
  31: rustc_interface::interface::run_compiler_in_existing_thread_pool
  32: std::thread::local::LocalKey<T>::with
  33: scoped_tls::ScopedKey<T>::set
  34: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [mir_borrowck] processing `frob::{{closure}}#0`
#1 [analysis] running analysis passes on this crate
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.37.0-nightly (3ade426ed 2019-05-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
