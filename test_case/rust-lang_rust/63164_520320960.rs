none
thread 'rustc' panicked at 'not a module', src/librustc/hir/map/mod.rs:528:18
stack backtrace:
   0:     0x7f0ad7d6c61b - backtrace::backtrace::libunwind::trace::ha0c5be6d84ff3577
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/libunwind.rs:88
   1:     0x7f0ad7d6c61b - backtrace::backtrace::trace_unsynchronized::h0140f1905fe5df01
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/mod.rs:66
   2:     0x7f0ad7d6c61b - std::sys_common::backtrace::_print::h71c90b55c1552895
                               at src/libstd/sys_common/backtrace.rs:47
   3:     0x7f0ad7d6c61b - std::sys_common::backtrace::print::h0d9ae497e80a66d3
                               at src/libstd/sys_common/backtrace.rs:36
   4:     0x7f0ad7d6c61b - std::panicking::default_hook::{{closure}}::h0f874d2b67342285
                               at src/libstd/panicking.rs:200
   5:     0x7f0ad7d6c2f6 - std::panicking::default_hook::hb16f7323e318d7e4
                               at src/libstd/panicking.rs:214
   6:     0x7f0ad9a864a1 - rustc::util::common::panic_hook::h429013132a47c4fc
   7:     0x7f0ad7d6ce76 - std::panicking::rust_panic_with_hook::h399c25a83b2171e5
                               at src/libstd/panicking.rs:481
   8:     0x7f0ad9aec655 - std::panicking::begin_panic::he315ffb87476d743
   9:     0x7f0ad97341e9 - rustc::hir::map::Map::get_module::h1210e98f2c905b81
  10:     0x7f0ad8796c55 - rustc_privacy::EmbargoVisitor::update_macro_reachable::h03ea7b13a3744e67
  11:     0x7f0ad8798b18 - <rustc_privacy::EmbargoVisitor as rustc::hir::intravisit::Visitor>::visit_macro_def::hacc66c1d53288339
  12:     0x7f0ad879dcd9 - rustc_privacy::privacy_access_levels::h2f299a855f11d2ea
  13:     0x7f0ad83f4995 - rustc::ty::query::__query_compute::privacy_access_levels::h0a7b4c7f05bc2167
  14:     0x7f0ad83f719c - rustc::dep_graph::graph::DepGraph::with_task_impl::h4453007938f8cfe3
  15:     0x7f0ad83e61c7 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hc392c801ffb5ed4c
  16:     0x7f0ad838d59e - rustc::util::common::time::h15d9e7ce0076925b
  17:     0x7f0ad82fa5db - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h12546751fd40c726
  18:     0x7f0ad7d7d70a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:80
  19:     0x7f0ad8392f62 - rustc_interface::passes::analysis::{{closure}}::h8734b7d05d191413
  20:     0x7f0ad838c820 - rustc::util::common::time::h02a48a90970b1bac
  21:     0x7f0ad8349b14 - rustc_interface::passes::analysis::h57ebb0969e450b3d
  22:     0x7f0ad8233f31 - rustc::ty::query::__query_compute::analysis::h6c096f0d36ed86f6
  23:     0x7f0ad82353b9 - rustc::dep_graph::graph::DepGraph::with_task_impl::h2550ad6665d41399
  24:     0x7f0ad824795e - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hc5055ce71170472f
  25:     0x7f0ad825509a - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::h4786e32a4ca81311
  26:     0x7f0ad83c351a - rustc_interface::passes::create_global_ctxt::{{closure}}::h597429a0e6926f58
  27:     0x7f0ad8257aaf - rustc_interface::interface::run_compiler_in_existing_thread_pool::hff30cd26e49be082
  28:     0x7f0ad82904f2 - std::thread::local::LocalKey<T>::with::hd8d9ab6c3632b928
  29:     0x7f0ad82a6a00 - syntax::with_globals::hab02661e6f94b740
  30:     0x7f0ad8228a62 - std::sys_common::backtrace::__rust_begin_short_backtrace::h142cd21c5b6016af
  31:     0x7f0ad7d7d70a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:80
  32:     0x7f0ad8259029 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hc6e3bc11b5dabea5
  33:     0x7f0ad7d502ff - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hc06a52252b8da4d8
                               at /rustc/00ee1b47f42129a0a6e33510578fbcf07c1e5382/src/liballoc/boxed.rs:787
  34:     0x7f0ad7d7c390 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::ha593cf28b5c6f358
                               at /rustc/00ee1b47f42129a0a6e33510578fbcf07c1e5382/src/liballoc/boxed.rs:787
  35:     0x7f0ad7d7c390 - std::sys_common::thread::start_thread::h0e1c86ca3f5e2a0a
                               at src/libstd/sys_common/thread.rs:13
  36:     0x7f0ad7d7c390 - std::sys::unix::thread::Thread::new::thread_start::h2c0daa2b9405abac
                               at src/libstd/sys/unix/thread.rs:79
  37:     0x7f0ad7cbb75a - start_thread
  38:     0x7f0ad7bd09f3 - __clone
  39:                0x0 - <unknown>
query stack during panic:
#0 [privacy_access_levels] privacy access levels
#1 [analysis] running analysis passes on this crate
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.38.0-nightly (00ee1b47f 2019-08-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin
