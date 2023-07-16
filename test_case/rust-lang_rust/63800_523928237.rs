text
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: DistinctSources(DistinctSources { begin: (Real("src/lib.rs"), BytePos(0)), end: (Real("src/ext.rs"), BytePos(179)) })', src/libcore/result.rs:1084:5
stack backtrace:
   0:     0x7fd5a7664742 - backtrace::backtrace::libunwind::trace::h5b672ea0408452ed
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.35/src/backtrace/libunwind.rs:88
   1:     0x7fd5a7664742 - backtrace::backtrace::trace_unsynchronized::h15bbd54ccd30253c
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.35/src/backtrace/mod.rs:66
   2:     0x7fd5a7664742 - std::sys_common::backtrace::_print::hbf32a02f9894e727
                               at src/libstd/sys_common/backtrace.rs:47
   3:     0x7fd5a7664742 - std::sys_common::backtrace::print::h127d55b04a8ca638
                               at src/libstd/sys_common/backtrace.rs:36
   4:     0x7fd5a7664742 - std::panicking::default_hook::{{closure}}::h41569aa1cabd15f2
                               at src/libstd/panicking.rs:200
   5:     0x7fd5a7664426 - std::panicking::default_hook::h2471d95c3baf1435
                               at src/libstd/panicking.rs:214
   6:     0x7fd5a9520331 - rustc::util::common::panic_hook::h2022bda27bb76336
   7:     0x7fd5a7664f7c - std::panicking::rust_panic_with_hook::hdb74b1d76abfd8b2
                               at src/libstd/panicking.rs:481
   8:     0x7fd5a7664a32 - std::panicking::continue_panic_fmt::h7aad477695105344
                               at src/libstd/panicking.rs:384
   9:     0x7fd5a7664926 - rust_begin_unwind
                               at src/libstd/panicking.rs:311
  10:     0x7fd5a76917ba - core::panicking::panic_fmt::h2a7634fba4671b05
                               at src/libcore/panicking.rs:85
  11:     0x7fd5a76919f7 - core::result::unwrap_failed::h395c34a928fec133
                               at src/libcore/result.rs:1084
  12:     0x7fd5a84e91b5 - rustc_mir::borrow_check::move_errors::<impl rustc_mir::borrow_check::MirBorrowckCtxt>::report::h25a9cf486d5266a9
  13:     0x7fd5a84f1b4b - rustc_mir::borrow_check::do_mir_borrowck::hda4dab5518004986
  14:     0x7fd5a85ab923 - rustc::ty::context::GlobalCtxt::enter_local::h7429ee56447ea8af
  15:     0x7fd5a84f00de - rustc_mir::borrow_check::mir_borrowck::h85ba4c70317d3291
  16:     0x7fd5a7c04ac2 - rustc::ty::query::__query_compute::mir_borrowck::h98fb41102a77b5f0
  17:     0x7fd5a7cd1c3c - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute::h9802436be570eac8
  18:     0x7fd5a7cffcc4 - rustc::dep_graph::graph::DepGraph::with_task_impl::h72798b490d7dda9f
  19:     0x7fd5a7ce2b53 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h923529f6824dea4f
  20:     0x7fd5a7cd1b1d - rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners::h61fa4efa258d8006
  21:     0x7fd5a7ca152c - rustc::util::common::time::h3cf71e821181dc48
  22:     0x7fd5a7c486da - rustc_interface::passes::analysis::h9f50cf021540f6a4
  23:     0x7fd5a7b29291 - rustc::ty::query::__query_compute::analysis::he59e8eea93928c08
  24:     0x7fd5a7b39d1d - rustc::dep_graph::graph::DepGraph::with_task_impl::h6dea995610f354d4
  25:     0x7fd5a7b460de - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h68b2231d3424095f
  26:     0x7fd5a7b652a5 - rustc::ty::context::tls::enter_global::he96502a63341a2b8
  27:     0x7fd5a7b4c487 - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::h9e5505cac0dd4781
  28:     0x7fd5a7ca650a - rustc_interface::passes::create_global_ctxt::{{closure}}::h63fdbd27f3ec6fd7
  29:     0x7fd5a7b4e68e - rustc_interface::interface::run_compiler_in_existing_thread_pool::h9e1e785d8bd30ef2
  30:     0x7fd5a7b87412 - std::thread::local::LocalKey<T>::with::hd628004bbe9136f1
  31:     0x7fd5a7b8ef3e - scoped_tls::ScopedKey<T>::set::hb63f3a057333c317
  32:     0x7fd5a7bc07f2 - syntax::with_globals::he4a92d8220bc2e1b
  33:     0x7fd5a7b1ec52 - std::sys_common::backtrace::__rust_begin_short_backtrace::hd1bf2fb1e8e27b88
  34:     0x7fd5a767583a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:80
  35:     0x7fd5a7b50209 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h2a4f847e90febcd1
  36:     0x7fd5a764840f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h63427f5093f1aac0
                               at /rustc/e44fdf97929d1315add3b76208adf99e8299252d/src/liballoc/boxed.rs:922
  37:     0x7fd5a76744c0 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h398cbfcb305a969f
                               at /rustc/e44fdf97929d1315add3b76208adf99e8299252d/src/liballoc/boxed.rs:922
  38:     0x7fd5a76744c0 - std::sys_common::thread::start_thread::h55519c3a5f7d36e1
                               at src/libstd/sys_common/thread.rs:13
  39:     0x7fd5a76744c0 - std::sys::unix::thread::Thread::new::thread_start::hace5e416996b8e04
                               at src/libstd/sys/unix/thread.rs:79
  40:     0x7fd5a75b375a - start_thread
  41:     0x7fd5a74ca9f3 - __clone
  42:                0x0 - <unknown>
query stack during panic:
#0 [mir_borrowck] processing `fff`
#1 [analysis] running analysis passes on this crate
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0-nightly (e44fdf979 2019-08-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z polonius
