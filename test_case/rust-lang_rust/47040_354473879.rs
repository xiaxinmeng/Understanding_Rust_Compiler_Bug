
error: internal compiler error: /checkout/src/librustc_metadata/cstore_impl.rs:131: get_optimized_mir: missing MIR for `DefId(13/0:8 ~ cortex_m_rt[3291]::lang_items[0]::start[0])`

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.24.0-nightly (77e189cd7 2017-12-28) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:504:9
stack backtrace:
   0:     0x7f2bb89b495b - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h58dc6be9d3517912
                               at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7f2bb89c354e - std::sys_common::backtrace::print::h29d8edc66d88ad72
                               at /checkout/src/libstd/sys_common/backtrace.rs:68
                               at /checkout/src/libstd/sys_common/backtrace.rs:57
   2:     0x7f2bb89a03e0 - std::panicking::default_hook::{{closure}}::hfc8c846278172641
                               at /checkout/src/libstd/panicking.rs:381
   3:     0x7f2bb899fea3 - std::panicking::default_hook::hba0a5113d76f0fa6
                               at /checkout/src/libstd/panicking.rs:391
   4:     0x7f2bb89a082b - std::panicking::rust_panic_with_hook::h8690e2643e90ef26
                               at /checkout/src/libstd/panicking.rs:577
   5:     0x7f2bb329af67 - std::panicking::begin_panic::h31285b5ad3eb8eaf
   6:     0x7f2bb32b5902 - rustc_errors::Handler::bug::hb4ef7d63515eff21
   7:     0x7f2bb485499f - <std::thread::local::LocalKey<T>>::with::h635a9a1ceefb00fe
   8:     0x7f2bb4c04c1e - rustc::ty::context::tls::with_opt::h62c3add116bd9642
   9:     0x7f2bb4781107 - rustc::session::opt_span_bug_fmt::h594c3e3c121b3419
  10:     0x7f2bb4781016 - rustc::session::bug_fmt::h0d1440e338bbde3b
  11:     0x7f2bb5ee5392 - rustc_metadata::cstore_impl::provide_extern::optimized_mir::h25d1820ce5ffc061
  12:     0x7f2bb4a975e8 - rustc::ty::maps::<impl rustc::ty::maps::queries::optimized_mir<'tcx>>::compute_result::hb17cf840f9f4cc25
  13:     0x7f2bb4a29069 - rustc::dep_graph::graph::DepGraph::with_task_impl::h94d90dae298db672
  14:     0x7f2bb471eb44 - rustc_errors::Handler::track_diagnostics::h661d6986824d5c9d
  15:     0x7f2bb46348c0 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check::hd2551a3bd0af1a7b
  16:     0x7f2bb4a9767e - rustc::ty::maps::<impl rustc::ty::maps::queries::optimized_mir<'tcx>>::force::hb6daafdcc2e05d69
  17:     0x7f2bb4a97c56 - rustc::ty::maps::<impl rustc::ty::maps::queries::optimized_mir<'tcx>>::try_get::hc746a2e57645cf36
  18:     0x7f2bb48e255a - rustc::ty::maps::TyCtxtAt::optimized_mir::h9710497377fc8934
  19:     0x7f2bb4654b2d - rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::instance_mir::ha6385a0aa2455c50
  20:     0x7f2bb6824300 - rustc_mir::monomorphize::collector::collect_items_rec::h67f6fdaf9ed9cb2b
  21:     0x7f2bb6823592 - rustc_mir::monomorphize::collector::collect_crate_mono_items::hf943e91c30a94df9
  22:     0x7f2bb72f097f - rustc::util::common::time::h379648ffbdb5d77f
  23:     0x7f2bb721aa90 - rustc_trans::base::collect_and_partition_translation_items::h1e7385a3d1cc61e4
  24:     0x7f2bb4a1667b - rustc::dep_graph::graph::DepGraph::with_task_impl::h557daa864f7c9d1c
  25:     0x7f2bb471cdd8 - rustc_errors::Handler::track_diagnostics::h5ce7f89b36296f3d
  26:     0x7f2bb4614f80 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check::h6a96c5b1186725a2
  27:     0x7f2bb4b1552f - rustc::ty::maps::<impl rustc::ty::maps::queries::collect_and_partition_translation_items<'tcx>>::force::h7f7499cd178b6b3d
  28:     0x7f2bb4b15ba9 - rustc::ty::maps::<impl rustc::ty::maps::queries::collect_and_partition_translation_items<'tcx>>::try_get::h8a6715c7eb0728b5
  29:     0x7f2bb48e7e47 - rustc::ty::maps::TyCtxtAt::collect_and_partition_translation_items::h8066b288e16cb5d6
  30:     0x7f2bb465aabe - rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::collect_and_partition_translation_items::h096e492b267cbab4
  31:     0x7f2bb7218a1e - rustc_trans::base::trans_crate::h7d473add94b2acd7
  32:     0x7f2bb72ac056 - <rustc_trans::LlvmTransCrate as rustc_trans_utils::trans_crate::TransCrate>::trans_crate::h9fc545c25180f5cb
  33:     0x7f2bb8da002d - rustc_driver::driver::phase_4_translate_to_llvm::h614a8de0fa0ddf86
  34:     0x7f2bb8de5bd6 - rustc_driver::driver::compile_input::{{closure}}::hc5b2c7178e0ac454
  35:     0x7f2bb8ddffaa - <std::thread::local::LocalKey<T>>::with::hbc92eac846a79418
  36:     0x7f2bb8ddf026 - <std::thread::local::LocalKey<T>>::with::haec87e0ca22b1895
  37:     0x7f2bb8e37b37 - rustc::ty::context::TyCtxt::create_and_enter::hd132f21621a44eea
  38:     0x7f2bb8d9566a - rustc_driver::driver::compile_input::h0e2327ce8f4bed35
  39:     0x7f2bb8e0a6d0 - rustc_driver::run_compiler::h2be3ed19babed634
  40:     0x7f2bb8d4fda1 - std::sys_common::backtrace::__rust_begin_short_backtrace::h07cdf0f93e07309f
  41:     0x7f2bb89ebbde - __rust_maybe_catch_panic
                               at /checkout/src/libpanic_unwind/lib.rs:101
  42:     0x7f2bb8d27792 - <F as alloc::boxed::FnBox<A>>::call_box::hccc20201a46480bd
  43:     0x7f2bb89c5e17 - std::sys_common::thread::start_thread::hce669877621f6bd9
                               at /checkout/src/liballoc/boxed.rs:827
                               at /checkout/src/libstd/sys_common/thread.rs:24
  44:     0x7f2bb89ca228 - std::sys::unix::thread::Thread::new::thread_start::h4be0de3ec68d7d44
                               at /checkout/src/libstd/sys/unix/thread.rs:90
  45:     0x7f2bb29e8618 - start_thread
  46:     0x7f2bb868a8be - __GI___clone
  47:                0x0 - <unknown>
