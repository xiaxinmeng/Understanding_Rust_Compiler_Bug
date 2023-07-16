text
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/fdc0011561c6365c596dfd8fa1ef388162bc89c7/src/libcore/macros/mod.rs:15:40
stack backtrace:
   0:     0x7f6c56fc45d4 - backtrace::backtrace::libunwind::trace::hc586f95f659e6084
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1:     0x7f6c56fc45d4 - backtrace::backtrace::trace_unsynchronized::ha9827fdb593fd967
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2:     0x7f6c56fc45d4 - std::sys_common::backtrace::_print_fmt::h00c888c95e07165a
                               at src/libstd/sys_common/backtrace.rs:84
   3:     0x7f6c56fc45d4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8407ffb2d059bc74
                               at src/libstd/sys_common/backtrace.rs:61
   4:     0x7f6c56ffccec - core::fmt::write::h4165a12a3856465f
                               at src/libcore/fmt/mod.rs:1024
   5:     0x7f6c56fb8937 - std::io::Write::write_fmt::h499a0566ceaa0048
                               at src/libstd/io/mod.rs:1428
   6:     0x7f6c56fc8a7e - std::sys_common::backtrace::_print::h05fbb11587298e2b
                               at src/libstd/sys_common/backtrace.rs:65
   7:     0x7f6c56fc8a7e - std::sys_common::backtrace::print::h8021a3ed2b5ff07e
                               at src/libstd/sys_common/backtrace.rs:50
   8:     0x7f6c56fc8a7e - std::panicking::default_hook::{{closure}}::hd3a6326f5c6c149f
                               at src/libstd/panicking.rs:193
   9:     0x7f6c56fc8771 - std::panicking::default_hook::h7088fb00a0cb1faf
                               at src/libstd/panicking.rs:210
  10:     0x7f6c5750e833 - rustc_driver::report_ice::h7d5c1d6a7c4e3fb5
  11:     0x7f6c49f8c918 - <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call::hd4fe407a7a08ec81
                               at /rustc/fdc0011561c6365c596dfd8fa1ef388162bc89c7/src/liballoc/boxed.rs:983
  12:     0x7f6c49f6a814 - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::hb9b2f3bc1e19f0f3
                               at /rustc/fdc0011561c6365c596dfd8fa1ef388162bc89c7/src/libproc_macro/bridge/client.rs:305
  13:     0x7f6c56fc9230 - std::panicking::rust_panic_with_hook::h6b223bff7721d4c1
                               at src/libstd/panicking.rs:475
  14:     0x7f6c56fc8d0e - rust_begin_unwind
                               at src/libstd/panicking.rs:375
  15:     0x7f6c56ff678e - core::panicking::panic_fmt::hed15e18c2b62a00c
                               at src/libcore/panicking.rs:82
  16:     0x7f6c56ff66da - core::panicking::panic::h676e2cfc7b84ffe6
                               at src/libcore/panicking.rs:49
  17:     0x7f6c57ec3df6 - rustc_mir::borrow_check::conflict_errors::<impl rustc_mir::borrow_check::MirBorrowckCtxt>::report_use_of_moved_or_uninitialized::hee7349248e2bcffb
  18:     0x7f6c57ede704 - rustc_mir::borrow_check::MirBorrowckCtxt::check_if_path_or_subpath_is_moved::he52872991422c41b
  19:     0x7f6c57eddf47 - rustc_mir::borrow_check::MirBorrowckCtxt::consume_operand::h9209f248792f8178
  20:     0x7f6c57ed83d7 - <rustc_mir::borrow_check::MirBorrowckCtxt as rustc_mir::dataflow::DataflowResultsConsumer>::visit_statement_entry::h3716ddc9fad46cc7
  21:     0x7f6c57ed4e89 - rustc_mir::borrow_check::do_mir_borrowck::h820ef35edc5ac038
  22:     0x7f6c581c205a - rustc::ty::context::GlobalCtxt::enter_local::h1ed6b75232f132b5
  23:     0x7f6c57ed2cfa - rustc_mir::borrow_check::mir_borrowck::h45f028e130853501
  24:     0x7f6c57e70b23 - rustc::ty::query::__query_compute::mir_borrowck::h25976ba17001de12
  25:     0x7f6c58195b58 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hc4244918c0627345
  26:     0x7f6c582d4f07 - rustc_mir::borrow_check::nll::type_check::TypeChecker::check_rvalue::h2bf06f16cb29ad88
  27:     0x7f6c582d9607 - rustc_mir::borrow_check::nll::type_check::TypeChecker::typeck_mir::hb499a59c8246eb57
  28:     0x7f6c582ccfda - rustc_mir::borrow_check::nll::type_check::type_check::ha16b660f3db7b1f1
  29:     0x7f6c582105e5 - rustc_mir::borrow_check::nll::compute_regions::h0260c515d4faa652
  30:     0x7f6c57ed380b - rustc_mir::borrow_check::do_mir_borrowck::h820ef35edc5ac038
  31:     0x7f6c581c205a - rustc::ty::context::GlobalCtxt::enter_local::h1ed6b75232f132b5
  32:     0x7f6c57ed2cfa - rustc_mir::borrow_check::mir_borrowck::h45f028e130853501
  33:     0x7f6c57657523 - rustc::ty::query::__query_compute::mir_borrowck::h4ee3c16667cb0a75
  34:     0x7f6c5761d5fc - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute::hf9e1e7d649716be5
  35:     0x7f6c575d9cf1 - rustc::dep_graph::graph::DepGraph::with_task_impl::h95e619e376fed8f1
  36:     0x7f6c576392d0 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hb2c0f4fa5a8fab0b
  37:     0x7f6c5761d1b8 - rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners::hb5ec544f452a4999
  38:     0x7f6c576637c3 - rustc_interface::passes::analysis::h11e465c53db707c6
  39:     0x7f6c574a1fc2 - rustc::ty::query::__query_compute::analysis::h8209330e6c35c935
  40:     0x7f6c574f1049 - rustc::dep_graph::graph::DepGraph::with_task_impl::hc0ff0e0262a656d9
  41:     0x7f6c57517658 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h364920cc587f967c
  42:     0x7f6c574bf755 - rustc::ty::context::tls::enter_global::h6be5e193e6b6506c
  43:     0x7f6c574d550f - rustc_interface::interface::run_compiler_in_existing_thread_pool::h450b3e47e99b2a02
  44:     0x7f6c574a5b72 - std::thread::local::LocalKey<T>::with::hc064f9c463ba87b1
  45:     0x7f6c5749f68e - scoped_tls::ScopedKey<T>::set::he618e0867346e071
  46:     0x7f6c575199a4 - syntax::with_globals::h05656a832d1edc4c
  47:     0x7f6c5751bb00 - std::sys_common::backtrace::__rust_begin_short_backtrace::hb45a6b809630e3e0
  48:     0x7f6c56fd9d0a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:78
  49:     0x7f6c574b6469 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h0091d3870173b469
  50:     0x7f6c56faa88f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h9ef9eb6ec2ee6be0
                               at /rustc/fdc0011561c6365c596dfd8fa1ef388162bc89c7/src/liballoc/boxed.rs:969
  51:     0x7f6c56fd8730 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hc0fc46e9a64f076e
                               at /rustc/fdc0011561c6365c596dfd8fa1ef388162bc89c7/src/liballoc/boxed.rs:969
  52:     0x7f6c56fd8730 - std::sys_common::thread::start_thread::h4eee21a391e25c99
                               at src/libstd/sys_common/thread.rs:13
  53:     0x7f6c56fd8730 - std::sys::unix::thread::Thread::new::thread_start::h673f7c20aae94594
                               at src/libstd/sys/unix/thread.rs:80
  54:     0x7f6c56f17669 - start_thread
  55:     0x7f6c56e2c323 - clone
  56:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-nightly (fdc001156 2019-12-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] processing `site::auth::initiate_auth::InitiateAuth::<S>::try_post::{{closure}}#0`
#1 [mir_borrowck] processing `site::auth::initiate_auth::InitiateAuth::<S>::try_post`
#2 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `pixurs`.
