rust
rustc 1.15.1 (021bd294c 2017-02-08)
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libcore/option.rs:323
stack backtrace:
   1:     0x7f9ac28863fa - std::sys::imp::backtrace::tracing::imp::write::h3188f035833a2635
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x7f9ac289475f - std::panicking::default_hook::{{closure}}::h6385b6959a2dd25b
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:349
   3:     0x7f9ac28942fd - std::panicking::default_hook::he4f3b61755d7fa95
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:359
   4:     0x7f9ac2894c07 - std::panicking::rust_panic_with_hook::hf00b8130f73095ec
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:553
   5:     0x7f9ac2894a44 - std::panicking::begin_panic::h6227f62cb2cdaeb4
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:515
   6:     0x7f9ac28949b9 - std::panicking::begin_panic_fmt::h173eadd80ae64bec
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:499
   7:     0x7f9ac2894947 - rust_begin_unwind
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:475
   8:     0x7f9ac28d023d - core::panicking::panic_fmt::h3b2d1e30090844ff
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libcore/panicking.rs:69
   9:     0x7f9ac28d0174 - core::panicking::panic::h2596388ccef1871c
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libcore/panicking.rs:49
  10:     0x7f9abfa77916 - rustc::cfg::construct::CFGBuilder::find_scope::h6a78449286f15d3c
  11:     0x7f9abfa75516 - rustc::cfg::construct::CFGBuilder::expr::h7246200667777f4f
  12:     0x7f9abfa7583d - rustc::cfg::construct::CFGBuilder::expr::h7246200667777f4f
  13:     0x7f9abfa75f9f - rustc::cfg::construct::CFGBuilder::expr::h7246200667777f4f
  14:     0x7f9abfa78bfa - rustc::cfg::CFG::new::hb9688f485b6ba99d
  15:     0x7f9ac1511416 - rustc_borrowck::borrowck::borrowck_fn::hba7e322cbfad105a
  16:     0x7f9ac150fdf9 - <rustc_borrowck::borrowck::BorrowckCtxt<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_fn::h788ea7541c494b7a
  17:     0x7f9ac14b7003 - rustc::hir::intravisit::walk_item::h61ea41b9a7b2d39d
  18:     0x7f9ac1510515 - rustc_borrowck::borrowck::check_crate::hab87168c428eeb48
  19:     0x7f9ac2c2aaa4 - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h16429aca33e82f30
  20:     0x7f9ac2c155fe - rustc_driver::driver::phase_3_run_analysis_passes::h1b30709031b4f637
  21:     0x7f9ac2c04775 - rustc_driver::driver::compile_input::h71ecf4df05846d25
  22:     0x7f9ac2c4db62 - rustc_driver::run_compiler::h180324a0694503f7
  23:     0x7f9ac2b6247b - std::panicking::try::do_call::ha2c776608407d6f3
  24:     0x7f9ac289d42a - __rust_maybe_catch_panic
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libpanic_unwind/lib.rs:98
  25:     0x7f9ac2b84021 - <F as alloc::boxed::FnBox<A>>::call_box::h08c51ed50888bb2f
  26:     0x7f9ac28935c4 - std::sys::imp::thread::Thread::new::thread_start::he018521f53b24939
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/liballoc/boxed.rs:615
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/sys_common/thread.rs:21
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/sys/unix/thread.rs:84
  27:     0x7f9abb6a66b9 - start_thread
  28:     0x7f9ac255082c - clone
  29:                0x0 - <unknown>
