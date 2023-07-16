
stack backtrace:
   1:     0x7f5c7562ac2c - std::sys::imp::backtrace::tracing::imp::write::h2a972e172776bc73
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x7f5c75638f5e - std::panicking::default_hook::{{closure}}::h3350be9abe4c8496
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:351
   3:     0x7f5c75638b03 - std::panicking::default_hook::h255964940ef72e84
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:361
   4:     0x7f5c75639407 - std::panicking::rust_panic_with_hook::ha02a85ff57cdb669
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:555
   5:     0x7f5c75639244 - std::panicking::begin_panic::h3472a76248a19370
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:517
   6:     0x7f5c756391b9 - std::panicking::begin_panic_fmt::h353221386e3766a5
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:501
   7:     0x7f5c75639147 - rust_begin_unwind
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:477
   8:     0x7f5c7567508d - core::panicking::panic_fmt::h2a5bed77fd55efc9
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libcore/panicking.rs:69
   9:     0x7f5c75674fc4 - core::panicking::panic::he8f88aa6c62724a9
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libcore/panicking.rs:49
  10:     0x7f5c748b973a - rustc_trans::mir::block::<impl rustc_trans::mir::MirContext<'a, 'tcx>>::trans_block::h5e3ee07a178066d3
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libcore/macros.rs:21
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_trans/mir/block.rs:435
  11:     0x7f5c748b2461 - rustc_trans::mir::trans_mir::h176dedb049c42b8c
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_trans/mir/mod.rs:330
  12:     0x7f5c748d20e7 - rustc_trans::trans_item::TransItem::define::ha6a9e40b644acbed
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_trans/base.rs:603
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_trans/trans_item.rs:88
  13:     0x7f5c74857889 - rustc_trans::base::trans_crate::h235ed17a97b3d72c
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_trans/base.rs:1226
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/dep_graph/graph.rs:83
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_trans/base.rs:1224
  14:     0x7f5c759e831d - rustc_driver::driver::phase_4_translate_to_llvm::h0d14452d0ac15506
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/driver.rs:1037
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/util/common.rs:34
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/driver.rs:1035
  15:     0x7f5c759b7fb0 - rustc_driver::driver::compile_input::{{closure}}::h1e56c24d01668b7b
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/driver.rs:196
  16:     0x7f5c759d6db5 - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h21a6521e909907a7
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/driver.rs:983
  17:     0x7f5c759c6ded - rustc_driver::driver::phase_3_run_analysis_passes::h20919355616dc567
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:992
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/thread/local.rs:253
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:989
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:976
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/thread/local.rs:253
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:973
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:743
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/driver.rs:859
  18:     0x7f5c759b6111 - rustc_driver::driver::compile_input::hc831e6818a12286f
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/driver.rs:163
  19:     0x7f5c759fc7d8 - rustc_driver::run_compiler::ha4de160a9d909b83
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/lib.rs:221
  20:     0x7f5c75908d28 - std::panicking::try::do_call::h2ff0d6696d3add9d
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/lib.rs:1117
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/lib.rs:137
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/lib.rs:1051
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panic.rs:296
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:460
  21:     0x7f5c7564226a - __rust_maybe_catch_panic
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libpanic_unwind/lib.rs:98
  22:     0x7f5c75932e7b - <F as alloc::boxed::FnBox<A>>::call_box::h81baf0e61cbf35f2
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:436
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panic.rs:361
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/thread/mod.rs:302
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/liballoc/boxed.rs:605
  23:     0x7f5c75637dc4 - std::sys::imp::thread::Thread::new::thread_start::h27fa49fe81f658e4
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/liballoc/boxed.rs:615
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/sys_common/thread.rs:21
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/sys/unix/thread.rs:84
  24:     0x7f5c6d779453 - start_thread
  25:     0x7f5c752ff7de - __GI___clone
  26:                0x0 - <unknown>
