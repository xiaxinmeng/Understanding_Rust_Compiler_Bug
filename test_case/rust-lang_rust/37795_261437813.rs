
error[E0428]: a value named `_IMPL_DESERIALIZE_FOR_Action` has already been defined in this module
  --> src/main.rs:15:39
   |
15 | #[derive(Debug, PartialEq, Serialize, Deserialize)]
   |                            ---------  ^^^^^^^^^^^ already defined
   |                            |
   |                            previous definition of `_IMPL_DESERIALIZE_FOR_Action` here

error[E0428]: a type named `Action` has already been defined in this module
  --> src/main.rs:16:1
   |
15 | #[derive(Debug, PartialEq, Serialize, Deserialize)]
   |                                       ----------- previous definition of `Action` here
16 | enum Action {
   | ^ already defined

error: internal compiler error: ../src/librustc_metadata/decoder.rs:490: entry: id not found: DefIndex(1) in crate "serde_derive" with number 12

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', ../src/librustc_errors/lib.rs:424
stack backtrace:
   1:     0x7f80fa72b17a - std::sys::imp::backtrace::tracing::imp::write::h944c02ac40aee2d7
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x7f80fa73a01f - std::panicking::default_hook::{{closure}}::h6875a2976258b020
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:247
   3:     0x7f80fa739bbd - std::panicking::default_hook::h88ffbc5922643264
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:257
   4:     0x7f80fa73a4c7 - std::panicking::rust_panic_with_hook::ha5aed1dfc0e220e3
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:451
   5:     0x7f80f321955a - std::panicking::begin_panic::h264cdc75d51b518b
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:413
   6:     0x7f80f322a11d - rustc_errors::Handler::bug::h620f7270292f0095
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_errors/lib.rs:424
   7:     0x7f80f7af8f21 - rustc::session::opt_span_bug_fmt::{{closure}}::h4a9b70c3df8b4b3a
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/session/mod.rs:771
   8:     0x7f80f7af8d3e - rustc::session::opt_span_bug_fmt::h7d83586c6e2c7ae6
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1048
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/session/mod.rs:767
   9:     0x7f80f7af89a2 - rustc::session::bug_fmt::he2d2f00a4afa9d1e
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/session/mod.rs:751
  10:     0x7f80f843f1f3 - rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::entry::h5e3d8114267e9116
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_metadata/decoder.rs:490
  11:     0x7f80f844780b - rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore<'tcx> for rustc_metadata::cstore::CStore>::visibility::h2949b511f18961ee
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_metadata/decoder.rs:654
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_metadata/cstore_impl.rs:58
  12:     0x7f80f844fc80 - rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore<'tcx> for rustc_metadata::cstore::CStore>::visible_parent_map::hab53861eb74bce8d
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_metadata/cstore_impl.rs:597
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_metadata/cstore_impl.rs:622
  13:     0x7f80f7b3e634 - rustc::ty::item_path::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::push_item_path::haa7f7a9afda5f72a
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/item_path.rs:115
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/item_path.rs:152
  14:     0x7f80f7b856e3 - rustc::util::ppaux::parameterized::h7bcfb1fd3d8a947a
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/item_path.rs:49
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/util/ppaux.rs:154
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1035
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/thread/local.rs:245
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1031
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/util/ppaux.rs:82
  15:     0x7f80fa78c635 - core::fmt::write::h01739b8f12f355f9
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libcore/fmt/mod.rs:829
  16:     0x7f80f7b285dd - rustc::traits::specialize::specialization_graph::Graph::insert::h62250bf0632d4218
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libcore/fmt/mod.rs:130
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libcollections/string.rs:1826
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/traits/specialize/specialization_graph.rs:125
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/infer/mod.rs:523
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1019
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/thread/local.rs:245
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1016
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:853
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/infer/mod.rs:523
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/traits/specialize/specialization_graph.rs:111
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/traits/specialize/specialization_graph.rs:224
  17:     0x7f80f7b583c2 - rustc::ty::trait_def::TraitDef::add_impl_for_specialization::h39896639aba068c7
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/trait_def.rs:202
  18:     0x7f80f8c3a0b7 - <rustc_typeck::coherence::overlap::OverlapChecker<'cx, 'tcx> as rustc::hir::intravisit::Visitor<'v>>::visit_item::h5ba64bfdace3da91
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_typeck/coherence/overlap.rs:143
  19:     0x7f80f8c4307b - rustc_typeck::coherence::check_coherence::h7b433ad2f4520d1a
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/dep_graph/visit.rs:46
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/hir/mod.rs:445
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/dep_graph/visit.rs:57
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/mod.rs:2703
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_typeck/coherence/overlap.rs:33
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_typeck/coherence/mod.rs:529
  20:     0x7f80f8c49cc5 - rustc_typeck::check_crate::h4045752b69a796e8
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_typeck/lib.rs:338
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/util/common.rs:38
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_typeck/lib.rs:337
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/session/mod.rs:222
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_typeck/lib.rs:336
  21:     0x7f80faad4c1e - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h97a3a12d948df547
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/driver.rs:891
  22:     0x7f80faad1ebd - rustc_driver::driver::phase_3_run_analysis_passes::hb0ad9de18d423e67
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1019
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/thread/local.rs:245
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1016
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1003
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/thread/local.rs:245
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:1000
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/ty/context.rs:789
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/driver.rs:869
  23:     0x7f80faab9644 - rustc_driver::driver::compile_input::h8e119234b60571d5
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/driver.rs:173
  24:     0x7f80faaff180 - rustc_driver::run_compiler::hbdfc4f84e2e0f4b9
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/lib.rs:222
  25:     0x7f80faa1a858 - std::panicking::try::do_call::hf679f17bf3b43b0b
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/lib.rs:1141
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/lib.rs:138
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/lib.rs:1075
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panic.rs:295
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:356
  26:     0x7f80fa744a0a - __rust_maybe_catch_panic
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libpanic_unwind/lib.rs:97
  27:     0x7f80faa3e418 - <F as alloc::boxed::FnBox<A>>::call_box::h506fb5d7b8891cd4
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:332
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panic.rs:351
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/thread/mod.rs:287
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/liballoc/boxed.rs:595
  28:     0x7f80fa738e94 - std::sys::imp::thread::Thread::new::thread_start::h8084b1107992ae5b
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/liballoc/boxed.rs:605
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/sys_common/thread.rs:21
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/sys/unix/thread.rs:84
  29:     0x7f80f29c5709 - start_thread
  30:     0x7f80fa3f382c - clone
  31:                0x0 - <unknown>

error: Could not compile `image-worker`.

To learn more, run the command again with --verbose.
