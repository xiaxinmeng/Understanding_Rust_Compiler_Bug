text
thread 'rustc' panicked at 'not a module', src/librustc/hir/map/mod.rs:528:18
stack backtrace:
   0:     0x7f99dcf3868b - backtrace::backtrace::libunwind::trace::h2577eef7cb82271e
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/libunwind.rs:88
   1:     0x7f99dcf3868b - backtrace::backtrace::trace_unsynchronized::ha9b72a77aa6c4ce9
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/mod.rs:66
   2:     0x7f99dcf3868b - std::sys_common::backtrace::_print::h8e2c1c5cda6e4e6e
                               at src/libstd/sys_common/backtrace.rs:47
   3:     0x7f99dcf3868b - std::sys_common::backtrace::print::h7e0f8aad0d8a6f3b
                               at src/libstd/sys_common/backtrace.rs:36
   4:     0x7f99dcf3868b - std::panicking::default_hook::{{closure}}::haae4327ede06a559
                               at src/libstd/panicking.rs:200
   5:     0x7f99dcf38366 - std::panicking::default_hook::hbf5f36093f15fccc
                               at src/libstd/panicking.rs:214
   6:     0x7f99deddf6c1 - rustc::util::common::panic_hook::h186d7378210995d5
   7:     0x7f99dcf38ee6 - std::panicking::rust_panic_with_hook::hf385e0a05e771036
                               at src/libstd/panicking.rs:481
   8:     0x7f99ded60535 - std::panicking::begin_panic::h98efcde0b33861fd
   9:     0x7f99dea1e8e9 - rustc::hir::map::Map::get_module::h8459c30767a59634
  10:     0x7f99dd976bd5 - rustc_privacy::EmbargoVisitor::update_macro_reachable::h6a3b276385f70843
  11:     0x7f99dd978ae5 - <rustc_privacy::EmbargoVisitor as rustc::hir::intravisit::Visitor>::visit_macro_def::hcbafa5fdfe7ec278
  12:     0x7f99dd97da19 - rustc_privacy::privacy_access_levels::hab5fb8ebf6879a00
  13:     0x7f99dd5c9875 - rustc::ty::query::__query_compute::privacy_access_levels::h997d37146502020c
  14:     0x7f99dd5ce5cc - rustc::dep_graph::graph::DepGraph::with_task_impl::h5b324a86c2bc7b54
  15:     0x7f99dd5baad7 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hb10373db1d5669d1
  16:     0x7f99dd55d12e - rustc::util::common::time::h48c99f483b081dea
  17:     0x7f99dd4c5edb - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h9e5808ec22ae1930
  18:     0x7f99dcf497da - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:80
  19:     0x7f99dd562182 - rustc_interface::passes::analysis::{{closure}}::h054b9f0f0c77f34b
  20:     0x7f99dd55d8e0 - rustc::util::common::time::h8409e1576e8c0d89
  21:     0x7f99dd518954 - rustc_interface::passes::analysis::heb3974148e32c83a
  22:     0x7f99dd3fe941 - rustc::ty::query::__query_compute::analysis::h83791fe1f4d00f5f
  23:     0x7f99dd3ffe99 - rustc::dep_graph::graph::DepGraph::with_task_impl::h28a3f59ca4aea2c5
  24:     0x7f99dd40e09e - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h598f4a5ef7d98b4a
  25:     0x7f99dd41f97a - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::h7aa52f166037c173
  26:     0x7f99dd59888a - rustc_interface::passes::create_global_ctxt::{{closure}}::h970097f89991677c
  27:     0x7f99dd42142f - rustc_interface::interface::run_compiler_in_existing_thread_pool::h2009c7257c3c167e
  28:     0x7f99dd45b302 - std::thread::local::LocalKey<T>::with::h5f3bc545c05a19af
  29:     0x7f99dd471ae0 - syntax::with_globals::h55d1d6dcdd17adef
  30:     0x7f99dd3f37c2 - std::sys_common::backtrace::__rust_begin_short_backtrace::hfa7060075d41bdcb
  31:     0x7f99dcf497da - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:80
  32:     0x7f99dd423c09 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hc822505d1ebef265
  33:     0x7f99dcf1c34f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hcb726170bbd77169
                               at /rustc/60960a260f7b5c695fd0717311d72ce62dd4eb43/src/liballoc/boxed.rs:787
  34:     0x7f99dcf48460 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h8f9a27f02cc198cd
                               at /rustc/60960a260f7b5c695fd0717311d72ce62dd4eb43/src/liballoc/boxed.rs:787
  35:     0x7f99dcf48460 - std::sys_common::thread::start_thread::h7b733f0fbf8d0251
                               at src/libstd/sys_common/thread.rs:13
  36:     0x7f99dcf48460 - std::sys::unix::thread::Thread::new::thread_start::h69a20ebd2130c891
                               at src/libstd/sys/unix/thread.rs:79
  37:     0x7f99dce8775a - start_thread
  38:     0x7f99dcd9c9f3 - __clone
  39:                0x0 - <unknown>
query stack during panic:
#0 [privacy_access_levels] privacy access levels
#1 [analysis] running analysis passes on this crate
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.38.0-nightly (60960a260 2019-08-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin
