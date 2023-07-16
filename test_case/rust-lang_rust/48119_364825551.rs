
thread 'rustc' panicked at 'assertion failed: should_monomorphize_locally(tcx, &instance)', src/librustc_mir/monomorphize/collector.rs:387:13
stack backtrace:
   0:     0x7f1ad282769b - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::ha625d7cdd67edd3b
                               at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7f1ad283642b - std::sys_common::backtrace::print::h0f70153d9958dae4
                               at src/libstd/sys_common/backtrace.rs:68
                               at src/libstd/sys_common/backtrace.rs:57
   2:     0x7f1ad280fec0 - std::panicking::default_hook::{{closure}}::hb6271c6134cfe096
                               at src/libstd/panicking.rs:381
   3:     0x7f1ad280f95d - std::panicking::default_hook::h668eaa93444bc252
                               at src/libstd/panicking.rs:391
   4:     0x7f1ad2810391 - std::panicking::rust_panic_with_hook::ha542e0b34394aacb
                               at src/libstd/panicking.rs:577
   5:     0x7f1ad04ac181 - std::panicking::begin_panic::h5020ce10476f793c
                               at /mnt/data/projects/rust/src/libstd/panicking.rs:538
   6:     0x7f1ad0584142 - rustc_mir::monomorphize::collector::collect_items_rec::h0367a64cc154aef1
                               at src/librustc_mir/monomorphize/collector.rs:387
   7:     0x7f1ad058322f - rustc_mir::monomorphize::collector::collect_crate_mono_items::h146c77803cd3cc42
                               at src/librustc_mir/monomorphize/collector.rs:312
   8:     0x7f1ad10c5805 - rustc::util::common::time::h45caa124e73abc67
                               at src/librustc_trans/base.rs:1029
                               at /mnt/data/projects/rust/src/librustc/util/common.rs:120
   9:     0x7f1ad0fde04f - rustc_trans::base::collect_and_partition_translation_items::h01c5fabb6d136987
                               at src/librustc_trans/base.rs:1028
  10:     0x7f1ace577804 - rustc::dep_graph::graph::DepGraph::with_task_impl::h2ff9231b0dc3cc6a
                               at src/librustc/dep_graph/graph.rs:281
  11:     0x7f1ace21713e - rustc_errors::Handler::track_diagnostics::h4347a7efb6f8547c
                               at src/librustc/dep_graph/graph.rs:191
                               at src/librustc/ty/maps/plumbing.rs:492
                               at /mnt/data/projects/rust/src/librustc_errors/lib.rs:567
  12:     0x7f1ace123568 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check::hdcb69a55c6b9e510
                               at src/librustc/ty/maps/plumbing.rs:485
                               at src/librustc/ty/maps/plumbing.rs:121
  13:     0x7f1ace6c6bfc - rustc::ty::maps::<impl rustc::ty::maps::queries::collect_and_partition_translation_items<'tcx>>::force::hbf2b8db2a9e35a44
                               at src/librustc/ty/maps/plumbing.rs:484
  14:     0x7f1ace6c7621 - rustc::ty::maps::<impl rustc::ty::maps::queries::collect_and_partition_translation_items<'tcx>>::try_get::h5958425409703470
                               at src/librustc/ty/maps/plumbing.rs:301
                               at src/librustc/ty/maps/plumbing.rs:526
  15:     0x7f1ace42a55f - rustc::ty::maps::TyCtxtAt::collect_and_partition_translation_items::h96449aa0f8a93fd1
                               at src/librustc/ty/maps/plumbing.rs:565
  16:     0x7f1ace14a4e1 - rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::collect_and_partition_translation_items::h6bb2ef04aa7b3299
                               at src/librustc/ty/maps/plumbing.rs:558
  17:     0x7f1ad0fdc0c4 - rustc_trans::base::trans_crate::h76237babf0b0cc6b
                               at src/librustc_trans/base.rs:773
  18:     0x7f1ad1078e2b - <rustc_trans::LlvmTransCrate as rustc_trans_utils::trans_crate::TransCrate>::trans_crate::h2dac86ac79b30dc3
                               at src/librustc_trans/lib.rs:179
  19:     0x7f1ad2c577d9 - rustc_driver::driver::phase_4_translate_to_llvm::h4209b19475db74ec
                               at src/librustc_driver/driver.rs:1118
                               at /mnt/data/projects/rust/src/librustc/util/common.rs:120
                               at src/librustc_driver/driver.rs:1117
  20:     0x7f1ad2ca4a5b - rustc_driver::driver::compile_input::{{closure}}::h08a5c72bfe678b46
                               at src/librustc_driver/driver.rs:261
  21:     0x7f1ad2ca0631 - <std::thread::local::LocalKey<T>>::with::heb7e5b686976b067
                               at src/librustc_driver/driver.rs:1101
                               at /mnt/data/projects/rust/src/librustc/ty/context.rs:1573
                               at /mnt/data/projects/rust/src/libstd/thread/local.rs:377
                               at /mnt/data/projects/rust/src/libstd/thread/local.rs:288
  22:     0x7f1ad2c9b2fc - <std::thread::local::LocalKey<T>>::with::h5359e8b947e56e00
                               at /mnt/data/projects/rust/src/librustc/ty/context.rs:1570
                               at /mnt/data/projects/rust/src/librustc/ty/context.rs:1557
                               at /mnt/data/projects/rust/src/libstd/thread/local.rs:377
                               at /mnt/data/projects/rust/src/libstd/thread/local.rs:288
  23:     0x7f1ad2cf4371 - rustc::ty::context::TyCtxt::create_and_enter::hc7fe0a092ffd0526
                               at /mnt/data/projects/rust/src/librustc/ty/context.rs:1554
                               at /mnt/data/projects/rust/src/librustc/ty/context.rs:1197
  24:     0x7f1ad2c4b9f4 - rustc_driver::driver::compile_input::h520a862dc8ba4be2
                               at src/librustc_driver/driver.rs:1024
                               at src/librustc_driver/driver.rs:224
  25:     0x7f1ad2cce60a - rustc_driver::run_compiler::h214301d8fc31edee
                               at src/librustc_driver/lib.rs:253
  26:     0x7f1ad2bffa95 - std::sys_common::backtrace::__rust_begin_short_backtrace::hfc03dfc03a0e5798
                               at src/librustc_driver/lib.rs:1302
                               at src/librustc_driver/lib.rs:132
                               at src/librustc_driver/lib.rs:1217
                               at /mnt/data/projects/rust/src/libstd/sys_common/backtrace.rs:133
  27:     0x7f1ad2863c8e - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:101
  28:     0x7f1ad2cd8db7 - std::panicking::try::hea7eb6b6f261821a
                               at /mnt/data/projects/rust/src/libstd/panicking.rs:459
  29:     0x7f1ad2bd4cf3 - <F as alloc::boxed::FnBox<A>>::call_box::h8826e38cc10e3a44
                               at /mnt/data/projects/rust/src/libstd/panic.rs:365
                               at /mnt/data/projects/rust/src/libstd/thread/mod.rs:405
                               at /mnt/data/projects/rust/src/liballoc/boxed.rs:817
  30:     0x7f1ad283a4ab - std::sys::unix::thread::Thread::new::thread_start::hc8fbc4cd2f0b78b9
                               at /mnt/data/projects/rust/src/liballoc/boxed.rs:827
                               at src/libstd/sys_common/thread.rs:24
                               at src/libstd/sys/unix/thread.rs:90
  31:     0x7f1acc3d0493 - start_thread
  32:     0x7f1ad2507afe - __clone
  33:                0x0 - <unknown>
