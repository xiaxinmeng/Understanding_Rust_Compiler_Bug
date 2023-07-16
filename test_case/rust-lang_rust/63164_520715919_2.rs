text
thread 'rustc' panicked at 'internal error: entered unreachable code', src/librustc_privacy/lib.rs:767:17
stack backtrace:
   0:     0x7fe45d04600b - backtrace::backtrace::libunwind::trace::h52f24a95439ef578
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/libunwind.rs:88
   1:     0x7fe45d04600b - backtrace::backtrace::trace_unsynchronized::h95d5121d267c42e7
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/mod.rs:66
   2:     0x7fe45d04600b - std::sys_common::backtrace::_print::hb84c1e80b3c39e3b
                               at src/libstd/sys_common/backtrace.rs:47
   3:     0x7fe45d04600b - std::sys_common::backtrace::print::hf6f56322692db8bf
                               at src/libstd/sys_common/backtrace.rs:36
   4:     0x7fe45d04600b - std::panicking::default_hook::{{closure}}::h19298f991e34e518
                               at src/libstd/panicking.rs:200
   5:     0x7fe45d045ce6 - std::panicking::default_hook::h24d4b216d9b958a7
                               at src/libstd/panicking.rs:214
   6:     0x7fe45ed242e1 - rustc::util::common::panic_hook::h1dd7f28a1964c2cb
   7:     0x7fe45d046866 - std::panicking::rust_panic_with_hook::hec63884fa234b28d
                               at src/libstd/panicking.rs:481
   8:     0x7fe45da20d45 - std::panicking::begin_panic::h57aa041734b11738
   9:     0x7fe45da4e1b0 - <rustc_privacy::EmbargoVisitor as rustc::hir::intravisit::Visitor>::visit_macro_def::hc1da9eff917bcabe
  10:     0x7fe45da53229 - rustc_privacy::privacy_access_levels::h783d64b1d818203b
  11:     0x7fe45d6b5f95 - rustc::ty::query::__query_compute::privacy_access_levels::hd8b8fec48aa4f30a
  12:     0x7fe45d6ba6ec - rustc::dep_graph::graph::DepGraph::with_task_impl::ha23193b852ec19e8
  13:     0x7fe45d68f787 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h062ebe411627cf5a
  14:     0x7fe45d6637be - rustc::util::common::time::hce9d9246aef66790
  15:     0x7fe45d5bbb1b - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h89d28534075d9ced
  16:     0x7fe45d0570fa - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:80
  17:     0x7fe45d667ab2 - rustc_interface::passes::analysis::{{closure}}::h984f4e19f4d1fe61
  18:     0x7fe45d661330 - rustc::util::common::time::h0a2f2eb1d7acc77d
  19:     0x7fe45d60d484 - rustc_interface::passes::analysis::h33ebf750c675f89b
  20:     0x7fe45d4f68e1 - rustc::ty::query::__query_compute::analysis::h63934db633e843ec
  21:     0x7fe45d4f8399 - rustc::dep_graph::graph::DepGraph::with_task_impl::ha5757adb20b9ea3b
  22:     0x7fe45d505e0e - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h2d5f2f2a098a1b29
  23:     0x7fe45d51808a - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::hb6313b20433cdbc4
  24:     0x7fe45d66696a - rustc_interface::passes::create_global_ctxt::{{closure}}::hccd7c77555c50c54
  25:     0x7fe45d519434 - rustc_interface::interface::run_compiler_in_existing_thread_pool::h02ba1ec2154be072
  26:     0x7fe45d552152 - std::thread::local::LocalKey<T>::with::he42ca5b61e1e7359
  27:     0x7fe45d5682c0 - syntax::with_globals::h287fa2caf2b24706
  28:     0x7fe45d4ebe22 - std::sys_common::backtrace::__rust_begin_short_backtrace::h113d82c9ba2144b4
  29:     0x7fe45d0570fa - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:80
  30:     0x7fe45d51b939 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hcddb7194ffd10ee0
  31:     0x7fe45d029d8f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h82a57145aa4239a7
                               at /rustc/dddb7fca09dc817ba275602b950bb81a9032fb6d/src/liballoc/boxed.rs:770
  32:     0x7fe45d055d80 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h167c1ef971e93086
                               at /rustc/dddb7fca09dc817ba275602b950bb81a9032fb6d/src/liballoc/boxed.rs:770
  33:     0x7fe45d055d80 - std::sys_common::thread::start_thread::h739b9b99c7f25b24
                               at src/libstd/sys_common/thread.rs:13
  34:     0x7fe45d055d80 - std::sys::unix::thread::Thread::new::thread_start::h79a2f27ba62f96ae
                               at src/libstd/sys/unix/thread.rs:79
  35:     0x7fe45cf9575a - start_thread
  36:     0x7fe45ceaa9f3 - __clone
  37:                0x0 - <unknown>
query stack during panic:
#0 [privacy_access_levels] privacy access levels
#1 [analysis] running analysis passes on this crate
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.38.0-nightly (dddb7fca0 2019-07-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin
