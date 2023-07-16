rust
$ RUST_BACKTRACE=1 /home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc -Z treat-err-as-bug=1 ./c.rs 
error[E0507]: cannot move out of `*array` which is behind a shared reference
  --> ./c.rs:16:13
   |
16 |             *array
   |             ^^^^^^
   |             |
   |             move occurs because `*array` has type `std::vec::Vec<Value>`, which does not implement the `Copy` trait
   |             help: consider borrowing here: `&*array`

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', src/librustc_errors/lib.rs:941:13
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
             at src/libcore/fmt/mod.rs:1028
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
  13: rustc_errors::HandlerInner::emit_diagnostic
  14: rustc_errors::diagnostic_builder::DiagnosticBuilder::into_diagnostic
  15: rustc_errors::diagnostic_builder::DiagnosticBuilder::buffer
  16: rustc_mir::borrow_check::move_errors::<impl rustc_mir::borrow_check::MirBorrowckCtxt>::report
  17: rustc_mir::borrow_check::do_mir_borrowck
  18: rustc::ty::context::GlobalCtxt::enter_local
  19: rustc_mir::borrow_check::mir_borrowck
  20: rustc::ty::query::__query_compute::mir_borrowck
  21: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute
  22: rustc::dep_graph::graph::DepGraph::with_task_impl
  23: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  24: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  25: rustc::util::common::time
  26: rustc_interface::passes::analysis
  27: rustc::ty::query::__query_compute::analysis
  28: rustc::dep_graph::graph::DepGraph::with_task_impl
  29: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  30: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  31: rustc_interface::passes::create_global_ctxt::{{closure}}
  32: rustc_interface::interface::run_compiler_in_existing_thread_pool
  33: std::thread::local::LocalKey<T>::with
  34: scoped_tls::ScopedKey<T>::set
  35: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (22bc9e1d9 2019-09-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z treat-err-as-bug=1

query stack during panic:
#0 [mir_borrowck] processing `foo`
#1 [analysis] running analysis passes on this crate
end of query stack
