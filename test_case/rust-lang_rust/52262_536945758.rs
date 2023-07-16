rust
$ RUST_BACKTRACE=1 cargo build
!! nightly-x86_64-unknown-linux-gnu (default)
!! Executing '/home/user/.cargo/bin/cargo' in pwd='/tmp/blah' with args(1): 'build'
   Compiling blah v0.1.0 (/tmp/blah)
error[E0507]: cannot move out of `*key` which is behind a shared reference
  --> src/main.rs:15:35
   |
15 |                 String::from_utf8(*key).unwrap()
   |                                   ^^^^ move occurs because `*key` has type `std::vec::Vec<u8>`, which does not implement the `Copy` trait

error: internal compiler error: src/librustc_mir/borrow_check/mod.rs:1949: Accessing `(*_4)` with the kind `Write(Move)` shouldn't be possible
  --> src/main.rs:15:35
   |
15 |                 String::from_utf8(*key).unwrap()
   |                                   ^^^^

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:871:9
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
  13: rustc_errors::HandlerInner::span_bug
  14: rustc_errors::Handler::span_bug
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc::ty::context::tls::with_opt::{{closure}}
  17: rustc::ty::context::tls::with_context_opt
  18: rustc::ty::context::tls::with_opt
  19: rustc::util::bug::opt_span_bug_fmt
  20: rustc::util::bug::span_bug_fmt
  21: rustc_mir::borrow_check::MirBorrowckCtxt::access_place
  22: rustc_mir::borrow_check::MirBorrowckCtxt::consume_operand
  23: <rustc_mir::borrow_check::MirBorrowckCtxt as rustc_mir::dataflow::DataflowResultsConsumer>::visit_statement_entry
  24: rustc_mir::borrow_check::do_mir_borrowck
  25: rustc::ty::context::GlobalCtxt::enter_local
  26: rustc_mir::borrow_check::mir_borrowck
  27: rustc::ty::query::__query_compute::mir_borrowck
  28: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute
  29: rustc::dep_graph::graph::DepGraph::with_task_impl
  30: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  31: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  32: rustc::util::common::time
  33: rustc_interface::passes::analysis
  34: rustc::ty::query::__query_compute::analysis
  35: rustc::dep_graph::graph::DepGraph::with_task_impl
  36: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  37: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  38: rustc_interface::passes::create_global_ctxt::{{closure}}
  39: rustc_interface::interface::run_compiler_in_existing_thread_pool
  40: std::thread::local::LocalKey<T>::with
  41: scoped_tls::ScopedKey<T>::set
  42: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (22bc9e1d9 2019-09-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z treat-err-as-bug=5 -Z external-macro-backtrace -C debuginfo=2 -C incremental -C target-cpu=native --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] processing `<MyError as std::fmt::Display>::fmt`
#1 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0507`.
error: could not compile `blah`.

To learn more, run the command again with --verbose.
