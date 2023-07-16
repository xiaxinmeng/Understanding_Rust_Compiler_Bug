rust
error: internal compiler error: /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/traits/specialize/mod.rs:97: When translating substitutions for specialization, the expected specializaiton failed to hold

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_errors/lib.rs:423
stack backtrace:
   1:      0x31f22813fbc - std::sys::imp::backtrace::tracing::imp::write::h9c41d2f69e5caabf
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:      0x31f2282243e - std::panicking::default_hook::{{closure}}::h1f61f3c769fffe7a
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:351
   3:      0x31f22821fe3 - std::panicking::default_hook::hd5bda4e453dfb4be
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:361
   4:      0x31f228228db - std::panicking::rust_panic_with_hook::hffbc74969c7b5d87
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:555
   5:      0x31f1aea09fa - std::panicking::begin_panic::h74ad4f050841c998
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:517
   6:      0x31f1aeb58bd - rustc_errors::Handler::bug::h7088c6afc4a008fb
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_errors/lib.rs:423
   7:      0x31f1fba6091 - rustc::session::opt_span_bug_fmt::{{closure}}::h8e30a0fda5e6d8ea
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/session/mod.rs:788
   8:      0x31f1fba5eae - rustc::session::opt_span_bug_fmt::h4fd6d943719c63ae
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:1003
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/session/mod.rs:784
   9:      0x31f1fba5b12 - rustc::session::bug_fmt::he01375c3004c5cfe
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/session/mod.rs:768
  10:      0x31f1fbd8ed8 - rustc::traits::specialize::translate_substs::h071bb45495291d7e
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/traits/specialize/mod.rs:97
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libcore/result.rs:714
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/traits/specialize/mod.rs:96
  11:      0x31f1fbc2427 - rustc::traits::project::confirm_select_candidate::h07fd59f837c7743e
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/traits/project.rs:1322
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/traits/project.rs:1108
  12:      0x31f1fbc0132 - rustc::traits::project::opt_normalize_projection_type::hd8a28bca86de1a97
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/traits/project.rs:1083
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/traits/project.rs:753
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/traits/project.rs:500
  13:      0x31f1fbbb316 - rustc::traits::project::project_and_unify_type::h09fb70f2ed0eadef
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/traits/project.rs:200
  14:      0x31f1fb1a70b - rustc::infer::InferCtxt::commit_if_ok::h05fc3ac42172c9d9
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/traits/project.rs:169
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/infer/mod.rs:889
  15:      0x31f1fbb8f7f - <rustc::traits::fulfill::FulfillProcessor<'a, 'b, 'gcx, 'tcx> as rustc_data_structures::obligation_forest::ObligationProcessor>::process_obligation::hbf3582fcaa2b4c03
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/traits/project.rs:164
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/traits/fulfill.rs:578
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/traits/fulfill.rs:392
  16:      0x31f1fa5e578 - <rustc_data_structures::obligation_forest::ObligationForest<O>>::process_obligations::h4872726ce8c21f13
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_data_structures/obligation_forest/mod.rs:305
  17:      0x31f1fbb787f - rustc::traits::fulfill::FulfillmentContext::select_where_possible::hd620afa6ef62ea0f
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/traits/fulfill.rs:343
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/traits/fulfill.rs:320
  18:      0x31f20cc3a2c - rustc_typeck::check::FnCtxt::select_obligations_where_possible::h832a1d17e92ede54
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:2303
  19:      0x31f20cc4c50 - rustc_typeck::check::FnCtxt::check_argument_types::h222b4f6520160c8e
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:2609
  20:      0x31f20cc43dd - rustc_typeck::check::FnCtxt::check_method_argument_types::hfcebfb40ae728216
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:2471
  21:      0x31f20cd2154 - rustc_typeck::check::FnCtxt::check_expr_kind::h7172316f2afc3db4
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:2892
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:3810
  22:      0x31f20ccf992 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h58cbaadb2bb31008
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:3450
  23:      0x31f20ce122d - rustc_typeck::check::FnCtxt::check_stmt::h5f0c409d0347ada0
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:2789
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:2793
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:4163
  24:      0x31f20ce162e - rustc_typeck::check::FnCtxt::check_block_with_expected::h5fabdeff5662cd10
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:4198
  25:      0x31f20ccfe15 - rustc_typeck::check::FnCtxt::check_expr_kind::h7172316f2afc3db4
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:3804
  26:      0x31f20ccf992 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h58cbaadb2bb31008
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:3450
  27:      0x31f20cb69be - rustc_typeck::check::check_fn::h7512874de533d1c6
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:2789
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:2783
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:2776
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:807
  28:      0x31f20cb5a45 - rustc_typeck::check::check_bare_fn::hff8cbad9cd9ca12c
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:675
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:501
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/infer/mod.rs:443
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:974
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/thread/local.rs:253
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:971
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:799
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/infer/mod.rs:443
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:501
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:665
  29:      0x31f20cb846d - rustc_typeck::check::check_item_body::h2a877e8ee0d16484
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:909
  30:      0x31f20cb2ef1 - rustc_typeck::check::check_item_bodies::hf9c5a615b1a27ce2
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:575
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/dep_graph/visit.rs:44
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/hir/mod.rs:457
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/dep_graph/visit.rs:75
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/mod.rs:2630
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:605
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/session/mod.rs:234
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/check/mod.rs:603
  31:      0x31f20d1fd8f - rustc_typeck::check_crate::h93455a3168487f9e
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/lib.rs:347
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/util/common.rs:48
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_typeck/lib.rs:347
  32:      0x31f22bc3801 - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::haccf5097991f72ff
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/driver.rs:896
  33:      0x31f22bb64dc - rustc_driver::driver::phase_3_run_analysis_passes::hc8affcb7a7c3c449
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:974
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/thread/local.rs:253
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:971
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:958
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/thread/local.rs:253
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:955
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:733
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/driver.rs:867
  34:      0x31f22ba0bd6 - rustc_driver::driver::compile_input::h44853ffed84a12cb
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/driver.rs:166
  35:      0x31f22be7558 - rustc_driver::run_compiler::hdc4bb0fcf7d0917a
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/lib.rs:221
  36:      0x31f22af4948 - std::panicking::try::do_call::ha583797d32a865bd
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/lib.rs:1119
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/lib.rs:137
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/lib.rs:1053
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panic.rs:296
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:460
  37:      0x31f2282b75a - __rust_maybe_catch_panic
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libpanic_unwind/lib.rs:98
  38:      0x31f22b1de1b - <F as alloc::boxed::FnBox<A>>::call_box::h6903719257a678be
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:436
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panic.rs:361
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/thread/mod.rs:357
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/liballoc/boxed.rs:605
  39:      0x31f22821294 - std::sys::imp::thread::Thread::new::thread_start::h76badbf9b0ecaf58
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/liballoc/boxed.rs:615
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/sys_common/thread.rs:21
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/sys/unix/thread.rs:84
  40:      0x31f1a64b453 - start_thread
  41:      0x31f224e57de - __GI___clone
  42:                0x0 - <unknown>
