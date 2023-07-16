
error: internal compiler error: librustc/hir/def.rs:249: attempted .def_id() on invalid def: Err

thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:554:9
stack backtrace:
   0:     0x7fc54d88f07e - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h1b3a1496bdd47d5b
                               at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7fc54d866046 - std::sys_common::backtrace::print::h2f040662be69463e
                               at libstd/sys_common/backtrace.rs:71
                               at libstd/sys_common/backtrace.rs:59
   2:     0x7fc54d8960ed - std::panicking::default_hook::{{closure}}::h796d09e60146b74e
                               at libstd/panicking.rs:211
   3:     0x7fc54d895e60 - std::panicking::default_hook::h591ad3dc45e922d2
                               at libstd/panicking.rs:227
   4:     0x7fc54d89681c - std::panicking::rust_panic_with_hook::h704ba6b5b270af4d
                               at libstd/panicking.rs:511
   5:     0x7fc54e70621e - std::panicking::begin_panic::h2992736397ee35b4
   6:     0x7fc54e701961 - rustc_errors::Handler::bug::h554f2c448ef9c119
   7:     0x7fc54f58b1ac - rustc::session::opt_span_bug_fmt::{{closure}}::h2c6e2219b9bc9cfc
   8:     0x7fc54f644ee9 - rustc::ty::context::tls::with_opt::{{closure}}::hef18231c8fd1d270
   9:     0x7fc54f58c16f - rustc::ty::context::tls::with_context_opt::h8d5ef6a7c76a84a2
  10:     0x7fc54f644746 - rustc::ty::context::tls::with_opt::haf89db8d0634e2b3
  11:     0x7fc54f4f05c4 - rustc::session::opt_span_bug_fmt::heeba45021378ece2
  12:     0x7fc54f4f0536 - rustc::session::bug_fmt::h83c885c8d9e7b309
  13:     0x7fc54f673266 - rustc::hir::def::Def::def_id::h8d0261bb38bcfabf
  14:     0x7fc5523a64cc - rustc_privacy::ObsoleteVisiblePrivateTypesVisitor::path_is_private_type::h7c353bb46816e7d3
  15:     0x7fc5523929c3 - rustc::hir::intravisit::walk_ty::h73e89da7517b1fad
  16:     0x7fc5523929ea - rustc::hir::intravisit::walk_ty::h73e89da7517b1fad
  17:     0x7fc552398889 - rustc::hir::intravisit::walk_item::h91f8d2a1af24c66e
  18:     0x7fc552398a36 - rustc::hir::intravisit::walk_item::h91f8d2a1af24c66e
  19:     0x7fc5523a90d4 - rustc_privacy::privacy_access_levels::hd31f9a283676d277
  20:     0x7fc54f645a38 - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::privacy_access_levels<'tcx>>::compute::hd400623c3414d3e2
  21:     0x7fc54f4abb46 - rustc::dep_graph::graph::DepGraph::with_task_impl::ha5a5ab6b46058ac3
  22:     0x7fc54f619e66 - rustc::ty::context::tls::with_related_context::ha15b602b83ee2289
  23:     0x7fc54f73da32 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job::hbe6ac0c34ea931e3
  24:     0x7fc54f7faf86 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query::h9b8bc4da9a1e7848
  25:     0x7fc55360cf02 - rustc::util::common::time::h5cb1caf806803c79
                               at /checkout/src/librustc_driver/driver.rs:1257
                               at /checkout/src/librustc/util/common.rs:166
                               at /checkout/src/librustc/util/common.rs:160
  26:     0x7fc55348bbc6 - rustc::ty::context::tls::enter_context::h49bbe398e026ec5b
                               at /checkout/src/librustc_driver/driver.rs:1257
                               at /checkout/src/librustc/ty/context.rs:1861
                               at /checkout/src/librustc/ty/context.rs:1829
                               at /checkout/src/librustc/ty/context.rs:1768
                               at /checkout/src/librustc/ty/context.rs:1828
  27:     0x7fc553460e75 - <std::thread::local::LocalKey<T>>::with::h2023723a488f5b39
                               at /checkout/src/librustc/ty/context.rs:1860
                               at /checkout/src/librustc/ty/context.rs:1818
                               at /checkout/src/libstd/thread/local.rs:294
                               at /checkout/src/libstd/thread/local.rs:248
                               at /checkout/src/librustc/ty/context.rs:1810
                               at /checkout/src/libstd/thread/local.rs:294
                               at /checkout/src/libstd/thread/local.rs:248
  28:     0x7fc553657adb - rustc::ty::context::TyCtxt::create_and_enter::h1dc717f5ab9cfb3d
                               at /checkout/src/librustc/ty/context.rs:1802
                               at /checkout/src/librustc/ty/context.rs:1840
                               at /checkout/src/librustc/ty/context.rs:1185
  29:     0x7fc5534e277e - rustc_driver::driver::phase_3_run_analysis_passes::h96fb344e54968fd5
                               at /checkout/src/librustc_driver/driver.rs:1218
  30:     0x7fc5535c1453 - <scoped_tls::ScopedKey<T>>::set::h173cdac992fc7603
                               at librustdoc/core.rs:303
                               at /checkout/src/librustc_driver/driver.rs:73
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.1/src/lib.rs:155
  31:     0x7fc5535f5be2 - rustdoc::core::run_core::h8cb60a0859be166b
                               at /checkout/src/librustc_driver/driver.rs:72
                               at librustdoc/core.rs:231
  32:     0x7fc5535c26cf - <scoped_tls::ScopedKey<T>>::set::h958242fdc5235104
                               at librustdoc/lib.rs:645
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.1/src/lib.rs:155
                               at /checkout/src/libsyntax/lib.rs:98
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.1/src/lib.rs:155
  33:     0x7fc553610d0d - syntax::with_globals::ha81c059769a61bdd
                               at /checkout/src/libsyntax/lib.rs:97
  34:     0x7fc553460117 - std::sys_common::backtrace::__rust_begin_short_backtrace::haaad03c578aabb3a
                               at librustdoc/lib.rs:641
                               at /checkout/src/librustc_driver/lib.rs:1613
                               at /checkout/src/libstd/sys_common/backtrace.rs:136
  35:     0x7fc553481da5 - std::panicking::try::do_call::h42c4cb447982246d
                               at /checkout/src/libstd/thread/mod.rs:409
                               at /checkout/src/libstd/panic.rs:313
                               at /checkout/src/libstd/panicking.rs:310
  36:     0x7fc54d8a49e9 - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:105
  37:     0x7fc553453a80 - <F as alloc::boxed::FnBox<A>>::call_box::h00353a92fe7e9aaf
                               at /checkout/src/libstd/panicking.rs:289
                               at /checkout/src/libstd/panic.rs:397
                               at /checkout/src/libstd/thread/mod.rs:408
                               at /checkout/src/liballoc/boxed.rs:640
  38:     0x7fc54d89561a - std::sys_common::thread::start_thread::h9f808e1cb13674ec
                               at /checkout/src/liballoc/boxed.rs:650
                               at libstd/sys_common/thread.rs:24
  39:     0x7fc54d865015 - std::sys::unix::thread::Thread::new::thread_start::h668a95d388fe6493
                               at libstd/sys/unix/thread.rs:90
  40:     0x7fc54d5e76b9 - start_thread
  41:     0x7fc54d0f741c - clone
  42:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0-nightly (01cc982e9 2018-06-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z unstable-options

error: Could not document `zerodmg`.

Caused by:
  process didn't exit successfully: `rustdoc -Zunstable-options --edition=2018 --crate-name zerodmg src/main.rs -o /mnt/c/Users/_/0dmg/target/doc -L dependency=/mnt/c/Users/_/0dmg/target/debug/deps --extern futures=/mnt/c/Users/_/0dmg/target/debug/deps/libfutures-839041d02c22f16f.rmeta --extern hyper=/mnt/c/Users/_/0dmg/target/debug/deps/libhyper-59f22dbfdbf31d90.rmeta --extern image=/mnt/c/Users/_/0dmg/target/debug/deps/libimage-b5966e2d05b7365d.rmeta --extern rand=/mnt/c/Users/_/0dmg/target/debug/deps/librand-8290a2bbec565ae8.rmeta --extern tokio=/mnt/c/Users/_/0dmg/target/debug/deps/libtokio-1f3f1dff24a2c94b.rmeta --extern zerodmg_emulator=/mnt/c/Users/_/0dmg/target/debug/deps/libzerodmg_emulator-c82403191bdbc04c.rmeta --extern zerodmg_utils=/mnt/c/Users/_/0dmg/target/debug/deps/libzerodmg_utils-ae73f0f234e525dd.rmeta` (exit code: 101)
